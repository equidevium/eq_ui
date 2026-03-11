use super::eq_image_card_styles as s;
use crate::atoms::{AspectRatio, AtomImageSize, EqImage, ObjectFit};
use crate::theme::merge_classes;
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

/// Internal helper — renders title, description, and attribution text.
#[component]
fn CaptionContent(
    title_class: &'static str,
    description_class: &'static str,
    attribution_class: &'static str,
    title: Option<String>,
    description: Option<String>,
    attribution: Option<String>,
) -> Element {
    rsx! {
        if let Some(t) = title {
            p { class: title_class, "{t}" }
        }
        if let Some(d) = description {
            p { class: description_class, "{d}" }
        }
        if let Some(a) = attribution {
            p { class: attribution_class, "{a}" }
        }
    }
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
    /// Optional class override — extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let has_caption = title.is_some() || description.is_some() || attribution.is_some();

    match mode {
        CaptionMode::Below => {
            let cls = merge_classes(s::CARD_WRAPPER, &class);
            rsx! {
            figure { class: "{cls}",
                EqImage { src, alt, size, aspect_ratio, object_fit, rounded }
                if has_caption {
                    figcaption { class: s::FIGCAPTION,
                        CaptionContent {
                            title_class: s::CAPTION_TITLE,
                            description_class: s::CAPTION_DESCRIPTION,
                            attribution_class: s::CAPTION_ATTRIBUTION,
                            title, description, attribution,
                        }
                    }
                }
            }
        }},
        CaptionMode::Overlay => {
            let cls = merge_classes(s::OVERLAY_CONTAINER, &class);
            rsx! {
            div { class: "{cls}",
                EqImage { src, alt, size, aspect_ratio, object_fit, rounded }
                if has_caption {
                    div { class: s::OVERLAY_GRADIENT,
                        div { class: s::OVERLAY_TEXT_WRAPPER,
                            CaptionContent {
                                title_class: s::OVERLAY_TITLE,
                                description_class: s::OVERLAY_DESCRIPTION,
                                attribution_class: s::OVERLAY_ATTRIBUTION,
                                title, description, attribution,
                            }
                        }
                    }
                }
            }
        }},
    }
}
