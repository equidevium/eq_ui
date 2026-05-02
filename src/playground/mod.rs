//! Playground - interactive component showcase toolkit.
//!
//! This module is feature-gated behind `playground`. It provides:
//!
//! - **`ComponentDescriptor`** - self-describing metadata each component exports
//! - **`EqPlayground`** - the reusable two-panel showcase organism
//! - **Helpers** - DemoSection, CodeBlock, StyleInfo, PropSelect, PropInput, PropToggle
//!
//! ```rust,ignore
//! use eq_ui::playground::{EqPlayground, ComponentDescriptor};
//! use eq_ui::all_component_descriptors;
//!
//! fn App() -> Element {
//!     rsx! { EqPlayground { descriptors: all_component_descriptors() } }
//! }
//! ```

pub mod playground_types;
pub mod playground_helpers;
pub mod eq_playground;
pub mod eq_playground_styles;
pub mod theme_showcase;
pub mod playground_guide;

pub use playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};
pub use playground_helpers::{
    CodeBlock, DemoSection, StyleInfo, PropSelect, PropInput, PropToggle,
    format_catalog, highlight_rust, highlight_styles,
    PROP_ROW, PROP_LABEL, PROP_CONTROL,
};
pub use eq_playground::EqPlayground;

pub use crate::playground_enum_trait::PlaygroundEnumInfo;
