use super::eq_scrollable_space_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

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
    /// Optional class override — extend or replace default styles.
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
