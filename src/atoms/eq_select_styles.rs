//! Style constants for EqSelect.

/// Wrapper — relative anchor for the listbox.
pub const WRAPPER: &str = "relative inline-flex flex-col";

/// The trigger button that shows the current selection.
pub const TRIGGER: &str =
    "inline-flex items-center justify-between gap-2 w-full px-3 py-2 rounded-md text-sm \
     cursor-pointer select-none \
     bg-[var(--color-input-bg)] text-[var(--color-label-primary)] \
     border border-[var(--color-input-border)] \
     hover:border-[var(--color-accent-primary)] \
     focus:outline-none focus:ring-2 focus:ring-[var(--color-accent-primary)]/40 \
     transition-colors duration-150";

/// Trigger when disabled.
pub const TRIGGER_DISABLED: &str =
    "inline-flex items-center justify-between gap-2 w-full px-3 py-2 rounded-md text-sm \
     cursor-not-allowed select-none opacity-50 \
     bg-[var(--color-input-bg)] text-[var(--color-label-primary)] \
     border border-[var(--color-input-border)]";

/// Placeholder text when nothing is selected.
pub const PLACEHOLDER: &str = "text-[var(--color-label-secondary)]";

/// Chevron icon in the trigger.
pub const CHEVRON: &str =
    "size-4 shrink-0 text-[var(--color-label-secondary)] transition-transform duration-150";

/// Chevron rotated when open.
pub const CHEVRON_OPEN: &str = "rotate-180";

/// The listbox panel.
pub const LISTBOX: &str =
    "absolute z-50 mt-1 w-full max-h-60 rounded-md py-1 \
     bg-[var(--color-card)] \
     border border-[var(--color-card-border)] \
     shadow-2xl shadow-black/40 overflow-y-auto";

/// Listbox open.
pub const LISTBOX_OPEN: &str = "visible";
/// Listbox closed.
pub const LISTBOX_CLOSED: &str = "invisible pointer-events-none";

/// Position: below trigger (default).
pub const POS_BOTTOM: &str = "top-full left-0";
/// Position: above trigger.
pub const POS_TOP: &str = "bottom-full left-0 mb-1 mt-0";

/// Search input inside the listbox.
pub const SEARCH: &str =
    "w-full px-3 py-2 text-sm \
     bg-transparent text-[var(--color-label-primary)] \
     border-b border-[var(--color-card-border)] \
     placeholder:text-[var(--color-label-secondary)] \
     focus:outline-none";

/// A single option.
pub const OPTION: &str =
    "w-full px-3 py-2 text-left text-sm cursor-pointer \
     text-[var(--color-label-primary)] \
     hover:bg-[var(--color-accent-primary)]/10 \
     focus:bg-[var(--color-accent-primary)]/10 \
     focus:outline-none transition-colors duration-100";

/// Selected option.
pub const OPTION_SELECTED: &str =
    "bg-[var(--color-accent-primary)]/10 font-medium";

/// Disabled option.
pub const OPTION_DISABLED: &str =
    "opacity-40 cursor-not-allowed pointer-events-none";

/// Check mark shown on selected option.
pub const CHECK: &str =
    "size-4 shrink-0 text-[var(--color-accent-primary)]";

/// Empty state when search yields no results.
pub const EMPTY: &str =
    "px-3 py-4 text-center text-sm text-[var(--color-label-secondary)]";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("WRAPPER", WRAPPER),
        ("TRIGGER", TRIGGER),
        ("TRIGGER_DISABLED", TRIGGER_DISABLED),
        ("PLACEHOLDER", PLACEHOLDER),
        ("CHEVRON", CHEVRON),
        ("CHEVRON_OPEN", CHEVRON_OPEN),
        ("LISTBOX", LISTBOX),
        ("LISTBOX_OPEN", LISTBOX_OPEN),
        ("LISTBOX_CLOSED", LISTBOX_CLOSED),
        ("POS_BOTTOM", POS_BOTTOM),
        ("POS_TOP", POS_TOP),
        ("SEARCH", SEARCH),
        ("OPTION", OPTION),
        ("OPTION_SELECTED", OPTION_SELECTED),
        ("OPTION_DISABLED", OPTION_DISABLED),
        ("CHECK", CHECK),
        ("EMPTY", EMPTY),
    ]
}
