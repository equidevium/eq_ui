use dioxus::prelude::*;
use super::eq_page_section_styles::*;
use crate::theme::{merge_classes, CONTAINER_LAYOUT};

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
    /// Optional class override — extend or replace default wrapper styles.
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
