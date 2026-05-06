//! Style constants for EqVideo.

/// Video wrapper container
pub const WRAPPER: &str = "relative";

/// Small (192px width)
pub const SM: &str = "w-48";
/// Medium (256px width) - default
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

/// Video element - fills wrapper, object-contain keeps controls visible
pub const VIDEO_ELEMENT: &str = "w-full h-full object-contain";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("WRAPPER", WRAPPER),
        ("SM", SM),
        ("MD", MD),
        ("LG", LG),
        ("FULL", FULL),
        ("RATIO_16_9", RATIO_16_9),
        ("RATIO_4_3", RATIO_4_3),
        ("RATIO_SQUARE", RATIO_SQUARE),
        ("RATIO_FREE", RATIO_FREE),
        ("ROUNDED", ROUNDED),
        ("VIDEO_ELEMENT", VIDEO_ELEMENT),
    ]
}
