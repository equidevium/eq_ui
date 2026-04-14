//! Style constants for EqSwitch.

/// Outer wrapper - inline-flex for alignment with labels.
pub const WRAPPER: &str =
    "inline-flex items-center gap-2.5 cursor-pointer select-none";

/// Wrapper when disabled.
pub const WRAPPER_DISABLED: &str =
    "inline-flex items-center gap-2.5 cursor-not-allowed select-none opacity-50";

/// Track (off) - the pill-shaped background.
pub const TRACK: &str =
    "inline-flex items-center shrink-0 rounded-full \
     bg-[var(--color-label-secondary)]/30 p-0.5";

/// Track (on).
pub const TRACK_ON: &str =
    "inline-flex items-center shrink-0 rounded-full \
     bg-[var(--color-accent-primary)] p-0.5";

/// The sliding thumb circle. Offset controlled via inline style.
pub const THUMB: &str =
    "rounded-full bg-white shadow-sm \
     transition-all duration-150 ease-in-out";

/// Small track dimensions.
pub const SM_TRACK: &str = "w-8 h-[18px]";
/// Small thumb dimensions.
pub const SM_THUMB: &str = "size-3.5";

/// Medium track dimensions.
pub const MD_TRACK: &str = "w-10 h-[22px]";
/// Medium thumb dimensions.
pub const MD_THUMB: &str = "size-[18px]";

/// Large track dimensions.
pub const LG_TRACK: &str = "w-12 h-[26px]";
/// Large thumb dimensions.
pub const LG_THUMB: &str = "size-[22px]";

/// Label text beside the switch.
pub const LABEL: &str =
    "text-sm text-[var(--color-label-primary)]";

/// Description text below the label.
pub const DESCRIPTION: &str =
    "text-xs text-[var(--color-label-secondary)]";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("WRAPPER", WRAPPER),
        ("WRAPPER_DISABLED", WRAPPER_DISABLED),
        ("TRACK", TRACK),
        ("TRACK_ON", TRACK_ON),
        ("THUMB", THUMB),
        ("SM_TRACK", SM_TRACK),
        ("SM_THUMB", SM_THUMB),
        ("MD_TRACK", MD_TRACK),
        ("MD_THUMB", MD_THUMB),
        ("LG_TRACK", LG_TRACK),
        ("LG_THUMB", LG_THUMB),
        ("LABEL", LABEL),
        ("DESCRIPTION", DESCRIPTION),
    ]
}
