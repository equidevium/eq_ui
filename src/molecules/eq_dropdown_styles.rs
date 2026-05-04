//! Style constants for EqDropdown.

/// Wrapper — relative anchor for the menu.
pub const WRAPPER: &str = "relative inline-flex";

/// The trigger button.
pub const TRIGGER: &str =
    "inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md text-sm \
     cursor-pointer select-none \
     bg-[var(--color-input-bg)] text-[var(--color-label-primary)] \
     border border-[var(--color-input-border)] \
     hover:border-[var(--color-accent-primary)] \
     focus:outline-none focus:ring-2 focus:ring-[var(--color-accent-primary)]/40 \
     transition-colors duration-150";

/// Chevron icon in the trigger.
pub const CHEVRON: &str =
    "size-4 text-[var(--color-label-secondary)] transition-transform duration-150";

/// Chevron rotated when open.
pub const CHEVRON_OPEN: &str = "rotate-180";

/// The dropdown menu panel.
pub const MENU: &str =
    "absolute z-50 mt-1 min-w-full rounded-md py-1 \
     bg-[var(--color-card)] \
     border border-[var(--color-card-border)] \
     shadow-2xl shadow-black/40 overflow-hidden";

/// Menu open.
pub const MENU_OPEN: &str = "visible";
/// Menu closed.
pub const MENU_CLOSED: &str = "invisible pointer-events-none";

/// Position: below trigger (default).
pub const POS_BOTTOM: &str = "top-full left-0";
/// Position: above trigger.
pub const POS_TOP: &str = "bottom-full left-0 mb-1 mt-0";

/// A single dropdown item.
pub const ITEM: &str =
    "w-full px-3 py-2 text-left text-sm cursor-pointer \
     text-[var(--color-label-primary)] \
     hover:bg-[var(--color-accent-primary)]/10 \
     focus:bg-[var(--color-accent-primary)]/10 \
     focus:outline-none transition-colors duration-100";

/// Active / selected item.
pub const ITEM_ACTIVE: &str =
    "bg-[var(--color-accent-primary)]/10 font-medium";

/// Disabled item.
pub const ITEM_DISABLED: &str =
    "opacity-40 cursor-not-allowed pointer-events-none";

/// Separator between item groups.
pub const SEPARATOR: &str =
    "my-1 border-t border-[var(--color-card-border)]";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("WRAPPER", WRAPPER),
        ("TRIGGER", TRIGGER),
        ("CHEVRON", CHEVRON),
        ("CHEVRON_OPEN", CHEVRON_OPEN),
        ("MENU", MENU),
        ("MENU_OPEN", MENU_OPEN),
        ("MENU_CLOSED", MENU_CLOSED),
        ("POS_BOTTOM", POS_BOTTOM),
        ("POS_TOP", POS_TOP),
        ("ITEM", ITEM),
        ("ITEM_ACTIVE", ITEM_ACTIVE),
        ("ITEM_DISABLED", ITEM_DISABLED),
        ("SEPARATOR", SEPARATOR),
    ]
}
