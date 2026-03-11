//! Style constants for EqScrollableSpace.

/// Outer container — fills available height and scrolls vertically.
pub const CONTAINER: &str = "overflow-y-auto flex-1 min-h-0";

/// Scrollbar styling — thin, themed.
/// Tailwind v4 supports `scrollbar-thin` and custom scrollbar colors.
pub const SCROLLBAR: &str = "scrollbar-thin scrollbar-thumb-[var(--color-label-secondary)]/30 scrollbar-track-transparent";
