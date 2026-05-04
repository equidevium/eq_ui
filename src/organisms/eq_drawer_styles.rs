//! Style constants for EqDrawer.

/// Full-viewport backdrop overlay.
pub const BACKDROP: &str =
    "fixed inset-0 z-50 \
     bg-[var(--color-surface-overlay)] \
     transition-opacity duration-200";

/// Backdrop when open.
pub const BACKDROP_OPEN: &str = "opacity-100";
/// Backdrop when closed.
pub const BACKDROP_CLOSED: &str = "opacity-0 pointer-events-none";

/// The drawer panel — shared base.
pub const PANEL: &str =
    "fixed z-50 flex flex-col \
     bg-[var(--color-card)] \
     border-[var(--color-card-border)] \
     shadow-2xl shadow-black/40 \
     transition-transform duration-300 ease-in-out";

/// Panel open (no transform offset).
pub const PANEL_OPEN: &str = "translate-x-0 translate-y-0";

// ── Side-specific base + closed transforms ────────────────────────

/// Left side: panel sizing and closed state.
pub const SIDE_LEFT: &str =
    "inset-y-0 left-0 border-r max-w-[85vw]";
pub const SIDE_LEFT_CLOSED: &str = "-translate-x-full";

/// Right side.
pub const SIDE_RIGHT: &str =
    "inset-y-0 right-0 border-l max-w-[85vw]";
pub const SIDE_RIGHT_CLOSED: &str = "translate-x-full";

/// Top side.
pub const SIDE_TOP: &str =
    "inset-x-0 top-0 border-b max-h-[85vh]";
pub const SIDE_TOP_CLOSED: &str = "-translate-y-full";

/// Bottom side.
pub const SIDE_BOTTOM: &str =
    "inset-x-0 bottom-0 border-t max-h-[85vh]";
pub const SIDE_BOTTOM_CLOSED: &str = "translate-y-full";

// ── Width / height presets ─────────────────────────────────────────

/// Small (320px for left/right, 240px for top/bottom).
pub const SIZE_SM_H: &str = "w-80";
pub const SIZE_SM_V: &str = "h-60";

/// Medium (420px / 320px).
pub const SIZE_MD_H: &str = "w-[420px]";
pub const SIZE_MD_V: &str = "h-80";

/// Large (560px / 420px).
pub const SIZE_LG_H: &str = "w-[560px]";
pub const SIZE_LG_V: &str = "h-[420px]";

/// Full width/height.
pub const SIZE_FULL_H: &str = "w-screen";
pub const SIZE_FULL_V: &str = "h-screen";

// ── Header / body / footer ────────────────────────────────────────

pub const HEADER: &str =
    "flex items-center justify-between px-5 py-4 shrink-0 \
     border-b border-[var(--color-card-border)]";

pub const HEADER_TITLE: &str =
    "text-lg font-semibold text-[var(--color-label-primary)]";

pub const CLOSE_BUTTON: &str =
    "shrink-0 p-1 rounded-md \
     text-[var(--color-label-secondary)] \
     hover:text-[var(--color-label-primary)] \
     hover:bg-[var(--color-tertiary-dark)]/60 \
     transition-colors cursor-pointer";

pub const BODY: &str =
    "flex-1 overflow-y-auto px-5 py-4 text-sm text-[var(--color-label-primary)]";

pub const FOOTER: &str =
    "flex items-center justify-end gap-2 px-5 py-4 shrink-0 \
     border-t border-[var(--color-card-border)]";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("BACKDROP", BACKDROP),
        ("PANEL", PANEL),
        ("PANEL_OPEN", PANEL_OPEN),
        ("SIDE_LEFT", SIDE_LEFT),
        ("SIDE_RIGHT", SIDE_RIGHT),
        ("SIDE_TOP", SIDE_TOP),
        ("SIDE_BOTTOM", SIDE_BOTTOM),
        ("SIZE_SM_H", SIZE_SM_H),
        ("SIZE_MD_H", SIZE_MD_H),
        ("SIZE_LG_H", SIZE_LG_H),
        ("SIZE_FULL_H", SIZE_FULL_H),
        ("SIZE_SM_V", SIZE_SM_V),
        ("SIZE_MD_V", SIZE_MD_V),
        ("SIZE_LG_V", SIZE_LG_V),
        ("SIZE_FULL_V", SIZE_FULL_V),
        ("HEADER", HEADER),
        ("HEADER_TITLE", HEADER_TITLE),
        ("CLOSE_BUTTON", CLOSE_BUTTON),
        ("BODY", BODY),
        ("FOOTER", FOOTER),
    ]
}
