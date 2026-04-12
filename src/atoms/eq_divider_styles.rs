//! Style constants for EqDivider.

/// Base divider - horizontal rule with theme border color.
pub const BASE: &str = "border-0 border-t border-[var(--color-card-border)]";

/// Dashed variant (- - -).
pub const DASHED: &str = "border-dashed";

/// Dotted variant.
pub const DOTTED: &str = "border-dotted";

/// Thick variant (2px).
pub const THICK: &str = "border-t-2";

/// Extra thick variant (4px).
pub const EXTRA_THICK: &str = "border-t-4";

/// Spacer - invisible divider that only adds vertical space.
pub const SPACER: &str = "border-0 my-4";

/// Default vertical spacing.
pub const SPACING_DEFAULT: &str = "my-4";

/// Compact vertical spacing.
pub const SPACING_COMPACT: &str = "my-2";

/// Wide vertical spacing.
pub const SPACING_WIDE: &str = "my-8";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("BASE", BASE),
        ("DASHED", DASHED),
        ("DOTTED", DOTTED),
        ("THICK", THICK),
        ("EXTRA_THICK", EXTRA_THICK),
        ("SPACER", SPACER),
        ("SPACING_DEFAULT", SPACING_DEFAULT),
        ("SPACING_COMPACT", SPACING_COMPACT),
        ("SPACING_WIDE", SPACING_WIDE),
    ]
}
