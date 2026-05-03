use super::eq_accordion_styles as s;
use crate::theme::merge_classes;
use crate::{PlaygroundEnum, playground};
use dioxus::document;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropInput, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Controls whether multiple panels can be open simultaneously.
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
pub enum AccordionMode {
    /// Only one panel open at a time - opening a panel closes the others.
    #[default]
    Single,
    /// Multiple panels can be open simultaneously.
    Multi,
}

/// Describes a single panel inside the accordion.
#[derive(Clone, PartialEq)]
pub struct AccordionItem {
    /// Unique identifier for this panel.
    pub id: String,
    /// Header content (any element - text, icons, badges, etc.).
    pub header: Element,
    /// Body content revealed when the panel is expanded.
    pub body: Element,
}

impl AccordionItem {
    /// Shorthand constructor.
    pub fn new(
        id: impl Into<String>,
        header: Element,
        body: Element,
    ) -> Self {
        Self {
            id: id.into(),
            header,
            body,
        }
    }
}

/// Collapsible accordion molecule.
///
/// Renders a vertical stack of panels, each with a clickable header
/// that reveals or hides its body content with a smooth height transition.
///
/// **Accessibility** – implements the WAI-ARIA [Accordion][acc] pattern:
/// `aria-expanded` on header buttons, `aria-controls` / `aria-labelledby`
/// linking headers to panels, `role="region"` on body panels, and keyboard
/// navigation (Up / Down / Home / End to move between headers).
///
/// [acc]: https://www.w3.org/WAI/ARIA/apg/patterns/accordion/
///
/// Use `class` to extend or replace the default styles.
#[playground(
    category = Molecule,
    description = "Collapsible accordion with single or multi-expand modes. Smooth height animation \
                   powered by CSS grid-rows transition.",
    examples = [
        ("Single expand", "let items = vec![\n    AccordionItem::new(\n        \"panel-1\",\n        rsx! { \"First panel\" },\n        rsx! { \"Content for the first panel.\" },\n    ),\n];\n\nEqAccordion { items }"),
        ("Multi expand", "EqAccordion {\n    items,\n    mode: AccordionMode::Multi,\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqAccordion(
    /// The panels to render.
    items: Vec<AccordionItem>,
    /// Expand behaviour.
    #[props(default)]
    mode: AccordionMode,
    /// Accessible label for screen readers (e.g. "FAQ", "Settings").
    #[props(into, default)]
    aria_label: String,
    /// Optional class override on the root container.
    #[props(into, default)]
    class: String,
) -> Element {
    // Stable unique prefix for DOM IDs.
    let acc_prefix = use_hook(|| {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("eq-acc-{id}")
    });

    // Track which panels are currently open by id.
    let mut open_ids = use_signal(Vec::<String>::new);

    let cls = merge_classes(s::ACCORDION, &class);
    let has_label = !aria_label.is_empty();

    // Collect item IDs for keyboard navigation closure.
    let item_ids: Vec<String> = items.iter().map(|it| it.id.clone()).collect();
    let item_ids_kb = item_ids.clone();
    let prefix_kb = acc_prefix.clone();

    rsx! {
        div {
            class: "{cls}",
            "aria-label": if has_label { "{aria_label}" } else { "" },

            // ── Keyboard navigation between headers ──────────
            onkeydown: move |evt: Event<KeyboardData>| {
                let key = evt.key();
                if key != Key::ArrowDown && key != Key::ArrowUp
                    && key != Key::Home && key != Key::End
                {
                    return;
                }

                let len = item_ids_kb.len();
                if len == 0 { return; }

                // Find the currently focused header by checking active element id.
                // Each header button has id "{prefix}-hdr-{item_id}".
                // We can infer the index from the item_ids list.
                // Fall back to first/last depending on key direction.
                let _cur_idx = {
                    // Try to figure out which header is focused from focused_id
                    // We don't store focused state — all buttons are natively
                    // focusable, so we rely on document.activeElement.
                    // For simplicity, just cycle from -1 (none).
                    None::<usize> // will be resolved by JS; we use a simple approach below
                };

                let target_idx = if key == Key::ArrowDown {
                    evt.prevent_default();
                    // Move to next header (wrap). We ask JS for the current index.
                    None // handled below via eval
                } else if key == Key::ArrowUp {
                    evt.prevent_default();
                    None
                } else if key == Key::Home {
                    evt.prevent_default();
                    Some(0usize)
                } else if key == Key::End {
                    evt.prevent_default();
                    Some(len - 1)
                } else {
                    None
                };

                if let Some(idx) = target_idx {
                    let hdr_id = format!("{}-hdr-{}", prefix_kb, item_ids_kb[idx]);
                    document::eval(&format!(
                        "document.getElementById('{hdr_id}')?.focus()"
                    ));
                } else {
                    // Arrow up/down: build a JS snippet that finds the current
                    // focused header index and moves to the next/previous.
                    let ids_json: Vec<String> = item_ids_kb
                        .iter()
                        .map(|id| format!("'{}-hdr-{}'", prefix_kb, id))
                        .collect();
                    let ids_arr = format!("[{}]", ids_json.join(","));
                    let delta = if key == Key::ArrowDown { 1i32 } else { -1i32 };
                    document::eval(&format!(
                        "(() => {{ \
                            const ids = {ids_arr}; \
                            const cur = document.activeElement?.id || ''; \
                            const idx = ids.indexOf(cur); \
                            const next = (idx + {delta} + ids.length) % ids.length; \
                            document.getElementById(ids[next])?.focus(); \
                        }})()"
                    ));
                }
            },

            for item in items {
                {
                    let id = item.id.clone();
                    let is_open = open_ids().contains(&id);

                    let hdr_id = format!("{}-hdr-{}", acc_prefix, id);
                    let panel_id = format!("{}-panel-{}", acc_prefix, id);

                    let toggle_id = id.clone();
                    let toggle_mode = mode;
                    let onclick = move |_| {
                        let mut current = open_ids();
                        if current.contains(&toggle_id) {
                            current.retain(|x| x != &toggle_id);
                        } else {
                            match toggle_mode {
                                AccordionMode::Single => {
                                    current.clear();
                                    current.push(toggle_id.clone());
                                }
                                AccordionMode::Multi => {
                                    current.push(toggle_id.clone());
                                }
                            }
                        }
                        open_ids.set(current);
                    };

                    let body_grid = if is_open {
                        format!("{} {}", s::BODY, s::BODY_OPEN)
                    } else {
                        format!("{} {}", s::BODY, s::BODY_CLOSED)
                    };
                    let chevron_cls = if is_open {
                        format!("{} {}", s::CHEVRON, s::CHEVRON_OPEN)
                    } else {
                        s::CHEVRON.to_string()
                    };

                    rsx! {
                        div {
                            key: "{id}",
                            class: s::PANEL,

                            // Header
                            button {
                                id: "{hdr_id}",
                                class: s::HEADER,
                                "aria-expanded": if is_open { "true" } else { "false" },
                                "aria-controls": "{panel_id}",
                                onclick: onclick,

                                div { class: s::HEADER_TEXT, {item.header} }

                                svg {
                                    class: "{chevron_cls}",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke_width: "2",
                                    stroke: "currentColor",
                                    "aria-hidden": "true",
                                    path { d: "m19.5 8.25-7.5 7.5-7.5-7.5" }
                                }
                            }

                            // Body - animated via CSS grid rows
                            div {
                                id: "{panel_id}",
                                class: "{body_grid}",
                                role: "region",
                                "aria-labelledby": "{hdr_id}",
                                div { class: s::BODY_INNER,
                                    div { class: s::CONTENT, {item.body} }
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
fn DemoEqAccordion() -> Element {
    let mut mode_str = use_signal(|| "Single".to_string());
    let mut panel_count_str = use_signal(|| "3".to_string());

    let mode = match mode_str().as_str() {
        "Multi" => AccordionMode::Multi,
        _ => AccordionMode::Single,
    };
    let panel_count: usize = panel_count_str().parse().unwrap_or(3).min(6).max(1);

    let sample_panels: Vec<(&str, &str, &str)> = vec![
        (
            "what-is",
            "What is eq_ui?",
            "A portable Dioxus 0.7 component library following atomic design principles. It ships atoms, molecules, and organisms with 21 built-in themes.",
        ),
        (
            "getting-started",
            "Getting started",
            "Add the crate to your Cargo.toml, include the Tailwind @source directive, and wire up the theme provider. Components are ready to use immediately.",
        ),
        (
            "theming",
            "How does theming work?",
            "Themes are CSS variable sets applied at the root. Switch themes at runtime with a single function call - all components update instantly.",
        ),
        (
            "customisation",
            "Can I customise styles?",
            "Every component exposes a class prop. Pass Tailwind utilities to extend defaults, or prefix with ! to replace them entirely.",
        ),
        (
            "accordion",
            "Is this an accordion?",
            "Yes. You are looking at a live EqAccordion right now. It supports single-expand and multi-expand modes.",
        ),
        (
            "animation",
            "How does the animation work?",
            "The body uses a CSS grid-rows transition between 0fr (collapsed) and 1fr (expanded), giving a smooth height animation without JavaScript measurement.",
        ),
    ];

    let items: Vec<AccordionItem> = sample_panels
        .into_iter()
        .take(panel_count)
        .map(|(id, title, body)| AccordionItem::new(id, rsx! { "{title}" }, rsx! { "{body}" }))
        .collect();

    let items_multi: Vec<AccordionItem> = vec![
        AccordionItem::new(
            "faq-1",
            rsx! { "Single mode" },
            rsx! { "Only one panel can be open at a time." },
        ),
        AccordionItem::new(
            "faq-2",
            rsx! { "Multi mode" },
            rsx! { "Multiple panels can be open simultaneously." },
        ),
    ];

    let code = "let items = vec![\n    AccordionItem::new(\n        \"panel-1\",\n        rsx! { \"First panel\" },\n        rsx! { \"Content for the first panel.\" },\n    ),\n    AccordionItem::new(\n        \"panel-2\",\n        rsx! { \"Second panel\" },\n        rsx! { \"Content for the second panel.\" },\n    ),\n];\n\n// Single expand (default)\nEqAccordion { items: items.clone() }\n\n// Multi expand\nEqAccordion {\n    items: items,\n    mode: AccordionMode::Multi,\n}".to_string();

    rsx! {
        DemoSection { title: "EqAccordion",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "mode",
                    value: mode_str(),
                    options: vec!["Single", "Multi"],
                    onchange: move |v: String| mode_str.set(v),
                }
                PropInput {
                    label: "panels",
                    value: panel_count_str(),
                    placeholder: "3",
                    onchange: move |v: String| panel_count_str.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] overflow-hidden",
                EqAccordion { items, mode }
            }
            div { class: "space-y-4 mt-6",
                EqText { variant: TextVariant::Emphasis, "Mode comparison" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    div { class: "space-y-2",
                        EqText { variant: TextVariant::Caption, "Single (default)" }
                        div { class: "rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                            EqAccordion {
                                items: items_multi.clone(),
                                mode: AccordionMode::Single,
                            }
                        }
                    }
                    div { class: "space-y-2",
                        EqText { variant: TextVariant::Caption, "Multi" }
                        div { class: "rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                            EqAccordion {
                                items: items_multi,
                                mode: AccordionMode::Multi,
                            }
                        }
                    }
                }
            }
            StyleInfo { file: "eq_accordion_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqAccordion() -> Element {
    let single_items = vec![
        AccordionItem::new(
            "gallery-1",
            rsx! { "Getting started" },
            rsx! { "Create your first component in minutes." },
        ),
        AccordionItem::new(
            "gallery-2",
            rsx! { "Theming" },
            rsx! { "Choose from 21 built-in themes or create your own." },
        ),
        AccordionItem::new(
            "gallery-3",
            rsx! { "Customization" },
            rsx! { "Extend or replace styles with Tailwind utilities." },
        ),
    ];

    let multi_items = vec![
        AccordionItem::new(
            "multi-1",
            rsx! { "Feature A" },
            rsx! { "Detailed description of feature A." },
        ),
        AccordionItem::new(
            "multi-2",
            rsx! { "Feature B" },
            rsx! { "Detailed description of feature B." },
        ),
        AccordionItem::new(
            "multi-3",
            rsx! { "Feature C" },
            rsx! { "Detailed description of feature C." },
        ),
    ];

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Single Expand" }
                div { class: "rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                    EqAccordion { items: single_items, mode: AccordionMode::Single }
                }
            }

            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Multi Expand" }
                div { class: "rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                    EqAccordion { items: multi_items, mode: AccordionMode::Multi }
                }
            }
        }
    }
}
