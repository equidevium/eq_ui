use super::eq_video_styles as s;
use super::{AspectRatio, AtomImageSize};
use crate::theme::merge_classes;
use dioxus::prelude::*;

/// Atomic video component.
///
/// Renders a styled `<video>` inside a sized wrapper. Uses the native HTML
/// `poster` attribute for thumbnail display — the browser handles the
/// poster-to-playback transition without any JavaScript.
#[component]
pub fn EqVideo(
    /// Video source URL.
    #[props(into)]
    src: String,
    /// Poster/thumbnail URL — rendered via the native HTML poster attribute.
    #[props(into, default)]
    poster: String,
    /// Size preset (reuses AtomImageSize from EqImage).
    #[props(default)]
    size: AtomImageSize,
    /// Aspect ratio constraint (reuses AspectRatio from EqImage).
    #[props(default = AspectRatio::Ratio16_9)]
    aspect_ratio: AspectRatio,
    /// Start playback automatically.
    #[props(default = false)]
    autoplay: bool,
    /// Start muted.
    #[props(default = false)]
    muted: bool,
    /// Loop the video.
    #[props(default = false)]
    loop_video: bool,
    /// Show native browser controls.
    #[props(default = true)]
    controls: bool,
    /// Apply rounded corners.
    #[props(default = false)]
    rounded: bool,
    /// Optional class override — extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let size_class = match size {
        AtomImageSize::Sm => s::SM,
        AtomImageSize::Md => s::MD,
        AtomImageSize::Lg => s::LG,
        AtomImageSize::Full => s::FULL,
    };
    let ratio_class = match &aspect_ratio {
        AspectRatio::Ratio16_9 => s::RATIO_16_9,
        AspectRatio::Ratio4_3 => s::RATIO_4_3,
        AspectRatio::Square => s::RATIO_SQUARE,
        AspectRatio::Free => s::RATIO_FREE,
    };
    let rounded_class = if rounded { s::ROUNDED } else { "" };

    let wrapper_base = format!(
        "{} {} {} {}",
        s::WRAPPER,
        size_class,
        ratio_class,
        rounded_class
    );
    let wrapper_cls = merge_classes(&wrapper_base, &class);

    rsx! {
        div { class: "{wrapper_cls}",
            video {
                class: "{s::VIDEO_ELEMENT} {rounded_class}",
                src: "{src}",
                poster: if !poster.is_empty() { "{poster}" },
                autoplay,
                muted,
                r#loop: loop_video,
                controls,
            }
        }
    }
}
