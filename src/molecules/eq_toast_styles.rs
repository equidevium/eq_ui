//! Style constants for EqToast and EqToastList.

// ── Toast container (fixed position) ────────────────────────────────

/// Fixed container anchored to a viewport corner.
pub const CONTAINER: &str =
    "fixed z-50 flex flex-col gap-2 pointer-events-none";

pub const POS_TOP_RIGHT: &str = "top-4 right-4";
pub const POS_TOP_LEFT: &str = "top-4 left-4";
pub const POS_TOP_CENTER: &str = "top-4 left-1/2 -translate-x-1/2";
pub const POS_BOTTOM_RIGHT: &str = "bottom-4 right-4";
pub const POS_BOTTOM_LEFT: &str = "bottom-4 left-4";
pub const POS_BOTTOM_CENTER: &str = "bottom-4 left-1/2 -translate-x-1/2";

// ── Individual toast ────────────────────────────────────────────────

pub const TOAST: &str =
    "pointer-events-auto w-80 max-w-[calc(100vw-2rem)] \
     rounded-lg border shadow-lg shadow-black/20 \
     px-4 py-3 flex items-start gap-3 \
     transition-all duration-300 transform";

pub const TOAST_ENTER: &str = "opacity-100 translate-y-0";
pub const TOAST_EXIT: &str = "opacity-0 translate-y-2";

// ── Severity variants ───────────────────────────────────────────────

pub const INFO: &str =
    "bg-[var(--color-card)] border-[var(--color-card-border)] \
     text-[var(--color-label-primary)]";

pub const SUCCESS: &str =
    "bg-emerald-950/80 border-emerald-700/50 text-emerald-100";

pub const WARNING: &str =
    "bg-amber-950/80 border-amber-700/50 text-amber-100";

pub const ERROR: &str =
    "bg-red-950/80 border-red-700/50 text-red-100";

// ── Inner layout ────────────────────────────────────────────────────

pub const ICON: &str = "shrink-0 mt-0.5";

pub const CONTENT: &str = "flex-1 min-w-0";

pub const TITLE: &str = "text-sm font-semibold";

pub const MESSAGE: &str = "text-sm opacity-80 mt-0.5";

pub const CLOSE: &str =
    "shrink-0 p-0.5 rounded opacity-60 hover:opacity-100 \
     transition-opacity cursor-pointer";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("CONTAINER", CONTAINER),
        ("TOAST", TOAST),
        ("INFO", INFO),
        ("SUCCESS", SUCCESS),
        ("WARNING", WARNING),
        ("ERROR", ERROR),
        ("ICON", ICON),
        ("CONTENT", CONTENT),
        ("TITLE", TITLE),
        ("MESSAGE", MESSAGE),
        ("CLOSE", CLOSE),
    ]
}
