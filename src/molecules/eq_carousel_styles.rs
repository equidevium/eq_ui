//! Style constants for EqCarousel.

/// Outer carousel wrapper
pub const CAROUSEL: &str = "relative w-full overflow-hidden";

/// The horizontal strip that holds all slides side-by-side.
/// `translateX` is set dynamically via inline style.
pub const SLIDE_STRIP: &str =
    "flex transition-transform duration-[var(--transition-normal)] ease-in-out";

/// A single slide within the strip - full width of the carousel.
pub const SLIDE: &str = "w-full shrink-0";

// ── Peek mode ────────────────────────────────────────────────────

/// Peek carousel wrapper - shows edges of neighbouring slides.
pub const CAROUSEL_PEEK: &str = "relative w-full overflow-hidden";

/// Peek strip - like SLIDE_STRIP but slides are narrower (80%).
pub const SLIDE_STRIP_PEEK: &str =
    "flex transition-transform duration-[var(--transition-normal)] ease-in-out";

/// Each slide in peek mode is 80% width so neighbours are visible.
pub const SLIDE_PEEK: &str = "shrink-0";

/// Left fade mask overlay for previous slide peek.
pub const PEEK_FADE_LEFT: &str =
    "absolute inset-y-0 left-0 z-10 pointer-events-none";

/// Right fade mask overlay for next slide peek.
pub const PEEK_FADE_RIGHT: &str =
    "absolute inset-y-0 right-0 z-10 pointer-events-none";

// ── Arrow buttons ────────────────────────────────────────────────

/// Shared arrow button base
pub const ARROW_BASE: &str =
    "absolute top-1/2 -translate-y-1/2 z-20 \
     flex items-center justify-center \
     size-10 rounded-full \
     bg-black/30 text-white \
     hover:bg-black/50 active:bg-black/50 \
     transition cursor-pointer";

/// Left arrow positioning
pub const ARROW_LEFT: &str = "left-2";

/// Right arrow positioning
pub const ARROW_RIGHT: &str = "right-2";

/// Arrow icon sizing
pub const ARROW_ICON: &str = "size-5";

// ── Dot indicators ───────────────────────────────────────────────

/// Dot indicator container
pub const DOTS: &str = "flex justify-center items-center gap-2 mt-4";

/// Inactive dot
pub const DOT: &str =
    "size-3 md:size-2.5 rounded-full bg-[var(--color-label-secondary)]/40 \
     hover:bg-[var(--color-label-secondary)]/70 active:bg-[var(--color-label-secondary)]/70 \
     transition cursor-pointer";

/// Active dot
pub const DOT_ACTIVE: &str =
    "size-3 md:size-2.5 rounded-full bg-[var(--color-label-bold)] \
     cursor-pointer";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("CAROUSEL", CAROUSEL),
        ("SLIDE_STRIP", SLIDE_STRIP),
        ("SLIDE", SLIDE),
        ("CAROUSEL_PEEK", CAROUSEL_PEEK),
        ("SLIDE_STRIP_PEEK", SLIDE_STRIP_PEEK),
        ("SLIDE_PEEK", SLIDE_PEEK),
        ("PEEK_FADE_LEFT", PEEK_FADE_LEFT),
        ("PEEK_FADE_RIGHT", PEEK_FADE_RIGHT),
        ("ARROW_BASE", ARROW_BASE),
        ("ARROW_LEFT", ARROW_LEFT),
        ("ARROW_RIGHT", ARROW_RIGHT),
        ("ARROW_ICON", ARROW_ICON),
        ("DOTS", DOTS),
        ("DOT", DOT),
        ("DOT_ACTIVE", DOT_ACTIVE),
    ]
}
