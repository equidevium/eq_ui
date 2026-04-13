//! EqSwitch - themed toggle switch atom.
//!
//! A boolean on/off toggle rendered as a pill-shaped track with a sliding
//! thumb circle. Pure CSS (transition on the thumb transform), three sizes,
//! optional label and description, disabled state, full accessibility.

use super::eq_switch_styles as s;
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

// ── Types ─────────────────────────────────────────────────────────

/// Size of the switch.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum SwitchSize {
    Sm,
    #[default]
    Md,
    Lg,
}

// ── Component ─────────────────────────────────────────────────────

/// Themed toggle switch.
///
/// Renders a pill-shaped track with a sliding thumb. Controlled component
/// pattern: pass `checked` + `on_change`.
#[component]
pub fn EqSwitch(
    /// Current state.
    #[props(default = false)]
    checked: bool,
    /// Fired when the user clicks the switch. Receives the new boolean state.
    #[props(default)]
    on_change: Option<EventHandler<bool>>,
    /// Disables interaction and dims the visual.
    #[props(default = false)]
    disabled: bool,
    /// Size of the switch.
    #[props(default)]
    size: SwitchSize,
    /// Optional label text rendered beside the switch.
    #[props(into, default)]
    label: String,
    /// Optional description shown below the label.
    #[props(into, default)]
    description: String,
    /// Optional class override on the wrapper element.
    #[props(into, default)]
    class: String,
) -> Element {
    let wrapper_base = if disabled { s::WRAPPER_DISABLED } else { s::WRAPPER };
    let wrapper_cls = merge_classes(wrapper_base, &class);

    let track_cls = if checked {
        format!("{} {}", s::TRACK_ON, match size {
            SwitchSize::Sm => s::SM_TRACK,
            SwitchSize::Md => s::MD_TRACK,
            SwitchSize::Lg => s::LG_TRACK,
        })
    } else {
        format!("{} {}", s::TRACK, match size {
            SwitchSize::Sm => s::SM_TRACK,
            SwitchSize::Md => s::MD_TRACK,
            SwitchSize::Lg => s::LG_TRACK,
        })
    };

    let thumb_base = match size {
        SwitchSize::Sm => format!("{} {}", s::THUMB, s::SM_THUMB),
        SwitchSize::Md => format!("{} {}", s::THUMB, s::MD_THUMB),
        SwitchSize::Lg => format!("{} {}", s::THUMB, s::LG_THUMB),
    };

    // Use inline style for thumb offset - Tailwind JIT may not pick up
    // arbitrary values from _styles.rs constants.
    let thumb_offset = if checked {
        match size {
            SwitchSize::Sm => "margin-left: 14px;",
            SwitchSize::Md => "margin-left: 18px;",
            SwitchSize::Lg => "margin-left: 22px;",
        }
    } else {
        "margin-left: 0px;"
    };

    let has_label = !label.is_empty();
    let has_description = !description.is_empty();

    rsx! {
        span {
            class: "{wrapper_cls}",
            role: "switch",
            "aria-checked": "{checked}",
            "aria-disabled": "{disabled}",
            onclick: move |_| {
                if !disabled {
                    if let Some(ref handler) = on_change {
                        handler.call(!checked);
                    }
                }
            },
            // Visual track + thumb
            span { class: "{track_cls}",
                span {
                    class: "{thumb_base}",
                    style: "{thumb_offset}",
                }
            }
            // Label + optional description
            if has_label && has_description {
                div { class: "flex flex-col",
                    span { class: s::LABEL, "{label}" }
                    span { class: s::DESCRIPTION, "{description}" }
                }
            } else if has_label {
                span { class: s::LABEL, "{label}" }
            }
        }
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-switch",
        name: "EqSwitch",
        category: ComponentCategory::Atom,
        description: "Toggle switch with pill track and sliding thumb. \
                      Three sizes, optional label/description, disabled state. \
                      Pure CSS transition.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Basic",
                code: "let mut enabled = use_signal(|| false);\n\n\
                       EqSwitch {\n\
                       \x20   checked: enabled(),\n\
                       \x20   on_change: move |v| enabled.set(v),\n\
                       }".into(),
            },
            UsageExample {
                label: "With label",
                code: "EqSwitch {\n\
                       \x20   checked: notifications(),\n\
                       \x20   label: \"Enable notifications\",\n\
                       \x20   on_change: move |v| notifications.set(v),\n\
                       }".into(),
            },
            UsageExample {
                label: "With description",
                code: "EqSwitch {\n\
                       \x20   checked: dark_mode(),\n\
                       \x20   label: \"Dark mode\",\n\
                       \x20   description: \"Use dark colors for the interface\",\n\
                       \x20   size: SwitchSize::Lg,\n\
                       \x20   on_change: move |v| dark_mode.set(v),\n\
                       }".into(),
            },
        ],
        render_demo: || rsx! { DemoEqSwitch {} },
        render_gallery: || rsx! { GalleryEqSwitch {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqSwitch() -> Element {
    let mut checked = use_signal(|| false);
    let mut disabled = use_signal(|| false);
    let mut size_str = use_signal(|| "Md".to_string());
    let mut label_text = use_signal(|| "Enable feature".to_string());
    let mut desc_text = use_signal(|| String::new());

    let size = match size_str().as_str() {
        "Sm" => SwitchSize::Sm,
        "Lg" => SwitchSize::Lg,
        _ => SwitchSize::Md,
    };

    let code = "use eq_ui::atoms::{EqSwitch, SwitchSize};\n\
        \n\
        let mut enabled = use_signal(|| false);\n\
        \n\
        EqSwitch {\n\
        \x20   checked: enabled(),\n\
        \x20   label: \"Enable feature\",\n\
        \x20   description: \"Toggle this to turn the feature on or off\",\n\
        \x20   size: SwitchSize::Md,\n\
        \x20   on_change: move |v| enabled.set(v),\n\
        }".to_string();

    rsx! {
        DemoSection { title: "EqSwitch",
            // Prop controls
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                div { class: "grid grid-cols-2 md:grid-cols-3 gap-3",
                    PropToggle {
                        label: "checked",
                        value: checked(),
                        onchange: move |v: bool| checked.set(v),
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
                    PropInput {
                        label: "description",
                        value: desc_text(),
                        placeholder: "Optional description",
                        onchange: move |v: String| desc_text.set(v),
                    }
                }
            }

            // Live preview
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 flex items-center gap-4",
                EqSwitch {
                    checked: checked(),
                    disabled: disabled(),
                    size: size,
                    label: label_text(),
                    description: desc_text(),
                    on_change: move |v: bool| checked.set(v),
                }
            }

            // Variant gallery
            div { class: "space-y-4",
                EqText { variant: TextVariant::Emphasis, "Sizes" }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                    for (label, sz) in [("Small", SwitchSize::Sm), ("Medium", SwitchSize::Md), ("Large", SwitchSize::Lg)] {
                        div { class: "space-y-3",
                            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "{label}" }
                            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                                EqSwitch { checked: true, size: sz, label: "On" }
                                EqSwitch { checked: false, size: sz, label: "Off" }
                            }
                        }
                    }
                }

                EqText { variant: TextVariant::Emphasis, "States" }
                div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                    EqSwitch { checked: false, label: "Default off" }
                    EqSwitch { checked: true, label: "Default on" }
                    EqSwitch { checked: false, disabled: true, label: "Disabled off" }
                    EqSwitch { checked: true, disabled: true, label: "Disabled on" }
                }

                EqText { variant: TextVariant::Emphasis, "With Descriptions" }
                div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                    EqSwitch {
                        checked: true,
                        label: "Dark mode",
                        description: "Use dark colors for the interface",
                    }
                    EqSwitch {
                        checked: false,
                        label: "Notifications",
                        description: "Receive email notifications for updates",
                    }
                    EqSwitch {
                        checked: true,
                        label: "Auto-save",
                        description: "Automatically save changes every 30 seconds",
                        disabled: true,
                    }
                }
            }

            StyleInfo { file: "eq_switch_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqSwitch() -> Element {
    rsx! {
        div { class: "space-y-4",
            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "On / Off" }
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqSwitch { checked: true, label: "Enabled" }
                EqSwitch { checked: false, label: "Disabled" }
            }

            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Sizes" }
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 flex items-center gap-6",
                EqSwitch { checked: true, size: SwitchSize::Sm }
                EqSwitch { checked: true, size: SwitchSize::Md }
                EqSwitch { checked: true, size: SwitchSize::Lg }
            }

            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "With Description" }
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4",
                EqSwitch {
                    checked: true,
                    label: "Auto-save",
                    description: "Save changes automatically",
                    size: SwitchSize::Lg,
                }
            }
        }
    }
}
