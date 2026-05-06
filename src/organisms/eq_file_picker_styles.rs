//! Style constants for EqFilePicker.

/// Outer wrapper.
pub const WRAPPER: &str = "flex flex-col gap-3";

/// Drop zone — dashed border area where files can be dropped.
pub const DROP_ZONE: &str =
    "flex flex-col items-center justify-center gap-3 \
     px-6 py-8 rounded-lg cursor-pointer select-none \
     border-2 border-dashed border-[var(--color-card-border)] \
     text-[var(--color-label-secondary)] \
     hover:border-[var(--color-accent-primary)]/60 \
     hover:bg-[var(--color-accent-primary)]/5 \
     transition-colors duration-150";

/// Drop zone when files are actively being dragged over it.
pub const DROP_ZONE_ACTIVE: &str =
    "border-[var(--color-accent-primary)] \
     bg-[var(--color-accent-primary)]/10";

/// Drop zone when disabled.
pub const DROP_ZONE_DISABLED: &str =
    "opacity-50 cursor-not-allowed pointer-events-none";

/// Upload icon in the drop zone.
pub const DROP_ICON: &str = "size-10 text-[var(--color-label-secondary)]";

/// Primary instruction text in the drop zone.
pub const DROP_TEXT: &str =
    "text-sm font-medium text-[var(--color-label-primary)]";

/// Secondary hint text (e.g. "or drag and drop").
pub const DROP_HINT: &str =
    "text-xs text-[var(--color-label-secondary)]";

/// Accepted formats hint.
pub const DROP_ACCEPT: &str =
    "text-xs text-[var(--color-label-secondary)]/70";

/// File list container.
pub const FILE_LIST: &str = "flex flex-col gap-2";

/// Single file row.
pub const FILE_ROW: &str =
    "flex items-center gap-3 px-3 py-2 rounded-md \
     bg-[var(--color-card)] \
     border border-[var(--color-card-border)]";

/// File thumbnail / icon container.
pub const FILE_THUMB: &str =
    "size-10 shrink-0 rounded-md overflow-hidden \
     bg-[var(--color-tertiary-dark)]/40 \
     flex items-center justify-center";

/// Thumbnail image.
pub const FILE_THUMB_IMG: &str = "size-full object-cover";

/// File type icon fallback.
pub const FILE_ICON: &str = "size-5 text-[var(--color-label-secondary)]";

/// File info (name + size).
pub const FILE_INFO: &str = "flex-1 min-w-0 flex flex-col";

/// File name.
pub const FILE_NAME: &str =
    "text-sm font-medium text-[var(--color-label-primary)] truncate";

/// File size text.
pub const FILE_SIZE: &str =
    "text-xs text-[var(--color-label-secondary)]";

/// Error text on a file (e.g. too large).
pub const FILE_ERROR: &str =
    "text-xs text-red-400";

/// Remove button.
pub const FILE_REMOVE: &str =
    "p-1 rounded-md shrink-0 \
     text-[var(--color-label-secondary)] \
     hover:text-red-400 \
     hover:bg-red-500/10 \
     transition-colors cursor-pointer";

/// Progress bar wrapper within a file row.
pub const FILE_PROGRESS: &str = "w-24 shrink-0";

/// Error banner when too many files, etc.
pub const ERROR_BANNER: &str =
    "px-3 py-2 rounded-md text-sm \
     bg-red-500/10 text-red-400 \
     border border-red-500/20";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("WRAPPER", WRAPPER),
        ("DROP_ZONE", DROP_ZONE),
        ("DROP_ZONE_ACTIVE", DROP_ZONE_ACTIVE),
        ("DROP_ZONE_DISABLED", DROP_ZONE_DISABLED),
        ("DROP_ICON", DROP_ICON),
        ("DROP_TEXT", DROP_TEXT),
        ("DROP_HINT", DROP_HINT),
        ("DROP_ACCEPT", DROP_ACCEPT),
        ("FILE_LIST", FILE_LIST),
        ("FILE_ROW", FILE_ROW),
        ("FILE_THUMB", FILE_THUMB),
        ("FILE_THUMB_IMG", FILE_THUMB_IMG),
        ("FILE_ICON", FILE_ICON),
        ("FILE_INFO", FILE_INFO),
        ("FILE_NAME", FILE_NAME),
        ("FILE_SIZE", FILE_SIZE),
        ("FILE_ERROR", FILE_ERROR),
        ("FILE_REMOVE", FILE_REMOVE),
        ("FILE_PROGRESS", FILE_PROGRESS),
        ("ERROR_BANNER", ERROR_BANNER),
    ]
}
