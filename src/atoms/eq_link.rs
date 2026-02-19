use dioxus::prelude::*;
use super::eq_link_styles as s;

/// Atomic link component.
/// Renders a plain `<a>` tag with consistent styling.
/// Platform crates should use the router `Link` component for internal
/// navigation and apply EqLink's style classes directly when needed.
#[component]
pub fn EqLink(
    /// Target URL.
    href: String,
    children: Element,
) -> Element {
    rsx! {
        a {
            class: s::LINK,
            href: "{href}",
            {children}
        }
    }
}
