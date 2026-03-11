use super::eq_carousel_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// CarouselMode
// ---------------------------------------------------------------------------

/// Controls the visual presentation of the carousel.
#[derive(Clone, PartialEq, Default)]
pub enum CarouselMode {
    /// Standard full-width slides — one at a time with a smooth slide transition.
    #[default]
    Default,
    /// Peek mode — the current slide is centred while the previous and next
    /// slides are partially visible on either side, fading out with a gradient mask.
    Peek,
}

// ---------------------------------------------------------------------------
// Internal arrow button helper
// ---------------------------------------------------------------------------

/// Reusable arrow button used by both Default and Peek carousel modes.
#[component]
fn CarouselArrow(
    /// Position class — `s::ARROW_LEFT` or `s::ARROW_RIGHT`.
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
    /// Visual mode — `Default` or `Peek`.
    #[props(default)]
    mode: CarouselMode,
    /// Gap between slides in pixels (only applies in Peek mode).
    #[props(default = 12)]
    gap: u32,
    /// Optional class override — extend or replace default wrapper styles.
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
