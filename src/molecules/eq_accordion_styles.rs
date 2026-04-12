//! Style constants for EqAccordion.

/// Outer accordion container.
pub const ACCORDION: &str = "flex flex-col divide-y divide-[var(--color-card-border)]";

/// A single panel wrapper.
pub const PANEL: &str = "";

/// The clickable header row.
pub const HEADER: &str =
    "flex items-center justify-between w-full gap-3 px-4 py-3 cursor-pointer select-none text-left transition-colors hover:bg-[var(--color-card)]/40 active:bg-[var(--color-card)]/40";

/// Header text defaults.
pub const HEADER_TEXT: &str =
    "font-medium text-sm text-[var(--color-label-primary)]";

/// Chevron icon for expand/collapse indicator.
pub const CHEVRON: &str =
    "size-5 shrink-0 text-[var(--color-label-secondary)] transition-transform duration-[var(--transition-fast)]";

/// Chevron rotated when panel is open.
pub const CHEVRON_OPEN: &str = "rotate-180";

/// Content body wrapper - uses grid for smooth height animation.
pub const BODY: &str =
    "grid transition-[grid-template-rows] duration-[var(--transition-fast)] ease-in-out";

/// Grid row sizing when collapsed.
pub const BODY_CLOSED: &str = "grid-rows-[0fr]";

/// Grid row sizing when expanded.
pub const BODY_OPEN: &str = "grid-rows-[1fr]";

/// Inner content wrapper - overflow hidden for the grid animation.
pub const BODY_INNER: &str = "overflow-hidden px-4";

/// Padding for the visible content inside the panel.
pub const CONTENT: &str = "pb-4 text-sm text-[var(--color-label-secondary)]";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("ACCORDION", ACCORDION),
        ("PANEL", PANEL),
        ("HEADER", HEADER),
        ("HEADER_TEXT", HEADER_TEXT),
        ("CHEVRON", CHEVRON),
        ("CHEVRON_OPEN", CHEVRON_OPEN),
        ("BODY", BODY),
        ("BODY_CLOSED", BODY_CLOSED),
        ("BODY_OPEN", BODY_OPEN),
        ("BODY_INNER", BODY_INNER),
        ("CONTENT", CONTENT),
    ]
}
