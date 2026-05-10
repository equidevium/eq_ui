//! Style constants for EqMobileAppShell.

/// Outer container. Fills available height, lays out the three regions
/// in a flex column, and pads for iOS safe areas. The `env(...)`
/// values are 0 on platforms that don't expose them.
pub const ROOT: &str = "relative flex flex-col h-full overflow-hidden \
     bg-[var(--color-primary-dark)] \
     pt-[env(safe-area-inset-top)] \
     pb-[env(safe-area-inset-bottom)]";

/// Toolbar region. Stays at its natural height.
pub const TOOLBAR_REGION: &str = "flex-shrink-0";

/// Scrollable middle. Fills remaining height; scrolls vertically.
pub const BODY: &str = "flex-1 overflow-y-auto \
     text-[var(--color-label-primary)]";

/// Bottom nav region. Stays at its natural height, anchored below
/// the body.
pub const BOTTOM_NAV_REGION: &str = "flex-shrink-0";

pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("ROOT", ROOT),
        ("TOOLBAR_REGION", TOOLBAR_REGION),
        ("BODY", BODY),
        ("BOTTOM_NAV_REGION", BOTTOM_NAV_REGION),
    ]
}
