//! Style constants for EqPageSection component.

pub const SECTION_WRAP: &str = "py-8 md:py-16";
pub const SECTION_TITLE: &str = "text-2xl md:text-3xl font-semibold tracking-tight";
pub const SECTION_DESC: &str = "mt-2 w-full text-[var(--color-label-secondary)]";
pub const SECTION_BODY: &str = "mt-6 md:mt-8";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("SECTION_WRAP", SECTION_WRAP),
        ("SECTION_TITLE", SECTION_TITLE),
        ("SECTION_DESC", SECTION_DESC),
        ("SECTION_BODY", SECTION_BODY),
    ]
}
