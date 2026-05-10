//! Style constants for EqBottomNav.

/// Wrapper. Horizontal flex row spanning the full width, with a hairline
/// top border to separate from the body content.
pub const WRAPPER: &str = "flex items-stretch w-full \
     bg-[var(--color-card)] \
     border-t border-[var(--color-card-border)]";

/// Single item button. Vertical stack of icon + label, equal width via
/// `flex-1`, minimum touch target of 56px tall.
pub const ITEM: &str = "flex-1 flex flex-col items-center justify-center gap-[2px] \
     min-h-[56px] py-[6px] px-[2px] \
     bg-transparent border-0 cursor-pointer \
     text-[var(--color-label-secondary)] \
     hover:text-[var(--color-label-primary)] \
     transition-colors";

/// Item in the active state. Uses the theme's accent color so the
/// selection is obvious without a separate indicator pill.
pub const ITEM_ACTIVE: &str = "text-[var(--color-accent-primary)]";

/// Item in the disabled state. Reduced opacity, no pointer events.
pub const ITEM_DISABLED: &str = "opacity-40 cursor-not-allowed pointer-events-none";

/// Icon container. Sized for visual balance with the label below.
/// `relative` so the badge can absolute-position against it.
pub const ITEM_ICON: &str = "relative flex items-center justify-center w-[24px] h-[24px]";

/// Label text under the icon.
pub const ITEM_LABEL: &str = "text-[11px] font-medium leading-tight";

/// Badge container, top-right of the icon.
pub const BADGE_BASE: &str = "absolute top-[-2px] right-[-6px] z-10 \
     flex items-center justify-center \
     bg-[var(--color-error)] text-white";

/// Count-style badge. Pill with the number inside.
pub const BADGE_COUNT: &str = "min-w-[16px] h-[16px] px-[4px] rounded-full \
     text-[10px] font-semibold leading-none";

/// Dot-style badge. Small filled circle, no number.
pub const BADGE_DOT: &str = "w-[8px] h-[8px] rounded-full";

pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("WRAPPER", WRAPPER),
        ("ITEM", ITEM),
        ("ITEM_ACTIVE", ITEM_ACTIVE),
        ("ITEM_DISABLED", ITEM_DISABLED),
        ("ITEM_ICON", ITEM_ICON),
        ("ITEM_LABEL", ITEM_LABEL),
        ("BADGE_BASE", BADGE_BASE),
        ("BADGE_COUNT", BADGE_COUNT),
        ("BADGE_DOT", BADGE_DOT),
    ]
}
