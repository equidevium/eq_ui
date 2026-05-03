//! EqRadioGroup - themed radio button group atom.
//!
//! Renders a group of mutually exclusive radio options with pure CSS
//! circle indicators (no icon dependency). Supports vertical/horizontal
//! layout, three sizes, optional descriptions per item, and disabled state.

use super::eq_radio_group_styles as s;
use crate::theme::merge_classes;
use crate::{PlaygroundEnum, playground};
use dioxus::document;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropToggle, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// ── Types ─────────────────────────────────────────────────────────

/// Size of the radio circle indicator.
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
pub enum RadioSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Layout direction for the radio group.
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
pub enum RadioLayout {
    /// Stack items vertically (default).
    #[default]
    Vertical,
    /// Arrange items in a horizontal row.
    Horizontal,
}

/// A single option in the radio group.
#[derive(Clone, PartialEq)]
pub struct RadioItem {
    /// Unique value for this option.
    pub value: String,
    /// Display label.
    pub label: String,
    /// Optional description shown below the label.
    pub description: Option<String>,
    /// Whether this specific item is disabled.
    pub disabled: bool,
}

impl RadioItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            disabled: false,
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

// ── Component ─────────────────────────────────────────────────────

/// Themed radio button group.
///
/// Renders mutually exclusive options as styled circles with labels.
/// The group manages selection state via `selected` + `on_change` props
/// (controlled component pattern).
#[playground(
    category = Atom,
    description = "Themed radio button group with mutually exclusive selection. \
                   Pure CSS circles, three sizes, vertical/horizontal layout.",
    examples = [
        ("Basic", "let mut selected = use_signal(|| \"opt1\".to_string());\n\nEqRadioGroup {\n    items: vec![\n        RadioItem::new(\"opt1\", \"Option One\"),\n        RadioItem::new(\"opt2\", \"Option Two\"),\n    ],\n    selected: selected(),\n    on_change: move |v| selected.set(v),\n}"),
        ("Horizontal", "EqRadioGroup {\n    items: vec![...],\n    layout: RadioLayout::Horizontal,\n    selected: size(),\n    on_change: move |v| size.set(v),\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqRadioGroup(
    /// The available options.
    items: Vec<RadioItem>,
    /// Currently selected value. Empty string = nothing selected.
    #[props(into, default)]
    selected: String,
    /// Fired when the user selects an option. Receives the value string.
    #[props(default)]
    on_change: Option<EventHandler<String>>,
    /// Disables the entire group.
    #[props(default = false)]
    disabled: bool,
    /// Size of the radio indicators.
    #[props(default)]
    size: RadioSize,
    /// Layout direction.
    #[props(default)]
    layout: RadioLayout,
    /// Shared name attribute for the radio group (accessibility).
    #[props(into, default = "radio-group".to_string())]
    name: String,
    /// Optional class override on the root container.
    #[props(into, default)]
    class: String,
) -> Element {
    let group_base = match layout {
        RadioLayout::Vertical => s::GROUP,
        RadioLayout::Horizontal => s::GROUP_HORIZONTAL,
    };
    let group_cls = merge_classes(group_base, &class);

    // Stable unique ID prefix for this radio group instance
    let radio_id_prefix = use_hook(|| {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("eq-radio-{id}")
    });

    // Collect enabled indices for keyboard navigation
    let enabled_indices: Vec<usize> = items
        .iter()
        .enumerate()
        .filter(|(_, item)| !disabled && !item.disabled)
        .map(|(i, _)| i)
        .collect();

    let items_for_keydown = items.clone();
    let enabled_for_keydown = enabled_indices.clone();
    let prefix_for_keydown = radio_id_prefix.clone();

    rsx! {
        div {
            class: "{group_cls}",
            role: "radiogroup",
            onkeydown: move |evt: Event<KeyboardData>| {
                if enabled_for_keydown.is_empty() { return; }
                let key = evt.key();

                // Find current index in the enabled list
                let current_enabled_pos = items_for_keydown
                    .iter()
                    .position(|item| item.value == selected)
                    .and_then(|idx| enabled_for_keydown.iter().position(|&i| i == idx));

                let next_idx = match key {
                    Key::ArrowDown | Key::ArrowRight => {
                        evt.prevent_default();
                        match current_enabled_pos {
                            Some(pos) => {
                                let next = (pos + 1) % enabled_for_keydown.len();
                                Some(enabled_for_keydown[next])
                            }
                            None => Some(enabled_for_keydown[0]),
                        }
                    }
                    Key::ArrowUp | Key::ArrowLeft => {
                        evt.prevent_default();
                        match current_enabled_pos {
                            Some(pos) => {
                                let next = if pos == 0 { enabled_for_keydown.len() - 1 } else { pos - 1 };
                                Some(enabled_for_keydown[next])
                            }
                            None => Some(*enabled_for_keydown.last().unwrap()),
                        }
                    }
                    Key::Home => {
                        evt.prevent_default();
                        Some(enabled_for_keydown[0])
                    }
                    Key::End => {
                        evt.prevent_default();
                        Some(*enabled_for_keydown.last().unwrap())
                    }
                    _ => None,
                };

                if let Some(idx) = next_idx {
                    if let Some(ref handler) = on_change {
                        handler.call(items_for_keydown[idx].value.clone());
                    }
                    // Move browser focus to the newly selected radio item
                    let focus_id = format!("{}-{idx}", prefix_for_keydown);
                    document::eval(&format!(
                        "document.getElementById('{focus_id}')?.focus()"
                    ));
                }
            },
            for (idx , item) in items.iter().enumerate() {
                {
                    let is_selected = item.value == selected;
                    let is_disabled = disabled || item.disabled;
                    let item_cls = if is_disabled { s::ITEM_DISABLED } else { s::ITEM };

                    let (_circle_base, dot_cls) = match size {
                        RadioSize::Sm => (s::SM_CIRCLE, s::SM_DOT),
                        RadioSize::Md => (s::CIRCLE, s::DOT),
                        RadioSize::Lg => (s::LG_CIRCLE, s::LG_DOT),
                    };

                    let circle_border = if is_selected {
                        match size {
                            RadioSize::Sm => merge_classes(s::SM_CIRCLE, "border-[var(--color-accent-primary)]"),
                            RadioSize::Md => s::CIRCLE_ACTIVE.to_string(),
                            RadioSize::Lg => merge_classes(s::LG_CIRCLE, "border-[var(--color-accent-primary)]"),
                        }
                    } else {
                        match size {
                            RadioSize::Sm => merge_classes(s::SM_CIRCLE, "border-[var(--color-label-secondary)]"),
                            RadioSize::Md => s::CIRCLE.to_string(),
                            RadioSize::Lg => merge_classes(s::LG_CIRCLE, "border-[var(--color-label-secondary)]"),
                        }
                    };

                    let value = item.value.clone();
                    let has_description = item.description.is_some();
                    let description = item.description.clone().unwrap_or_default();

                    // Roving tabindex: only the selected (or first enabled) item is tabbable
                    let is_tabbable = if is_disabled {
                        false
                    } else if is_selected {
                        true
                    } else {
                        // If nothing is selected, first enabled item is tabbable
                        selected.is_empty() && enabled_indices.first() == Some(&idx)
                    };
                    let tab_idx = if is_tabbable { "0" } else { "-1" };
                    let item_id = format!("{radio_id_prefix}-{idx}");

                    rsx! {
                        span {
                            id: "{item_id}",
                            class: "{item_cls}",
                            role: "radio",
                            "aria-checked": "{is_selected}",
                            "aria-disabled": "{is_disabled}",
                            tabindex: "{tab_idx}",
                            onclick: move |evt| {
                                evt.stop_propagation();
                                if !is_disabled {
                                    if let Some(ref handler) = on_change {
                                        handler.call(value.clone());
                                    }
                                }
                            },
                            // Visual circle
                            span { class: "{circle_border}",
                                if is_selected {
                                    span { class: "{dot_cls}" }
                                }
                            }
                            // Label + optional description
                            if has_description {
                                div { class: "flex flex-col",
                                    span { class: s::LABEL, "{item.label}" }
                                    span { class: s::DESCRIPTION, "{description}" }
                                }
                            } else {
                                span { class: s::LABEL, "{item.label}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Custom demo (Vec<RadioItem> + selection state) ────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqRadioGroup() -> Element {
    let mut selected = use_signal(|| "opt1".to_string());
    let mut disabled = use_signal(|| false);
    let mut size_str = use_signal(|| "Md".to_string());
    let mut layout_str = use_signal(|| "Vertical".to_string());

    let size = match size_str().as_str() {
        "Sm" => RadioSize::Sm,
        "Lg" => RadioSize::Lg,
        _ => RadioSize::Md,
    };

    let layout = match layout_str().as_str() {
        "Horizontal" => RadioLayout::Horizontal,
        _ => RadioLayout::Vertical,
    };

    let items = vec![
        RadioItem::new("opt1", "Option One")
            .description("First option with a description"),
        RadioItem::new("opt2", "Option Two")
            .description("Second option with details"),
        RadioItem::new("opt3", "Option Three"),
        RadioItem::new("opt4", "Disabled Option")
            .disabled(true),
    ];

    let code = "use eq_ui::atoms::{EqRadioGroup, RadioItem, RadioSize, RadioLayout};\n\
        \n\
        let mut selected = use_signal(|| \"opt1\".to_string());\n\
        \n\
        EqRadioGroup {\n\
        \x20   items: vec![\n\
        \x20       RadioItem::new(\"opt1\", \"Option One\")\n\
        \x20           .description(\"First option\"),\n\
        \x20       RadioItem::new(\"opt2\", \"Option Two\"),\n\
        \x20       RadioItem::new(\"opt3\", \"Disabled\")\n\
        \x20           .disabled(true),\n\
        \x20   ],\n\
        \x20   selected: selected(),\n\
        \x20   on_change: move |v| selected.set(v),\n\
        \x20   size: RadioSize::Md,\n\
        \x20   layout: RadioLayout::Vertical,\n\
        }".to_string();

    rsx! {
        DemoSection { title: "EqRadioGroup",
            // Prop controls
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                div { class: "grid grid-cols-2 md:grid-cols-3 gap-3",
                    PropSelect {
                        label: "size",
                        value: size_str(),
                        options: vec!["Sm", "Md", "Lg"],
                        onchange: move |v: String| size_str.set(v),
                    }
                    PropSelect {
                        label: "layout",
                        value: layout_str(),
                        options: vec!["Vertical", "Horizontal"],
                        onchange: move |v: String| layout_str.set(v),
                    }
                    PropToggle {
                        label: "disabled",
                        value: disabled(),
                        onchange: move |v: bool| disabled.set(v),
                    }
                }
            }

            // Live preview
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                EqRadioGroup {
                    items: items,
                    selected: selected(),
                    disabled: disabled(),
                    size: size,
                    layout: layout,
                    on_change: move |v: String| selected.set(v),
                }
                div { class: "mt-4 text-xs text-[var(--color-label-secondary)]",
                    "Selected: {selected()}"
                }
            }

            // Variant gallery
            div { class: "space-y-4",
                EqText { variant: TextVariant::Emphasis, "Sizes" }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                    for (label, sz) in [("Small", RadioSize::Sm), ("Medium", RadioSize::Md), ("Large", RadioSize::Lg)] {
                        div { class: "space-y-2",
                            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "{label}" }
                            div { class: "rounded-lg border border-[var(--color-card-border)] p-4",
                                EqRadioGroup {
                                    items: vec![
                                        RadioItem::new("a", "Alpha"),
                                        RadioItem::new("b", "Beta"),
                                    ],
                                    selected: "a",
                                    size: sz,
                                }
                            }
                        }
                    }
                }

                EqText { variant: TextVariant::Emphasis, "Horizontal Layout" }
                div { class: "rounded-lg border border-[var(--color-card-border)] p-4",
                    EqRadioGroup {
                        items: vec![
                            RadioItem::new("s", "Small"),
                            RadioItem::new("m", "Medium"),
                            RadioItem::new("l", "Large"),
                            RadioItem::new("xl", "XL"),
                        ],
                        selected: "m",
                        layout: RadioLayout::Horizontal,
                    }
                }

                EqText { variant: TextVariant::Emphasis, "With Descriptions" }
                div { class: "rounded-lg border border-[var(--color-card-border)] p-4",
                    EqRadioGroup {
                        items: vec![
                            RadioItem::new("free", "Free Tier")
                                .description("5 projects, 1 GB storage"),
                            RadioItem::new("pro", "Pro")
                                .description("Unlimited projects, 100 GB storage"),
                            RadioItem::new("enterprise", "Enterprise")
                                .description("Custom limits, SSO, priority support")
                                .disabled(true),
                        ],
                        selected: "pro",
                    }
                }
            }

            StyleInfo { file: "eq_radio_group_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqRadioGroup() -> Element {
    rsx! {
        div { class: "space-y-4",
            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Vertical" }
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4",
                EqRadioGroup {
                    items: vec![
                        RadioItem::new("a", "Option A"),
                        RadioItem::new("b", "Option B"),
                        RadioItem::new("c", "Option C"),
                    ],
                    selected: "b",
                }
            }

            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Horizontal" }
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4",
                EqRadioGroup {
                    items: vec![
                        RadioItem::new("x", "Red"),
                        RadioItem::new("y", "Green"),
                        RadioItem::new("z", "Blue"),
                    ],
                    selected: "x",
                    layout: RadioLayout::Horizontal,
                }
            }

            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "With Descriptions" }
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4",
                EqRadioGroup {
                    items: vec![
                        RadioItem::new("light", "Light")
                            .description("For daytime use"),
                        RadioItem::new("dark", "Dark")
                            .description("For nighttime use"),
                    ],
                    selected: "dark",
                }
            }
        }
    }
}
