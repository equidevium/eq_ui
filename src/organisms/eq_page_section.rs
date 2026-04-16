use dioxus::prelude::*;
use super::eq_page_section_styles::*;
use crate::theme::{merge_classes, CONTAINER_LAYOUT};

#[cfg(feature = "playground")]
use super::eq_page_section_styles as s;
#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{CodeBlock, DemoSection, PropInput, StyleInfo, format_catalog};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// A structural wrapper for a page block/section.
/// Use this in pages to keep spacing and width consistent.
#[component]
pub fn EqPageSection(
    /// Optional id for anchor links, e.g. "services"
    id: Option<&'static str>,
    /// Optional title shown above the section content
    #[props(into)]
    title: Option<String>,
    /// Optional description/subtitle
    #[props(into)]
    description: Option<String>,
    /// Optional section content
    #[props(default)]
    children: Element,
    /// Optional class override - extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let base = format!("{CONTAINER_LAYOUT} {SECTION_WRAP}");
    let cls = merge_classes(&base, &class);
    rsx! {
        section { id,
            div { class: "{cls}",
                if let Some(title) = title {
                    h2 { class: SECTION_TITLE, "{title}" }
                }
                if let Some(description) = description {
                    p { class: SECTION_DESC, "{description}" }
                }
                div { class: SECTION_BODY,
                    {children}
                }
            }
        }
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-page-section",
        name: "EqPageSection",
        category: ComponentCategory::Organism,
        description: "Page section wrapper with optional title, description, and consistent spacing.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Basic",
                code: "EqPageSection {\n    title: \"Features\",\n    description: \"What makes this product special.\",\n}".into(),
            },
            UsageExample {
                label: "With children",
                code: "EqPageSection {\n    title: \"With Children\",\n    description: \"Extra content below.\",\n    div { \"Nested child content\" }\n}".into(),
            },
        ],
        render_demo: || rsx! { DemoEqPageSection {} },
        render_gallery: || rsx! { GalleryEqPageSection {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqPageSection() -> Element {
    let mut title = use_signal(|| "Section Title".to_string());
    let mut description =
        use_signal(|| "A description of this section with some context.".to_string());

    let title_val: Option<String> = if title().is_empty() {
        None
    } else {
        Some(title())
    };
    let desc_val: Option<String> = if description().is_empty() {
        None
    } else {
        Some(description())
    };

    let code = "EqPageSection {\n    title: \"Features\",\n    description: \"What makes this product special.\",\n}\n\nEqPageSection {\n    title: \"With Children\",\n    description: \"Extra content below.\",\n    div { \"Nested child content\" }\n}".to_string();

    rsx! {
        DemoSection { title: "EqPageSection",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropInput {
                    label: "title",
                    value: title(),
                    placeholder: "Section title",
                    onchange: move |v: String| title.set(v),
                }
                PropInput {
                    label: "description",
                    value: description(),
                    placeholder: "Section description",
                    onchange: move |v: String| description.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] overflow-hidden",
                EqPageSection { title: title_val, description: desc_val,
                    div { class: "mt-4 p-4 rounded-lg bg-[var(--color-card)]/40",
                        "Child content inside a PageSection."
                    }
                }
            }
            StyleInfo { file: "eq_page_section_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqPageSection() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Page Section Gallery" }

                div { class: "space-y-3",
                    EqPageSection {
                        title: Some("Section 1".to_string()),
                        description: Some("This is a sample section.".to_string()),
                        div { "Content here" }
                    }
                }
            }
        }
    }
}
