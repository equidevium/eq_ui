//! Style constants for EqPlayground - pure Tailwind utility classes.

/// Sidebar container - fixed width, border, scrollable.
pub const SIDEBAR: &str =
    "w-64 shrink-0 border-r border-[var(--color-card-border)] p-3 flex flex-col";

/// Sidebar visible on mobile (overlay).
pub const SIDEBAR_MOBILE_OPEN: &str =
    "fixed inset-y-0 left-0 z-40 w-64 bg-[var(--color-primary-dark)] \
     border-r border-[var(--color-card-border)] p-3 flex flex-col pt-16 \
     md:relative md:inset-auto md:z-auto md:pt-3";

/// Sidebar hidden on mobile, visible on desktop.
pub const SIDEBAR_MOBILE_CLOSED: &str =
    "hidden md:flex w-64 shrink-0 border-r border-[var(--color-card-border)] p-3 flex-col";

/// Mobile overlay backdrop.
pub const MOBILE_BACKDROP: &str = "fixed inset-0 z-30 bg-black/50 md:hidden";

/// Two-panel layout wrapper.
pub const LAYOUT: &str = "flex min-h-[calc(100vh-8rem)] relative";

/// Right preview panel.
pub const PREVIEW_PANEL: &str = "flex-1 overflow-y-auto";

/// Empty state - centered placeholder.
pub const EMPTY_STATE: &str =
    "flex flex-col items-center justify-center h-full min-h-[60vh] gap-4 \
     text-[var(--color-label-secondary)]";

/// Empty state icon.
pub const EMPTY_ICON: &str = "size-16 opacity-30";

/// Hamburger button wrapper (mobile only).
pub const HAMBURGER: &str = "md:hidden";

/// Hamburger button style.
pub const HAMBURGER_BTN: &str =
    "p-2 rounded-md text-[var(--color-label-secondary)] \
     hover:text-[var(--color-label-primary)] active:text-[var(--color-label-primary)] transition";

/// Hamburger icon size.
pub const HAMBURGER_ICON: &str = "size-5";

/// Theme switcher select control.
pub const THEME_SELECT: &str =
    "rounded-md bg-[var(--color-card)] text-[var(--color-label-primary)] \
     border border-[var(--color-card-border)] px-2 py-1 text-sm";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("SIDEBAR", SIDEBAR),
        ("SIDEBAR_MOBILE_OPEN", SIDEBAR_MOBILE_OPEN),
        ("SIDEBAR_MOBILE_CLOSED", SIDEBAR_MOBILE_CLOSED),
        ("MOBILE_BACKDROP", MOBILE_BACKDROP),
        ("LAYOUT", LAYOUT),
        ("PREVIEW_PANEL", PREVIEW_PANEL),
        ("EMPTY_STATE", EMPTY_STATE),
        ("EMPTY_ICON", EMPTY_ICON),
        ("HAMBURGER", HAMBURGER),
        ("HAMBURGER_BTN", HAMBURGER_BTN),
        ("HAMBURGER_ICON", HAMBURGER_ICON),
        ("THEME_SELECT", THEME_SELECT),
    ]
}
