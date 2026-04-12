//! EqProgress — themed progress bar atom (pure Tailwind).
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

/// Semantic colour variant for the progress fill.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ProgressVariant {
    /// Theme gradient fill (default).
    #[default]
    Default,
    /// Green — success / complete.
    Success,
    /// Amber — warning / caution.
    Warning,
    /// Red — danger / error.
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
