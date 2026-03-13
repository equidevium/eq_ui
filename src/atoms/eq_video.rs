use super::eq_video_styles as s;
use super::{EqImage, AtomImageSize, AspectRatio, ObjectFit};
use crate::theme::merge_classes;
use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

/// Global counter for unique video element IDs.
static VIDEO_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Atomic video component.
///
/// Renders a styled `<video>` inside a sized wrapper. Supports an optional
/// EqImage poster overlay with a play icon and native controls toggle.
#[component]
pub fn EqVideo(
    /// Video source URL.
    #[props(into)]
    src: String,
    /// Poster/thumbnail URL — rendered as EqImage overlay with play icon.
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
    let mut show_poster = use_signal(|| !poster.is_empty());
    let video_id = use_memo(|| {
        let n = VIDEO_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("eq-video-{n}")
    });

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

    let wrapper_base = format!("{} {} {} {}", s::WRAPPER, size_class, ratio_class, rounded_class);
    let wrapper_cls = merge_classes(&wrapper_base, &class);

    let vid = video_id.to_string();

    // When poster is clicked, hide overlay and start playback.
    let on_poster_click = {
        let v = vid.clone();
        move |_| {
            show_poster.set(false);
            let js = format!(
                "var v=document.getElementById('{v}');\
                 if(v){{if(v.readyState>=3)v.play();\
                 else v.addEventListener('canplay',function(){{v.play();}},{{once:true}});}}"
            );
            document::eval(&js);
        }
    };

    rsx! {
        div { class: "{wrapper_cls}",
            video {
                id: "{vid}",
                class: "{s::VIDEO_ELEMENT} {rounded_class}",
                src: "{src}",
                autoplay: autoplay && poster.is_empty(),
                muted: muted,
                r#loop: loop_video,
                controls: controls,
            }

            if show_poster() {
                div {
                    class: s::POSTER_OVERLAY,
                    onclick: on_poster_click,
                    EqImage {
                        src: poster.clone(),
                        alt: "Video thumbnail",
                        size: AtomImageSize::Full,
                        aspect_ratio: aspect_ratio.clone(),
                        object_fit: ObjectFit::Cover,
                    }
                    div { class: s::PLAY_ICON,
                        div { class: s::PLAY_CIRCLE,
                            svg {
                                class: s::PLAY_SVG,
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                fill: "currentColor",
                                path { d: "M8 5v14l11-7z" }
                            }
                        }
                    }
                }
            }
        }
    }
}
