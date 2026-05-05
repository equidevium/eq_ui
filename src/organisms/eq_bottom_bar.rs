use dioxus::prelude::*;
use super::eq_bottom_bar_styles as s;
use crate::atoms::{EqIcon, EqText, IconSize, TextVariant};
use crate::atoms::eq_icon_paths::{CHECK, FUNNEL, MAGNIFYING_GLASS, SQUARE};
use crate::theme::merge_classes;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{CodeBlock, DemoSection, StyleInfo, format_catalog};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// ── Types ─────────────────────────────────────────────────────────

/// Descriptor for a single item in the bar.
#[derive(Clone, PartialEq)]
pub struct BarItem {
    /// Display label rendered inside the item button.
    pub label: String,
    /// Optional badge count (shown as a small pill beside the label).
    pub badge: Option<usize>,
    /// Optional SVG path for the item icon.
    pub icon_path: Option<&'static str>,
}

impl BarItem {
    /// Create a new item with just a label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            badge: None,
            icon_path: None,
        }
    }

    /// Builder: attach a badge count.
    pub fn badge(mut self, count: usize) -> Self {
        self.badge = Some(count);
        self
    }

    /// Builder: set an icon (SVG `d` path string).
    pub fn icon(mut self, path: &'static str) -> Self {
        self.icon_path = Some(path);
        self
    }
}

// ── Component ─────────────────────────────────────────────────────

/// Mobile bottom bar component.
#[component]
pub fn EqBottomBar(
    /// Items to display.
    items: Vec<BarItem>,
    /// Index of the active item (0-based).
    #[props(default = 0)]
    active: usize,
    /// Callback when the user selects a different item.
    #[props(default)]
    on_change: Option<EventHandler<usize>>,
    /// Optional class override - extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let cls = merge_classes(s::BOTTOM_BAR, &class);

    rsx! {
        nav { class: "{cls}",
            div { class: s::BOTTOM_BAR_INNER,
                for (idx , item) in items.iter().enumerate() {
                    {
                        let is_active = idx == active;
                        let mut btn_cls = s::ITEM_BUTTON.to_string();
                        if is_active {
                            btn_cls = format!("{} {}", btn_cls, s::ITEM_BUTTON_ACTIVE);
                        }
                        let icon_path = item.icon_path;
                        let label = item.label.clone();
                        let on_change = on_change.clone();

                        rsx! {
                            button {
                                key: "{idx}",
                                class: "{btn_cls}",
                                role: "tab",
                                "aria-selected": "{is_active}",
                                onclick: move |_| {
                                    if !is_active {
                                        if let Some(ref handler) = on_change {
                                            handler.call(idx);
                                        }
                                    }
                                },
                                if icon_path.is_some() {
                                    EqIcon { path: icon_path.unwrap().to_string(), size: IconSize::Md }
                                }
                                EqText { variant: TextVariant::Caption, class: s::ITEM_LABEL, "{label}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-bottom-bar",
        name: "EqBottomBar",
        category: ComponentCategory::Organism,
        description: "Fixed bottom tab bar with icon+label tabs. Mirrors ion-tab-bar.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Basic",
                code: "EqBottomBar {\n    items: vec![\n        BarItem::new(\"Home\"),\n        BarItem::new(\"Search\"),\n        BarItem::new(\"Profile\"),\n    ],\n    active: 0,\n}".into(),
            },
            UsageExample {
                label: "With icons",
                code: "EqBottomBar {\n    items: vec![\n        BarItem::new(\"Home\").icon(eq_icon_paths::SQUARE),\n        BarItem::new(\"Search\").icon(eq_icon_paths::MAGNIFYING_GLASS),\n        BarItem::new(\"Profile\").icon(eq_icon_paths::CHECK),\n    ],\n    active: active(),\n    on_change: move |idx| active.set(idx),\n}".into(),
            },
        ],
        render_demo: || rsx! {
            DemoEqBottomBar {}
        },
        render_gallery: || rsx! {
            GalleryEqBottomBar {}
        },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqBottomBar() -> Element {
    let mut active = use_signal(|| 0usize);

    let items = vec![
        BarItem::new("Home").icon(SQUARE),
        BarItem::new("Search").icon(MAGNIFYING_GLASS),
        BarItem::new("Profile").icon(CHECK),
        BarItem::new("Cart").icon(FUNNEL),
    ];

    let code = "let mut active = use_signal(|| 0usize);\n\nEqBottomBar {\n    items: vec![\n        BarItem::new(\"Home\").icon(/* svg_path */),\n        BarItem::new(\"Search\").icon(/* svg_path */),\n        BarItem::new(\"Profile\").icon(/* svg_path */),\n        BarItem::new(\"Cart\").icon(/* svg_path */),\n    ],\n    active: active(),\n    on_change: move |idx: usize| active.set(idx),\n}".to_string();

    rsx! {
        DemoSection { title: "EqBottomBar",
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] overflow-hidden",
                EqBottomBar {
                    items,
                    active: active(),
                    on_change: move |idx: usize| active.set(idx),
                }
            }
            StyleInfo {
                file: "eq_bottom_bar_styles.rs",
                styles: format_catalog(&s::catalog()),
            }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqBottomBar() -> Element {
    let mut gallery_active = use_signal(|| 0usize);

    let gallery_items = vec![
        BarItem::new("Home").icon(SQUARE),
        BarItem::new("Search").icon(MAGNIFYING_GLASS),
        BarItem::new("Profile").icon(CHECK),
    ];

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Bottom Bar Gallery"
                }

                div { class: "space-y-2",
                    EqText { variant: TextVariant::Muted, "With icons (ion-tab-bar style)" }
                    div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] overflow-hidden",
                        EqBottomBar {
                            items: gallery_items,
                            active: gallery_active(),
                            on_change: move |idx: usize| gallery_active.set(idx),
                        }
                    }
                }
            }
        }
    }
}
