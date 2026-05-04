//! Style constants for EqCalendar.

/// Outer container (month mode).
pub const WRAPPER: &str =
    "rounded-lg p-4 \
     bg-[var(--color-card)] \
     border border-[var(--color-card-border)]";

/// Width for month mode.
pub const WRAPPER_MONTH: &str = "w-[300px]";

/// Width for week mode.
pub const WRAPPER_WEEK: &str = "w-full min-w-[600px]";

/// Month/year header row.
pub const HEADER: &str =
    "flex items-center justify-between mb-3";

/// Month/year label.
pub const HEADER_LABEL: &str =
    "text-sm font-semibold text-[var(--color-label-primary)]";

/// Nav button (prev/next month).
pub const NAV_BUTTON: &str =
    "p-1.5 rounded-md \
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

/// Day outside current month.
pub const DAY_OUTSIDE: &str = "text-[var(--color-label-secondary)]/40";

/// Today indicator.
pub const DAY_TODAY: &str =
    "font-bold ring-1 ring-[var(--color-accent-primary)]/50";

/// Selected day.
pub const DAY_SELECTED: &str =
    "bg-[var(--color-accent-primary)] text-white font-semibold \
     hover:bg-[var(--color-accent-primary)]";

/// Range start (for future range selection).
pub const DAY_RANGE_START: &str =
    "bg-[var(--color-accent-primary)] text-white rounded-r-none";

/// Range middle.
pub const DAY_RANGE_MID: &str =
    "bg-[var(--color-accent-primary)]/20 rounded-none";

/// Range end.
pub const DAY_RANGE_END: &str =
    "bg-[var(--color-accent-primary)] text-white rounded-l-none";

/// Disabled day.
pub const DAY_DISABLED: &str =
    "opacity-30 cursor-not-allowed pointer-events-none";

/// Day cell inner — stacks number + dot vertically.
pub const DAY_INNER: &str =
    "flex flex-col items-center justify-center gap-0.5 leading-none";

/// Event indicator dot (base).
pub const EVENT_DOT: &str =
    "size-1 rounded-full";

/// Event dot — default accent color.
pub const EVENT_DOT_DEFAULT: &str =
    "bg-[var(--color-accent-primary)]";

/// Event dot — success / green.
pub const EVENT_DOT_SUCCESS: &str = "bg-emerald-500";

/// Event dot — warning / amber.
pub const EVENT_DOT_WARNING: &str = "bg-amber-500";

/// Event dot — danger / red.
pub const EVENT_DOT_DANGER: &str = "bg-red-500";

/// Event dot — info / blue.
pub const EVENT_DOT_INFO: &str = "bg-sky-500";

/// Clickable month label in the header.
pub const HEADER_MONTH: &str =
    "text-sm font-semibold text-[var(--color-label-primary)] \
     cursor-pointer hover:text-[var(--color-accent-primary)] \
     transition-colors duration-100";

/// Clickable year label in the header.
pub const HEADER_YEAR: &str =
    "text-sm font-semibold text-[var(--color-label-secondary)] \
     cursor-pointer hover:text-[var(--color-accent-primary)] \
     transition-colors duration-100";

/// Month/year picker grid (3×4 for months, 4×3 for years).
pub const PICKER_GRID: &str = "grid grid-cols-3 gap-2";

/// A cell in the month or year picker.
pub const PICKER_CELL: &str =
    "flex items-center justify-center py-2.5 px-1 text-sm rounded-md cursor-pointer \
     text-[var(--color-label-primary)] \
     hover:bg-[var(--color-accent-primary)]/15 \
     transition-colors duration-100";

/// Active (currently viewed) cell in the picker.
pub const PICKER_CELL_ACTIVE: &str =
    "bg-[var(--color-accent-primary)] text-white font-semibold \
     hover:bg-[var(--color-accent-primary)]";

// ── Week view ─────────────────────────────────────────────────────

/// Mode toggle button (Month / Week).
pub const MODE_TOGGLE: &str =
    "px-2.5 py-1 text-xs rounded-md cursor-pointer select-none \
     text-[var(--color-label-secondary)] \
     hover:text-[var(--color-label-primary)] \
     hover:bg-[var(--color-tertiary-dark)]/40 \
     transition-colors duration-100";

/// Active mode toggle.
pub const MODE_TOGGLE_ACTIVE: &str =
    "bg-[var(--color-accent-primary)]/15 text-[var(--color-accent-primary)] font-medium";

/// Week view container.
pub const WEEK_GRID: &str =
    "grid border-t border-[var(--color-card-border)]";

/// Week header row with day names + dates.
pub const WEEK_HEADER: &str =
    "grid grid-cols-8 border-b border-[var(--color-card-border)]";

/// Time gutter label (left column).
pub const WEEK_TIME_GUTTER: &str =
    "w-14 shrink-0 text-right pr-2 text-xs text-[var(--color-label-secondary)] \
     border-r border-[var(--color-card-border)]";

/// Day column header in week view.
pub const WEEK_DAY_HEADER: &str =
    "flex flex-col items-center py-2 text-xs \
     border-r border-[var(--color-card-border)]";

/// Day name in week header.
pub const WEEK_DAY_NAME: &str =
    "font-medium text-[var(--color-label-secondary)]";

/// Day number in week header.
pub const WEEK_DAY_NUM: &str =
    "text-sm font-semibold text-[var(--color-label-primary)]";

/// Day number when it's today.
pub const WEEK_DAY_NUM_TODAY: &str =
    "text-sm font-bold size-7 flex items-center justify-center rounded-full \
     bg-[var(--color-accent-primary)] text-white";

/// A single hour row in the week grid.
pub const WEEK_HOUR_ROW: &str =
    "grid grid-cols-8 min-h-[48px] border-b border-[var(--color-card-border)]/30";

/// A day cell within an hour row.
pub const WEEK_DAY_CELL: &str =
    "relative border-r border-[var(--color-card-border)]/30";

/// An event block positioned within a day cell.
pub const WEEK_EVENT: &str =
    "absolute left-0.5 right-0.5 rounded px-1.5 py-0.5 text-xs \
     overflow-hidden cursor-pointer \
     border-l-2 truncate";

/// Event block colors per EventColor.
pub const WEEK_EVENT_DEFAULT: &str =
    "bg-[var(--color-accent-primary)]/20 border-[var(--color-accent-primary)] \
     text-[var(--color-label-primary)]";
pub const WEEK_EVENT_SUCCESS: &str =
    "bg-emerald-500/20 border-emerald-500 text-[var(--color-label-primary)]";
pub const WEEK_EVENT_WARNING: &str =
    "bg-amber-500/20 border-amber-500 text-[var(--color-label-primary)]";
pub const WEEK_EVENT_DANGER: &str =
    "bg-red-500/20 border-red-500 text-[var(--color-label-primary)]";
pub const WEEK_EVENT_INFO: &str =
    "bg-sky-500/20 border-sky-500 text-[var(--color-label-primary)]";

/// All-day events bar at top of week view.
pub const WEEK_ALLDAY_ROW: &str =
    "grid grid-cols-8 border-b border-[var(--color-card-border)] min-h-[32px]";

/// All-day event chip.
pub const WEEK_ALLDAY_CHIP: &str =
    "mx-0.5 my-0.5 px-1.5 py-0.5 text-xs rounded truncate \
     border-l-2";

/// Scrollable body for the hour rows.
pub const WEEK_BODY: &str =
    "overflow-y-auto max-h-[480px]";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("WRAPPER", WRAPPER),
        ("HEADER", HEADER),
        ("HEADER_LABEL", HEADER_LABEL),
        ("NAV_BUTTON", NAV_BUTTON),
        ("WEEKDAYS", WEEKDAYS),
        ("WEEKDAY", WEEKDAY),
        ("DAYS_GRID", DAYS_GRID),
        ("DAY", DAY),
        ("DAY_OUTSIDE", DAY_OUTSIDE),
        ("DAY_TODAY", DAY_TODAY),
        ("DAY_SELECTED", DAY_SELECTED),
        ("DAY_RANGE_START", DAY_RANGE_START),
        ("DAY_RANGE_MID", DAY_RANGE_MID),
        ("DAY_RANGE_END", DAY_RANGE_END),
        ("DAY_DISABLED", DAY_DISABLED),
        ("DAY_INNER", DAY_INNER),
        ("EVENT_DOT", EVENT_DOT),
        ("EVENT_DOT_DEFAULT", EVENT_DOT_DEFAULT),
        ("EVENT_DOT_SUCCESS", EVENT_DOT_SUCCESS),
        ("EVENT_DOT_WARNING", EVENT_DOT_WARNING),
        ("EVENT_DOT_DANGER", EVENT_DOT_DANGER),
        ("EVENT_DOT_INFO", EVENT_DOT_INFO),
        ("HEADER_MONTH", HEADER_MONTH),
        ("HEADER_YEAR", HEADER_YEAR),
        ("PICKER_GRID", PICKER_GRID),
        ("PICKER_CELL", PICKER_CELL),
        ("PICKER_CELL_ACTIVE", PICKER_CELL_ACTIVE),
        ("WRAPPER_MONTH", WRAPPER_MONTH),
        ("WRAPPER_WEEK", WRAPPER_WEEK),
        ("MODE_TOGGLE", MODE_TOGGLE),
        ("MODE_TOGGLE_ACTIVE", MODE_TOGGLE_ACTIVE),
        ("WEEK_HEADER", WEEK_HEADER),
        ("WEEK_TIME_GUTTER", WEEK_TIME_GUTTER),
        ("WEEK_DAY_HEADER", WEEK_DAY_HEADER),
        ("WEEK_DAY_NAME", WEEK_DAY_NAME),
        ("WEEK_DAY_NUM", WEEK_DAY_NUM),
        ("WEEK_DAY_NUM_TODAY", WEEK_DAY_NUM_TODAY),
        ("WEEK_HOUR_ROW", WEEK_HOUR_ROW),
        ("WEEK_DAY_CELL", WEEK_DAY_CELL),
        ("WEEK_EVENT", WEEK_EVENT),
        ("WEEK_EVENT_DEFAULT", WEEK_EVENT_DEFAULT),
        ("WEEK_EVENT_SUCCESS", WEEK_EVENT_SUCCESS),
        ("WEEK_EVENT_WARNING", WEEK_EVENT_WARNING),
        ("WEEK_EVENT_DANGER", WEEK_EVENT_DANGER),
        ("WEEK_EVENT_INFO", WEEK_EVENT_INFO),
        ("WEEK_ALLDAY_ROW", WEEK_ALLDAY_ROW),
        ("WEEK_ALLDAY_CHIP", WEEK_ALLDAY_CHIP),
        ("WEEK_BODY", WEEK_BODY),
    ]
}
