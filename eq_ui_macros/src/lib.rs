//! # eq_ui_macros
//!
//! Procedural macros for the eq_ui component library.
//!
//! ## `#[preview]` — Auto-generate playground boilerplate
//!
//! Place on a `#[component]` function to generate the `descriptor()`,
//! `Demo*`, and `Gallery*` functions behind `#[cfg(feature = "playground")]`.
//!
//! ```rust,ignore
//! #[preview(
//!     category = Atom,
//!     description = "Toggle switch with pill track.",
//!     examples = [
//!         ("Basic", "EqSwitch { checked: true }"),
//!         ("Disabled", "EqSwitch { disabled: true }"),
//!     ],
//! )]
//! #[component]
//! pub fn EqSwitch(
//!     #[props(default = false)] checked: bool,
//!     #[props(default)] size: SwitchSize,
//!     #[props(into, default)] label: String,
//!     #[props(into, default)] class: String,
//! ) -> Element { ... }
//! ```
//!
//! ## `#[derive(PreviewEnum)]` — Expose enum variants for demo controls
//!
//! ```rust,ignore
//! #[derive(Clone, PartialEq, Default, PreviewEnum)]
//! pub enum SwitchSize {
//!     Sm,
//!     #[default]
//!     Md,
//!     Lg,
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod preview_attr;
mod preview_enum;
mod codegen;
mod parse_props;

/// Derive macro that implements `PreviewEnumInfo` for an enum,
/// exposing variant names and a `from_name` constructor for use
/// in auto-generated playground demos.
///
/// # Requirements
/// - Only works on unit enums (no fields)
/// - Expects exactly one variant annotated `#[default]`
#[proc_macro_derive(PreviewEnum)]
pub fn derive_preview_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    preview_enum::expand(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Attribute macro that auto-generates playground boilerplate for a
/// Dioxus `#[component]` function.
///
/// The component function is emitted unchanged. Additionally, behind
/// `#[cfg(feature = "playground")]`, the macro generates:
/// - `pub fn descriptor() -> ComponentDescriptor { ... }`
/// - A `Demo{Name}` component with prop controls
/// - A `Gallery{Name}` component with representative instances
///
/// # Attributes
///
/// ```text
/// #[preview(
///     category = Atom | Molecule | Organism,
///     description = "...",
///     examples = [("Label", "code"), ...],
///     no_styles,        // optional: skip style_tokens
///     custom_demo,      // optional: skip Demo generation (write manually)
///     custom_gallery,   // optional: skip Gallery generation
/// )]
/// ```
#[proc_macro_attribute]
pub fn preview(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item2: proc_macro2::TokenStream = item.clone().into();
    preview_attr::expand(attr.into(), item.into())
        .unwrap_or_else(|e| {
            let err = e.to_compile_error();
            // Still emit the original item so the component compiles
            quote! { #item2 #err }
        })
        .into()
}
