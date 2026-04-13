use dioxus::prelude::*;
use super::eq_card_styles as s;
use crate::theme::merge_classes;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropToggle, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Card container molecule.
///
/// Use `class` to extend or replace the default styles .
#[component]
pub fn EqCard(
    /// Optional class override - extend or replace default styles.
    #[props(into, default)]
    class: String,
    children: Element,
) -> Element {
    let cls = merge_classes(s::CARD, &class);
    rsx! {
        div {
            class: "{cls}",
            {children}
        }
    }
}

/// Card header section.
///
/// Use `class` to extend or replace the default styles .
#[component]
pub fn EqCardHeader(
    #[props(into, default)]
    class: String,
    children: Element,
) -> Element {
    let cls = merge_classes(s::CARD_HEADER, &class);
    rsx! {
        div {
            class: "{cls}",
            {children}
        }
    }
}

/// Card body section.
///
/// Use `class` to extend or replace the default styles .
#[component]
pub fn EqCardBody(
    #[props(into, default)]
    class: String,
    children: Element,
) -> Element {
    let cls = merge_classes(s::CARD_BODY, &class);
    rsx! {
        div {
            class: "{cls}",
            {children}
        }
    }
}

/// Card footer section.
///
/// Use `class` to extend or replace the default styles .
#[component]
pub fn EqCardFooter(
    #[props(into, default)]
    class: String,
    children: Element,
) -> Element {
    let cls = merge_classes(s::CARD_FOOTER, &class);
    rsx! {
        div {
            class: "{cls}",
            {children}
        }
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-card",
        name: "EqCard",
        category: ComponentCategory::Molecule,
        description: "Flexible card container with optional header, body, and footer sections.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Complete",
                code: "EqCard {\n    EqCardHeader { \"Card Title\" }\n    EqCardBody { \"Card content goes here.\" }\n    EqCardFooter { \"Footer content\" }\n}".into(),
            },
            UsageExample {
                label: "Body only",
                code: "EqCard {\n    EqCardBody { \"Body only - no header or footer.\" }\n}".into(),
            },
        ],
        render_demo: || rsx! { DemoEqCard {} },
        render_gallery: || rsx! { GalleryEqCard {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqCard() -> Element {
    let mut show_header = use_signal(|| true);
    let mut show_footer = use_signal(|| true);

    let code = "EqCard {\n    EqCardHeader { \"Card Title\" }\n    EqCardBody { \"Card body content goes here.\" }\n    EqCardFooter { \"Footer content\" }\n}\n\n// Minimal card\nEqCard {\n    EqCardBody { \"Body only - no header or footer.\" }\n}".to_string();

    rsx! {
        DemoSection { title: "EqCard",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropToggle {
                    label: "header",
                    value: show_header(),
                    onchange: move |v: bool| show_header.set(v),
                }
                PropToggle {
                    label: "footer",
                    value: show_footer(),
                    onchange: move |v: bool| show_footer.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 max-w-md",
                EqCard {
                    if show_header() {
                        EqCardHeader { "Card Title" }
                    }
                    EqCardBody { "Card body content goes here." }
                    if show_footer() {
                        EqCardFooter { "Footer content" }
                    }
                }
            }
            StyleInfo { file: "eq_card_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqCard() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Card Layouts" }

                div { class: "space-y-3",
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Complete (header, body, footer)" }
                        EqCard {
                            EqCardHeader { "Header" }
                            EqCardBody { "Body content" }
                            EqCardFooter { "Footer" }
                        }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Body only" }
                        EqCard {
                            EqCardBody { "Body-only card" }
                        }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Header and body" }
                        EqCard {
                            EqCardHeader { "Header" }
                            EqCardBody { "Body without footer" }
                        }
                    }
                }
            }
        }
    }
}
