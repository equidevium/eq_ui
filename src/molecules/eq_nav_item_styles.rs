//! Style constants for EqNavItem.

/// Base wrapper: inline-flex layout with icon + label aligned.
pub const NAV_ITEM_BASE: &str =
    "inline-flex items-center rounded-lg \
     text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] \
     hover:bg-[var(--color-card)]/30 transition-colors cursor-pointer select-none";

/// Size: small (default original size).
pub const NAV_ITEM_SM: &str = "gap-2 px-2 py-1.5 text-sm";

/// Size: medium.
pub const NAV_ITEM_MD: &str = "gap-2.5 px-3 py-2 text-base";

/// Size: large.
pub const NAV_ITEM_LG: &str = "gap-3 px-4 py-2.5 text-lg";

/// Active state: highlighted text + subtle background.
pub const NAV_ITEM_ACTIVE: &str =
    "text-[var(--color-label-primary)] bg-[var(--color-card)]/40 font-medium";

/// Icon sizing — small.
pub const ICON_SM: &str = "shrink-0 w-4 h-4";

/// Icon sizing — medium.
pub const ICON_MD: &str = "shrink-0 w-5 h-5";

/// Icon sizing — large.
pub const ICON_LG: &str = "shrink-0 w-6 h-6";

/// Label text.
pub const LABEL: &str = "whitespace-nowrap";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("NAV_ITEM_BASE", NAV_ITEM_BASE),
        ("NAV_ITEM_SM", NAV_ITEM_SM),
        ("NAV_ITEM_MD", NAV_ITEM_MD),
        ("NAV_ITEM_LG", NAV_ITEM_LG),
        ("NAV_ITEM_ACTIVE", NAV_ITEM_ACTIVE),
        ("ICON_SM", ICON_SM),
        ("ICON_MD", ICON_MD),
        ("ICON_LG", ICON_LG),
        ("LABEL", LABEL),
    ]
}
