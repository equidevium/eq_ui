use super::eq_carousel_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropInput, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant, AtomImageSize, AspectRatio, ObjectFit};
#[cfg(feature = "playground")]
use crate::molecules::eq_image_card::{EqImageCard, CaptionMode};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// ---------------------------------------------------------------------------
// CarouselMode
// ---------------------------------------------------------------------------

/// Controls the visual presentation of the carousel.
#[derive(Clone, PartialEq, Default)]
pub enum CarouselMode {
    /// Standard full-width slides - one at a time with a smooth slide transition.
    #[default]
    Default,
    /// Peek mode - the current slide is centred while the previous and next
    /// slides are partially visible on either side, fading out with a gradient mask.
    Peek,
}

// ---------------------------------------------------------------------------
// Internal arrow button helper
// ---------------------------------------------------------------------------

/// Reusable arrow button used by both Default and Peek carousel modes.
#[component]
fn CarouselArrow(
    /// Position class - `s::ARROW_LEFT` or `s::ARROW_RIGHT`.
    position: &'static str,
    /// SVG path data for the chevron direction.
    chevron: &'static str,
    /// Click handler.
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "{s::ARROW_BASE} {position}",
            onclick: move |evt| onclick.call(evt),
            svg {
                class: s::ARROW_ICON,
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none", view_box: "0 0 24 24",
                stroke_width: "2", stroke: "currentColor",
                path { d: "{chevron}" }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// EqCarousel
// ---------------------------------------------------------------------------

/// Generic carousel molecule.
///
/// Cycles through any content passed as a `Vec<Element>`.
/// Shows previous/next arrows and dot indicators when there are multiple slides.
/// Supports two modes: `Default` (full-width) and `Peek` (show neighbours).
#[component]
pub fn EqCarousel(
    /// The slides to cycle through.
    slides: Vec<Element>,
    /// Visual mode - `Default` or `Peek`.
    #[props(default)]
    mode: CarouselMode,
    /// Gap between slides in pixels (only applies in Peek mode).
    #[props(default = 12)]
    gap: u32,
    /// Optional class override - extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let mut current = use_signal(|| 0usize);
    let len = slides.len();

    let go_prev = move |_| {
        current.set(if current() == 0 { len - 1 } else { current() - 1 });
    };
    let go_next = move |_| {
        current.set(if current() + 1 >= len { 0 } else { current() + 1 });
    };

    let show_controls = len > 1;

    match mode {
        CarouselMode::Default => {
            let offset = format!("transform: translateX(-{}%);", current() * 100);

            let cls = merge_classes(s::CAROUSEL, &class);
            rsx! {
                div { class: "{cls}",

                    // Slide strip
                    div {
                        class: s::SLIDE_STRIP,
                        style: "{offset}",
                        for (i, slide) in slides.into_iter().enumerate() {
                            div {
                                key: "{i}",
                                class: s::SLIDE,
                                {slide}
                            }
                        }
                    }

                    // Arrows
                    if show_controls {
                        CarouselArrow { position: s::ARROW_LEFT, chevron: "M15.75 19.5 8.25 12l7.5-7.5", onclick: go_prev }
                        CarouselArrow { position: s::ARROW_RIGHT, chevron: "m8.25 4.5 7.5 7.5-7.5 7.5", onclick: go_next }
                    }

                    // Dots
                    if show_controls {
                        div { class: s::DOTS,
                            for i in 0..len {
                                span {
                                    key: "{i}",
                                    class: if current() == i { s::DOT_ACTIVE } else { s::DOT },
                                    onclick: move |_| current.set(i),
                                }
                            }
                        }
                    }
                }
            }
        }

        CarouselMode::Peek => {
            let idx = current() as u32;
            let offset = format!(
                "transform: translateX(calc(-{}% + 10% - {}px));",
                idx * 80,
                idx * gap,
            );
            let strip_gap = format!("gap: {}px;", gap);

            let fade_left = "background: linear-gradient(to right, var(--color-background) 0%, transparent 100%);";
            let fade_right = "background: linear-gradient(to left, var(--color-background) 0%, transparent 100%);";

            let cls = merge_classes(s::CAROUSEL_PEEK, &class);
            rsx! {
                div { class: "{cls}",

                    // Slide strip
                    div {
                        class: s::SLIDE_STRIP_PEEK,
                        style: "{strip_gap} {offset}",
                        for (i, slide) in slides.into_iter().enumerate() {
                            div {
                                key: "{i}",
                                class: s::SLIDE_PEEK,
                                style: "width: 80%;",
                                {slide}
                            }
                        }
                    }

                    // Fade masks
                    div {
                        class: s::PEEK_FADE_LEFT,
                        style: "width: 12%; {fade_left}",
                    }
                    div {
                        class: s::PEEK_FADE_RIGHT,
                        style: "width: 12%; {fade_right}",
                    }

                    // Arrows
                    if show_controls {
                        CarouselArrow { position: s::ARROW_LEFT, chevron: "M15.75 19.5 8.25 12l7.5-7.5", onclick: go_prev }
                        CarouselArrow { position: s::ARROW_RIGHT, chevron: "m8.25 4.5 7.5 7.5-7.5 7.5", onclick: go_next }
                    }

                    // Dots
                    if show_controls {
                        div { class: s::DOTS,
                            for i in 0..len {
                                span {
                                    key: "{i}",
                                    class: if current() == i { s::DOT_ACTIVE } else { s::DOT },
                                    onclick: move |_| current.set(i),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-carousel",
        name: "EqCarousel",
        category: ComponentCategory::Molecule,
        description: "Carousel molecule cycling through slides with arrow navigation and dot indicators. \
                      Supports default full-width and peek (neighbours visible) modes.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Default mode",
                code: "EqCarousel {\n    slides: vec![\n        rsx! { /* slide content */ },\n        rsx! { /* slide content */ },\n    ],\n}".into(),
            },
            UsageExample {
                label: "Peek mode",
                code: "EqCarousel {\n    mode: CarouselMode::Peek,\n    gap: 24,\n    slides: vec![ /* ... */ ],\n}".into(),
            },
        ],
        render_demo: || rsx! { DemoEqCarousel {} },
        render_gallery: || rsx! { GalleryEqCarousel {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqCarousel() -> Element {
    let mut mode_str = use_signal(|| "Default".to_string());
    let mut gap_str = use_signal(|| "12".to_string());

    let mode = match mode_str().as_str() {
        "Peek" => CarouselMode::Peek,
        _ => CarouselMode::Default,
    };
    let gap_val: u32 = gap_str().parse().unwrap_or(12);

    let slides = vec![
        rsx! {
            EqImageCard {
                src: "https://picsum.photos/seed/carousel1/800/450",
                alt: "Slide one",
                mode: CaptionMode::Overlay,
                size: AtomImageSize::Full,
                aspect_ratio: AspectRatio::Ratio16_9,
                rounded: true,
                title: "First Slide",
                description: "A beautiful mountain landscape.",
            }
        },
        rsx! {
            EqImageCard {
                src: "https://picsum.photos/seed/carousel2/800/450",
                alt: "Slide two",
                mode: CaptionMode::Overlay,
                size: AtomImageSize::Full,
                aspect_ratio: AspectRatio::Ratio16_9,
                rounded: true,
                title: "Second Slide",
                description: "Waves crashing on the shore.",
            }
        },
        rsx! {
            EqImageCard {
                src: "https://picsum.photos/seed/carousel3/800/450",
                alt: "Slide three",
                mode: CaptionMode::Overlay,
                size: AtomImageSize::Full,
                aspect_ratio: AspectRatio::Ratio16_9,
                rounded: true,
                title: "Third Slide",
                description: "A dense forest at dawn.",
            }
        },
        rsx! {
            EqImageCard {
                src: "https://picsum.photos/seed/carousel4/800/450",
                alt: "Slide four",
                mode: CaptionMode::Overlay,
                size: AtomImageSize::Full,
                aspect_ratio: AspectRatio::Ratio16_9,
                rounded: true,
                title: "Fourth Slide",
                description: "Sunset over the desert.",
            }
        },
    ];

    let code = "// Default carousel with slide animation\nEqCarousel {\n    slides: vec![\n        rsx! { /* slide content */ },\n        rsx! { /* slide content */ },\n    ],\n}\n\n// Peek mode - shows neighbours with fade\nEqCarousel {\n    mode: CarouselMode::Peek,\n    gap: 24,  // default 12px\n    slides: vec![ /* ... */ ],\n}".to_string();

    rsx! {
        DemoSection { title: "EqCarousel",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "mode",
                    value: mode_str(),
                    options: vec!["Default", "Peek"],
                    onchange: move |v: String| mode_str.set(v),
                }
                PropInput {
                    label: "gap (px)",
                    value: gap_str(),
                    placeholder: "12",
                    onchange: move |v: String| gap_str.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                EqCarousel { mode, gap: gap_val, slides }
            }
            StyleInfo { file: "eq_carousel_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqCarousel() -> Element {
    let slides = vec![
        rsx! {
            EqImageCard {
                src: "https://picsum.photos/seed/carousel-gallery-1/600/350",
                alt: "Gallery slide 1",
                mode: CaptionMode::Overlay,
                title: "Slide 1",
            }
        },
        rsx! {
            EqImageCard {
                src: "https://picsum.photos/seed/carousel-gallery-2/600/350",
                alt: "Gallery slide 2",
                mode: CaptionMode::Overlay,
                title: "Slide 2",
            }
        },
        rsx! {
            EqImageCard {
                src: "https://picsum.photos/seed/carousel-gallery-3/600/350",
                alt: "Gallery slide 3",
                mode: CaptionMode::Overlay,
                title: "Slide 3",
            }
        },
    ];

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Default Mode" }
                EqCarousel { mode: CarouselMode::Default, slides: slides.clone() }
            }

            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Peek Mode" }
                EqCarousel { mode: CarouselMode::Peek, gap: 16, slides }
            }
        }
    }
}
