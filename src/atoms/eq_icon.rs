use dioxus::prelude::*;
use super::eq_icon_styles as s;

/// Icon size variant.
#[derive(Clone, PartialEq, Default)]
pub enum IconSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Atomic icon wrapper.
/// Pass an inline SVG or an `<img>` element as children.
/// The wrapper applies consistent sizing and colour.
///
/// **Inline SVG** — inherits `currentColor` from the colour class
/// so both size and colour are controlled.
///
/// **Image (`<img>`)** — add `class: "w-full h-full"` to the image
/// so it fills the wrapper.
#[component]
pub fn EqIcon(
    #[props(default)]
    size: IconSize,
    #[props(default = false)]
    muted: bool,
    children: Element,
) -> Element {
    let size_class = match size {
        IconSize::Sm => s::SM,
        IconSize::Md => s::MD,
        IconSize::Lg => s::LG,
    };
    let color_class = if muted { s::MUTED } else { s::DEFAULT };

    rsx! {
        span { class: "{s::WRAPPER} {size_class} {color_class}",
            {children}
        }
    }
}
