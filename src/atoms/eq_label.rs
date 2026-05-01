use dioxus::prelude::*;
use super::eq_label_styles as s;
use crate::theme::merge_classes;
use crate::preview;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropInput, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Atomic form label component.
/// Renders a `<label>` element with consistent styling.
///
/// Use `class` to extend or replace the default styles .
#[preview(
    category = Atom,
    description = "Form label component with consistent styling. Associates with \
                   form controls via the `for_id` attribute.",
    examples = [
        ("Basic", "EqLabel { for_id: \"username\", \"Username\" }"),
        ("Without for attribute", "EqLabel { \"Label without for attribute\" }"),
    ],
)]
#[component]
pub fn EqLabel(
    /// The `id` of the form control this label is associated with.
    #[props(default = "")]
    for_id: &'static str,
    /// Optional class override - extend or replace default styles.
    #[props(into, default)]
    class: String,
    children: Element,
) -> Element {
    let cls = merge_classes(s::LABEL, &class);
    rsx! {
        label {
            class: "{cls}",
            r#for: "{for_id}",
            {children}
        }
    }
}
