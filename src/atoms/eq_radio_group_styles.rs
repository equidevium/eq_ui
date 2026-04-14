//! Style constants for EqRadioGroup.

/// Container for the whole group.
pub const GROUP: &str = "flex flex-col gap-2";

/// Horizontal layout variant.
pub const GROUP_HORIZONTAL: &str = "flex flex-row flex-wrap gap-4";

/// Single radio item wrapper.
pub const ITEM: &str =
    "inline-flex items-center gap-2 cursor-pointer select-none";

/// Item when disabled.
pub const ITEM_DISABLED: &str =
    "inline-flex items-center gap-2 cursor-not-allowed select-none opacity-50";

/// The outer circle (unselected).
pub const CIRCLE: &str =
    "size-5 shrink-0 rounded-full border-2 \
     border-[var(--color-label-secondary)] \
     flex items-center justify-center";

/// The outer circle (selected).
pub const CIRCLE_ACTIVE: &str =
    "size-5 shrink-0 rounded-full border-2 \
     border-[var(--color-accent-primary)] \
     flex items-center justify-center";

/// The inner dot (visible when selected).
pub const DOT: &str =
    "size-2.5 rounded-full bg-[var(--color-accent-primary)]";

/// Label text beside the radio.
pub const LABEL: &str =
    "text-sm text-[var(--color-label-primary)]";

/// Description text below the label.
pub const DESCRIPTION: &str =
    "text-xs text-[var(--color-label-secondary)]";

/// Small size overrides.
pub const SM_CIRCLE: &str =
    "size-4 shrink-0 rounded-full border-2 \
     flex items-center justify-center";

/// Small inner dot.
pub const SM_DOT: &str =
    "size-2 rounded-full bg-[var(--color-accent-primary)]";

/// Large size overrides.
pub const LG_CIRCLE: &str =
    "size-6 shrink-0 rounded-full border-2 \
     flex items-center justify-center";

/// Large inner dot.
pub const LG_DOT: &str =
    "size-3 rounded-full bg-[var(--color-accent-primary)]";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("GROUP", GROUP),
        ("GROUP_HORIZONTAL", GROUP_HORIZONTAL),
        ("ITEM", ITEM),
        ("ITEM_DISABLED", ITEM_DISABLED),
        ("CIRCLE", CIRCLE),
        ("CIRCLE_ACTIVE", CIRCLE_ACTIVE),
        ("DOT", DOT),
        ("LABEL", LABEL),
        ("DESCRIPTION", DESCRIPTION),
        ("SM_CIRCLE", SM_CIRCLE),
        ("SM_DOT", SM_DOT),
        ("LG_CIRCLE", LG_CIRCLE),
        ("LG_DOT", LG_DOT),
    ]
}
