//! EqProgress - themed progress bar atom (pure Tailwind).
//!
//! Determinate mode shows a fill bar at a given percentage.
//! Indeterminate mode shows a sliding shimmer animation.
//! Three size presets control track height. Four semantic
//! variants set the fill colour. Gradient fill reuses the
//! theme's button gradient palette by default.
//!
//! ```rust,ignore
//! EqProgress { value: 0.65 }
//! EqProgress { value: 0.3, variant: ProgressVariant::Warning, label: true }
//! EqProgress { indeterminate: true, size: ProgressSize::Lg }
//! ```

use super::eq_progress_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropToggle, StyleInfo, format_catalog,
    PROP_ROW, PROP_LABEL, PROP_CONTROL,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Semantic colour variant for the progress fill.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ProgressVariant {
    /// Theme gradient fill (default).
    #[default]
    Default,
    /// Green - success / complete.
    Success,
    /// Amber - warning / caution.
    Warning,
    /// Red - danger / error.
    Danger,
}

/// Height preset for the progress track.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ProgressSize {
    /// 4px track height.
    Sm,
    /// 8px track height (default).
    #[default]
    Md,
    /// 12px track height.
    Lg,
}

/// Themed progress bar atom.
///
/// Renders a track with a fill bar whose width reflects the `value` prop
/// (0.0–1.0). When `indeterminate` is true the fill animates continuously
/// regardless of `value`. An optional label displays the percentage.
#[component]
pub fn EqProgress(
    /// Fill amount from 0.0 (empty) to 1.0 (full). Clamped internally.
    /// Ignored when `indeterminate` is true.
    #[props(default = 0.0)]
    value: f64,
    /// When true, shows a looping shimmer animation instead of a
    /// fixed fill. The `value` prop is ignored.
    #[props(default = false)]
    indeterminate: bool,
    /// Track height preset.
    #[props(default)]
    size: ProgressSize,
    /// Semantic colour variant for the fill bar.
    #[props(default)]
    variant: ProgressVariant,
    /// Use the theme gradient for the fill (default true).
    /// When false, uses a flat solid accent colour.
    /// Ignored when variant is Success/Warning/Danger.
    #[props(default = true)]
    gradient: bool,
    /// Show a percentage label to the right of the bar.
    #[props(default = false)]
    label: bool,
    /// Optional class override on the outermost element.
    #[props(into, default)]
    class: String,
) -> Element {
    let clamped = value.clamp(0.0, 1.0);
    let pct = (clamped * 100.0).round() as u32;

    let size_cls = match size {
        ProgressSize::Sm => s::SM,
        ProgressSize::Md => s::MD,
        ProgressSize::Lg => s::LG,
    };

    // Pick fill classes based on variant and gradient preference
    let fill_base = match variant {
        ProgressVariant::Success => s::FILL_SUCCESS,
        ProgressVariant::Warning => s::FILL_WARNING,
        ProgressVariant::Danger => s::FILL_DANGER,
        ProgressVariant::Default => {
            if gradient { s::FILL_GRADIENT } else { s::FILL_SOLID }
        }
    };

    // Track: base + size + user override
    let track_base = format!("{} {}", s::TRACK, size_cls);
    let track_cls = merge_classes(&track_base, &class);

    // Fill: base classes + indeterminate animation or width
    let (fill_cls, fill_style) = if indeterminate {
        (format!("{} {}", fill_base, s::INDETERMINATE_FILL), String::new())
    } else {
        (fill_base.to_string(), format!("width: {}%;", pct))
    };

    let label_text = format!("{}%", pct);

    if label {
        rsx! {
            div { class: "{s::WRAPPER}",
                div {
                    class: "{track_cls}",
                    role: "progressbar",
                    "aria-valuenow": if !indeterminate { "{clamped}" } else { "" },
                    "aria-valuemin": "0",
                    "aria-valuemax": "1",
                    div { class: "{fill_cls}", style: "{fill_style}" }
                }
                span { class: "{s::LABEL}", "{label_text}" }
            }
        }
    } else {
        rsx! {
            div {
                class: "{track_cls}",
                role: "progressbar",
                "aria-valuenow": if !indeterminate { "{clamped}" } else { "" },
                "aria-valuemin": "0",
                "aria-valuemax": "1",
                div { class: "{fill_cls}", style: "{fill_style}" }
            }
        }
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-progress",
        name: "EqProgress",
        category: ComponentCategory::Atom,
        description: "Themed progress bar with determinate/indeterminate modes, \
                      semantic variants, gradient fill, and size presets.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Basic",
                code: "EqProgress { value: 0.65 }".into(),
            },
            UsageExample {
                label: "With label & variant",
                code: "EqProgress {\n    value: 0.3,\n    variant: ProgressVariant::Warning,\n    label: true,\n}".into(),
            },
            UsageExample {
                label: "Indeterminate",
                code: "EqProgress { indeterminate: true, size: ProgressSize::Lg }".into(),
            },
        ],
        render_demo: || rsx! { DemoEqProgress {} },
        render_gallery: || rsx! { GalleryEqProgress {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqProgress() -> Element {
    let mut value = use_signal(|| 0.65f64);
    let mut variant_str = use_signal(|| "Default".to_string());
    let mut size_str = use_signal(|| "Md".to_string());
    let mut indeterminate = use_signal(|| false);
    let mut gradient = use_signal(|| true);
    let mut show_label = use_signal(|| false);

    let variant = match variant_str().as_str() {
        "Success" => ProgressVariant::Success,
        "Warning" => ProgressVariant::Warning,
        "Danger" => ProgressVariant::Danger,
        _ => ProgressVariant::Default,
    };

    let size = match size_str().as_str() {
        "Sm" => ProgressSize::Sm,
        "Lg" => ProgressSize::Lg,
        _ => ProgressSize::Md,
    };

    let pct_display = format!("{}%", (value() * 100.0).round() as u32);

    let code = "use eq_ui::atoms::{EqProgress, ProgressVariant, ProgressSize};\n\
        \n\
        // Determinate\n\
        EqProgress { value: 0.65 }\n\
        \n\
        // With label and variant\n\
        EqProgress {\n\
        \x20   value: 0.3,\n\
        \x20   variant: ProgressVariant::Warning,\n\
        \x20   label: true,\n\
        }\n\
        \n\
        // Indeterminate loading\n\
        EqProgress { indeterminate: true, size: ProgressSize::Lg }".to_string();

    rsx! {
        DemoSection { title: "EqProgress",
            // Prop controls
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                div { class: "grid grid-cols-2 md:grid-cols-3 gap-3",
                    PropSelect {
                        label: "variant",
                        value: variant_str(),
                        options: vec!["Default", "Success", "Warning", "Danger"],
                        onchange: move |v: String| variant_str.set(v),
                    }
                    PropSelect {
                        label: "size",
                        value: size_str(),
                        options: vec!["Sm", "Md", "Lg"],
                        onchange: move |v: String| size_str.set(v),
                    }
                    PropToggle {
                        label: "indeterminate",
                        value: indeterminate(),
                        onchange: move |v: bool| indeterminate.set(v),
                    }
                    PropToggle {
                        label: "gradient",
                        value: gradient(),
                        onchange: move |v: bool| gradient.set(v),
                    }
                    PropToggle {
                        label: "label",
                        value: show_label(),
                        onchange: move |v: bool| show_label.set(v),
                    }
                }

                // Value slider
                div { class: "pt-2",
                    EqText {
                        variant: TextVariant::Caption,
                        class: "font-semibold uppercase tracking-wider mb-1",
                        "Value: {pct_display}"
                    }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "100",
                        value: format!("{}", (value() * 100.0).round() as u32),
                        class: "w-full accent-[var(--color-accent-primary)]",
                        oninput: move |evt: Event<FormData>| {
                            if let Ok(v) = evt.value().parse::<f64>() {
                                value.set(v / 100.0);
                            }
                        },
                    }
                }
            }

            // Live preview
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Preview" }

                div { class: "max-w-md",
                    EqProgress {
                        value: value(),
                        variant: variant,
                        size: size,
                        indeterminate: indeterminate(),
                        gradient: gradient(),
                        label: show_label(),
                    }
                }
            }

            // Style tokens
            StyleInfo { file: "eq_progress_styles.rs", styles: format_catalog(&s::catalog()) }

            // Usage code
            CodeBlock { code: code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqProgress() -> Element {
    rsx! {
        div { class: "space-y-4",
            // Variant gallery
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Variant Gallery" }

                div { class: "space-y-3 max-w-lg",
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Default (gradient)" }
                        EqProgress { value: 0.7, label: true }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Default (solid)" }
                        EqProgress { value: 0.7, gradient: false, label: true }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Success" }
                        EqProgress { value: 0.85, variant: ProgressVariant::Success, label: true }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Warning" }
                        EqProgress { value: 0.45, variant: ProgressVariant::Warning, label: true }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Danger" }
                        EqProgress { value: 0.2, variant: ProgressVariant::Danger, label: true }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Indeterminate" }
                        EqProgress { indeterminate: true }
                    }
                }
            }

            // Size gallery
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Sizes" }

                div { class: "space-y-3 max-w-lg",
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Small (4px)" }
                        EqProgress { value: 0.6, size: ProgressSize::Sm }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Medium (8px)" }
                        EqProgress { value: 0.6, size: ProgressSize::Md }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Large (12px)" }
                        EqProgress { value: 0.6, size: ProgressSize::Lg }
                    }
                }
            }
        }
    }
}
