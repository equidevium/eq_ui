use dioxus::prelude::*;
use super::eq_page_section_styles::*;
use crate::theme::CONTAINER_LAYOUT;

/// A structural wrapper for a page block/section.
/// Use this in pages to keep spacing and width consistent.
#[component]
pub fn EqPageSection(
    /// Optional id for anchor links, e.g. "services"
    id: Option<&'static str>,
    /// Optional title shown above the section content
    title: Option<&'static str>,
    /// Optional description/subtitle
    description: Option<&'static str>,
    /// Optional section content
    #[props(default)]
    children: Element,
) -> Element {
    rsx! {
        section { id,
            div { class: "{CONTAINER_LAYOUT} {SECTION_WRAP}",
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
