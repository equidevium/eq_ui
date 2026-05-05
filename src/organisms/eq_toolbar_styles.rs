//! Style constants for EqToolbar.

/// Wrapper. Vertical stack so the optional secondary row sits below
/// the primary row.
pub const WRAPPER: &str =
    "w-full bg-[var(--color-card)] \
     border-b border-[var(--color-card-border)]";

/// Primary row. Three-region flex layout: start | title | end.
pub const PRIMARY: &str =
    "flex items-center gap-2 \
     min-h-[56px] px-3";

/// Start slot. Flex-shrink-0 so leading buttons don't get squeezed.
pub const START: &str = "flex-shrink-0 flex items-center gap-1";

/// Title slot. Takes the remaining horizontal space. Truncates long
/// text rather than wrapping or pushing the end slot.
pub const TITLE: &str =
    "flex-1 min-w-0 text-base font-semibold \
     text-[var(--color-label-primary)] \
     truncate";

/// End slot. Flex-shrink-0, justifies to the right.
pub const END: &str = "flex-shrink-0 flex items-center gap-1 justify-end";

/// Optional secondary row. No specific styling beyond a top hairline
/// divider; the consumer fills it with whatever component fits.
pub const SECONDARY: &str =
    "px-3 py-2 \
     border-t border-[var(--color-card-border)]";

pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("WRAPPER", WRAPPER),
        ("PRIMARY", PRIMARY),
        ("START", START),
        ("TITLE", TITLE),
        ("END", END),
        ("SECONDARY", SECONDARY),
    ]
}
