//! Style constants for EqCarousel.

/// Outer carousel wrapper
pub const CAROUSEL: &str = "relative w-full";

/// Container for the current slide
pub const SLIDE_CONTAINER: &str = "w-full";

/// Shared arrow button base
pub const ARROW_BASE: &str =
    "absolute top-1/2 -translate-y-1/2 z-10 \
     flex items-center justify-center \
     size-10 rounded-full \
     bg-black/30 text-white \
     hover:bg-black/50 \
     transition cursor-pointer";

/// Left arrow positioning
pub const ARROW_LEFT: &str = "left-2";

/// Right arrow positioning
pub const ARROW_RIGHT: &str = "right-2";

/// Arrow icon sizing
pub const ARROW_ICON: &str = "size-5";

/// Dot indicator container
pub const DOTS: &str = "flex justify-center items-center gap-2 mt-4";

/// Inactive dot
pub const DOT: &str =
    "size-2.5 rounded-full bg-[var(--color-label-secondary)]/40 \
     hover:bg-[var(--color-label-secondary)]/70 \
     transition cursor-pointer";

/// Active dot
pub const DOT_ACTIVE: &str =
    "size-2.5 rounded-full bg-[var(--color-label-bold)] \
     cursor-pointer";
