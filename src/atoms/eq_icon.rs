use dioxus::prelude::*;
use super::eq_icon_styles as s;
use crate::theme::merge_classes;

/// Icon size variant.
#[derive(Clone, PartialEq, Default)]
pub enum IconSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Atomic icon wrapper with two rendering modes:
///
/// **Path mode** — pass an SVG path data string via the `path` prop.
/// The component renders an inline `<svg>` with `fill="currentColor"`,
/// so color is controlled by the wrapper's text color class.
///
/// ```rust,ignore
/// EqIcon { path: eq_ui::atoms::icons::CARET_UP, size: IconSize::Sm }
/// ```
///
/// **Children mode** — pass any element (custom SVG, `<img>`, etc.)
/// as children. The wrapper applies consistent sizing and color.
///
/// ```rust,ignore
/// EqIcon { size: IconSize::Md,
///     svg { /* custom svg */ }
/// }
/// ```
#[component]
pub fn EqIcon(
    /// SVG path data (`d` attribute). When set, an inline `<svg>` is
    /// rendered automatically. Mutually exclusive with children.
    #[props(into, default)]
    path: String,
    #[props(default)]
    size: IconSize,
    #[props(default = false)]
    muted: bool,
    /// Optional class override — extend or replace default styles.
    #[props(into, default)]
    class: String,
    children: Option<Element>,
) -> Element {
    let size_class = match size {
        IconSize::Sm => s::SM,
        IconSize::Md => s::MD,
        IconSize::Lg => s::LG,
    };
    let color_class = if muted { s::MUTED } else { s::DEFAULT };
    let base = format!("{} {} {}", s::WRAPPER, size_class, color_class);
    let cls = merge_classes(&base, &class);

    rsx! {
        span { class: "{cls}",
            if !path.is_empty() {
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 256 256",
                    fill: "currentColor",
                    class: "w-full h-full",
                    path { d: "{path}" }
                }
            } else if let Some(content) = children {
                {content}
            }
        }
    }
}
