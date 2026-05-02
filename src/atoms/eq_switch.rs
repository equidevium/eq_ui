//! EqSwitch - themed toggle switch atom.
//!
//! A boolean on/off toggle rendered as a pill-shaped track with a sliding
//! thumb circle. Pure CSS (transition on the thumb transform), three sizes,
//! optional label and description, disabled state, full accessibility.

use super::eq_switch_styles as s;
use crate::theme::merge_classes;
use crate::{PlaygroundEnum, playground};
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
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
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
#[playground(
    category = Atom,
    description = "Toggle switch with pill track and sliding thumb. \
                   Three sizes, optional label/description, disabled state. \
                   Pure CSS transition.",
    examples = [
        ("Basic", "let mut enabled = use_signal(|| false);\n\nEqSwitch {\n    checked: enabled(),\n    on_change: move |v| enabled.set(v),\n}"),
        ("With label", "EqSwitch {\n    checked: notifications(),\n    label: \"Enable notifications\",\n    on_change: move |v| notifications.set(v),\n}"),
        ("With description", "EqSwitch {\n    checked: dark_mode(),\n    label: \"Dark mode\",\n    description: \"Use dark colors for the interface\",\n    size: SwitchSize::Lg,\n    on_change: move |v| dark_mode.set(v),\n}"),
    ],
)]
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
            tabindex: if disabled { "-1" } else { "0" },
            onclick: move |_| {
                if !disabled {
                    if let Some(ref handler) = on_change {
                        handler.call(!checked);
                    }
                }
            },
            onkeydown: move |evt: Event<KeyboardData>| {
                if disabled { return; }
                let key = evt.key();
                // Space toggles, Enter is intentionally blocked for switches
                if key == Key::Character(" ".into()) {
                    evt.prevent_default();
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

