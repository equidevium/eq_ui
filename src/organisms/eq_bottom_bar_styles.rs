//! Style constants for EqBottomBar component.

/// Bottom bar wrapper - fixed bottom, elevated surface with shadow.
pub const BOTTOM_BAR: &str =
    "sticky bottom-0 left-0 right-0 z-50 border-t border-[var(--color-card-border)] bg-[var(--color-primary-dark)] \
     shadow-[0_-2px_10px_rgba(0,0,0,0.1)]";

/// Inner container - horizontal flex row for items.
pub const BOTTOM_BAR_INNER: &str = "flex items-center justify-around py-2 px-1";

/// Item button - column layout for icon above label.
pub const ITEM_BUTTON: &str =
    "flex flex-col items-center justify-center gap-0.5 px-3 py-1 rounded-lg transition-colors cursor-pointer \
     text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)]";

/// Active item - semi-transparent accent background with accent text.
pub const ITEM_BUTTON_ACTIVE: &str =
    "bg-[var(--color-accent-primary)]/15 text-[var(--color-accent-primary)]";

/// Icon size in items.
pub const ITEM_ICON: &str = "w-6 h-6";

/// Label text in items.
pub const ITEM_LABEL: &str = "text-[10px] font-medium";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("BOTTOM_BAR", BOTTOM_BAR),
        ("BOTTOM_BAR_INNER", BOTTOM_BAR_INNER),
        ("ITEM_BUTTON", ITEM_BUTTON),
        ("ITEM_BUTTON_ACTIVE", ITEM_BUTTON_ACTIVE),
        ("ITEM_ICON", ITEM_ICON),
        ("ITEM_LABEL", ITEM_LABEL),
    ]
}
