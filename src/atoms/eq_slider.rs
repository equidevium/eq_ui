//! EqSlider — range slider atom.
//!
//! A themed wrapper around `<input type="range">` with size variants,
//! optional value label, disabled state, and full accessibility.
//!
//! ```rust,ignore
//! let mut volume = use_signal(|| 50.0);
//!
//! EqSlider {
//!     value: volume(),
//!     on_change: move |v| volume.set(v),
//!     min: 0.0,
//!     max: 100.0,
//!     step: 1.0,
//!     show_label: true,
//! }
//! ```

use super::eq_slider_styles as s;
use crate::theme::merge_classes;
use crate::{PlaygroundEnum, playground};
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropToggle, PropInput, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// ── Types ─────────────────────────────────────────────────────────

/// Size of the slider track.
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
pub enum SliderSize {
    Sm,
    #[default]
    Md,
    Lg,
}

// ── Component ─────────────────────────────────────────────────────

/// Themed range slider.
///
/// Wraps a native `<input type="range">` with accent-color theming,
/// size variants, and an optional numeric label. Controlled component
/// pattern: pass `value` + `on_change`.
///
/// **Accessibility** — uses native `<input type="range">` which
/// provides built-in keyboard navigation (arrow keys) and screen
/// reader support. An `aria-label` prop is available for custom
/// accessible names. `aria-valuemin`, `aria-valuemax`, and
/// `aria-valuenow` are set automatically.
#[playground(
    category = Atom,
    description = "Range slider wrapping native <input type=\"range\"> with \
                   accent-color theming, three sizes, optional value label, \
                   and disabled state.",
    examples = [
        ("Basic", "let mut val = use_signal(|| 50.0);\n\nEqSlider {\n    value: val(),\n    on_change: move |v| val.set(v),\n}"),
        ("With label", "EqSlider {\n    value: brightness(),\n    min: 0.0,\n    max: 100.0,\n    show_label: true,\n    on_change: move |v| brightness.set(v),\n}"),
        ("Custom range", "EqSlider {\n    value: temperature(),\n    min: -20.0,\n    max: 45.0,\n    step: 0.5,\n    size: SliderSize::Lg,\n    show_label: true,\n    on_change: move |v| temperature.set(v),\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqSlider(
    /// Current value.
    #[props(default = 50.0)]
    value: f64,
    /// Minimum value.
    #[props(default = 0.0)]
    min: f64,
    /// Maximum value.
    #[props(default = 100.0)]
    max: f64,
    /// Step increment.
    #[props(default = 1.0)]
    step: f64,
    /// Fired when the user moves the slider. Receives the new value.
    #[props(default)]
    on_change: Option<EventHandler<f64>>,
    /// Disables interaction and dims the visual.
    #[props(default = false)]
    disabled: bool,
    /// Size of the slider track.
    #[props(default)]
    size: SliderSize,
    /// Show a numeric value label beside the slider.
    #[props(default = false)]
    show_label: bool,
    /// Accessible label for screen readers.
    #[props(into, default)]
    aria_label: String,
    /// Optional class override on the wrapper element.
    #[props(into, default)]
    class: String,
) -> Element {
    let wrapper_cls = merge_classes(s::WRAPPER, &class);

    let size_cls = match size {
        SliderSize::Sm => s::SM,
        SliderSize::Md => s::MD,
        SliderSize::Lg => s::LG,
    };

    let input_cls = if disabled {
        format!("{} {} {}", s::INPUT, size_cls, s::DISABLED)
    } else {
        format!("{} {}", s::INPUT, size_cls)
    };

    // Format label: show integer if step >= 1, otherwise one decimal place.
    let label_text = if step >= 1.0 {
        format!("{}", value as i64)
    } else {
        format!("{:.1}", value)
    };

    let aria = if aria_label.is_empty() { None } else { Some(aria_label.clone()) };

    rsx! {
        div { class: "{wrapper_cls}",
            input {
                r#type: "range",
                class: "{input_cls}",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: "{value}",
                disabled: disabled,
                "aria-label": aria,
                "aria-valuemin": "{min}",
                "aria-valuemax": "{max}",
                "aria-valuenow": "{value}",
                oninput: move |evt: Event<FormData>| {
                    if let Some(handler) = &on_change {
                        if let Ok(v) = evt.value().parse::<f64>() {
                            handler.call(v);
                        }
                    }
                },
            }
            if show_label {
                span { class: "{s::LABEL}", "{label_text}" }
            }
        }
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqSlider() -> Element {
    let mut value = use_signal(|| 50.0);
    let mut min = use_signal(|| "0".to_string());
    let mut max = use_signal(|| "100".to_string());
    let mut step = use_signal(|| "1".to_string());
    let mut size_str = use_signal(|| "Md".to_string());
    let mut show_label = use_signal(|| true);
    let mut disabled = use_signal(|| false);

    let size = match size_str().as_str() {
        "Sm" => SliderSize::Sm,
        "Lg" => SliderSize::Lg,
        _ => SliderSize::Md,
    };

    let min_val: f64 = min().parse().unwrap_or(0.0);
    let max_val: f64 = max().parse().unwrap_or(100.0);
    let step_val: f64 = step().parse().unwrap_or(1.0);

    let code = format!(
        r#"let mut val = use_signal(|| {val});

EqSlider {{
    value: val(),
    min: {mn},
    max: {mx},
    step: {st},
    size: SliderSize::{sz},
    show_label: {sl},
    disabled: {dis},
    on_change: move |v| val.set(v),
}}"#,
        val = value(),
        mn = min_val,
        mx = max_val,
        st = step_val,
        sz = size_str(),
        sl = show_label(),
        dis = disabled(),
    );

    rsx! {
        DemoSection { title: "EqSlider",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "size",
                    value: size_str(),
                    options: vec!["Sm", "Md", "Lg"],
                    onchange: move |v: String| size_str.set(v),
                }
                PropInput {
                    label: "min",
                    value: min(),
                    placeholder: "0",
                    onchange: move |v: String| min.set(v),
                }
                PropInput {
                    label: "max",
                    value: max(),
                    placeholder: "100",
                    onchange: move |v: String| max.set(v),
                }
                PropInput {
                    label: "step",
                    value: step(),
                    placeholder: "1",
                    onchange: move |v: String| step.set(v),
                }
                PropToggle {
                    label: "show_label",
                    value: show_label(),
                    onchange: move |v: bool| show_label.set(v),
                }
                PropToggle {
                    label: "disabled",
                    value: disabled(),
                    onchange: move |v: bool| disabled.set(v),
                }
            }

            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 space-y-6",
                // Main interactive slider
                div { class: "space-y-2",
                    EqText { variant: TextVariant::Muted, "Drag the slider:" }
                    EqSlider {
                        value: value(),
                        min: min_val,
                        max: max_val,
                        step: step_val,
                        size,
                        show_label: show_label(),
                        disabled: disabled(),
                        on_change: move |v: f64| value.set(v),
                    }
                }

                // Size comparison
                div { class: "space-y-3",
                    EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Size comparison" }
                    for (sz_label, sz_variant) in [("Sm", SliderSize::Sm), ("Md", SliderSize::Md), ("Lg", SliderSize::Lg)] {
                        div { class: "flex items-center gap-3",
                            EqText { variant: TextVariant::Muted, class: "w-8 shrink-0", "{sz_label}" }
                            EqSlider {
                                value: value(),
                                min: min_val,
                                max: max_val,
                                step: step_val,
                                size: sz_variant,
                                show_label: true,
                                on_change: move |v: f64| value.set(v),
                            }
                        }
                    }
                }
            }

            StyleInfo { file: "eq_slider_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery ───────────────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqSlider() -> Element {
    let mut vol = use_signal(|| 70.0);
    let mut temp = use_signal(|| 22.0);
    let mut progress = use_signal(|| 33.0);

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Slider Gallery" }

                div { class: "space-y-3",
                    // Volume
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Volume" }
                        EqSlider {
                            value: vol(),
                            show_label: true,
                            on_change: move |v: f64| vol.set(v),
                        }
                    }

                    // Temperature
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Temperature (\u{00B0}C)" }
                        EqSlider {
                            value: temp(),
                            min: -20.0,
                            max: 45.0,
                            step: 0.5,
                            size: SliderSize::Lg,
                            show_label: true,
                            on_change: move |v: f64| temp.set(v),
                        }
                    }

                    // Progress (small, disabled)
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Progress (disabled)" }
                        EqSlider {
                            value: progress(),
                            size: SliderSize::Sm,
                            show_label: true,
                            disabled: true,
                            on_change: move |v: f64| progress.set(v),
                        }
                    }
                }
            }
        }
    }
}
