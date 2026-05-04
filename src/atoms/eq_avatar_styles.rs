//! Style constants for EqAvatar.

/// Shared base: centered flex, rounded-full, overflow hidden, shrink-0.
pub const BASE: &str =
    "inline-flex items-center justify-center rounded-full overflow-hidden shrink-0 \
     select-none font-semibold uppercase";

/// Small size (32×32).
pub const SM: &str = "size-8 text-xs";
/// Medium size (40×40, default).
pub const MD: &str = "size-10 text-sm";
/// Large size (48×48).
pub const LG: &str = "size-12 text-base";
/// Extra-large size (64×64).
pub const XL: &str = "size-16 text-lg";

/// Image element inside the avatar.
pub const IMAGE: &str = "w-full h-full object-cover";

/// Initials fallback background.
pub const INITIALS: &str =
    "bg-[var(--color-accent-primary)] text-white";

/// Icon fallback (generic person silhouette).
pub const ICON_FALLBACK: &str =
    "bg-[var(--color-tertiary-dark)] text-[var(--color-label-secondary)]";

/// Online status indicator dot (absolute positioned).
pub const STATUS_DOT: &str =
    "absolute bottom-0 right-0 rounded-full border-2 \
     border-[var(--color-core-dark)]";

/// Status dot sizes per avatar size.
pub const STATUS_SM: &str = "size-2";
pub const STATUS_MD: &str = "size-2.5";
pub const STATUS_LG: &str = "size-3";
pub const STATUS_XL: &str = "size-3.5";

/// Online color.
pub const STATUS_ONLINE: &str = "bg-emerald-500";
/// Offline color.
pub const STATUS_OFFLINE: &str = "bg-[var(--color-label-secondary)]/40";
/// Busy color.
pub const STATUS_BUSY: &str = "bg-red-500";

/// Ring around avatar (e.g. for selected state).
pub const RING: &str = "ring-2 ring-[var(--color-accent-primary)] ring-offset-2 \
     ring-offset-[var(--color-core-dark)]";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("BASE", BASE),
        ("SM", SM),
        ("MD", MD),
        ("LG", LG),
        ("XL", XL),
        ("IMAGE", IMAGE),
        ("INITIALS", INITIALS),
        ("ICON_FALLBACK", ICON_FALLBACK),
        ("STATUS_DOT", STATUS_DOT),
        ("STATUS_SM", STATUS_SM),
        ("STATUS_MD", STATUS_MD),
        ("STATUS_LG", STATUS_LG),
        ("STATUS_XL", STATUS_XL),
        ("STATUS_ONLINE", STATUS_ONLINE),
        ("STATUS_OFFLINE", STATUS_OFFLINE),
        ("STATUS_BUSY", STATUS_BUSY),
        ("RING", RING),
    ]
}
