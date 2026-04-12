//! Style constants for EqProgress — pure Tailwind utility classes.
//!
//! No external CSS file needed. The only exception is the
//! `@keyframes progress-shimmer` defined in `index.css` for
//! the indeterminate animation.

// ── Track (outer container) ────────────────────────────────────────

/// Base track — rounded pill, themed background, overflow hidden.
pub const TRACK: &str = "relative w-full rounded-full overflow-hidden bg-[var(--color-surface-elevated)]";

// ── Fill bar ───────────────────────────────────────────────────────

/// Default gradient fill using the button gradient palette.
pub const FILL_GRADIENT: &str =
    "h-full rounded-full transition-[width] duration-[var(--transition-normal,0.25s)] ease-out \
     bg-gradient-to-r from-[var(--color-btn-default-start)] via-[var(--color-btn-default-mid)] to-[var(--color-btn-default-end)]";

/// Solid accent fill (no gradient).
pub const FILL_SOLID: &str =
    "h-full rounded-full transition-[width] duration-[var(--transition-normal,0.25s)] ease-out \
     bg-[var(--color-accent-primary)]";

// ── Variant fills ──────────────────────────────────────────────────

/// Success — green fill.
pub const FILL_SUCCESS: &str =
    "h-full rounded-full transition-[width] duration-[var(--transition-normal,0.25s)] ease-out \
     bg-[var(--color-success)]";

/// Warning — amber fill.
pub const FILL_WARNING: &str =
    "h-full rounded-full transition-[width] duration-[var(--transition-normal,0.25s)] ease-out \
     bg-[var(--color-warning)]";

/// Danger — red fill.
pub const FILL_DANGER: &str =
    "h-full rounded-full transition-[width] duration-[var(--transition-normal,0.25s)] ease-out \
     bg-[var(--color-error)]";

// ── Sizes (track height) ──────────────────────────────────────────

/// 4px track.
pub const SM: &str = "h-1";

/// 8px track.
pub const MD: &str = "h-2";

/// 12px track.
pub const LG: &str = "h-3";

// ── Indeterminate modifier ─────────────────────────────────────────

/// Applied to the fill bar for the sliding shimmer effect.
/// Uses the `progress-shimmer` keyframes from index.css.
pub const INDETERMINATE_FILL: &str = "w-[40%] animate-[progress-shimmer_1.4s_ease-in-out_infinite]";

// ── Wrapper (label mode) ───────────────────────────────────────────

/// Flex row holding track + percentage label.
pub const WRAPPER: &str = "flex items-center gap-3 w-full";

/// Percentage text to the right of the track.
pub const LABEL: &str = "text-xs font-medium text-[var(--color-label-secondary)] whitespace-nowrap min-w-[2.5rem] text-right";
