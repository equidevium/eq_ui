//! Style constants for EqCheckbox.

/// Outer wrapper - inline-flex for alignment with labels.
pub const WRAPPER: &str =
    "inline-flex items-center gap-2 cursor-pointer select-none";

/// Wrapper when disabled.
pub const WRAPPER_DISABLED: &str =
    "inline-flex items-center gap-2 cursor-not-allowed select-none opacity-50";

/// The icon element representing the checkbox visual.
pub const ICON: &str =
    "size-5 shrink-0 text-[var(--color-label-secondary)] \
     transition-colors";

/// Icon when checked or indeterminate.
pub const ICON_ACTIVE: &str =
    "size-5 shrink-0 text-[var(--color-accent-primary)] \
     transition-colors";

/// Optional label text beside the checkbox.
pub const LABEL: &str =
    "text-sm text-[var(--color-label-primary)]";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("WRAPPER", WRAPPER),
        ("WRAPPER_DISABLED", WRAPPER_DISABLED),
        ("ICON", ICON),
        ("ICON_ACTIVE", ICON_ACTIVE),
        ("LABEL", LABEL),
    ]
}
