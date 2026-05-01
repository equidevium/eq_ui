//! Code generation for Demo and Gallery components.

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::parse_props::{PropInfo, PropKind};

/// Generate `handler_name: move |_| {}` for every Handler prop.
/// Used in static gallery instances where handlers are required but not interactive.
fn noop_handler_props(props: &[PropInfo]) -> Vec<TokenStream> {
    props.iter()
        .filter(|p| p.kind == PropKind::Handler)
        .map(|p| {
            let name = &p.name;
            quote! { #name: move |_| {}, }
        })
        .collect()
}

/// Detect if a Handler prop is a FormEvent handler paired with a `value: String` prop.
/// This is the standard "controlled input" pattern in Dioxus:
///   `oninput: EventHandler<FormEvent>` + `value: String`
/// Returns pairs of (handler_name, value_prop_name).
fn find_form_handler_pairs(props: &[PropInfo]) -> Vec<(Ident, Ident)> {
    let mut pairs = Vec::new();

    let handlers: Vec<_> = props.iter().filter(|p| p.kind == PropKind::Handler).collect();
    let string_props: Vec<_> = props.iter()
        .filter(|p| matches!(p.kind, PropKind::String) && p.name != "class")
        .collect();

    for handler in &handlers {
        let handler_ty = &handler.ty;
        let ty_str = quote::quote!(#handler_ty).to_string();
        let is_form_handler = ty_str.contains("FormEvent") || ty_str.contains("FormData");

        if !is_form_handler {
            continue;
        }

        // Try to pair with a `value` prop first, then fall back to first String prop
        let value_prop = string_props.iter()
            .find(|p| p.name == "value")
            .or_else(|| string_props.first());

        if let Some(target) = value_prop {
            pairs.push((handler.name.clone(), target.name.clone()));
        }
    }

    pairs
}

/// Detect if a Handler prop controls a Bool prop.
/// Convention: `on_change` pairs with `checked`, `on_toggle` pairs with `toggled`, etc.
/// Fallback: any `Option<EventHandler<bool>>` pairs with the first `bool` prop.
fn find_bool_handler_pairs(props: &[PropInfo]) -> Vec<(Ident, Ident)> {
    let mut pairs = Vec::new();

    let handlers: Vec<_> = props.iter().filter(|p| p.kind == PropKind::Handler).collect();
    let bools: Vec<_> = props.iter().filter(|p| p.kind == PropKind::Bool).collect();

    for handler in &handlers {
        let handler_name = handler.name.to_string();
        // Check if the handler type contains "bool" (EventHandler<bool>)
        let handler_ty = &handler.ty;
        let ty_str = quote::quote!(#handler_ty).to_string();
        let is_bool_handler = ty_str.contains("bool");

        if !is_bool_handler {
            continue;
        }

        // Try to match by naming convention:
        // on_change -> checked (first bool prop)
        // on_toggle -> toggled
        // on_X -> X
        let target_name = handler_name
            .strip_prefix("on_")
            .unwrap_or(&handler_name);

        // Look for a bool prop with matching name
        let matched = bools.iter().find(|b| b.name.to_string() == target_name);

        if let Some(bool_prop) = matched {
            pairs.push((handler.name.clone(), bool_prop.name.clone()));
        } else if target_name == "change" || target_name == "toggle" {
            // Generic handler name, pair with first bool prop
            if let Some(first_bool) = bools.first() {
                pairs.push((handler.name.clone(), first_bool.name.clone()));
            }
        }
    }

    pairs
}

/// Generate the `__PreviewDemo{Name}` component.
///
/// Creates `use_signal` for each non-skipped prop, PropToggle/PropSelect/PropInput
/// controls, a live preview instance with handler wiring, and a CodeBlock with examples.
pub fn gen_demo(
    comp_name: &Ident,
    props: &[PropInfo],
    examples: &[(String, String)],
    no_variant_gallery: bool,
) -> TokenStream {
    let demo_name = format_ident!("__PreviewDemo{}", comp_name);

    // Separate controllable props (get signals + controls) from children (get sample content)
    let controllable: Vec<_> = props.iter()
        .filter(|p| !p.skip && !matches!(p.kind, PropKind::Children))
        .collect();
    let has_children = props.iter().any(|p| matches!(p.kind, PropKind::Children));
    let handler_pairs = find_bool_handler_pairs(props);
    let form_handler_pairs = find_form_handler_pairs(props);

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
                PropKind::String | PropKind::StaticStr => {
                    let default = match &p.default_expr {
                        Some(expr) => quote! { #expr.to_string() },
                        None => quote! { String::new() },
                    };
                    quote! { let mut #sig_name = use_signal(|| #default); }
                }
                PropKind::Enum(_) => {
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
                PropKind::String | PropKind::StaticStr => {
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
                PropKind::StaticStr => {
                    // Leak the String into &'static str. Safe in WASM — page lifetime = app lifetime.
                    quote! { #prop_name: Box::leak(#sig_name().into_boxed_str()), }
                }
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

    // ── Children: editable signal + control ────────────────────────
    // For components with `children: Element`, we add a `sig_content` signal
    // that lets the user type custom children text in the demo.
    let comp_label = comp_name.to_string()
        .trim_start_matches("Eq")
        .to_string();
    let default_children_text = format!("Sample {}", comp_label);

    let children_signal_decl = if has_children {
        let default = &default_children_text;
        quote! { let mut sig_content = use_signal(|| #default.to_string()); }
    } else {
        quote! {}
    };

    let children_control = if has_children {
        quote! {
            {
                const __LABEL_CONTENT: &str = "content";
                const __PH_CONTENT: &str = "Child content text";
                rsx! {
                    PropInput {
                        label: __LABEL_CONTENT,
                        value: sig_content(),
                        placeholder: __PH_CONTENT,
                        onchange: move |v: String| sig_content.set(v),
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    // In the live preview, children come from the signal
    let children_preview = if has_children {
        quote! { "{sig_content}" }
    } else {
        quote! {}
    };

    // In gallery/variant sections, children use a static sample
    let children_static = if has_children {
        let sample = &default_children_text;
        quote! { #sample }
    } else {
        quote! {}
    };

    // ── Handler wiring ──────────────────────────────────────────
    // For each handler that pairs with a bool prop, generate an on_change/on_click
    // prop that toggles the bool signal.
    let bool_handler_props: Vec<TokenStream> = handler_pairs
        .iter()
        .map(|(handler_name, bool_name)| {
            let sig_name = format_ident!("sig_{}", bool_name);
            quote! { #handler_name: move |v: bool| #sig_name.set(v), }
        })
        .collect();

    // For each form handler (EventHandler<FormEvent>) paired with a value prop,
    // wire it to update the value signal: `oninput: move |e: FormEvent| sig_value.set(e.value())`
    let form_handler_props: Vec<TokenStream> = form_handler_pairs
        .iter()
        .map(|(handler_name, value_name)| {
            let sig_name = format_ident!("sig_{}", value_name);
            quote! { #handler_name: move |e: FormEvent| #sig_name.set(e.value()), }
        })
        .collect();

    let title_str = comp_name.to_string();
    let style_file_display = format!("{}_styles.rs", to_snake_case(&comp_name.to_string()));

    // Build a combined code string from all examples
    let code_block = if examples.is_empty() {
        quote! {}
    } else {
        let combined_code = examples
            .iter()
            .map(|(label, code)| format!("// {}\n{}", label, code))
            .collect::<Vec<_>>()
            .join("\n\n");
        quote! {
            CodeBlock { code: #combined_code.to_string() }
        }
    };

    // ── Variant gallery sections inside the demo ────────────────
    let enum_props: Vec<_> = props.iter().filter(|p| matches!(p.kind, PropKind::Enum(_))).collect();
    let bool_props: Vec<_> = props.iter()
        .filter(|p| p.kind == PropKind::Bool && !p.skip)
        .collect();
    let string_props: Vec<_> = props.iter()
        .filter(|p| matches!(p.kind, PropKind::String | PropKind::StaticStr) && !p.skip && p.name != "class")
        .collect();

    // No-op handlers for static instances in variant galleries.
    // Handlers are skipped in normal prop generation, but required handlers
    // still need a value passed. Generate `handler: move |_| {}` for each.
    let noop_handlers: Vec<TokenStream> = noop_handler_props(props);

    // Section 1: Sizes — one column per enum variant, with bool on/off rows
    let sizes_section = if let Some(first_enum) = enum_props.first() {
        let enum_ty = &first_enum.ty;
        let enum_name = &first_enum.name;
        let first_bool_name = bool_props.first().map(|p| &p.name);
        let enum_label = capitalize_first(&first_enum.name.to_string());

        if let Some(bool_name) = first_bool_name {
            // Show each variant with on/off states
            let label_prop = string_props.first().map(|p| {
                let pname = &p.name;
                (pname.clone(), true)
            });
            let on_label = label_prop.as_ref().map(|(pname, _)| {
                quote! { #pname: "On", }
            }).unwrap_or(quote! {});
            let off_label = label_prop.as_ref().map(|(pname, _)| {
                quote! { #pname: "Off", }
            }).unwrap_or(quote! {});

            quote! {
                div { class: "space-y-4",
                    EqText { variant: TextVariant::Emphasis, #enum_label }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                        for variant_name in <#enum_ty as crate::preview_enum_trait::PreviewEnumInfo>::variant_names() {
                            div { class: "space-y-3",
                                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "{variant_name}" }
                                div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                                    #comp_name {
                                        #bool_name: true,
                                        #enum_name: <#enum_ty as crate::preview_enum_trait::PreviewEnumInfo>::from_name(variant_name),
                                        #on_label
                                        #(#noop_handlers)*
                                        #children_static
                                    }
                                    #comp_name {
                                        #bool_name: false,
                                        #enum_name: <#enum_ty as crate::preview_enum_trait::PreviewEnumInfo>::from_name(variant_name),
                                        #off_label
                                        #(#noop_handlers)*
                                        #children_static
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            // No bool prop, just show each variant with label above
            quote! {
                div { class: "space-y-4",
                    EqText { variant: TextVariant::Emphasis, #enum_label }
                    div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                        for variant_name in <#enum_ty as crate::preview_enum_trait::PreviewEnumInfo>::variant_names() {
                            div { class: "space-y-2",
                                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "{variant_name}" }
                                #comp_name {
                                    #enum_name: <#enum_ty as crate::preview_enum_trait::PreviewEnumInfo>::from_name(variant_name),
                                    #(#noop_handlers)*
                                    #children_static
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    // Section 2: States — show bool prop combinations (e.g. disabled on/off)
    let states_section = if bool_props.len() >= 2 {
        // Generate instances for interesting bool combinations
        // Show: all-false, first-true, second-true, both-true
        let first_bool = &bool_props[0];
        let second_bool = &bool_props[1];
        let b1 = &first_bool.name;
        let b2 = &second_bool.name;
        let b1_label = capitalize_first(&first_bool.name.to_string());
        let b2_label = capitalize_first(&second_bool.name.to_string());

        let label_prop_tokens = string_props.first().map(|p| {
            let pname = &p.name;
            quote! { #pname: }
        });

        let children_ref = &children_static;
        let noop_ref = &noop_handlers;
        let make_instance = |v1: bool, v2: bool| -> TokenStream {
            let label_text = match (v1, v2) {
                (false, false) => format!("Default off"),
                (true, false) => format!("Default on"),
                (false, true) => format!("{} off", b2_label),
                (true, true) => format!("{} on", b2_label),
            };
            if let Some(ref lt) = label_prop_tokens {
                quote! {
                    #comp_name { #b1: #v1, #b2: #v2, #lt #label_text, #(#noop_ref)* #children_ref }
                }
            } else {
                quote! {
                    #comp_name { #b1: #v1, #b2: #v2, #(#noop_ref)* #children_ref }
                }
            }
        };

        let inst1 = make_instance(false, false);
        let inst2 = make_instance(true, false);
        let inst3 = make_instance(false, true);
        let inst4 = make_instance(true, true);

        quote! {
            div { class: "space-y-4",
                EqText { variant: TextVariant::Emphasis, "States" }
                div { class: "rounded-lg border border-[var(--color-card-border)] p-4 flex flex-wrap items-center gap-4",
                    #inst1
                    #inst2
                    #inst3
                    #inst4
                }
            }
        }
    } else {
        quote! {}
    };

    // Section 3: With populated strings — show component with all string props filled
    let strings_section = if string_props.len() >= 2 {
        // Show a few instances with strings populated
        let string_filled: Vec<TokenStream> = string_props.iter().map(|p| {
            let pname = &p.name;
            let sample = match p.name.to_string().as_str() {
                "label" => "Dark mode".to_string(),
                "description" => "Use dark colors for the interface".to_string(),
                "title" => "Feature Title".to_string(),
                "subtitle" => "A brief subtitle".to_string(),
                "placeholder" => "Type here...".to_string(),
                other => format!("Sample {}", other),
            };
            quote! { #pname: #sample, }
        }).collect();

        let first_bool_on = bool_props.first().map(|p| {
            let bname = &p.name;
            quote! { #bname: true, }
        }).unwrap_or(quote! {});

        let first_bool_off = bool_props.first().map(|p| {
            let bname = &p.name;
            quote! { #bname: false, }
        }).unwrap_or(quote! {});

        quote! {
            div { class: "space-y-4",
                EqText { variant: TextVariant::Emphasis, "With Details" }
                div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                    #comp_name {
                        #first_bool_on
                        #(#string_filled)*
                        #(#noop_handlers)*
                        #children_static
                    }
                    #comp_name {
                        #first_bool_off
                        #(#string_filled)*
                        #(#noop_handlers)*
                        #children_static
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    // Section 4: Children examples — for children-based components without enums/bools,
    // show a small gallery of instances with different sample content
    let children_examples_section = if has_children && enum_props.is_empty() && bool_props.is_empty() {
        // Build 3 sample instances with varied children text
        let samples = vec![
            format!("First {}", comp_label),
            format!("Second {}", comp_label),
            format!("Third {}", comp_label),
        ];

        // For each string/static-str prop, provide sensible defaults
        let default_prop_vals: Vec<TokenStream> = controllable.iter().filter_map(|p| {
            let pname = &p.name;
            match &p.kind {
                PropKind::String | PropKind::StaticStr => {
                    let val = match p.name.to_string().as_str() {
                        "href" => "https://example.com".to_string(),
                        "for_id" => "field".to_string(),
                        other => format!("sample-{}", other),
                    };
                    Some(quote! { #pname: #val, })
                }
                _ => None,
            }
        }).collect();

        let instances: Vec<TokenStream> = samples.iter().map(|sample_text| {
            quote! {
                #comp_name {
                    #(#default_prop_vals)*
                    #(#noop_handlers)*
                    #sample_text
                }
            }
        }).collect();

        quote! {
            div { class: "space-y-4",
                EqText { variant: TextVariant::Emphasis, "Examples" }
                div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                    #(#instances)*
                }
            }
        }
    } else {
        quote! {}
    };

    let variant_gallery_section = if no_variant_gallery {
        quote! {}
    } else {
        quote! {
            div { class: "space-y-4",
                #sizes_section
                #states_section
                #strings_section
                #children_examples_section
            }
        }
    };

    quote! {
        #[cfg(feature = "playground")]
        #[component]
        fn #demo_name() -> Element {
            #(#signal_decls)*
            #children_signal_decl

            rsx! {
                DemoSection { title: #title_str,
                    // Prop controls
                    div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                        EqText {
                            variant: TextVariant::Caption,
                            class: "font-semibold uppercase tracking-wider",
                            "Props"
                        }
                        #(#prop_controls)*
                        #children_control
                    }
                    // Live preview
                    div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 space-y-4",
                        #comp_name {
                            #(#preview_props)*
                            #(#bool_handler_props)*
                            #(#form_handler_props)*
                            #children_preview
                        }
                    }
                    // Variant gallery
                    #variant_gallery_section
                    // Style info + code
                    StyleInfo {
                        file: #style_file_display,
                        styles: format_catalog(&s::catalog()),
                    }
                    #code_block
                }
            }
        }
    }
}

/// Generate the `__PreviewGallery{Name}` component.
///
/// Renders a compact showcase with representative instances.
/// If there's an enum prop, shows one instance per variant.
/// If there's a bool prop, shows both on and off states per variant.
pub fn gen_gallery(
    comp_name: &Ident,
    props: &[PropInfo],
) -> TokenStream {
    let gallery_name = format_ident!("__PreviewGallery{}", comp_name);

    let enum_props: Vec<_> = props.iter().filter(|p| matches!(p.kind, PropKind::Enum(_))).collect();
    let bool_props: Vec<_> = props.iter().filter(|p| p.kind == PropKind::Bool && !p.skip).collect();

    let has_children = props.iter().any(|p| matches!(p.kind, PropKind::Children));
    let children_content = if has_children {
        let comp_label = comp_name.to_string()
            .trim_start_matches("Eq")
            .to_string();
        let sample = format!("Sample {}", comp_label);
        quote! { #sample }
    } else {
        quote! {}
    };

    // No-op handlers for static gallery instances
    let noop_handlers: Vec<TokenStream> = noop_handler_props(props);

    // For String and StaticStr props, provide sensible defaults
    let string_defaults: Vec<TokenStream> = props
        .iter()
        .filter(|p| matches!(p.kind, PropKind::String | PropKind::StaticStr) && !p.skip && p.name != "class")
        .map(|p| {
            let pname = &p.name;
            let val = p.name.to_string().replace('_', " ");
            let val = capitalize_first(&val);
            match &p.kind {
                PropKind::StaticStr => {
                    // Use a string literal directly for &'static str
                    quote! { #pname: #val, }
                }
                _ => quote! { #pname: #val, }
            }
        })
        .collect();

    let gallery_body = if let Some(first_enum) = enum_props.first() {
        let prop_name = &first_enum.name;
        let ty = &first_enum.ty;
        let comp = comp_name;

        // If there's also a primary bool prop (like `checked`), show both states
        if let Some(first_bool) = bool_props.first() {
            let bool_name = &first_bool.name;
            quote! {
                div { class: "space-y-4",
                    div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                        EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider",
                            "All Variants"
                        }
                        div { class: "flex flex-wrap items-center gap-6",
                            for variant_name in <#ty as crate::preview_enum_trait::PreviewEnumInfo>::variant_names() {
                                div { class: "flex flex-col items-center gap-2",
                                    EqText { variant: TextVariant::Caption, "{variant_name}" }
                                    #comp {
                                        #prop_name: <#ty as crate::preview_enum_trait::PreviewEnumInfo>::from_name(variant_name),
                                        #bool_name: true,
                                        #(#string_defaults)*
                                        #(#noop_handlers)*
                                        #children_content
                                    }
                                    #comp {
                                        #prop_name: <#ty as crate::preview_enum_trait::PreviewEnumInfo>::from_name(variant_name),
                                        #bool_name: false,
                                        #(#string_defaults)*
                                        #(#noop_handlers)*
                                        #children_content
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            quote! {
                div { class: "space-y-4",
                    div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                        EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider",
                            "All Variants"
                        }
                        div { class: "flex flex-wrap items-center gap-4",
                            for variant_name in <#ty as crate::preview_enum_trait::PreviewEnumInfo>::variant_names() {
                                div { class: "flex flex-col items-center gap-1",
                                    #comp {
                                        #prop_name: <#ty as crate::preview_enum_trait::PreviewEnumInfo>::from_name(variant_name),
                                        #(#string_defaults)*
                                        #(#noop_handlers)*
                                        #children_content
                                    }
                                    EqText { variant: TextVariant::Caption, "{variant_name}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else if !bool_props.is_empty() {
        // No enum but has bool props - show both states
        let first_bool = &bool_props[0];
        let bool_name = &first_bool.name;
        let comp = comp_name;

        quote! {
            div { class: "space-y-4",
                div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                    EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider",
                        "States"
                    }
                    div { class: "flex flex-wrap items-center gap-4",
                        #comp {
                            #bool_name: true,
                            #(#string_defaults)*
                            #(#noop_handlers)*
                            #children_content
                        }
                        #comp {
                            #bool_name: false,
                            #(#string_defaults)*
                            #(#noop_handlers)*
                            #children_content
                        }
                    }
                }
            }
        }
    } else {
        // No enum, no bool - just render a single default instance
        let comp = comp_name;
        quote! {
            div { class: "space-y-4",
                div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                    EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider",
                        "Gallery"
                    }
                    #comp {
                        #(#string_defaults)*
                        #(#noop_handlers)*
                        #children_content
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

/// Capitalize first letter of a string.
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
