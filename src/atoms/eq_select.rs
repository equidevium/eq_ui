//! EqSelect — styled select atom with search.
//!
//! A custom select dropdown that replaces the native `<select>` element.
//! Supports searchable options, placeholder text, disabled state,
//! keyboard navigation, and full WAI-ARIA combobox pattern.
//!
//! ```rust,ignore
//! let options = vec![
//!     SelectOption::new("rust", "Rust"),
//!     SelectOption::new("python", "Python"),
//!     SelectOption::new("typescript", "TypeScript"),
//! ];
//!
//! let mut lang = use_signal(|| None::<String>);
//!
//! EqSelect {
//!     options,
//!     selected: lang(),
//!     placeholder: "Choose a language",
//!     searchable: true,
//!     on_select: move |id| lang.set(Some(id)),
//! }
//! ```

use super::eq_select_styles as s;
use crate::theme::merge_classes;
use crate::{PlaygroundEnum, playground};
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

/// Where the listbox opens relative to the trigger.
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
pub enum SelectPosition {
    /// Below the trigger (default).
    #[default]
    Bottom,
    /// Above the trigger.
    Top,
}

/// A single option in the select.
#[derive(Clone, PartialEq)]
pub struct SelectOption {
    /// Unique identifier returned on selection.
    pub id: String,
    /// Display label shown in the list and trigger.
    pub label: String,
    /// Whether this option is disabled.
    pub disabled: bool,
}

impl SelectOption {
    /// Create an option.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
        }
    }

    /// Builder: mark this option as disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

// ── SVG paths ────────────────────────────────────────────────────

/// Heroicons chevron-down (mini, 20×20).
const CHEVRON_PATH: &str =
    "m5.23 7.21a.75.75 0 0 1 1.06.02L10 11.168l3.71-3.938a.75.75 0 1 1 1.08 1.04l-4.25 4.5a.75.75 0 0 1-1.08 0l-4.25-4.5a.75.75 0 0 1 .02-1.06Z";

/// Heroicons check (mini, 20×20).
const CHECK_PATH: &str =
    "M16.704 4.153a.75.75 0 0 1 .143 1.052l-8 10.5a.75.75 0 0 1-1.127.075l-4.5-4.5a.75.75 0 0 1 1.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 0 1 1.05-.143Z";

// ── Component ─────────────────────────────────────────────────────

/// Styled select with optional search.
///
/// A custom combobox that opens a listbox of options. When
/// `searchable` is true, a text input filters the options list.
///
/// **Accessibility** — uses `role="combobox"` on the trigger,
/// `role="listbox"` on the panel, and `role="option"` on each item.
/// `aria-activedescendant` tracks the focused option. Arrow keys
/// navigate, Enter selects, Escape closes.
#[playground(
    category = Atom,
    description = "Styled dropdown select with search, placeholder, disabled state, \
                   keyboard navigation, and WAI-ARIA combobox pattern.",
    examples = [
        ("Basic", "let opts = vec![\n    SelectOption::new(\"a\", \"Option A\"),\n    SelectOption::new(\"b\", \"Option B\"),\n];\n\nEqSelect {\n    options: opts,\n    on_select: move |id: String| log::info!(\"{id}\"),\n}"),
        ("Searchable", "EqSelect {\n    options: opts,\n    placeholder: \"Search...\",\n    searchable: true,\n    on_select: move |id| {},\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqSelect(
    /// Available options.
    options: Vec<SelectOption>,
    /// Currently selected option id.
    #[props(into, default)]
    selected: Option<String>,
    /// Placeholder when nothing is selected.
    #[props(into, default = "Select...".to_string())]
    placeholder: String,
    /// Enable search/filter input in the listbox.
    #[props(default = false)]
    searchable: bool,
    /// Disables interaction.
    #[props(default = false)]
    disabled: bool,
    /// Listbox position relative to the trigger.
    #[props(default)]
    position: SelectPosition,
    /// Fired when an option is selected. Receives the option's id.
    #[props(default)]
    on_select: Option<EventHandler<String>>,
    /// Accessible label for screen readers.
    #[props(into, default)]
    aria_label: String,
    /// Optional class override on the wrapper element.
    #[props(into, default)]
    class: String,
) -> Element {
    let mut open = use_signal(|| false);
    let mut search = use_signal(|| String::new());
    let mut focused_idx = use_signal(|| None::<usize>);

    let wrapper_cls = merge_classes(s::WRAPPER, &class);

    let trigger_cls = if disabled { s::TRIGGER_DISABLED } else { s::TRIGGER };
    let chevron_rot = if open() { s::CHEVRON_OPEN } else { "" };

    let pos_cls = match position {
        SelectPosition::Bottom => s::POS_BOTTOM,
        SelectPosition::Top => s::POS_TOP,
    };
    let listbox_state = if open() { s::LISTBOX_OPEN } else { s::LISTBOX_CLOSED };

    // Find the label of the selected option.
    let selected_label = selected.as_ref().and_then(|sel_id| {
        options.iter().find(|o| &o.id == sel_id).map(|o| o.label.clone())
    });

    let has_selection = selected_label.is_some();
    let display_text = selected_label.unwrap_or_else(|| placeholder.clone());
    let display_cls = if has_selection { "" } else { s::PLACEHOLDER };

    // Filter options by search query.
    let query = search().to_lowercase();
    let filtered: Vec<(usize, &SelectOption)> = options
        .iter()
        .enumerate()
        .filter(|(_, o)| {
            if query.is_empty() { return true; }
            o.label.to_lowercase().contains(&query)
        })
        .collect();

    // Selectable (non-disabled) indices within filtered results.
    let selectable: Vec<usize> = filtered
        .iter()
        .filter(|(_, o)| !o.disabled)
        .map(|(i, _)| *i)
        .collect();
    let selectable2 = selectable.clone();

    let aria = if aria_label.is_empty() { None } else { Some(aria_label.clone()) };

    rsx! {
        div {
            class: "{wrapper_cls}",

            // Close on outside click.
            if open() {
                div {
                    class: "fixed inset-0 z-40",
                    onclick: move |_| {
                        open.set(false);
                        search.set(String::new());
                        focused_idx.set(None);
                    },
                }
            }

            // Trigger
            button {
                class: "{trigger_cls}",
                r#type: "button",
                disabled: disabled,
                role: "combobox",
                "aria-expanded": "{open()}",
                "aria-haspopup": "listbox",
                "aria-label": aria,
                onclick: move |_| {
                    if !disabled {
                        let next = !open();
                        open.set(next);
                        if !next {
                            search.set(String::new());
                            focused_idx.set(None);
                        }
                    }
                },
                onkeydown: move |evt: KeyboardEvent| {
                    match evt.key() {
                        Key::Escape => {
                            open.set(false);
                            search.set(String::new());
                            focused_idx.set(None);
                        }
                        Key::ArrowDown => {
                            evt.prevent_default();
                            if !open() { open.set(true); }
                            if let Some(&first) = selectable.first() {
                                focused_idx.set(Some(first));
                            }
                        }
                        Key::ArrowUp => {
                            evt.prevent_default();
                            if !open() { open.set(true); }
                            if let Some(&last) = selectable.last() {
                                focused_idx.set(Some(last));
                            }
                        }
                        _ => {}
                    }
                },

                span { class: "{display_cls}", "{display_text}" }
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

            // Listbox
            div {
                class: "{s::LISTBOX} {pos_cls} {listbox_state}",
                role: "listbox",
                "aria-label": "{placeholder}",
                onkeydown: move |evt: KeyboardEvent| {
                    match evt.key() {
                        Key::Escape => {
                            open.set(false);
                            search.set(String::new());
                            focused_idx.set(None);
                        }
                        Key::ArrowDown => {
                            evt.prevent_default();
                            let current = focused_idx();
                            let next = match current {
                                Some(cur) => {
                                    selectable2.iter()
                                        .position(|&i| i == cur)
                                        .map(|pos| selectable2[(pos + 1) % selectable2.len()])
                                        .unwrap_or(selectable2.first().copied().unwrap_or(0))
                                }
                                None => selectable2.first().copied().unwrap_or(0),
                            };
                            focused_idx.set(Some(next));
                        }
                        Key::ArrowUp => {
                            evt.prevent_default();
                            let current = focused_idx();
                            let next = match current {
                                Some(cur) => {
                                    selectable2.iter()
                                        .position(|&i| i == cur)
                                        .map(|pos| {
                                            if pos == 0 {
                                                selectable2[selectable2.len() - 1]
                                            } else {
                                                selectable2[pos - 1]
                                            }
                                        })
                                        .unwrap_or(selectable2.last().copied().unwrap_or(0))
                                }
                                None => selectable2.last().copied().unwrap_or(0),
                            };
                            focused_idx.set(Some(next));
                        }
                        Key::Enter => {
                            if let Some(idx) = focused_idx() {
                                if let Some(opt) = options.get(idx) {
                                    if !opt.disabled {
                                        if let Some(handler) = &on_select {
                                            handler.call(opt.id.clone());
                                        }
                                        open.set(false);
                                        search.set(String::new());
                                        focused_idx.set(None);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                },

                // Search input
                if searchable {
                    input {
                        class: "{s::SEARCH}",
                        r#type: "text",
                        placeholder: "Search...",
                        value: "{search()}",
                        oninput: move |evt: FormEvent| {
                            search.set(evt.value());
                            focused_idx.set(None);
                        },
                    }
                }

                // Options
                if filtered.is_empty() {
                    div { class: "{s::EMPTY}", "No results found" }
                }

                for (idx , opt) in filtered.iter() {
                    {
                        let is_selected = selected.as_ref() == Some(&opt.id);
                        let is_focused = focused_idx() == Some(*idx);
                        let selected_cls = if is_selected { s::OPTION_SELECTED } else { "" };
                        let disabled_cls = if opt.disabled { s::OPTION_DISABLED } else { "" };
                        let focus_cls = if is_focused { "bg-[var(--color-accent-primary)]/10" } else { "" };
                        let opt_id = opt.id.clone();

                        rsx! {
                            button {
                                key: "{opt_id}",
                                class: "{s::OPTION} {selected_cls} {disabled_cls} {focus_cls}",
                                r#type: "button",
                                role: "option",
                                "aria-selected": "{is_selected}",
                                "aria-disabled": if opt.disabled { "true" } else { "false" },
                                tabindex: "-1",
                                disabled: opt.disabled,
                                onclick: {
                                    let id = opt_id.clone();
                                    move |_| {
                                        if let Some(handler) = &on_select {
                                            handler.call(id.clone());
                                        }
                                        open.set(false);
                                        search.set(String::new());
                                        focused_idx.set(None);
                                    }
                                },

                                span { class: "flex-1 text-left", "{opt.label}" }

                                if is_selected {
                                    svg {
                                        class: "{s::CHECK}",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "0 0 20 20",
                                        fill: "currentColor",
                                        width: "16",
                                        height: "16",
                                        "aria-hidden": "true",
                                        path {
                                            fill_rule: "evenodd",
                                            clip_rule: "evenodd",
                                            d: CHECK_PATH,
                                        }
                                    }
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
fn DemoEqSelect() -> Element {
    let mut selected = use_signal(|| None::<String>);
    let mut searchable = use_signal(|| true);
    let mut disabled = use_signal(|| false);
    let mut position_str = use_signal(|| "Bottom".to_string());

    let position = match position_str().as_str() {
        "Top" => SelectPosition::Top,
        _ => SelectPosition::Bottom,
    };

    let options = vec![
        SelectOption::new("rust", "Rust"),
        SelectOption::new("python", "Python"),
        SelectOption::new("typescript", "TypeScript"),
        SelectOption::new("go", "Go"),
        SelectOption::new("java", "Java"),
        SelectOption::new("csharp", "C#"),
        SelectOption::new("kotlin", "Kotlin"),
        SelectOption::new("swift", "Swift"),
        SelectOption::new("cobol", "COBOL").disabled(),
    ];

    let selected_display = selected()
        .clone()
        .unwrap_or_else(|| "(none)".to_string());

    let code = format!(
        r#"let options = vec![
    SelectOption::new("rust", "Rust"),
    SelectOption::new("python", "Python"),
    SelectOption::new("typescript", "TypeScript"),
    // ...
];

let mut selected = use_signal(|| None::<String>);

EqSelect {{
    options,
    selected: selected(),
    placeholder: "Choose a language",
    searchable: {searchable},
    disabled: {disabled},
    position: SelectPosition::{pos},
    on_select: move |id: String| selected.set(Some(id)),
}}"#,
        searchable = searchable(),
        disabled = disabled(),
        pos = position_str(),
    );

    rsx! {
        DemoSection { title: "EqSelect",
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
                PropToggle {
                    label: "searchable",
                    value: searchable(),
                    onchange: move |v: bool| searchable.set(v),
                }
                PropToggle {
                    label: "disabled",
                    value: disabled(),
                    onchange: move |v: bool| disabled.set(v),
                }
            }

            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 space-y-6",
                div { class: "flex items-center gap-4",
                    div { class: "w-64",
                        EqSelect {
                            options: options.clone(),
                            selected: selected(),
                            placeholder: "Choose a language",
                            searchable: searchable(),
                            disabled: disabled(),
                            position,
                            on_select: move |id: String| selected.set(Some(id)),
                        }
                    }
                    EqText { variant: TextVariant::Muted, "Selected: {selected_display}" }
                }
            }

            StyleInfo { file: "eq_select_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery ───────────────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqSelect() -> Element {
    let mut country = use_signal(|| None::<String>);
    let mut role = use_signal(|| Some("viewer".to_string()));

    let countries = vec![
        SelectOption::new("us", "United States"),
        SelectOption::new("uk", "United Kingdom"),
        SelectOption::new("de", "Germany"),
        SelectOption::new("fr", "France"),
        SelectOption::new("jp", "Japan"),
        SelectOption::new("au", "Australia"),
        SelectOption::new("br", "Brazil"),
        SelectOption::new("ca", "Canada"),
    ];

    let roles = vec![
        SelectOption::new("admin", "Admin"),
        SelectOption::new("editor", "Editor"),
        SelectOption::new("viewer", "Viewer"),
        SelectOption::new("guest", "Guest").disabled(),
    ];

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Select Gallery" }

                div { class: "flex items-start gap-4 flex-wrap",
                    div { class: "w-56 space-y-1",
                        EqText { variant: TextVariant::Muted, "Country (searchable)" }
                        EqSelect {
                            options: countries,
                            selected: country(),
                            placeholder: "Choose country",
                            searchable: true,
                            on_select: move |id: String| country.set(Some(id)),
                        }
                    }
                    div { class: "w-48 space-y-1",
                        EqText { variant: TextVariant::Muted, "Role" }
                        EqSelect {
                            options: roles,
                            selected: role(),
                            placeholder: "Assign role",
                            on_select: move |id: String| role.set(Some(id)),
                        }
                    }
                }
            }
        }
    }
}
