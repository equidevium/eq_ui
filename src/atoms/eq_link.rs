use dioxus::prelude::*;
use super::eq_link_styles as s;
use crate::theme::merge_classes;
use crate::preview;

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
#[preview(
    category = Atom,
    description = "Atomic link component with consistent styling. Use the router Link \
                   component for internal navigation.",
    examples = [
        ("Internal link", "EqLink { href: \"#\", \"Internal link\" }"),
        ("External link", "EqLink { href: \"https://example.com\", \"External link\" }"),
    ],
)]
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
