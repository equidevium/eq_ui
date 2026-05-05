//! Style constants for EqScrollableSpace.

/// Outer container - fills available height and scrolls vertically.
pub const CONTAINER: &str = "overflow-y-auto flex-1 min-h-0";

/// Scrollbar styling - thin, themed.
/// Tailwind v4 supports `scrollbar-thin` and custom scrollbar colors. However, this only works with the tailwindcss-scrollbar plugin so can't be used unless that is installed.
pub const SCROLLBAR: &str = "[&::-webkit-scrollbar]:w-1 [&::-webkit-scrollbar-thumb]:bg-[var(--color-scrollbar-thumb)] [&::-webkit-scrollbar-track]:bg-[var(--color-scrollbar-track)] [&::-webkit-scrollbar-track]:bg-transparent [&::-webkit-scrollbar-track]:shadow-none";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("CONTAINER", CONTAINER),
        ("SCROLLBAR", SCROLLBAR),
    ]
}
