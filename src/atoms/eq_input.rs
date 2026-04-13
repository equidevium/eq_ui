use dioxus::prelude::*;
use super::eq_input_styles as s;
use crate::theme::merge_classes;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropToggle, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant, EqLabel};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Input kind - determines the rendered element and `type` attribute.
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
///
/// Use `class` to extend or replace the default styles .
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

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-input",
        name: "EqInput",
        category: ComponentCategory::Atom,
        description: "Atomic input component supporting text, email, password, and \
                      textarea kinds with consistent styling.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Text input",
                code: "let mut value = use_signal(|| String::new());\nEqInput {\n    kind: InputKind::Text,\n    value: value(),\n    oninput: move |e| value.set(e.value()),\n}".into(),
            },
            UsageExample {
                label: "Email input",
                code: "EqInput {\n    kind: InputKind::Email,\n    placeholder: \"you@example.com\",\n}".into(),
            },
        ],
        render_demo: || rsx! { DemoEqInput {} },
        render_gallery: || rsx! { GalleryEqInput {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqInput() -> Element {
    let mut kind_str = use_signal(|| "Text".to_string());
    let mut placeholder_str = use_signal(|| "Type something...".to_string());
    let mut disabled = use_signal(|| false);
    let mut required = use_signal(|| false);
    let mut demo_value = use_signal(|| String::new());

    let kind = match kind_str().as_str() {
        "Email" => InputKind::Email,
        "Password" => InputKind::Password,
        "Textarea" => InputKind::Textarea,
        _ => InputKind::Text,
    };
    let placeholder: &'static str = match placeholder_str().as_str() {
        "you@example.com" => "you@example.com",
        "Enter password..." => "Enter password...",
        "Write a message..." => "Write a message...",
        "(none)" => "",
        _ => "Type something...",
    };

    let code = r#"let mut value = use_signal(|| String::new());

EqInput {
    id: "email",
    kind: InputKind::Email,
    placeholder: "you@example.com",
    value: value(),
    oninput: move |e: FormEvent| value.set(e.value()),
}

EqInput {
    kind: InputKind::Textarea,
    placeholder: "Write a message...",
    value: value(),
    oninput: move |e: FormEvent| value.set(e.value()),
}

EqInput {
    placeholder: "Cannot edit",
    disabled: true,
    value: String::new(),
    oninput: move |_| {},
}"#
    .to_string();

    rsx! {
        DemoSection { title: "EqInput",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "kind",
                    value: kind_str(),
                    options: vec!["Text", "Email", "Password", "Textarea"],
                    onchange: move |v: String| kind_str.set(v),
                }
                PropSelect {
                    label: "placeholder",
                    value: placeholder_str(),
                    options: vec![
                        "Type something...",
                        "you@example.com",
                        "Enter password...",
                        "Write a message...",
                        "(none)",
                    ],
                    onchange: move |v: String| placeholder_str.set(v),
                }
                PropToggle {
                    label: "disabled",
                    value: disabled(),
                    onchange: move |v: bool| disabled.set(v),
                }
                PropToggle {
                    label: "required",
                    value: required(),
                    onchange: move |v: bool| required.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 max-w-md",
                EqInput {
                    kind,
                    placeholder,
                    disabled: disabled(),
                    required: required(),
                    value: demo_value(),
                    oninput: move |e: FormEvent| demo_value.set(e.value()),
                }
            }
            StyleInfo { file: "eq_input_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqInput() -> Element {
    rsx! {
        div { class: "space-y-4 max-w-md",
            EqText { variant: TextVariant::Emphasis, "All kinds" }
            for (label , k) in [
                ("Text", InputKind::Text),
                ("Email", InputKind::Email),
                ("Password", InputKind::Password),
                ("Textarea", InputKind::Textarea),
            ]
            {
                div { class: "space-y-1",
                    EqLabel { "{label}" }
                    EqInput {
                        kind: k,
                        placeholder: "Example...",
                        value: String::new(),
                        oninput: move |_| {},
                    }
                }
            }
        }
    }
}
