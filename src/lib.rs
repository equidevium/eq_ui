//! EqUI — Portable Dioxus UI component library.
//!
//! Atomic design building blocks (atoms, molecules, organisms) + theme.
//! This crate has a single dependency: `dioxus`.

use dioxus::prelude::*;

pub const UI_TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub const UI_INDEX_CSS: Asset = asset!("/assets/theme/index.css");
pub const UI_BUTTONS_CSS: Asset = asset!("/assets/theme/buttons.css");

pub mod theme;
pub mod atoms;
pub mod molecules;
pub mod organisms;
pub mod eq_theme;

pub use theme::*;
