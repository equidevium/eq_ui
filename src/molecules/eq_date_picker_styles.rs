//! Style constants for EqDatePicker.

/// Wrapper — relative anchor for the calendar popup.
pub const WRAPPER: &str = "relative inline-flex flex-col";

/// The trigger input that shows the current date.
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

/// Placeholder text when no date is selected.
pub const PLACEHOLDER: &str = "text-[var(--color-label-secondary)]";

/// Calendar icon in the trigger.
pub const CALENDAR_ICON: &str =
    "size-4 shrink-0 text-[var(--color-label-secondary)]";

/// The calendar popup panel.
pub const PANEL: &str =
    "absolute z-50 mt-1 rounded-lg p-4 w-[300px] \
     bg-[var(--color-card)] \
     border border-[var(--color-card-border)] \
     shadow-2xl shadow-black/40";

/// Panel open.
pub const PANEL_OPEN: &str = "visible";
/// Panel closed.
pub const PANEL_CLOSED: &str = "invisible pointer-events-none";

/// Position: below trigger (default).
pub const POS_BOTTOM: &str = "top-full left-0";
/// Position: above trigger.
pub const POS_TOP: &str = "bottom-full left-0 mb-1 mt-0";

/// Month/year header row.
pub const HEADER: &str =
    "flex items-center justify-between mb-2";

/// Month/year label.
pub const HEADER_LABEL: &str =
    "text-sm font-semibold text-[var(--color-label-primary)]";

/// Nav button (prev/next month).
pub const NAV_BUTTON: &str =
    "p-1 rounded-md \
     text-[var(--color-label-secondary)] \
     hover:text-[var(--color-label-primary)] \
     hover:bg-[var(--color-tertiary-dark)]/60 \
     focus:outline-none focus:ring-1 focus:ring-[var(--color-accent-primary)]/40 \
     transition-colors cursor-pointer";

/// Weekday header row.
pub const WEEKDAYS: &str = "grid grid-cols-7 gap-1 mb-1";

/// Single weekday label.
pub const WEEKDAY: &str =
    "flex items-center justify-center size-9 text-xs font-medium \
     text-[var(--color-label-secondary)]";

/// Days grid.
pub const DAYS_GRID: &str = "grid grid-cols-7 gap-1";

/// Base day cell.
pub const DAY: &str =
    "flex items-center justify-center size-9 text-sm rounded-md cursor-pointer \
     text-[var(--color-label-primary)] \
     hover:bg-[var(--color-accent-primary)]/15 \
     transition-colors duration-100";

/// Day in current month.
pub const DAY_CURRENT: &str = "";

/// Day outside current month (prev/next month padding).
pub const DAY_OUTSIDE: &str = "text-[var(--color-label-secondary)]/40";

/// Today indicator.
pub const DAY_TODAY: &str =
    "font-bold ring-1 ring-[var(--color-accent-primary)]/50";

/// Selected day.
pub const DAY_SELECTED: &str =
    "bg-[var(--color-accent-primary)] text-white font-semibold \
     hover:bg-[var(--color-accent-primary)]";

/// Disabled day.
pub const DAY_DISABLED: &str =
    "opacity-30 cursor-not-allowed pointer-events-none";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("WRAPPER", WRAPPER),
        ("TRIGGER", TRIGGER),
        ("TRIGGER_DISABLED", TRIGGER_DISABLED),
        ("PLACEHOLDER", PLACEHOLDER),
        ("CALENDAR_ICON", CALENDAR_ICON),
        ("PANEL", PANEL),
        ("PANEL_OPEN", PANEL_OPEN),
        ("PANEL_CLOSED", PANEL_CLOSED),
        ("POS_BOTTOM", POS_BOTTOM),
        ("POS_TOP", POS_TOP),
        ("HEADER", HEADER),
        ("HEADER_LABEL", HEADER_LABEL),
        ("NAV_BUTTON", NAV_BUTTON),
        ("WEEKDAYS", WEEKDAYS),
        ("WEEKDAY", WEEKDAY),
        ("DAYS_GRID", DAYS_GRID),
        ("DAY", DAY),
        ("DAY_CURRENT", DAY_CURRENT),
        ("DAY_OUTSIDE", DAY_OUTSIDE),
        ("DAY_TODAY", DAY_TODAY),
        ("DAY_SELECTED", DAY_SELECTED),
        ("DAY_DISABLED", DAY_DISABLED),
    ]
}
