use super::eq_carousel_styles as s;
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
    /// Defaults to 12 when omitted.
    #[props(into, default)]
    gap: Option<u32>,
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

    // Shared arrow SVG paths
    let chevron_left = "M15.75 19.5 8.25 12l7.5-7.5";
    let chevron_right = "m8.25 4.5 7.5 7.5-7.5 7.5";

    match mode {
        CarouselMode::Default => {
            // translateX offset: slide index × 100%
            let offset = format!("transform: translateX(-{}%);", current() * 100);

            rsx! {
                div { class: s::CAROUSEL,

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
                        button {
                            class: "{s::ARROW_BASE} {s::ARROW_LEFT}",
                            onclick: go_prev,
                            svg {
                                class: s::ARROW_ICON,
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none", view_box: "0 0 24 24",
                                stroke_width: "2", stroke: "currentColor",
                                path { d: "{chevron_left}" }
                            }
                        }
                        button {
                            class: "{s::ARROW_BASE} {s::ARROW_RIGHT}",
                            onclick: go_next,
                            svg {
                                class: s::ARROW_ICON,
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none", view_box: "0 0 24 24",
                                stroke_width: "2", stroke: "currentColor",
                                path { d: "{chevron_right}" }
                            }
                        }
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
            let gap_px = gap.unwrap_or(12);

            // In peek mode each slide is 80% wide with a gap between them.
            // The strip uses CSS `gap`, so the Nth slide starts at:
            //   N * 80% + N * gap_px
            // We centre it by adding 10% (half of the remaining 20%).
            // Use calc() to mix % and px units.
            let idx = current() as u32;
            let offset = format!(
                "transform: translateX(calc(-{}% + 10% - {}px));",
                idx * 80,
                idx * gap_px,
            );
            let strip_gap = format!("gap: {}px;", gap_px);

            // Fade gradient using the page background colour
            let fade_left = "background: linear-gradient(to right, var(--color-background) 0%, transparent 100%);";
            let fade_right = "background: linear-gradient(to left, var(--color-background) 0%, transparent 100%);";

            rsx! {
                div { class: s::CAROUSEL_PEEK,

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
                        button {
                            class: "{s::ARROW_BASE} {s::ARROW_LEFT}",
                            onclick: go_prev,
                            svg {
                                class: s::ARROW_ICON,
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none", view_box: "0 0 24 24",
                                stroke_width: "2", stroke: "currentColor",
                                path { d: "{chevron_left}" }
                            }
                        }
                        button {
                            class: "{s::ARROW_BASE} {s::ARROW_RIGHT}",
                            onclick: go_next,
                            svg {
                                class: s::ARROW_ICON,
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none", view_box: "0 0 24 24",
                                stroke_width: "2", stroke: "currentColor",
                                path { d: "{chevron_right}" }
                            }
                        }
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

