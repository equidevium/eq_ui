//! Style constants for EqModal.

/// Full-viewport backdrop overlay.
pub const BACKDROP: &str =
    "fixed inset-0 z-50 flex items-center justify-center \
     bg-[var(--color-surface-overlay)] \
     transition-opacity duration-200";

/// Backdrop when the modal is open (visible).
pub const BACKDROP_OPEN: &str = "opacity-100";

/// Backdrop when the modal is closed (hidden, pointer-events off).
pub const BACKDROP_CLOSED: &str = "opacity-0 pointer-events-none";

/// The dialog panel itself.
pub const PANEL: &str =
    "relative w-full mx-4 rounded-xl border border-[var(--color-card-border)] \
     bg-[var(--color-card)] shadow-2xl shadow-black/40 \
     flex flex-col max-h-[85vh] \
     transition-all duration-200 transform";

/// Panel when open (scaled up, visible).
pub const PANEL_OPEN: &str = "scale-100 opacity-100";

/// Panel when closed (scaled down, invisible).
pub const PANEL_CLOSED: &str = "scale-95 opacity-0";

// ── Size presets ────────────────────────────────────────────────────

pub const SIZE_SM: &str = "max-w-sm";
pub const SIZE_MD: &str = "max-w-lg";
pub const SIZE_LG: &str = "max-w-2xl";
pub const SIZE_XL: &str = "max-w-4xl";
pub const SIZE_FULL: &str = "max-w-[calc(100vw-2rem)] md:max-w-[calc(100vw-4rem)]";

// ── Header / body / footer ──────────────────────────────────────────

pub const HEADER: &str =
    "flex items-center justify-between px-5 py-4 \
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
    "px-5 py-4 overflow-y-auto text-sm text-[var(--color-label-primary)]";

pub const FOOTER: &str =
    "flex items-center justify-end gap-2 px-5 py-4 \
     border-t border-[var(--color-card-border)]";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("BACKDROP", BACKDROP),
        ("PANEL", PANEL),
        ("SIZE_SM", SIZE_SM),
        ("SIZE_MD", SIZE_MD),
        ("SIZE_LG", SIZE_LG),
        ("SIZE_XL", SIZE_XL),
        ("SIZE_FULL", SIZE_FULL),
        ("HEADER", HEADER),
        ("HEADER_TITLE", HEADER_TITLE),
        ("CLOSE_BUTTON", CLOSE_BUTTON),
        ("BODY", BODY),
        ("FOOTER", FOOTER),
    ]
}
