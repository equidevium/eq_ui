use dioxus::prelude::*;
use super::eq_header_styles as s;
use crate::theme::CONTAINER_LAYOUT;

/// Portable header component.
/// Accepts nav content as an Element prop so the platform crate
/// can provide router-aware Links or plain `<a>` tags.
#[component]
pub fn EqHeader(
    #[props(default = "Equidevium")]
    site_title: &'static str,
    /// Navigation content — the caller provides `<li>` elements.
    /// EqHeader wraps them in `<nav><ul>` with correct styling.
    nav: Option<Element>,
) -> Element {
    rsx! {
        header { class: s::HEADER,
            div { class: "{CONTAINER_LAYOUT} {s::HEADER_INNER}",
                h1 {
                    a {
                        class: s::BRAND,
                        href: "/",
                        "{site_title}"
                    }
                }
                if let Some(nav_content) = nav {
                    nav {
                        ul { class: s::NAV_UL,
                            {nav_content}
                        }
                    }
                }
            }
        }
    }
}
