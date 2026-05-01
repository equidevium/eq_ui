use dioxus::prelude::*;
use super::eq_input_styles as s;
use crate::theme::merge_classes;
use crate::{PreviewEnum, preview};

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropToggle, PropInput, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Input kind - determines the rendered element and `type` attribute.
#[derive(Clone, PartialEq, Default, PreviewEnum)]
pub enum InputKind {
    #[default]
    Text,
    Email,
    Password,
    Textarea,
}

/// Atomic input component.
/// Renders a styled `<input>` or `<textarea>` depending on `kind`.
///
/// Use `class` to extend or replace the default styles .
#[preview(
    category = Atom,
    description = "Atomic input component supporting text, email, password, and \
                   textarea kinds with consistent styling.",
    examples = [
        ("Text input", "let mut value = use_signal(|| String::new());\nEqInput {\n    kind: InputKind::Text,\n    value: value(),\n    oninput: move |e| value.set(e.value()),\n}"),
        ("Email input", "EqInput {\n    kind: InputKind::Email,\n    placeholder: \"you@example.com\",\n}"),
        ("Disabled", "EqInput {\n    placeholder: \"Cannot edit\",\n    disabled: true,\n}"),
    ],
    no_variant_gallery,
)]
#[component]
pub fn EqInput(
    #[props(default)]
    kind: InputKind,
    #[props(default = "")]
    placeholder: &'static str,
    #[props(default = "")]
    name: &'static str,
    #[props(default = "")]
    id: &'static str,
    #[props(default = false)]
    disabled: bool,
    #[props(default = false)]
    required: bool,
    #[props(default = String::new())]
    value: String,
    /// Optional class override - extend or replace default styles.
    #[props(into, default)]
    class: String,
    oninput: EventHandler<FormEvent>,
) -> Element {
    let disabled_class = if disabled { s::DISABLED } else { "" };

    match kind {
        InputKind::Textarea => {
            let base = format!("{} {} {}", s::CONTROL, s::TEXTAREA, disabled_class);
            let cls = merge_classes(&base, &class);
            rsx! {
                textarea {
                    class: "{cls}",
                    name: "{name}",
                    id: "{id}",
                    placeholder: "{placeholder}",
                    disabled: disabled,
                    required: required,
                    value: "{value}",
                    oninput: move |e| oninput.call(e),
                }
            }
        },
        _ => {
            let input_type = match kind {
                InputKind::Email => "email",
                InputKind::Password => "password",
                _ => "text",
            };
            let base = format!("{} {}", s::CONTROL, disabled_class);
            let cls = merge_classes(&base, &class);
            rsx! {
                input {
                    class: "{cls}",
                    r#type: "{input_type}",
                    name: "{name}",
                    id: "{id}",
                    placeholder: "{placeholder}",
                    disabled: disabled,
                    required: required,
                    value: "{value}",
                    oninput: move |e| oninput.call(e),
                }
            }
        }
    }
}
