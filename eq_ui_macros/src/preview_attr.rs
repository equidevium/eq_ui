//! `#[preview(...)]` attribute macro implementation.

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{ItemFn, LitStr, Result, Token};

use crate::codegen;
use crate::parse_props;

// ── Attribute parsing ────────────────────────────────────────────

/// Parsed contents of `#[preview(...)]`.
struct PreviewAttr {
    category: Ident,
    description: String,
    examples: Vec<(String, String)>,
    no_styles: bool,
    no_variant_gallery: bool,
    custom_demo: bool,
    custom_gallery: bool,
}

impl Parse for PreviewAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut category = None;
        let mut description = None;
        let mut examples = Vec::new();
        let mut no_styles = false;
        let mut no_variant_gallery = false;
        let mut custom_demo = false;
        let mut custom_gallery = false;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            match key.to_string().as_str() {
                "category" => {
                    input.parse::<Token![=]>()?;
                    category = Some(input.parse::<Ident>()?);
                }
                "description" => {
                    input.parse::<Token![=]>()?;
                    description = Some(input.parse::<LitStr>()?.value());
                }
                "examples" => {
                    input.parse::<Token![=]>()?;
                    let content;
                    syn::bracketed!(content in input);
                    while !content.is_empty() {
                        let inner;
                        syn::parenthesized!(inner in content);
                        let label: LitStr = inner.parse()?;
                        inner.parse::<Token![,]>()?;
                        let code: LitStr = inner.parse()?;
                        examples.push((label.value(), code.value()));
                        if content.peek(Token![,]) {
                            content.parse::<Token![,]>()?;
                        }
                    }
                }
                "no_styles" => {
                    no_styles = true;
                }
                "no_variant_gallery" => {
                    no_variant_gallery = true;
                }
                "custom_demo" => {
                    custom_demo = true;
                }
                "custom_gallery" => {
                    custom_gallery = true;
                }
                other => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!("unknown preview attribute: `{other}`"),
                    ));
                }
            }
            // Consume optional trailing comma
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(PreviewAttr {
            category: category.ok_or_else(|| {
                syn::Error::new(Span::call_site(), "missing `category` in #[preview]")
            })?,
            description: description.ok_or_else(|| {
                syn::Error::new(Span::call_site(), "missing `description` in #[preview]")
            })?,
            examples,
            no_styles,
            no_variant_gallery,
            custom_demo,
            custom_gallery,
        })
    }
}

// ── Main expansion ───────────────────────────────────────────────

pub fn expand(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let attr: PreviewAttr = syn::parse2(attr)?;
    let func: ItemFn = syn::parse2(item.clone())?;

    let comp_name = &func.sig.ident;
    let comp_name_str = comp_name.to_string();

    // Kebab-case ID: "EqSwitch" → "eq-switch"
    let id_str = to_kebab_case(&comp_name_str);

    // Snake-case for styles file display name
    let styles_file_display = format!("{}_styles.rs", to_snake_case(&comp_name_str));

    // Category ident: Atom → ComponentCategory::Atom
    let category = &attr.category;
    let description = &attr.description;

    // Parse props from function signature
    let props = parse_props::extract_props(&func.sig)?;

    // ── Usage examples ───────────────────────────────────────────
    let example_tokens: Vec<TokenStream> = attr
        .examples
        .iter()
        .map(|(label, code)| {
            quote! {
                UsageExample {
                    label: #label,
                    code: #code.into(),
                }
            }
        })
        .collect();

    // ── Style tokens ─────────────────────────────────────────────
    let style_tokens_fn = if attr.no_styles {
        quote! { || Vec::new() }
    } else {
        quote! { || s::catalog() }
    };

    // ── Demo and Gallery names ───────────────────────────────────
    let demo_name = format_ident!("__PreviewDemo{}", comp_name);
    let gallery_name = format_ident!("__PreviewGallery{}", comp_name);

    // ── Generate demo component ──────────────────────────────────
    let demo_component = if attr.custom_demo {
        quote! {} // User writes their own Demo component
    } else {
        codegen::gen_demo(comp_name, &props, &attr.examples, attr.no_variant_gallery)
    };

    // ── Generate gallery component ───────────────────────────────
    let gallery_component = if attr.custom_gallery {
        quote! {}
    } else {
        codegen::gen_gallery(comp_name, &props)
    };

    // ── Render closures ──────────────────────────────────────────
    let render_demo = if attr.custom_demo {
        // Expect user-defined Demo{Name} component
        let user_demo = format_ident!("Demo{}", comp_name);
        quote! { || rsx! { #user_demo {} } }
    } else {
        quote! { || rsx! { #demo_name {} } }
    };

    let render_gallery = if attr.custom_gallery {
        let user_gallery = format_ident!("Gallery{}", comp_name);
        quote! { || rsx! { #user_gallery {} } }
    } else {
        quote! { || rsx! { #gallery_name {} } }
    };

    // ── Styles module alias ──────────────────────────────────────
    // The component file already has `use super::eq_switch_styles as s;`
    // so we reference `s` in the generated code. The import is assumed
    // to exist in the component's module.

    // ── Assemble output ──────────────────────────────────────────
    Ok(quote! {
        // Emit the original component function unchanged
        #func

        // ── Playground imports (feature-gated) ───────────────────
        // NOTE: These are expected to already exist in the component file.
        // The macro relies on the standard import block being present:
        //   #[cfg(feature = "playground")]
        //   use crate::playground::playground_helpers::{...};
        //   use crate::atoms::{EqText, TextVariant};
        //   use crate::playground::playground_types::{...};

        // ── Descriptor ───────────────────────────────────────────
        #[cfg(feature = "playground")]
        pub fn descriptor() -> ComponentDescriptor {
            ComponentDescriptor {
                id: #id_str,
                name: #comp_name_str,
                category: ComponentCategory::#category,
                description: #description,
                style_tokens: #style_tokens_fn,
                usage_examples: || vec![#(#example_tokens),*],
                render_demo: #render_demo,
                render_gallery: #render_gallery,
            }
        }

        // ── Demo component ───────────────────────────────────────
        #demo_component

        // ── Gallery component ────────────────────────────────────
        #gallery_component
    })
}

// ── Helpers ──────────────────────────────────────────────────────

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

fn to_kebab_case(s: &str) -> String {
    to_snake_case(s).replace('_', "-")
}
