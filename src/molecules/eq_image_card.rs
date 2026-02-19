use super::eq_image_card_styles as s;
use crate::atoms::{AspectRatio, AtomImageSize, EqImage, ObjectFit};
use dioxus::prelude::*;

/// Controls how the caption is displayed relative to the image.
#[derive(Clone, PartialEq, Default)]
pub enum CaptionMode {
    /// Caption text below the image (figure/figcaption).
    #[default]
    Below,
    /// Caption text overlaid on the image with a gradient backdrop.
    Overlay,
}

/// Image card molecule.
/// Composes EqImage with title, description, and attribution text.
/// Supports two layout modes: caption below or overlay.
#[component]
pub fn EqImageCard(
    /// Image source — Asset or URL string.
    #[props(into)]
    src: String,
    /// Alt text (required for accessibility).
    #[props(into)]
    alt: String,
    /// Caption layout mode.
    #[props(default)]
    mode: CaptionMode,
    /// Image size preset.
    #[props(default)]
    size: AtomImageSize,
    /// Aspect ratio constraint.
    #[props(default)]
    aspect_ratio: AspectRatio,
    /// How the image fills its container.
    #[props(default)]
    object_fit: ObjectFit,
    /// Apply rounded corners.
    #[props(default = false)]
    rounded: bool,
    /// Optional title above description.
    #[props(into)]
    title: Option<String>,
    /// Optional description text.
    #[props(into)]
    description: Option<String>,
    /// Optional attribution / photo credit.
    #[props(into)]
    attribution: Option<String>,
) -> Element {
    let has_caption = title.is_some() || description.is_some() || attribution.is_some();

    match mode {
        CaptionMode::Below => rsx! {
            figure { class: s::CARD_WRAPPER,
                EqImage {
                    src,
                    alt,
                    size,
                    aspect_ratio,
                    object_fit,
                    rounded,
                }
                if has_caption {
                    figcaption { class: s::FIGCAPTION,
                        if let Some(t) = title {
                            p { class: s::CAPTION_TITLE, "{t}" }
                        }
                        if let Some(d) = description {
                            p { class: s::CAPTION_DESCRIPTION, "{d}" }
                        }
                        if let Some(a) = attribution {
                            p { class: s::CAPTION_ATTRIBUTION, "{a}" }
                        }
                    }
                }
            }
        },
        CaptionMode::Overlay => rsx! {
            div { class: s::OVERLAY_CONTAINER,
                EqImage {
                    src,
                    alt,
                    size,
                    aspect_ratio,
                    object_fit,
                    rounded,
                }
                if has_caption {
                    div { class: s::OVERLAY_GRADIENT,
                        div { class: s::OVERLAY_TEXT_WRAPPER,
                            if let Some(t) = title {
                                p { class: s::OVERLAY_TITLE, "{t}" }
                            }
                            if let Some(d) = description {
                                p { class: s::OVERLAY_DESCRIPTION, "{d}" }
                            }
                            if let Some(a) = attribution {
                                p { class: s::OVERLAY_ATTRIBUTION, "{a}" }
                            }
                        }
                    }
                }
            }
        },
    }
}
