//! Code generation for Demo and Gallery components.

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

use crate::parse_props::{PropInfo, PropKind};

/// Generate the `__PreviewDemo{Name}` component.
///
/// Creates `use_signal` for each non-skipped prop, PropToggle/PropSelect/PropInput
/// controls, and a live preview instance.
pub fn gen_demo(
    comp_name: &Ident,
    styles_mod: &Ident,
    props: &[PropInfo],
) -> TokenStream {
    let demo_name = format_ident!("__PreviewDemo{}", comp_name);

    let controllable: Vec<_> = props.iter().filter(|p| !p.skip).collect();

    // ── Signal declarations ──────────────────────────────────────
    let signal_decls: Vec<TokenStream> = controllable
        .iter()
        .map(|p| {
            let sig_name = format_ident!("sig_{}", p.name);
            match &p.kind {
                PropKind::Bool => {
                    let default = match &p.default_expr {
                        Some(expr) => quote! { #expr },
                        None => quote! { false },
                    };
                    quote! { let mut #sig_name = use_signal(|| #default); }
                }
                PropKind::String => {
                    let default = match &p.default_expr {
                        Some(expr) => quote! { #expr.to_string() },
                        None => quote! { String::new() },
                    };
                    quote! { let mut #sig_name = use_signal(|| #default); }
                }
                PropKind::Enum(_) => {
                    // Use PreviewEnumInfo::default_name()
                    let ty = &p.ty;
                    quote! {
                        let mut #sig_name = use_signal(|| <#ty as crate::preview_enum_trait::PreviewEnumInfo>::default_name().to_string());
                    }
                }
                _ => quote! {},
            }
        })
        .collect();

    // ── Prop controls (PropToggle / PropSelect / PropInput) ──────
    let prop_controls: Vec<TokenStream> = controllable
        .iter()
        .map(|p| {
            let sig_name = format_ident!("sig_{}", p.name);
            let label_str = p.name.to_string();
            // Leak the string into a &'static str at compile time via a const
            let label_const = format_ident!("__LABEL_{}", p.name.to_string().to_uppercase());
            match &p.kind {
                PropKind::Bool => {
                    quote! {
                        {
                            const #label_const: &str = #label_str;
                            rsx! {
                                PropToggle {
                                    label: #label_const,
                                    value: #sig_name(),
                                    onchange: move |v: bool| #sig_name.set(v),
                                }
                            }
                        }
                    }
                }
                PropKind::String => {
                    let placeholder = format!("Enter {}", label_str);
                    let ph_const = format_ident!("__PH_{}", p.name.to_string().to_uppercase());
                    quote! {
                        {
                            const #label_const: &str = #label_str;
                            const #ph_const: &str = #placeholder;
                            rsx! {
                                PropInput {
                                    label: #label_const,
                                    value: #sig_name(),
                                    placeholder: #ph_const,
                                    onchange: move |v: String| #sig_name.set(v),
                                }
                            }
                        }
                    }
                }
                PropKind::Enum(_) => {
                    let ty = &p.ty;
                    quote! {
                        {
                            const #label_const: &str = #label_str;
                            let options: Vec<&'static str> = <#ty as crate::preview_enum_trait::PreviewEnumInfo>::variant_names().to_vec();
                            rsx! {
                                PropSelect {
                                    label: #label_const,
                                    value: #sig_name(),
                                    options: options,
                                    onchange: move |v: String| #sig_name.set(v),
                                }
                            }
                        }
                    }
                }
                _ => quote! {},
            }
        })
        .collect();

    // ── Prop values passed to the live preview component ─────────
    let preview_props: Vec<TokenStream> = controllable
        .iter()
        .map(|p| {
            let prop_name = &p.name;
            let sig_name = format_ident!("sig_{}", p.name);
            match &p.kind {
                PropKind::Bool => quote! { #prop_name: #sig_name(), },
                PropKind::String => quote! { #prop_name: #sig_name(), },
                PropKind::Enum(_) => {
                    let ty = &p.ty;
                    quote! {
                        #prop_name: <#ty as crate::preview_enum_trait::PreviewEnumInfo>::from_name(&#sig_name()),
                    }
                }
                _ => quote! {},
            }
        })
        .collect();

    let title_str = comp_name.to_string();
    let styles_file = format!("{}_styles.rs", to_snake_case(&comp_name.to_string()).trim_start_matches("eq_"));
    let styles_file_full = format!("eq_{}", styles_file);

    // Use the actual styles file name pattern
    let style_file_display = format!("{}_styles.rs", to_snake_case(&comp_name.to_string()));

    quote! {
        #[cfg(feature = "playground")]
        #[component]
        fn #demo_name() -> Element {
            #(#signal_decls)*

            rsx! {
                DemoSection { title: #title_str,
                    div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                        EqText {
                            variant: TextVariant::Caption,
                            class: "font-semibold uppercase tracking-wider",
                            "Props"
                        }
                        #(#prop_controls)*
                    }
                    div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                        #comp_name {
                            #(#preview_props)*
                        }
                    }
                    StyleInfo {
                        file: #style_file_display,
                        styles: format_catalog(&#styles_mod::catalog()),
                    }
                }
            }
        }
    }
}

/// Generate the `__PreviewGallery{Name}` component.
///
/// Renders a compact showcase with a few representative instances.
pub fn gen_gallery(
    comp_name: &Ident,
    props: &[PropInfo],
) -> TokenStream {
    let gallery_name = format_ident!("__PreviewGallery{}", comp_name);

    // Find enum props to generate variant showcase
    let enum_props: Vec<_> = props.iter().filter(|p| matches!(p.kind, PropKind::Enum(_))).collect();

    let gallery_body = if let Some(first_enum) = enum_props.first() {
        // Generate one instance per variant of the first enum
        let prop_name = &first_enum.name;
        let ty = &first_enum.ty;
        let comp = comp_name;

        // For String props, provide sensible defaults
        let string_defaults: Vec<TokenStream> = props
            .iter()
            .filter(|p| matches!(p.kind, PropKind::String) && !p.skip && p.name != "class")
            .map(|p| {
                let pname = &p.name;
                let val = format!("Demo {}", p.name);
                quote! { #pname: #val, }
            })
            .collect();

        quote! {
            div { class: "space-y-4",
                div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                    EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider",
                        "Gallery"
                    }
                    div { class: "flex flex-wrap items-center gap-4",
                        for variant_name in <#ty as crate::preview_enum_trait::PreviewEnumInfo>::variant_names() {
                            div { class: "flex flex-col items-center gap-1",
                                #comp {
                                    #prop_name: <#ty as crate::preview_enum_trait::PreviewEnumInfo>::from_name(variant_name),
                                    #(#string_defaults)*
                                }
                                EqText { variant: TextVariant::Caption, "{variant_name}" }
                            }
                        }
                    }
                }
            }
        }
    } else {
        // No enum props — just render a single default instance
        let string_defaults: Vec<TokenStream> = props
            .iter()
            .filter(|p| matches!(p.kind, PropKind::String) && !p.skip && p.name != "class")
            .map(|p| {
                let pname = &p.name;
                let val = format!("Demo {}", p.name);
                quote! { #pname: #val, }
            })
            .collect();

        quote! {
            div { class: "space-y-4",
                div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                    EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider",
                        "Gallery"
                    }
                    #comp_name {
                        #(#string_defaults)*
                    }
                }
            }
        }
    };

    quote! {
        #[cfg(feature = "playground")]
        #[component]
        fn #gallery_name() -> Element {
            rsx! {
                #gallery_body
            }
        }
    }
}

/// Convert PascalCase to snake_case.
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
