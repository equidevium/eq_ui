//! Style constants for EqNavItem.

/// Base wrapper: inline-flex layout with icon + label aligned.
pub const NAV_ITEM: &str =
    "inline-flex items-center gap-2 px-2 py-1.5 rounded-lg text-sm \
     text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] \
     hover:bg-[var(--color-card)]/30 transition-colors cursor-pointer select-none";

/// Active state: highlighted text + subtle background.
pub const NAV_ITEM_ACTIVE: &str =
    "text-[var(--color-label-primary)] bg-[var(--color-card)]/40 font-medium";

/// Icon sizing — keeps icon and text baseline-aligned.
pub const ICON_WRAP: &str = "shrink-0 w-4 h-4";

/// Label text.
pub const LABEL: &str = "whitespace-nowrap";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("NAV_ITEM", NAV_ITEM),
        ("NAV_ITEM_ACTIVE", NAV_ITEM_ACTIVE),
        ("ICON_WRAP", ICON_WRAP),
        ("LABEL", LABEL),
    ]
}
