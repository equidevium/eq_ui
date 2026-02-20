use super::eq_carousel_styles as s;
use dioxus::prelude::*;

/// Generic carousel molecule.
/// Cycles through any content passed as a `Vec<Element>`.
/// Shows previous/next arrows and dot indicators when there are multiple slides.
#[component]
pub fn EqCarousel(
    /// The slides to cycle through.
    slides: Vec<Element>,
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

    rsx! {
        div { class: s::CAROUSEL,

            // Current slide
            div { class: s::SLIDE_CONTAINER,
                {slides.into_iter().nth(current()).unwrap_or(rsx! {})}
            }

            // Arrow buttons
            if show_controls {
                button {
                    class: "{s::ARROW_BASE} {s::ARROW_LEFT}",
                    onclick: go_prev,
                    svg {
                        class: s::ARROW_ICON,
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "2",
                        stroke: "currentColor",
                        path { d: "M15.75 19.5 8.25 12l7.5-7.5" }
                    }
                }
                button {
                    class: "{s::ARROW_BASE} {s::ARROW_RIGHT}",
                    onclick: go_next,
                    svg {
                        class: s::ARROW_ICON,
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "2",
                        stroke: "currentColor",
                        path { d: "m8.25 4.5 7.5 7.5-7.5 7.5" }
                    }
                }
            }

            // Dot indicators
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
