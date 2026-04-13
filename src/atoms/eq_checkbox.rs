//! EqCheckbox - themed checkbox atom with checked, unchecked, and
//! indeterminate states. Renders Phosphor square icons internally.

use super::eq_checkbox_styles as s;
use super::eq_icon_paths;
use super::{EqIcon, IconSize};
use crate::theme::merge_classes;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropToggle, PropInput, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Visual state of the checkbox.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum CheckboxState {
    /// Empty square.
    #[default]
    Unchecked,
    /// Square with checkmark.
    Checked,
    /// Square with a horizontal dash (partial selection).
    Indeterminate,
}

/// Themed checkbox with three visual states.
///
/// Can be used standalone or composed inside other components (e.g. EqGrid
/// row selection). The component renders an icon-based checkbox - no native
/// `<input type="checkbox">` - for full theme control.
#[component]
pub fn EqCheckbox(
    /// Current visual state.
    #[props(default)]
    state: CheckboxState,
    /// Fired when the user clicks the checkbox. Receives the *new* state
    /// that would result from a toggle (Unchecked ↔ Checked). Indeterminate
    /// always transitions to Checked on click.
    #[props(default)]
    on_change: Option<EventHandler<CheckboxState>>,
    /// Disables interaction and dims the visual.
    #[props(default = false)]
    disabled: bool,
    /// Icon size override.
    #[props(default = IconSize::Sm)]
    size: IconSize,
    /// Optional label text rendered beside the checkbox.
    #[props(into, default)]
    label: String,
    /// Optional class override on the wrapper element.
    #[props(into, default)]
    class: String,
) -> Element {
    let (icon_path, icon_cls) = match state {
        CheckboxState::Unchecked => (eq_icon_paths::SQUARE, s::ICON),
        CheckboxState::Checked => (eq_icon_paths::CHECK_SQUARE, s::ICON_ACTIVE),
        CheckboxState::Indeterminate => (eq_icon_paths::MINUS_SQUARE, s::ICON_ACTIVE),
    };

    let wrapper_base = if disabled { s::WRAPPER_DISABLED } else { s::WRAPPER };
    let wrapper_cls = merge_classes(wrapper_base, &class);

    rsx! {
        span {
            class: "{wrapper_cls}",
            onclick: move |evt| {
                evt.stop_propagation();
                if !disabled {
                    if let Some(ref handler) = on_change {
                        let next = match state {
                            CheckboxState::Checked => CheckboxState::Unchecked,
                            _ => CheckboxState::Checked,
                        };
                        handler.call(next);
                    }
                }
            },
            EqIcon { path: icon_path, size: size, class: icon_cls }
            if !label.is_empty() {
                span { class: s::LABEL, "{label}" }
            }
        }
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-checkbox",
        name: "EqCheckbox",
        category: ComponentCategory::Atom,
        description: "Themed checkbox with checked, unchecked, and indeterminate states. \
                      Icon-based rendering for full theme control, supports optional labels and disabled state.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Basic checkbox",
                code: "let mut agreed = use_signal(|| CheckboxState::Unchecked);\n\nEqCheckbox {\n    state: agreed(),\n    label: \"I agree\",\n    on_change: move |next| agreed.set(next),\n}".into(),
            },
            UsageExample {
                label: "Indeterminate (select all)",
                code: "EqCheckbox {\n    state: CheckboxState::Indeterminate,\n    label: \"Select all\",\n    on_change: move |_| { /* select all items */ },\n}".into(),
            },
        ],
        render_demo: || rsx! { DemoEqCheckbox {} },
        render_gallery: || rsx! { GalleryEqCheckbox {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqCheckbox() -> Element {
    let mut state_idx = use_signal(|| 0usize); // 0=Unchecked, 1=Checked, 2=Indeterminate
    let mut disabled = use_signal(|| false);
    let mut size_str = use_signal(|| "Sm".to_string());
    let mut label_text = use_signal(|| String::new());

    let state = match state_idx() {
        1 => CheckboxState::Checked,
        2 => CheckboxState::Indeterminate,
        _ => CheckboxState::Unchecked,
    };

    let size = match size_str().as_str() {
        "Md" => IconSize::Md,
        "Lg" => IconSize::Lg,
        _ => IconSize::Sm,
    };

    let code = "use eq_ui::atoms::{EqCheckbox, CheckboxState};\n\
        \n\
        let mut agreed = use_signal(|| CheckboxState::Unchecked);\n\
        \n\
        EqCheckbox {\n\
        \x20   state: agreed(),\n\
        \x20   label: \"I agree to the terms\",\n\
        \x20   on_change: move |next| agreed.set(next),\n\
        }\n\
        \n\
        // Indeterminate (e.g. header select-all with partial selection)\n\
        EqCheckbox {\n\
        \x20   state: CheckboxState::Indeterminate,\n\
        \x20   on_change: move |_| { /* select all */ },\n\
        }".to_string();

    rsx! {
        DemoSection { title: "EqCheckbox",
            // Prop controls
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                div { class: "grid grid-cols-2 md:grid-cols-3 gap-3",
                    PropSelect {
                        label: "state",
                        value: match state_idx() { 1 => "Checked", 2 => "Indeterminate", _ => "Unchecked" }.to_string(),
                        options: vec!["Unchecked", "Checked", "Indeterminate"],
                        onchange: move |v: String| state_idx.set(match v.as_str() { "Checked" => 1, "Indeterminate" => 2, _ => 0 }),
                    }
                    PropToggle {
                        label: "disabled",
                        value: disabled(),
                        onchange: move |v: bool| disabled.set(v),
                    }
                    PropSelect {
                        label: "size",
                        value: size_str(),
                        options: vec!["Sm", "Md", "Lg"],
                        onchange: move |v: String| size_str.set(v),
                    }
                    PropInput {
                        label: "label",
                        value: label_text(),
                        placeholder: "Optional label",
                        onchange: move |v: String| label_text.set(v),
                    }
                }
            }

            // Live preview
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 flex items-center gap-4",
                EqCheckbox {
                    state: state,
                    disabled: disabled(),
                    size: size,
                    label: label_text(),
                    on_change: move |next: CheckboxState| {
                        state_idx.set(match next {
                            CheckboxState::Checked => 1,
                            CheckboxState::Indeterminate => 2,
                            CheckboxState::Unchecked => 0,
                        });
                    },
                }
            }

            // Variant gallery
            div { class: "space-y-4",
                EqText { variant: TextVariant::Emphasis, "All States" }
                div { class: "flex flex-wrap items-center gap-6",
                    for (label , st) in [
                        ("Unchecked", CheckboxState::Unchecked),
                        ("Checked", CheckboxState::Checked),
                        ("Indeterminate", CheckboxState::Indeterminate),
                    ] {
                        div { class: "flex items-center gap-2",
                            EqCheckbox { state: st, label: label }
                        }
                    }
                }

                EqText { class: "font-semibold uppercase tracking-wider", "Disabled" }
                div { class: "flex flex-wrap items-center gap-6",
                    for (label , st) in [
                        ("Unchecked", CheckboxState::Unchecked),
                        ("Checked", CheckboxState::Checked),
                        ("Indeterminate", CheckboxState::Indeterminate),
                    ] {
                        div { class: "flex items-center gap-2",
                            EqCheckbox { state: st, label: label, disabled: true }
                        }
                    }
                }

                EqText { variant: TextVariant::Emphasis, "Sizes" }
                div { class: "flex flex-wrap items-center gap-6",
                    for (label , sz) in [("Sm", IconSize::Sm), ("Md", IconSize::Md), ("Lg", IconSize::Lg)] {
                        div { class: "flex items-center gap-2",
                            EqCheckbox { state: CheckboxState::Checked, size: sz, label: label }
                        }
                    }
                }
            }

            StyleInfo { file: "eq_checkbox_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqCheckbox() -> Element {
    rsx! {
        div { class: "space-y-4",
            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "States" }
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                for (label, state) in [
                    ("Unchecked", CheckboxState::Unchecked),
                    ("Checked", CheckboxState::Checked),
                    ("Indeterminate", CheckboxState::Indeterminate),
                ] {
                    div { class: "flex items-center gap-3",
                        EqCheckbox { state: state }
                        EqText { variant: TextVariant::Body, class: "text-sm", "{label}" }
                    }
                }
            }

            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "With Labels" }
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqCheckbox { state: CheckboxState::Checked, label: "Agree to terms" }
                EqCheckbox { state: CheckboxState::Unchecked, label: "Subscribe to newsletter" }
                EqCheckbox { state: CheckboxState::Indeterminate, label: "Partial selection" }
            }
        }
    }
}
