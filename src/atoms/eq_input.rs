use dioxus::prelude::*;
use super::eq_input_styles as s;

/// Input kind — determines the rendered element and `type` attribute.
#[derive(Clone, PartialEq, Default)]
pub enum InputKind {
    #[default]
    Text,
    Email,
    Password,
    Textarea,
}

/// Atomic input component.
/// Renders a styled `<input>` or `<textarea>` depending on `kind`.
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
    oninput: EventHandler<FormEvent>,
) -> Element {
    let disabled_class = if disabled { s::DISABLED } else { "" };

    match kind {
        InputKind::Textarea => rsx! {
            textarea {
                class: "{s::CONTROL} {s::TEXTAREA} {disabled_class}",
                name: "{name}",
                id: "{id}",
                placeholder: "{placeholder}",
                disabled: disabled,
                required: required,
                value: "{value}",
                oninput: move |e| oninput.call(e),
            }
        },
        _ => {
            let input_type = match kind {
                InputKind::Email => "email",
                InputKind::Password => "password",
                _ => "text",
            };
            rsx! {
                input {
                    class: "{s::CONTROL} {disabled_class}",
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
