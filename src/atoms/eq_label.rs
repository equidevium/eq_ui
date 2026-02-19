use dioxus::prelude::*;
use super::eq_label_styles as s;

/// Atomic form label component.
/// Renders a `<label>` element with consistent styling.
#[component]
pub fn EqLabel(
    /// The `id` of the form control this label is associated with.
    #[props(default = "")]
    for_id: &'static str,
    children: Element,
) -> Element {
    rsx! {
        label {
            class: s::LABEL,
            r#for: "{for_id}",
            {children}
        }
    }
}
