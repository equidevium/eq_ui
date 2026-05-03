use dioxus::prelude::*;
use super::eq_card_styles as s;
use crate::theme::merge_classes;
use crate::playground;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropToggle, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Card container molecule.
#[playground(
    category = Molecule,
    description = "Flexible card container with optional header, body, and footer sections.",
    examples = [
        ("Complete", "EqCard {\n    EqCardHeader { \"Card Title\" }\n    EqCardBody { \"Card content goes here.\" }\n    EqCardFooter { \"Footer content\" }\n}"),
        ("Body only", "EqCard {\n    EqCardBody { \"Body only - no header or footer.\" }\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqCard(
    /// Optional class override - extend or replace default styles.
    #[props(into, default)]
    class: String,
    /// Accessible label for screen readers. When set, the card becomes
    /// a named region so assistive technology can announce it
    /// (e.g. "User profile, region").
    #[props(into, default)]
    aria_label: String,
    /// Semantic role override. Common values:
    /// - `"article"` for cards in a feed
    /// - `"group"` for a related set of controls
    /// - empty (default) for a plain container
    /// When `aria_label` is set and `role` is empty, defaults to `"region"`.
    #[props(into, default)]
    role: String,
    children: Element,
) -> Element {
    let cls = merge_classes(s::CARD, &class);
    let has_label = !aria_label.is_empty();
    let effective_role = if !role.is_empty() {
        role.as_str()
    } else if has_label {
        "region"
    } else {
        ""
    };

    rsx! {
        div {
            class: "{cls}",
            role: if !effective_role.is_empty() { "{effective_role}" } else { "" },
            "aria-label": if has_label { "{aria_label}" } else { "" },
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

// ── Custom demo (sub-component composition) ──────────────────────

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
