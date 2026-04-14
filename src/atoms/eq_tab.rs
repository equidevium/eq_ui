//! EqTab - themed tab bar atom (pure Tailwind).
//!
//! Renders a row of tab buttons with built-in active state management.
//! Three visual variants (Underline, Pill, Card) and three size presets
//! let you match the tab style to context. Each tab can carry an optional
//! badge count and disabled state.
//!
//! ```rust,ignore
//! let mut active = use_signal(|| 0usize);
//!
//! EqTab {
//!     tabs: vec![
//!         TabItem::new("Overview"),
//!         TabItem::new("Details").badge(3),
//!         TabItem::new("Settings"),
//!     ],
//!     active: active(),
//!     on_change: move |idx| active.set(idx),
//! }
//! ```

use super::eq_tab_styles as s;
use crate::theme::merge_classes;
use dioxus::document;
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

/// Visual variant for the tab bar.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum TabVariant {
    /// Bottom-border indicator on the active tab (default).
    #[default]
    Underline,
    /// Rounded-pill highlight behind the active tab.
    Pill,
    /// Card-style raised tab sitting on a border.
    Card,
}

/// Size preset for tab buttons.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum TabSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Descriptor for a single tab in the bar.
#[derive(Clone, PartialEq)]
pub struct TabItem {
    /// Display label rendered inside the tab button.
    pub label: String,
    /// Optional badge count (shown as a small pill beside the label).
    pub badge: Option<usize>,
    /// When true, the tab is rendered but cannot be selected.
    pub disabled: bool,
}

impl TabItem {
    /// Create a new tab with just a label.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            badge: None,
            disabled: false,
        }
    }

    /// Builder: attach a badge count.
    pub fn badge(mut self, count: usize) -> Self {
        self.badge = Some(count);
        self
    }

    /// Builder: mark this tab as disabled.
    pub fn disabled(mut self, yes: bool) -> Self {
        self.disabled = yes;
        self
    }
}

// ── Component ─────────────────────────────────────────────────────

/// Themed tab bar atom.
///
/// Renders a horizontal list of tab buttons. The caller owns the active
/// index via a signal and is notified through `on_change` when the user
/// clicks a different tab. Content switching is handled externally -
/// EqTab only renders the tab row itself.
#[component]
pub fn EqTab(
    /// The list of tabs to display.
    tabs: Vec<TabItem>,
    /// Index of the currently active tab (0-based).
    #[props(default = 0)]
    active: usize,
    /// Visual variant.
    #[props(default)]
    variant: TabVariant,
    /// Size preset.
    #[props(default)]
    size: TabSize,
    /// Callback fired when the user clicks a non-disabled, non-active tab.
    /// Receives the new tab index.
    #[props(default)]
    on_change: Option<EventHandler<usize>>,
    /// Optional class override on the outermost container.
    #[props(into, default)]
    class: String,
) -> Element {
    let size_cls = match size {
        TabSize::Sm => s::SM,
        TabSize::Md => s::MD,
        TabSize::Lg => s::LG,
    };

    let (container_extra, base_cls, active_cls) = match variant {
        TabVariant::Underline => (s::CONTAINER_UNDERLINE, s::UNDERLINE_BASE, s::UNDERLINE_ACTIVE),
        TabVariant::Pill => ("", s::PILL_BASE, s::PILL_ACTIVE),
        TabVariant::Card => (s::CONTAINER_CARD, s::CARD_BASE, s::CARD_ACTIVE),
    };

    let container_base = format!("{} {}", s::CONTAINER, container_extra);
    let container_cls = merge_classes(&container_base, &class);

    // Stable unique ID prefix for this tab bar instance
    let tab_id_prefix = use_hook(|| {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("eq-tab-{id}")
    });

    // Collect enabled tab indices for keyboard navigation
    let enabled_indices: Vec<usize> = tabs
        .iter()
        .enumerate()
        .filter(|(_, tab)| !tab.disabled)
        .map(|(i, _)| i)
        .collect();

    let enabled_for_keydown = enabled_indices.clone();
    let prefix_for_keydown = tab_id_prefix.clone();

    rsx! {
        div {
            class: "{container_cls}",
            role: "tablist",
            "aria-orientation": "horizontal",
            onkeydown: move |evt: Event<KeyboardData>| {
                if enabled_for_keydown.is_empty() { return; }
                let key = evt.key();

                // Find current position in enabled list
                let current_pos = enabled_for_keydown.iter().position(|&i| i == active);

                let next_idx = match key {
                    Key::ArrowRight => {
                        evt.prevent_default();
                        match current_pos {
                            Some(pos) => {
                                let next = (pos + 1) % enabled_for_keydown.len();
                                Some(enabled_for_keydown[next])
                            }
                            None => Some(enabled_for_keydown[0]),
                        }
                    }
                    Key::ArrowLeft => {
                        evt.prevent_default();
                        match current_pos {
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
                        handler.call(idx);
                    }
                    // Move browser focus to the newly active tab button
                    let focus_id = format!("{}-{idx}", prefix_for_keydown);
                    document::eval(&format!(
                        "document.getElementById('{focus_id}')?.focus()"
                    ));
                }
            },
            for (idx , tab) in tabs.iter().enumerate() {
                {
                    let is_active = idx == active;
                    let is_disabled = tab.disabled;

                    let mut btn_cls = format!("{} {}", base_cls, size_cls);
                    if is_active {
                        btn_cls = format!("{} {}", btn_cls, active_cls);
                    }
                    if is_disabled {
                        btn_cls = format!("{} {}", btn_cls, s::DISABLED);
                    }

                    let has_badge = tab.badge.is_some();
                    if has_badge {
                        btn_cls = format!("{} {}", btn_cls, s::WITH_ICON);
                    }

                    let badge_val = tab.badge;
                    let label = tab.label.clone();
                    let on_change = on_change.clone();

                    // Roving tabindex: only the active tab is in the tab order
                    let tab_idx = if is_active { "0" } else { "-1" };
                    let btn_id = format!("{tab_id_prefix}-{idx}");

                    rsx! {
                        button {
                            key: "{idx}",
                            id: "{btn_id}",
                            class: "{btn_cls}",
                            role: "tab",
                            "aria-selected": "{is_active}",
                            "aria-disabled": "{is_disabled}",
                            tabindex: "{tab_idx}",
                            disabled: is_disabled,
                            onclick: move |_| {
                                if !is_active && !is_disabled {
                                    if let Some(ref handler) = on_change {
                                        handler.call(idx);
                                    }
                                }
                            },
                            "{label}"
                            if let Some(count) = badge_val {
                                span { class: "{s::BADGE}", "{count}" }
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
        id: "eq-tab",
        name: "EqTab",
        category: ComponentCategory::Atom,
        description: "Themed tab bar with underline, pill, and card variants. \
                      Supports badges, disabled tabs, and three size presets.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Basic underline",
                code: "let mut active = use_signal(|| 0usize);\n\n\
                       EqTab {\n\
                       \x20   tabs: vec![\n\
                       \x20       TabItem::new(\"Overview\"),\n\
                       \x20       TabItem::new(\"Details\"),\n\
                       \x20       TabItem::new(\"Settings\"),\n\
                       \x20   ],\n\
                       \x20   active: active(),\n\
                       \x20   on_change: move |idx| active.set(idx),\n\
                       }".into(),
            },
            UsageExample {
                label: "Pill variant with badge",
                code: "EqTab {\n\
                       \x20   tabs: vec![\n\
                       \x20       TabItem::new(\"Inbox\").badge(12),\n\
                       \x20       TabItem::new(\"Sent\"),\n\
                       \x20       TabItem::new(\"Trash\").disabled(true),\n\
                       \x20   ],\n\
                       \x20   variant: TabVariant::Pill,\n\
                       \x20   active: active(),\n\
                       \x20   on_change: move |idx| active.set(idx),\n\
                       }".into(),
            },
            UsageExample {
                label: "Card variant",
                code: "EqTab {\n\
                       \x20   variant: TabVariant::Card,\n\
                       \x20   size: TabSize::Lg,\n\
                       \x20   tabs: vec![TabItem::new(\"Code\"), TabItem::new(\"Preview\")],\n\
                       \x20   active: active(),\n\
                       \x20   on_change: move |idx| active.set(idx),\n\
                       }".into(),
            },
        ],
        render_demo: || rsx! { DemoEqTab {} },
        render_gallery: || rsx! { GalleryEqTab {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqTab() -> Element {
    let mut active = use_signal(|| 0usize);
    let mut variant_str = use_signal(|| "Underline".to_string());
    let mut size_str = use_signal(|| "Md".to_string());

    let variant = match variant_str().as_str() {
        "Pill" => TabVariant::Pill,
        "Card" => TabVariant::Card,
        _ => TabVariant::Underline,
    };

    let size = match size_str().as_str() {
        "Sm" => TabSize::Sm,
        "Lg" => TabSize::Lg,
        _ => TabSize::Md,
    };

    let tabs = vec![
        TabItem::new("Overview"),
        TabItem::new("Details").badge(5),
        TabItem::new("Analytics"),
        TabItem::new("Disabled").disabled(true),
    ];

    let tab_content = match active() {
        0 => "Overview - a summary of the most important information at a glance.",
        1 => "Details - in-depth data with all fields expanded.",
        2 => "Analytics - charts and metrics for the selected period.",
        _ => "This tab is disabled and cannot be reached.",
    };

    let code = "use eq_ui::atoms::{EqTab, TabItem, TabVariant, TabSize};\n\
        \n\
        let mut active = use_signal(|| 0usize);\n\
        \n\
        EqTab {\n\
        \x20   tabs: vec![\n\
        \x20       TabItem::new(\"Overview\"),\n\
        \x20       TabItem::new(\"Details\").badge(5),\n\
        \x20       TabItem::new(\"Analytics\"),\n\
        \x20       TabItem::new(\"Disabled\").disabled(true),\n\
        \x20   ],\n\
        \x20   variant: TabVariant::Underline,\n\
        \x20   size: TabSize::Md,\n\
        \x20   active: active(),\n\
        \x20   on_change: move |idx| active.set(idx),\n\
        }".to_string();

    rsx! {
        DemoSection { title: "EqTab",
            // Prop controls
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                div { class: "grid grid-cols-2 md:grid-cols-3 gap-3",
                    PropSelect {
                        label: "variant",
                        value: variant_str(),
                        options: vec!["Underline", "Pill", "Card"],
                        onchange: move |v: String| variant_str.set(v),
                    }
                    PropSelect {
                        label: "size",
                        value: size_str(),
                        options: vec!["Sm", "Md", "Lg"],
                        onchange: move |v: String| size_str.set(v),
                    }
                }
            }

            // Live preview
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Preview" }

                EqTab {
                    tabs: tabs,
                    variant: variant,
                    size: size,
                    active: active(),
                    on_change: move |idx: usize| active.set(idx),
                }

                // Content area to show tab switching in action
                div { class: "mt-4 p-4 rounded-lg bg-[var(--color-card)] border border-[var(--color-card-border)]",
                    EqText { variant: TextVariant::Body, "{tab_content}" }
                }
            }

            // Style tokens
            StyleInfo { file: "eq_tab_styles.rs", styles: format_catalog(&s::catalog()) }

            // Usage code
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqTab() -> Element {
    let mut underline_active = use_signal(|| 0usize);
    let mut pill_active = use_signal(|| 1usize);
    let mut card_active = use_signal(|| 0usize);
    let mut size_sm_active = use_signal(|| 0usize);
    let mut size_lg_active = use_signal(|| 0usize);

    let basic_tabs = || vec![
        TabItem::new("First"),
        TabItem::new("Second"),
        TabItem::new("Third"),
    ];

    let badge_tabs = || vec![
        TabItem::new("Inbox").badge(12),
        TabItem::new("Drafts").badge(3),
        TabItem::new("Sent"),
        TabItem::new("Spam").disabled(true),
    ];

    rsx! {
        div { class: "space-y-4",
            // Variant gallery
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-5",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Variant Gallery" }

                div { class: "space-y-1",
                    EqText { variant: TextVariant::Muted, "Underline (default)" }
                    EqTab {
                        tabs: basic_tabs(),
                        active: underline_active(),
                        on_change: move |idx: usize| underline_active.set(idx),
                    }
                }

                div { class: "space-y-1",
                    EqText { variant: TextVariant::Muted, "Pill" }
                    EqTab {
                        tabs: badge_tabs(),
                        variant: TabVariant::Pill,
                        active: pill_active(),
                        on_change: move |idx: usize| pill_active.set(idx),
                    }
                }

                div { class: "space-y-1",
                    EqText { variant: TextVariant::Muted, "Card" }
                    EqTab {
                        tabs: basic_tabs(),
                        variant: TabVariant::Card,
                        active: card_active(),
                        on_change: move |idx: usize| card_active.set(idx),
                    }
                }
            }

            // Size gallery
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-5",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Sizes" }

                div { class: "space-y-1",
                    EqText { variant: TextVariant::Muted, "Small" }
                    EqTab {
                        tabs: basic_tabs(),
                        size: TabSize::Sm,
                        active: size_sm_active(),
                        on_change: move |idx: usize| size_sm_active.set(idx),
                    }
                }

                div { class: "space-y-1",
                    EqText { variant: TextVariant::Muted, "Medium (default)" }
                    EqTab {
                        tabs: basic_tabs(),
                        active: 1usize,
                    }
                }

                div { class: "space-y-1",
                    EqText { variant: TextVariant::Muted, "Large" }
                    EqTab {
                        tabs: basic_tabs(),
                        size: TabSize::Lg,
                        active: size_lg_active(),
                        on_change: move |idx: usize| size_lg_active.set(idx),
                    }
                }
            }
        }
    }
}
