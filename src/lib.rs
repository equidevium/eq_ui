//! EqUI - Portable Dioxus UI component library.
//!
//! Atomic design building blocks (atoms, molecules, organisms) + theme.
//! This crate has a single dependency: `dioxus`.
//!
//! Enable the `playground` feature to access interactive component demos
//! and the `EqPlayground` showcase.

use dioxus::prelude::*;

pub const UI_TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub const UI_INDEX_CSS: Asset = asset!("/assets/theme/index.css");
pub const UI_BUTTONS_CSS: Asset = asset!("/assets/theme/buttons.css");

#[cfg(feature = "playground")]
pub mod playground;
pub mod theme;
pub mod atoms;
pub mod molecules;
pub mod organisms;
pub mod eq_theme;
pub mod preview_enum_trait;

pub use eq_ui_macros::{preview, PreviewEnum};

#[cfg(feature = "playground")]
pub use playground::{ComponentDescriptor, ComponentCategory, UsageExample, EqPlayground};
pub use theme::*;

/// Returns descriptors for all built-in components.
///
/// Pass this to `EqPlayground` to get the full component showcase.
/// Append your own descriptors to include custom components.
#[cfg(feature = "playground")]
pub fn all_component_descriptors() -> Vec<ComponentDescriptor> {
    vec![
        // Guide
        playground::playground_guide::descriptor(),
        // Atoms
        atoms::eq_text::descriptor(),
        atoms::eq_label::descriptor(),
        atoms::eq_link::descriptor(),
        atoms::eq_input::descriptor(),
        atoms::eq_icon::descriptor(),
        atoms::eq_image::descriptor(),
        atoms::eq_checkbox::descriptor(),
        atoms::eq_button::descriptor(),
        atoms::eq_divider::descriptor(),
        atoms::eq_scrollable_space::descriptor(),
        atoms::eq_video::descriptor(),
        atoms::eq_progress::descriptor(),
        atoms::eq_tab::descriptor(),
        atoms::eq_radio_group::descriptor(),
        atoms::eq_switch::descriptor(),
        // Molecules
        molecules::eq_card::descriptor(),
        molecules::eq_image_card::descriptor(),
        molecules::eq_carousel::descriptor(),
        molecules::eq_tree::descriptor(),
        molecules::eq_accordion::descriptor(),
        molecules::eq_nav_item::descriptor(),
        // Organisms
        organisms::eq_header::descriptor(),
        organisms::eq_footer::descriptor(),
        organisms::eq_hero_shell::descriptor(),
        organisms::eq_page_section::descriptor(),
        organisms::eq_app_shell::descriptor(),
        organisms::eq_navbar::descriptor(),
        organisms::eq_grid::grid::descriptor(),
        // Theming
        playground::theme_showcase::descriptor(),
    ]
}
