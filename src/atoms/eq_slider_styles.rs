//! Style constants for EqSlider.

/// Outermost wrapper (contains slider + optional label).
pub const WRAPPER: &str = "flex items-center gap-3 w-full";

/// The native range input, themed via accent color.
pub const INPUT: &str =
    "w-full h-2 appearance-none rounded-full cursor-pointer \
     bg-[var(--color-tertiary-dark)] \
     accent-[var(--color-accent-primary)] \
     focus:outline-none focus:ring-2 focus:ring-[var(--color-accent-primary)]/40";

/// Small size track.
pub const SM: &str = "h-1";
/// Medium size track (default).
pub const MD: &str = "h-2";
/// Large size track.
pub const LG: &str = "h-3";

/// Value label displayed beside the slider.
pub const LABEL: &str =
    "shrink-0 min-w-[3ch] text-right text-sm tabular-nums \
     text-[var(--color-label-secondary)]";

/// Disabled state.
pub const DISABLED: &str = "opacity-50 cursor-not-allowed";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("WRAPPER", WRAPPER),
        ("INPUT", INPUT),
        ("SM", SM),
        ("MD", MD),
        ("LG", LG),
        ("LABEL", LABEL),
        ("DISABLED", DISABLED),
    ]
}
