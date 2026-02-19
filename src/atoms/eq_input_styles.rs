//! Style constants for EqInput.

/// Form field wrapper — adds vertical spacing between label and input
pub const FIELD: &str = "space-y-2";

/// Helper / hint text below an input
pub const HELP: &str =
    "text-sm text-[var(--color-label-secondary)]";

/// Error message text
pub const ERROR: &str =
    "text-sm text-red-400";

/// Base input / textarea control
pub const CONTROL: &str =
    "w-full rounded-md border px-3 py-2 text-sm outline-none transition \
     bg-[var(--color-card)]/40 text-[var(--color-label-primary)] \
     border-[var(--color-card-border)] \
     placeholder:text-[var(--color-label-secondary)]/70 \
     focus:ring-2 focus:ring-[var(--color-hover-button)]/60 focus:border-[var(--color-hover-button)]";

/// Extra classes applied only to `<textarea>`
pub const TEXTAREA: &str = "min-h-[120px] resize-y";

/// Disabled state overlay
pub const DISABLED: &str = "opacity-60 cursor-not-allowed";
