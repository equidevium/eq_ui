use super::eq_video_styles as s;
use super::{AspectRatio, AtomImageSize};
use crate::theme::merge_classes;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropToggle, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Atomic video component.
///
/// Renders a styled `<video>` inside a sized wrapper. Uses the native HTML
/// `poster` attribute for thumbnail display - the browser handles the
/// poster-to-playback transition without any JavaScript.
#[component]
pub fn EqVideo(
    /// Video source URL.
    #[props(into)]
    src: String,
    /// Poster/thumbnail URL - rendered via the native HTML poster attribute.
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
    /// Optional class override - extend or replace default wrapper styles.
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

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-video",
        name: "EqVideo",
        category: ComponentCategory::Atom,
        description: "Styled video player with size presets, aspect ratio constraints, \
                      native browser controls, and optional poster image.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Basic video with controls",
                code: "EqVideo {\n    src: \"https://example.com/video.mp4\",\n    controls: true,\n    rounded: true,\n}".into(),
            },
            UsageExample {
                label: "With poster and autoplay",
                code: "EqVideo {\n    src: \"https://example.com/video.mp4\",\n    poster: \"https://example.com/thumb.jpg\",\n    muted: true,\n    autoplay: true,\n}".into(),
            },
        ],
        render_demo: || rsx! { DemoEqVideo {} },
        render_gallery: || rsx! { GalleryEqVideo {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqVideo() -> Element {
    let mut autoplay = use_signal(|| false);
    let mut muted = use_signal(|| true);
    let mut loop_video = use_signal(|| false);
    let mut controls = use_signal(|| true);
    let mut rounded = use_signal(|| true);
    let mut size_str = use_signal(|| "Full".to_string());
    let mut ratio_str = use_signal(|| "Ratio16_9".to_string());
    let mut show_poster = use_signal(|| true);

    let size = match size_str().as_str() {
        "Sm" => AtomImageSize::Sm,
        "Md" => AtomImageSize::Md,
        "Lg" => AtomImageSize::Lg,
        _ => AtomImageSize::Full,
    };
    let aspect_ratio = match ratio_str().as_str() {
        "Ratio4_3" => AspectRatio::Ratio4_3,
        "Square" => AspectRatio::Square,
        "Free" => AspectRatio::Free,
        _ => AspectRatio::Ratio16_9,
    };

    let poster_url = if show_poster() {
        "https://picsum.photos/seed/eq-video/1280/720".to_string()
    } else {
        String::new()
    };

    let code = "EqVideo {\n    src: \"https://example.com/video.mp4\",\n    controls: true,\n    rounded: true,\n}\n\nEqVideo {\n    src: \"https://example.com/video.mp4\",\n    poster: \"https://example.com/thumb.jpg\",\n    muted: true,\n    loop_video: true,\n}".to_string();

    rsx! {
        DemoSection { title: "EqVideo",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "size",
                    value: size_str(),
                    options: vec!["Sm", "Md", "Lg", "Full"],
                    onchange: move |v: String| size_str.set(v),
                }
                PropSelect {
                    label: "ratio",
                    value: ratio_str(),
                    options: vec!["Ratio16_9", "Ratio4_3", "Square", "Free"],
                    onchange: move |v: String| ratio_str.set(v),
                }
                PropToggle {
                    label: "autoplay",
                    value: autoplay(),
                    onchange: move |v: bool| autoplay.set(v),
                }
                PropToggle {
                    label: "muted",
                    value: muted(),
                    onchange: move |v: bool| muted.set(v),
                }
                PropToggle {
                    label: "loop",
                    value: loop_video(),
                    onchange: move |v: bool| loop_video.set(v),
                }
                PropToggle {
                    label: "controls",
                    value: controls(),
                    onchange: move |v: bool| controls.set(v),
                }
                PropToggle {
                    label: "rounded",
                    value: rounded(),
                    onchange: move |v: bool| rounded.set(v),
                }
                PropToggle {
                    label: "poster",
                    value: show_poster(),
                    onchange: move |v: bool| show_poster.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] overflow-hidden p-4",
                EqVideo {
                    src: "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
                    poster: poster_url,
                    size,
                    aspect_ratio,
                    autoplay: autoplay(),
                    muted: muted(),
                    loop_video: loop_video(),
                    controls: controls(),
                    rounded: rounded(),
                }
            }
            StyleInfo { file: "eq_video_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqVideo() -> Element {
    rsx! {
        div { class: "space-y-4",
            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Size Presets" }
            div { class: "space-y-3",
                div { class: "space-y-1",
                    EqText { variant: TextVariant::Muted, class: "text-xs font-medium uppercase", "Small" }
                    EqVideo {
                        src: "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
                        poster: "https://picsum.photos/seed/eq-video-sm/400/300",
                        size: AtomImageSize::Sm,
                        controls: true,
                        rounded: true,
                    }
                }
                div { class: "space-y-1",
                    EqText { class: "text-xs font-medium uppercase text-[var(--color-label-secondary)]", "Medium" }
                    EqVideo {
                        src: "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
                        poster: "https://picsum.photos/seed/eq-video-md/512/384",
                        size: AtomImageSize::Md,
                        controls: true,
                        rounded: true,
                    }
                }
            }

            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider mt-6", "Aspect Ratios" }
            div { class: "space-y-3",
                div { class: "space-y-1",
                    EqText { variant: TextVariant::Muted, class: "text-xs font-medium uppercase", "16:9" }
                    EqVideo {
                        src: "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
                        poster: "https://picsum.photos/seed/eq-video-16-9/640/360",
                        size: AtomImageSize::Lg,
                        aspect_ratio: AspectRatio::Ratio16_9,
                        controls: true,
                        rounded: true,
                    }
                }
                div { class: "space-y-1",
                    EqText { variant: TextVariant::Muted, class: "text-xs font-medium uppercase", "Square" }
                    EqVideo {
                        src: "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
                        poster: "https://picsum.photos/seed/eq-video-square/400/400",
                        size: AtomImageSize::Lg,
                        aspect_ratio: AspectRatio::Square,
                        controls: true,
                        rounded: true,
                    }
                }
            }
        }
    }
}
