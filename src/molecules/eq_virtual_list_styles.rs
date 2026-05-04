//! Style constants for EqVirtualList.

/// Outer scrollable viewport container (vertical mode).
pub const VIEWPORT: &str =
    "overflow-auto relative \
     bg-[var(--color-card)] \
     border border-[var(--color-card-border)] rounded-lg";

/// Outer scrollable viewport container (horizontal mode).
pub const VIEWPORT_HORIZONTAL: &str =
    "overflow-x-auto overflow-y-hidden relative \
     bg-[var(--color-card)] \
     border border-[var(--color-card-border)] rounded-lg";

/// Inner sizer div — sized to total content extent.
/// Provides scroll-thumb sizing without rendering every row.
pub const SIZER: &str = "relative w-full";

/// Horizontal sizer — width represents total content.
pub const SIZER_HORIZONTAL: &str = "relative h-full flex";

/// Positioned window of visible items.
pub const WINDOW: &str = "absolute top-0 left-0 w-full";

/// Horizontal positioned window.
pub const WINDOW_HORIZONTAL: &str = "absolute top-0 left-0 h-full flex";

/// A single virtualized row.
pub const ITEM: &str = "w-full";

/// Horizontal item — inline element.
pub const ITEM_HORIZONTAL: &str = "h-full shrink-0";

/// Sticky header sitting above the scrollable area.
pub const STICKY_HEADER: &str =
    "sticky top-0 z-10 \
     bg-[var(--color-card)] \
     border-b border-[var(--color-card-border)]";

/// Sticky header in horizontal mode.
pub const STICKY_HEADER_HORIZONTAL: &str =
    "sticky left-0 z-10 \
     bg-[var(--color-card)] \
     border-r border-[var(--color-card-border)]";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("VIEWPORT", VIEWPORT),
        ("VIEWPORT_HORIZONTAL", VIEWPORT_HORIZONTAL),
        ("SIZER", SIZER),
        ("SIZER_HORIZONTAL", SIZER_HORIZONTAL),
        ("WINDOW", WINDOW),
        ("WINDOW_HORIZONTAL", WINDOW_HORIZONTAL),
        ("ITEM", ITEM),
        ("ITEM_HORIZONTAL", ITEM_HORIZONTAL),
        ("STICKY_HEADER", STICKY_HEADER),
        ("STICKY_HEADER_HORIZONTAL", STICKY_HEADER_HORIZONTAL),
    ]
}
