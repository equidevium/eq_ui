use dioxus::prelude::*;
use super::eq_label_styles as s;
use crate::theme::merge_classes;

/// Atomic form label component.
/// Renders a `<label>` element with consistent styling.
///
/// Use `class` to extend or replace the default styles .
#[component]
pub fn EqLabel(
    /// The `id` of the form control this label is associated with.
    #[props(default = "")]
    for_id: &'static str,
    /// Optional class override — extend or replace default styles.
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
