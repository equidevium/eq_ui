//! EqDropdown — dropdown menu molecule.
//!
//! A trigger button that opens a positioned menu of selectable items.
//! Supports single selection, disabled items, separators, keyboard
//! navigation, and close-on-Escape / close-on-outside-click.
//!
//! ```rust,ignore
//! let items = vec![
//!     DropdownItem::new("edit", "Edit"),
//!     DropdownItem::new("duplicate", "Duplicate"),
//!     DropdownItem::separator(),
//!     DropdownItem::new("delete", "Delete").disabled(),
//! ];
//!
//! let mut selected = use_signal(|| None::<String>);
//!
//! EqDropdown {
//!     label: "Actions",
//!     items,
//!     selected: selected(),
//!     on_select: move |id: String| selected.set(Some(id)),
//! }
//! ```

use super::eq_dropdown_styles as s;
use crate::theme::merge_classes;
use crate::{PlaygroundEnum, playground};
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// ── Types ─────────────────────────────────────────────────────────

/// Where the menu opens relative to the trigger.
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
pub enum DropdownPosition {
    /// Below the trigger (default).
    #[default]
    Bottom,
    /// Above the trigger.
    Top,
}

/// A single item (or separator) in the dropdown menu.
#[derive(Clone, PartialEq)]
pub struct DropdownItem {
    /// Unique identifier returned on selection. Empty for separators.
    pub id: String,
    /// Display label. Empty for separators.
    pub label: String,
    /// Whether this item is disabled.
    pub disabled: bool,
    /// Whether this entry is a visual separator.
    pub is_separator: bool,
}

impl DropdownItem {
    /// Create a selectable item.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
            is_separator: false,
        }
    }

    /// Create a separator line.
    pub fn separator() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            disabled: false,
            is_separator: true,
        }
    }

    /// Builder: mark this item as disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

// ── Chevron SVG path ──────────────────────────────────────────────

/// Heroicons chevron-down (mini, 20×20).
const CHEVRON_PATH: &str = "m5.23 7.21a.75.75 0 0 1 1.06.02L10 11.168l3.71-3.938a.75.75 0 1 1 1.08 1.04l-4.25 4.5a.75.75 0 0 1-1.08 0l-4.25-4.5a.75.75 0 0 1 .02-1.06Z";

// ── Component ─────────────────────────────────────────────────────

/// Dropdown menu with selectable items.
///
/// Click the trigger to toggle the menu. Selecting an item fires
/// `on_select` with the item's `id` and closes the menu.
///
/// **Accessibility** — the trigger uses `aria-haspopup="listbox"` and
/// `aria-expanded`. The menu uses `role="listbox"` with
/// `role="option"` on each item. Arrow keys navigate items, Enter
/// selects, and Escape closes. Items marked `disabled` receive
/// `aria-disabled`.
#[playground(
    category = Molecule,
    description = "Dropdown menu with selectable items, separators, disabled state, \
                   keyboard navigation (arrows/Enter/Escape), and two positions.",
    examples = [
        ("Basic", "let items = vec![\n    DropdownItem::new(\"a\", \"Option A\"),\n    DropdownItem::new(\"b\", \"Option B\"),\n];\n\nEqDropdown {\n    label: \"Choose\",\n    items,\n    on_select: move |id: String| log::info!(\"Selected {id}\"),\n}"),
        ("With separators", "let items = vec![\n    DropdownItem::new(\"edit\", \"Edit\"),\n    DropdownItem::new(\"copy\", \"Copy\"),\n    DropdownItem::separator(),\n    DropdownItem::new(\"delete\", \"Delete\").disabled(),\n];\n\nEqDropdown { label: \"Actions\", items, on_select: move |id| {} }"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqDropdown(
    /// Label shown on the trigger button.
    #[props(into)]
    label: String,
    /// Menu items (and separators).
    items: Vec<DropdownItem>,
    /// Currently selected item id (highlights that item).
    #[props(into, default)]
    selected: Option<String>,
    /// Fired when an enabled item is clicked. Receives the item's `id`.
    #[props(default)]
    on_select: Option<EventHandler<String>>,
    /// Position of the menu relative to the trigger.
    #[props(default)]
    position: DropdownPosition,
    /// Optional class override on the wrapper element.
    #[props(into, default)]
    class: String,
) -> Element {
    let mut open = use_signal(|| false);
    let mut focused_idx = use_signal(|| None::<usize>);

    let wrapper_cls = merge_classes(s::WRAPPER, &class);

    let pos_cls = match position {
        DropdownPosition::Bottom => s::POS_BOTTOM,
        DropdownPosition::Top => s::POS_TOP,
    };

    let menu_state = if open() { s::MENU_OPEN } else { s::MENU_CLOSED };
    let chevron_rot = if open() { s::CHEVRON_OPEN } else { "" };

    // Collect indices of selectable (non-separator, non-disabled) items.
    let selectable_indices: Vec<usize> = items
        .iter()
        .enumerate()
        .filter(|(_, it)| !it.is_separator && !it.disabled)
        .map(|(i, _)| i)
        .collect();
    let selectable_indices2 = selectable_indices.clone();

    rsx! {
        div {
            class: "{wrapper_cls}",

            // Close on outside click — a backdrop behind everything.
            if open() {
                div {
                    class: "fixed inset-0 z-40",
                    onclick: move |_| {
                        open.set(false);
                        focused_idx.set(None);
                    },
                }
            }

            // Trigger button
            button {
                class: "{s::TRIGGER}",
                r#type: "button",
                "aria-haspopup": "listbox",
                "aria-expanded": "{open()}",
                onclick: move |_| {
                    let next = !open();
                    open.set(next);
                    if !next { focused_idx.set(None); }
                },
                onkeydown: move |evt: KeyboardEvent| {
                    match evt.key() {
                        Key::Escape => {
                            open.set(false);
                            focused_idx.set(None);
                        }
                        Key::ArrowDown => {
                            evt.prevent_default();
                            if !open() {
                                open.set(true);
                            }
                            // Focus first selectable item.
                            if let Some(&first) = selectable_indices.first() {
                                focused_idx.set(Some(first));
                            }
                        }
                        Key::ArrowUp => {
                            evt.prevent_default();
                            if !open() {
                                open.set(true);
                            }
                            if let Some(&last) = selectable_indices.last() {
                                focused_idx.set(Some(last));
                            }
                        }
                        _ => {}
                    }
                },

                "{label}"
                svg {
                    class: "{s::CHEVRON} {chevron_rot}",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 20 20",
                    fill: "currentColor",
                    width: "16",
                    height: "16",
                    "aria-hidden": "true",
                    path {
                        fill_rule: "evenodd",
                        clip_rule: "evenodd",
                        d: CHEVRON_PATH,
                    }
                }
            }

            // Menu
            div {
                class: "{s::MENU} {pos_cls} {menu_state}",
                role: "listbox",
                "aria-label": "{label}",
                onkeydown: move |evt: KeyboardEvent| {
                    match evt.key() {
                        Key::Escape => {
                            open.set(false);
                            focused_idx.set(None);
                        }
                        Key::ArrowDown => {
                            evt.prevent_default();
                            let current = focused_idx();
                            let next = match current {
                                Some(cur) => {
                                    selectable_indices2.iter()
                                        .position(|&i| i == cur)
                                        .map(|pos| selectable_indices2[(pos + 1) % selectable_indices2.len()])
                                        .unwrap_or(selectable_indices2.first().copied().unwrap_or(0))
                                }
                                None => selectable_indices2.first().copied().unwrap_or(0),
                            };
                            focused_idx.set(Some(next));
                        }
                        Key::ArrowUp => {
                            evt.prevent_default();
                            let current = focused_idx();
                            let next = match current {
                                Some(cur) => {
                                    selectable_indices2.iter()
                                        .position(|&i| i == cur)
                                        .map(|pos| {
                                            if pos == 0 {
                                                selectable_indices2[selectable_indices2.len() - 1]
                                            } else {
                                                selectable_indices2[pos - 1]
                                            }
                                        })
                                        .unwrap_or(selectable_indices2.last().copied().unwrap_or(0))
                                }
                                None => selectable_indices2.last().copied().unwrap_or(0),
                            };
                            focused_idx.set(Some(next));
                        }
                        Key::Enter => {
                            if let Some(idx) = focused_idx() {
                                if let Some(item) = items.get(idx) {
                                    if !item.disabled && !item.is_separator {
                                        if let Some(handler) = &on_select {
                                            handler.call(item.id.clone());
                                        }
                                        open.set(false);
                                        focused_idx.set(None);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                },

                for (idx , item) in items.iter().enumerate() {
                    if item.is_separator {
                        div {
                            key: "sep-{idx}",
                            class: "{s::SEPARATOR}",
                            role: "separator",
                        }
                    } else {
                        {
                            let is_active = selected.as_ref() == Some(&item.id);
                            let is_focused = focused_idx() == Some(idx);
                            let active_cls = if is_active { s::ITEM_ACTIVE } else { "" };
                            let disabled_cls = if item.disabled { s::ITEM_DISABLED } else { "" };
                            let focus_cls = if is_focused { "bg-[var(--color-accent-primary)]/10" } else { "" };
                            let item_id = item.id.clone();

                            rsx! {
                                button {
                                    key: "{item_id}",
                                    class: "{s::ITEM} {active_cls} {disabled_cls} {focus_cls}",
                                    r#type: "button",
                                    role: "option",
                                    "aria-selected": "{is_active}",
                                    "aria-disabled": if item.disabled { "true" } else { "false" },
                                    tabindex: "-1",
                                    disabled: item.disabled,
                                    onclick: {
                                        let id = item_id.clone();
                                        move |_| {
                                            if let Some(handler) = &on_select {
                                                handler.call(id.clone());
                                            }
                                            open.set(false);
                                            focused_idx.set(None);
                                        }
                                    },
                                    "{item.label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqDropdown() -> Element {
    let mut selected = use_signal(|| None::<String>);
    let mut position_str = use_signal(|| "Bottom".to_string());

    let position = match position_str().as_str() {
        "Top" => DropdownPosition::Top,
        _ => DropdownPosition::Bottom,
    };

    let items = vec![
        DropdownItem::new("edit", "Edit"),
        DropdownItem::new("duplicate", "Duplicate"),
        DropdownItem::new("rename", "Rename"),
        DropdownItem::separator(),
        DropdownItem::new("archive", "Archive"),
        DropdownItem::new("delete", "Delete").disabled(),
    ];

    let selected_label = selected()
        .clone()
        .unwrap_or_else(|| "(none)".to_string());

    let code = r#"let items = vec![
    DropdownItem::new("edit", "Edit"),
    DropdownItem::new("duplicate", "Duplicate"),
    DropdownItem::new("rename", "Rename"),
    DropdownItem::separator(),
    DropdownItem::new("archive", "Archive"),
    DropdownItem::new("delete", "Delete").disabled(),
];

let mut selected = use_signal(|| None::<String>);

EqDropdown {
    label: "Actions",
    items,
    selected: selected(),
    on_select: move |id: String| selected.set(Some(id)),
}"#
    .to_string();

    rsx! {
        DemoSection { title: "EqDropdown",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "position",
                    value: position_str(),
                    options: vec!["Bottom", "Top"],
                    onchange: move |v: String| position_str.set(v),
                }
            }

            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 space-y-6",
                div { class: "flex items-center gap-4",
                    EqDropdown {
                        label: "Actions",
                        items: items.clone(),
                        selected: selected(),
                        position,
                        on_select: move |id: String| selected.set(Some(id)),
                    }
                    EqText { variant: TextVariant::Muted, "Selected: {selected_label}" }
                }
            }

            StyleInfo { file: "eq_dropdown_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery ───────────────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqDropdown() -> Element {
    let mut sort_sel = use_signal(|| Some("name".to_string()));
    let mut action_sel = use_signal(|| None::<String>);

    let sort_items = vec![
        DropdownItem::new("name", "Name"),
        DropdownItem::new("date", "Date modified"),
        DropdownItem::new("size", "Size"),
        DropdownItem::new("type", "Type"),
    ];

    let action_items = vec![
        DropdownItem::new("new", "New file"),
        DropdownItem::new("upload", "Upload"),
        DropdownItem::separator(),
        DropdownItem::new("settings", "Settings"),
    ];

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Dropdown Gallery" }

                div { class: "flex items-center gap-4 flex-wrap",
                    EqDropdown {
                        label: "Sort by",
                        items: sort_items,
                        selected: sort_sel(),
                        on_select: move |id: String| sort_sel.set(Some(id)),
                    }
                    EqDropdown {
                        label: "File",
                        items: action_items,
                        selected: action_sel(),
                        on_select: move |id: String| action_sel.set(Some(id)),
                    }
                }
            }
        }
    }
}
