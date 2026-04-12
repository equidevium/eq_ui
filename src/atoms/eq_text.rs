use dioxus::prelude::*;
use super::eq_text_styles as s;
use crate::theme::merge_classes;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropInput, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Text variant - determines the HTML element and style applied.
#[derive(Clone, PartialEq, Default)]
pub enum TextVariant {
    H1,
    H2,
    H3,
    #[default]
    Body,
    Muted,
    Caption,
    Emphasis,
    Mono,
}

/// Atomic text component.
/// Renders the correct semantic HTML element based on the chosen variant.
///
/// Use `class` to extend or replace the default styles .
#[component]
pub fn EqText(
    #[props(default)]
    variant: TextVariant,
    #[props(into, default)]
    class: String,
    children: Element,
) -> Element {
    let base = match &variant {
        TextVariant::H1       => s::H1,
        TextVariant::H2       => s::H2,
        TextVariant::H3       => s::H3,
        TextVariant::Body     => s::BODY,
        TextVariant::Muted    => s::MUTED,
        TextVariant::Caption  => s::CAPTION,
        TextVariant::Emphasis => s::EMPHASIS,
        TextVariant::Mono     => s::MONO,
    };
    let cls = merge_classes(base, &class);

    match variant {
        TextVariant::H1       => rsx! { h1     { class: "{cls}", {children} } },
        TextVariant::H2       => rsx! { h2     { class: "{cls}", {children} } },
        TextVariant::H3       => rsx! { h3     { class: "{cls}", {children} } },
        TextVariant::Body     => rsx! { p      { class: "{cls}", {children} } },
        TextVariant::Muted    => rsx! { p      { class: "{cls}", {children} } },
        TextVariant::Caption  => rsx! { span   { class: "{cls}", {children} } },
        TextVariant::Emphasis => rsx! { strong { class: "{cls}", {children} } },
        TextVariant::Mono     => rsx! { code   { class: "{cls}", {children} } },
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-text",
        name: "EqText",
        category: ComponentCategory::Atom,
        description: "Semantic text component with variants for headings, body, muted, \
                      caption, emphasis, and monospace text.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Heading",
                code: "EqText { variant: TextVariant::H1, \"Heading 1\" }".into(),
            },
            UsageExample {
                label: "Body text",
                code: "EqText { variant: TextVariant::Body, \"Body text - the default variant.\" }".into(),
            },
            UsageExample {
                label: "Muted",
                code: "EqText { variant: TextVariant::Muted, \"Muted - secondary colour.\" }".into(),
            },
        ],
        render_demo: || rsx! { DemoEqText {} },
        render_gallery: || rsx! { GalleryEqText {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqText() -> Element {
    let mut variant_str = use_signal(|| "Body".to_string());
    let mut content = use_signal(|| "The quick brown fox jumps over the lazy dog.".to_string());

    let variant = match variant_str().as_str() {
        "H1" => TextVariant::H1,
        "H2" => TextVariant::H2,
        "H3" => TextVariant::H3,
        "Muted" => TextVariant::Muted,
        "Caption" => TextVariant::Caption,
        "Emphasis" => TextVariant::Emphasis,
        "Mono" => TextVariant::Mono,
        _ => TextVariant::Body,
    };

    let code = r#"EqText { variant: TextVariant::H1, "Heading 1" }

EqText { variant: TextVariant::Body,
    "Body text - the default variant."
}

EqText { variant: TextVariant::Muted,
    "Muted - secondary colour."
}"#
    .to_string();

    rsx! {
        DemoSection { title: "EqText",
            // Interactive controls
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "variant",
                    value: variant_str(),
                    options: vec!["Body", "H1", "H2", "H3", "Muted", "Caption", "Emphasis", "Mono"],
                    onchange: move |v: String| variant_str.set(v),
                }
                PropInput {
                    label: "content",
                    value: content(),
                    placeholder: "Enter text content...",
                    onchange: move |v: String| content.set(v),
                }
            }

            // Live preview
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                EqText { variant, "{content}" }
            }

            // Style tokens
            StyleInfo { file: "eq_text_styles.rs", styles: format_catalog(&s::catalog()) }

            // Usage code
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqText() -> Element {
    rsx! {
        div { class: "space-y-3",
            EqText { variant: TextVariant::Emphasis, "All variants" }
            EqText { variant: TextVariant::H1, "Heading 1" }
            EqText { variant: TextVariant::H2, "Heading 2" }
            EqText { variant: TextVariant::H3, "Heading 3" }
            EqText { variant: TextVariant::Body, "Body text - the default variant." }
            EqText { variant: TextVariant::Muted, "Muted text - secondary colour." }
            EqText { variant: TextVariant::Caption, "Caption text" }
            EqText { variant: TextVariant::Emphasis, "Emphasis text" }
            EqText { variant: TextVariant::Mono, "Mono text - code snippets" }
        }
    }
}
