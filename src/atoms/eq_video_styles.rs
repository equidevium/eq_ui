//! Style constants for EqVideo.

/// Video wrapper container
pub const WRAPPER: &str = "relative overflow-hidden";

/// Small (192px width)
pub const SM: &str = "w-48";
/// Medium (256px width) — default
pub const MD: &str = "w-64";
/// Large (384px width)
pub const LG: &str = "w-96";
/// Full width
pub const FULL: &str = "w-full";

/// 16:9 aspect ratio
pub const RATIO_16_9: &str = "aspect-video";
/// 4:3 aspect ratio
pub const RATIO_4_3: &str = "aspect-[4/3]";
/// 1:1 square aspect ratio
pub const RATIO_SQUARE: &str = "aspect-square";
/// No aspect constraint
pub const RATIO_FREE: &str = "";

/// Rounded corners
pub const ROUNDED: &str = "rounded-lg";

/// Video element — fills wrapper
pub const VIDEO_ELEMENT: &str = "w-full h-full";

/// Poster overlay container — covers the video, click to play
pub const POSTER_OVERLAY: &str = "absolute inset-0 z-10 cursor-pointer";

/// Play icon circle centered on poster
pub const PLAY_ICON: &str = "absolute inset-0 flex items-center justify-center";

/// Play icon background circle
pub const PLAY_CIRCLE: &str = "size-16 rounded-full bg-black/60 flex items-center justify-center backdrop-blur-sm transition hover:bg-black/80";

/// Play triangle SVG
pub const PLAY_SVG: &str = "size-7 text-white ml-1";
