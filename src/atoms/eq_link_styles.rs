//! Style constants for EqLink.

/// Default styled anchor link
pub const LINK: &str =
    "text-[var(--color-label-primary)] underline hover:text-[var(--color-label-bold)] active:text-[var(--color-label-bold)] transition";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("LINK", LINK),
    ]
}
