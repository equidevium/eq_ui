//! Style constants for EqTopBar component.

/// Top bar wrapper - sticky, backdrop blur, no bottom border (flush).
pub const TOP_BAR: &str =
    "sticky top-0 z-50 bg-[var(--color-primary-dark)]/80 backdrop-blur";

/// Inner container - flex column layout for multi-row support.
pub const TOP_BAR_INNER: &str = "flex flex-col";

/// Primary row - horizontal layout for left/title/right zones.
pub const PRIMARY_ROW: &str = "flex items-center justify-between px-4 py-2";

/// Title text - centered, bold, primary color.
pub const TITLE: &str = "text-base font-semibold text-[var(--color-label-primary)] truncate max-w-[60%]";

/// Left element area - icon/button wrapper.
pub const LEFT_ZONE: &str = "flex items-center gap-1 shrink-0";

/// Right element area - icon/button wrapper.
pub const RIGHT_ZONE: &str = "flex items-center gap-1 shrink-0";

/// Secondary row - stacks search and tabs vertically.
pub const SECONDARY_ROW: &str = "flex flex-col flex-1 pt-2";

/// Search bar wrapper styling - padded like a standard input row.
pub const SEARCH_BAR: &str = "w-full px-4 py-2";

/// Tab selector wrapper styling - full width, pushed to bottom of the bar.
pub const TAB_SELECTOR: &str = "w-full mt-auto justify-items-center";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("TOP_BAR", TOP_BAR),
        ("TOP_BAR_INNER", TOP_BAR_INNER),
        ("PRIMARY_ROW", PRIMARY_ROW),
        ("TITLE", TITLE),
        ("LEFT_ZONE", LEFT_ZONE),
        ("RIGHT_ZONE", RIGHT_ZONE),
        ("SECONDARY_ROW", SECONDARY_ROW),
        ("SEARCH_BAR", SEARCH_BAR),
        ("TAB_SELECTOR", TAB_SELECTOR),
    ]
}
