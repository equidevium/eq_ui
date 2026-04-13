use dioxus::prelude::*;
use super::eq_link_styles as s;
use crate::theme::merge_classes;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropInput, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Atomic link component.
/// Renders a plain `<a>` tag with consistent styling.
/// Platform crates should use the router `Link` component for internal
/// navigation and apply EqLink's style classes directly when needed.
///
/// Use `class` to extend or replace the default styles .
#[component]
pub fn EqLink(
    /// Target URL.
    href: String,
    /// Optional class override - extend or replace default styles.
    #[props(into, default)]
    class: String,
    children: Element,
) -> Element {
    let cls = merge_classes(s::LINK, &class);
    rsx! {
        a {
            class: "{cls}",
            href: "{href}",
            {children}
        }
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-link",
        name: "EqLink",
        category: ComponentCategory::Atom,
        description: "Atomic link component with consistent styling. Use the router Link \
                      component for internal navigation.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Internal link",
                code: "EqLink { href: \"#\", \"Internal link\" }".into(),
            },
            UsageExample {
                label: "External link",
                code: "EqLink { href: \"https://example.com\", \"External link\" }".into(),
            },
        ],
        render_demo: || rsx! { DemoEqLink {} },
        render_gallery: || rsx! { GalleryEqLink {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqLink() -> Element {
    let mut href = use_signal(|| "https://example.com".to_string());
    let mut content = use_signal(|| "Click me".to_string());

    let code = r##"EqLink { href: "#", "Internal link" }

EqLink { href: "https://example.com", "External link" }"##
        .to_string();

    rsx! {
        DemoSection { title: "EqLink",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropInput {
                    label: "href",
                    value: href(),
                    placeholder: "https://...",
                    onchange: move |v: String| href.set(v),
                }
                PropInput {
                    label: "content",
                    value: content(),
                    placeholder: "Link text",
                    onchange: move |v: String| content.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                EqLink { href: href(), "{content}" }
            }
            StyleInfo { file: "eq_link_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqLink() -> Element {
    rsx! {
        div { class: "space-y-3",
            EqText { variant: TextVariant::Emphasis, "Link examples" }
            div { class: "space-y-2",
                EqLink { href: "#", "Internal link" }
                EqLink { href: "https://example.com", "External link" }
                EqLink { href: "https://example.com/page", "Navigation link" }
            }
        }
    }
}
