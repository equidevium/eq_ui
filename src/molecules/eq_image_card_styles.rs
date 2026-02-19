//! Style constants for EqImageCard and its modes.

/// Card wrapper for caption-below mode
pub const CARD_WRAPPER: &str = "space-y-3";

/// Figcaption container
pub const FIGCAPTION: &str = "space-y-2 px-1 py-2";
/// Caption title
pub const CAPTION_TITLE: &str =
    "text-lg font-semibold text-[var(--color-label-primary)]";
/// Caption description
pub const CAPTION_DESCRIPTION: &str =
    "text-sm text-[var(--color-label-secondary)] leading-relaxed";
/// Caption attribution / credit
pub const CAPTION_ATTRIBUTION: &str =
    "text-xs text-[var(--color-label-secondary)] italic pt-1 border-t border-[var(--color-card-border)]";

/// Overlay container — positions children relatively
pub const OVERLAY_CONTAINER: &str = "relative";
/// Dark gradient from bottom for text legibility
pub const OVERLAY_GRADIENT: &str =
    "absolute inset-0 bg-gradient-to-t from-black/80 via-black/20 to-transparent";
/// Text wrapper pinned to bottom of overlay
pub const OVERLAY_TEXT_WRAPPER: &str =
    "absolute inset-0 flex flex-col justify-end p-4";
/// Overlay title — white for contrast
pub const OVERLAY_TITLE: &str = "text-lg font-semibold text-white";
/// Overlay description
pub const OVERLAY_DESCRIPTION: &str = "text-sm text-white/90 leading-relaxed mt-1";
/// Overlay attribution
pub const OVERLAY_ATTRIBUTION: &str = "text-xs text-white/70 italic mt-2";
