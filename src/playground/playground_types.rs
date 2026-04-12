//! Playground types - data structures for component self-description.
//!
//! These types are always compiled (no feature gate) because they are
//! lightweight data definitions that may be useful for tooling beyond
//! the playground. The actual demo rendering code in each component
//! is gated behind `#[cfg(feature = "playground")]`.

use dioxus::prelude::*;

/// Atomic design layer a component belongs to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ComponentCategory {
    /// Documentation and guides - always rendered first in the tree.
    Guide,
    Atom,
    Molecule,
    Organism,
    Theming,
}

impl ComponentCategory {
    /// Display label for the sidebar tree group.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Guide => "Guide",
            Self::Atom => "Atoms",
            Self::Molecule => "Molecules",
            Self::Organism => "Organisms",
            Self::Theming => "Theming",
        }
    }

    /// Sort order for consistent tree rendering.
    pub fn sort_order(&self) -> u8 {
        match self {
            Self::Guide => 0,
            Self::Atom => 1,
            Self::Molecule => 2,
            Self::Organism => 3,
            Self::Theming => 4,
        }
    }
}

/// A single usage example displayed as a code block in the playground.
#[derive(Clone, PartialEq)]
pub struct UsageExample {
    /// Short label for the example (e.g. "Basic", "With icon", "Disabled state").
    pub label: &'static str,
    /// Rust code string rendered in the code block.
    pub code: String,
}

/// Complete playground metadata for a component.
///
/// Each eq_ui component exposes a `descriptor()` function (behind the
/// `playground` feature) that returns one of these. The playground
/// collects all descriptors and builds its UI from them.
///
/// External users write their own `descriptor()` for custom components
/// and pass them alongside the built-in ones.
#[derive(Clone, PartialEq)]
pub struct ComponentDescriptor {
    /// URL-safe identifier for routing (e.g. "button", "progress").
    pub id: &'static str,
    /// Display name in the tree and header (e.g. "EqButton").
    pub name: &'static str,
    /// Atomic design layer - determines tree grouping.
    pub category: ComponentCategory,
    /// One-line description shown below the component name.
    pub description: &'static str,
    /// Returns the style token catalog from the component's `_styles.rs`.
    /// Each entry is `(CONSTANT_NAME, "tailwind class string")`.
    pub style_tokens: fn() -> Vec<(&'static str, &'static str)>,
    /// Returns usage examples as displayable code blocks.
    pub usage_examples: fn() -> Vec<UsageExample>,
    /// Renders the interactive demo (prop controls + live preview).
    pub render_demo: fn() -> Element,
    /// Renders a static gallery of all variants.
    pub render_gallery: fn() -> Element,
}
