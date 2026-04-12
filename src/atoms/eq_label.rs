use dioxus::prelude::*;
use super::eq_label_styles as s;
use crate::theme::merge_classes;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropInput, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::EqText;
#[cfg(feature = "playground")]
use crate::atoms::TextVariant;
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Atomic form label component.
/// Renders a `<label>` element with consistent styling.
///
/// Use `class` to extend or replace the default styles .
#[component]
pub fn EqLabel(
    /// The `id` of the form control this label is associated with.
    #[props(default = "")]
    for_id: &'static str,
    /// Optional class override - extend or replace default styles.
    #[props(into, default)]
    class: String,
    children: Element,
) -> Element {
    let cls = merge_classes(s::LABEL, &class);
    rsx! {
        label {
            class: "{cls}",
            r#for: "{for_id}",
            {children}
        }
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-label",
        name: "EqLabel",
        category: ComponentCategory::Atom,
        description: "Form label component with consistent styling. Associates with \
                      form controls via the `for_id` attribute.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Basic",
                code: "EqLabel { for_id: \"username\", \"Username\" }".into(),
            },
            UsageExample {
                label: "Without for attribute",
                code: "EqLabel { \"Label without for attribute\" }".into(),
            },
        ],
        render_demo: || rsx! { DemoEqLabel {} },
        render_gallery: || rsx! { GalleryEqLabel {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqLabel() -> Element {
    let mut for_id_str = use_signal(|| "username".to_string());
    let mut content = use_signal(|| "Username".to_string());

    let for_id: &'static str = match for_id_str().as_str() {
        "email" => "email",
        "password" => "password",
        "(none)" => "",
        _ => "username",
    };

    let code = r#"EqLabel { for_id: "username", "Username" }

EqLabel { "Label without for attribute" }"#
        .to_string();

    rsx! {
        DemoSection { title: "EqLabel",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "for_id",
                    value: for_id_str(),
                    options: vec!["username", "email", "password", "(none)"],
                    onchange: move |v: String| for_id_str.set(v),
                }
                PropInput {
                    label: "content",
                    value: content(),
                    placeholder: "Label text",
                    onchange: move |v: String| content.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                EqLabel { for_id, "{content}" }
            }
            StyleInfo { file: "eq_label_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqLabel() -> Element {
    rsx! {
        div { class: "space-y-3",
            EqText { variant: TextVariant::Emphasis, "Label examples" }
            div { class: "space-y-2",
                EqLabel { for_id: "username", "Username" }
                EqLabel { for_id: "email", "Email address" }
                EqLabel { for_id: "password", "Password" }
                EqLabel { "Label without for attribute" }
            }
        }
    }
}
