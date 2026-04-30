//! The `PreviewEnumInfo` trait — always compiled so `#[derive(PreviewEnum)]`
//! works regardless of the `playground` feature flag.

/// Trait implemented by `#[derive(PreviewEnum)]` to expose enum variant
/// names for auto-generated playground demo controls.
///
/// This enables the `#[preview]` macro to generate `PropSelect` dropdowns
/// for enum props without the macro needing to see the enum definition.
pub trait PreviewEnumInfo: Clone + PartialEq + Default {
    /// All variant names as string slices (e.g. `["Sm", "Md", "Lg"]`).
    fn variant_names() -> &'static [&'static str];
    /// Construct a variant from its name. Returns the `#[default]` variant
    /// if the name doesn't match.
    fn from_name(s: &str) -> Self;
    /// The name of the `#[default]` variant.
    fn default_name() -> &'static str;
}
