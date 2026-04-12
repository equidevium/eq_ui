use super::eq_scrollable_space_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropInput, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// A scrollable container atom.
/// Wraps content with vertical overflow scrolling and themed scrollbar.
/// Designed for sidebars, panels, and any area that needs independent scroll.
///
/// Use `class` to extend or replace the default styles .
#[component]
pub fn EqScrollableSpace(
    /// The content to render inside the scrollable area.
    children: Element,
    /// Optional max height (e.g. "h-96", "max-h-[500px]").
    /// When omitted, the container uses flex-1 to fill available height.
    #[props(into, default)]
    max_height: Option<String>,
    /// Optional class override - extend or replace default styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let height_class = max_height.as_deref().unwrap_or("");
    let base = format!("{} {} {}", s::CONTAINER, s::SCROLLBAR, height_class);
    let cls = merge_classes(&base, &class);

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
        id: "eq-scrollable-space",
        name: "EqScrollableSpace",
        category: ComponentCategory::Atom,
        description: "Scrollable container with vertical overflow and themed scrollbar. \
                      Ideal for sidebars, panels, and independent scroll areas.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Basic scrollable area",
                code: "// Wrap in a flex-col container with fixed height\ndiv { class: \"h-48 flex flex-col\",\n    EqScrollableSpace {\n        p { \"Scrollable content...\" }\n    }\n}".into(),
            },
            UsageExample {
                label: "With max-height",
                code: "EqScrollableSpace { max_height: \"max-h-96\",\n    // Content that overflows will scroll\n}".into(),
            },
        ],
        render_demo: || rsx! { DemoEqScrollableSpace {} },
        render_gallery: || rsx! { GalleryEqScrollableSpace {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqScrollableSpace() -> Element {
    let mut item_count = use_signal(|| "20".to_string());
    let count: usize = item_count().parse().unwrap_or(20).min(200);

    let code = "// Wrap in a flex-col container with fixed height\ndiv { class: \"h-48 flex flex-col\",\n    EqScrollableSpace {\n        // Content that overflows will scroll\n        for item in items {\n            p { \"{item}\" }\n        }\n    }\n}\n\n// Or with a custom max-height\nEqScrollableSpace { max_height: \"max-h-96\",\n    // ...children\n}".to_string();

    rsx! {
        DemoSection { title: "EqScrollableSpace",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropInput {
                    label: "item count",
                    value: item_count(),
                    placeholder: "20",
                    onchange: move |v: String| item_count.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                div { class: "w-80 h-48 flex flex-col border border-[var(--color-card-border)] rounded-lg",
                    EqScrollableSpace {
                        div { class: "p-4 space-y-3",
                            for i in 1..=count {
                                p {
                                    key: "{i}",
                                    class: "text-sm text-[var(--color-label)]",
                                    "Scrollable item {i}"
                                }
                            }
                        }
                    }
                }
            }
            StyleInfo { file: "eq_scrollable_space_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqScrollableSpace() -> Element {
    rsx! {
        div { class: "space-y-4",
            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Scrollable Container" }
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4",
                div { class: "w-full h-48 flex flex-col border border-[var(--color-card-border)] rounded-lg",
                    EqScrollableSpace {
                        div { class: "p-4 space-y-2",
                            for i in 1..=20 {
                                p {
                                    key: "{i}",
                                    class: "text-sm text-[var(--color-label)]",
                                    "Item {i}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
