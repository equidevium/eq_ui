use super::eq_image_styles as s;
use dioxus::prelude::*;

/// Image size preset.
#[derive(Clone, PartialEq, Default)]
pub enum AtomImageSize {
    Sm,
    #[default]
    Md,
    Lg,
    Full,
}

/// Aspect ratio constraint.
#[derive(Clone, PartialEq, Default)]
pub enum AspectRatio {
    Ratio16_9,
    Ratio4_3,
    Square,
    #[default]
    Free,
}

/// How the image fills its container.
#[derive(Clone, PartialEq, Default)]
pub enum ObjectFit {
    #[default]
    Cover,
    Contain,
    Fill,
}

/// Atomic image component.
/// Renders a styled `<img>` inside a sized wrapper with lazy loading.
/// Accepts both Dioxus `Asset` (via `asset!()`) and URL strings as `src`.
#[component]
pub fn EqImage(
    /// Image source — Asset or URL string.
    #[props(into)]
    src: String,
    /// Alt text (required for accessibility).
    #[props(into)]
    alt: String,
    /// Size preset.
    #[props(default)]
    size: AtomImageSize,
    /// Aspect ratio constraint.
    #[props(default)]
    aspect_ratio: AspectRatio,
    /// How the image fills the container.
    #[props(default)]
    object_fit: ObjectFit,
    /// Apply rounded corners.
    #[props(default = false)]
    rounded: bool,
) -> Element {
    let size_class = match size {
        AtomImageSize::Sm => s::SM,
        AtomImageSize::Md => s::MD,
        AtomImageSize::Lg => s::LG,
        AtomImageSize::Full => s::FULL,
    };
    let ratio_class = match aspect_ratio {
        AspectRatio::Ratio16_9 => s::RATIO_16_9,
        AspectRatio::Ratio4_3 => s::RATIO_4_3,
        AspectRatio::Square => s::RATIO_SQUARE,
        AspectRatio::Free => s::RATIO_FREE,
    };
    let fit_class = match object_fit {
        ObjectFit::Cover => s::OBJECT_COVER,
        ObjectFit::Contain => s::OBJECT_CONTAIN,
        ObjectFit::Fill => s::OBJECT_FILL,
    };
    let rounded_class = if rounded { s::ROUNDED } else { "" };

    rsx! {
        div { class: "{s::WRAPPER} {size_class} {ratio_class} {rounded_class}",
            img {
                class: "{s::IMAGE_ELEMENT} {fit_class} {rounded_class}",
                src: "{src}",
                alt: "{alt}",
                loading: "lazy",
            }
        }
    }
}
