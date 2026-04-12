//! Style constants for EqTab - pure Tailwind utility classes.
//!
//! Three visual variants (Underline, Pill, Card) and three size presets.
//! All state transitions (active, hover, disabled) are handled via
//! Tailwind classes referencing theme CSS custom properties.

// ── Container ─────────────────────────────────────────────────────

/// Tab list container - horizontal flex with accessible role.
pub const CONTAINER: &str = "flex items-center gap-1";

// ── Underline variant ─────────────────────────────────────────────

/// Container border for underline variant - bottom border.
pub const CONTAINER_UNDERLINE: &str = "border-b border-[var(--color-card-border)]";

/// Shared button reset for underline tabs.
pub const UNDERLINE_BASE: &str =
    "relative cursor-pointer font-medium transition-colors \
     text-[var(--color-label-secondary)] \
     hover:text-[var(--color-label-primary)]";

/// Active state for underline tabs - accent bottom border + primary text.
pub const UNDERLINE_ACTIVE: &str =
    "text-[var(--color-label-primary)] \
     after:absolute after:bottom-0 after:left-0 after:right-0 after:h-[2px] \
     after:bg-[var(--color-accent-primary)] after:rounded-full";

// ── Pill variant ──────────────────────────────────────────────────

/// Shared button reset for pill tabs.
pub const PILL_BASE: &str =
    "cursor-pointer font-medium rounded-full transition-colors \
     text-[var(--color-label-secondary)] \
     hover:bg-[var(--color-surface-elevated)] \
     hover:text-[var(--color-label-primary)]";

/// Active state for pill tabs - elevated surface fill + primary text.
pub const PILL_ACTIVE: &str =
    "bg-[var(--color-surface-elevated)] text-[var(--color-label-primary)] \
     shadow-sm";

// ── Card variant ──────────────────────────────────────────────────

/// Container for card tabs - bottom border that active tabs sit on top of.
pub const CONTAINER_CARD: &str = "border-b border-[var(--color-card-border)]";

/// Shared button reset for card tabs.
pub const CARD_BASE: &str =
    "cursor-pointer font-medium rounded-t-lg transition-colors \
     text-[var(--color-label-secondary)] \
     hover:text-[var(--color-label-primary)] \
     hover:bg-[var(--color-card)]/30";

/// Active card tab - solid background, border on 3 sides, overlaps container border.
pub const CARD_ACTIVE: &str =
    "bg-[var(--color-card)] text-[var(--color-label-primary)] \
     border border-[var(--color-card-border)] border-b-transparent \
     -mb-px";

// ── Disabled modifier ─────────────────────────────────────────────

/// Disabled tab - reduced opacity and no pointer events.
pub const DISABLED: &str = "opacity-40 cursor-not-allowed pointer-events-none";

// ── Sizes ─────────────────────────────────────────────────────────

/// Small - compact padding and smaller text.
pub const SM: &str = "px-3 py-1.5 text-xs";

/// Medium (default) - standard padding and text.
pub const MD: &str = "px-4 py-2 text-sm";

/// Large - generous padding and larger text.
pub const LG: &str = "px-5 py-2.5 text-base";

// ── Icon support ──────────────────────────────────────────────────

/// Layout for tab with icon + label.
pub const WITH_ICON: &str = "inline-flex items-center gap-1.5";

// ── Badge / count ─────────────────────────────────────────────────

/// Optional count badge inside a tab.
pub const BADGE: &str =
    "ml-1.5 inline-flex items-center justify-center min-w-[1.25rem] h-5 \
     px-1 rounded-full text-[10px] font-semibold \
     bg-[var(--color-accent-primary)]/15 text-[var(--color-accent-primary)]";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("CONTAINER", CONTAINER),
        ("CONTAINER_UNDERLINE", CONTAINER_UNDERLINE),
        ("UNDERLINE_BASE", UNDERLINE_BASE),
        ("UNDERLINE_ACTIVE", UNDERLINE_ACTIVE),
        ("PILL_BASE", PILL_BASE),
        ("PILL_ACTIVE", PILL_ACTIVE),
        ("CONTAINER_CARD", CONTAINER_CARD),
        ("CARD_BASE", CARD_BASE),
        ("CARD_ACTIVE", CARD_ACTIVE),
        ("DISABLED", DISABLED),
        ("SM", SM),
        ("MD", MD),
        ("LG", LG),
        ("WITH_ICON", WITH_ICON),
        ("BADGE", BADGE),
    ]
}
