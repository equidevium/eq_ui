//! Style constants for EqLabel.

/// Form label text
pub const LABEL: &str =
    "text-sm font-medium text-[var(--color-label-primary)]";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("LABEL", LABEL),
    ]
}
