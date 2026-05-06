//! Style constants for EqTooltip.

/// Wrapper around the trigger element. Relative positioning anchor.
pub const WRAPPER: &str = "relative inline-flex";

/// The tooltip bubble — shared base.
pub const TOOLTIP: &str =
    "absolute z-50 px-2.5 py-1.5 rounded-md text-xs font-medium \
     whitespace-nowrap pointer-events-none \
     bg-[var(--color-surface-overlay)] text-[var(--color-label-primary)] \
     shadow-lg border border-[var(--color-card-border)] \
     transition-opacity duration-150";

/// Visible state.
pub const VISIBLE: &str = "opacity-100";
/// Hidden state.
pub const HIDDEN: &str = "opacity-0";

/// Position: top (default).
pub const POS_TOP: &str =
    "bottom-full left-1/2 -translate-x-1/2 mb-2";
/// Position: bottom.
pub const POS_BOTTOM: &str =
    "top-full left-1/2 -translate-x-1/2 mt-2";
/// Position: left.
pub const POS_LEFT: &str =
    "right-full top-1/2 -translate-y-1/2 mr-2";
/// Position: right.
pub const POS_RIGHT: &str =
    "left-full top-1/2 -translate-y-1/2 ml-2";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("WRAPPER", WRAPPER),
        ("TOOLTIP", TOOLTIP),
        ("VISIBLE", VISIBLE),
        ("HIDDEN", HIDDEN),
        ("POS_TOP", POS_TOP),
        ("POS_BOTTOM", POS_BOTTOM),
        ("POS_LEFT", POS_LEFT),
        ("POS_RIGHT", POS_RIGHT),
    ]
}
