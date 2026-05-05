use dioxus::prelude::*;
use super::eq_top_bar_styles as s;
use crate::theme::merge_classes;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{CodeBlock, DemoSection, PropSelect, StyleInfo, format_catalog};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, EqIcon};
#[cfg(feature = "playground")]
use crate::atoms::TextVariant;
#[cfg(feature = "playground")]
use crate::atoms::eq_icon_paths::{CARET_LEFT, DOTS_SIX_VERTICAL};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Mobile top bar component.
/// Mirrors ion-toolbar: sticky header with optional search bar or tab selector below.
#[component]
pub fn EqTopBar(
    #[props(default = "Equidevium")]
    title: &'static str,
    /// Left zone - back button, menu icon, etc.
    left_element: Option<Element>,
    /// Right zone - action buttons, settings icon, etc.
    right_element: Option<Element>,
    /// Optional search bar rendered below the primary row.
    search_bar: Option<Element>,
    /// Optional tab selector rendered below the primary row.
    tab_selector: Option<Element>,
    /// Optional class override - extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let cls = merge_classes(s::TOP_BAR, &class);
    rsx! {
        header { class: "{cls}",
            div { class: "{s::TOP_BAR_INNER}",
                // Primary row: left + title + right
                div { class: s::PRIMARY_ROW,
                    if let Some(left) = left_element {
                        div { class: s::LEFT_ZONE, {left} }
                    }
                    h1 { class: s::TITLE, "{title}" }
                    if let Some(right) = right_element {
                        div { class: s::RIGHT_ZONE, {right} }
                    }
                }
                // Optional secondary row: search or tabs
                if search_bar.is_some() || tab_selector.is_some() {
                    div { class: s::SECONDARY_ROW,
                        if let Some(search) = search_bar {
                            div { class: s::SEARCH_BAR, {search} }
                        }
                        if let Some(tabs) = tab_selector {
                            div { class: s::TAB_SELECTOR, {tabs} }
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
        id: "eq-top-bar",
        name: "EqTopBar",
        category: ComponentCategory::Organism,
        description: "Sticky mobile top bar with optional search bar or tab selector below the title row.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Basic",
                code: "EqTopBar { title: \"My App\" }".into(),
            },
            UsageExample {
                label: "With search",
                code: "EqTopBar {\n    title: \"My App\",\n    search_bar: rsx! { input { class: \"w-full p-2\", placeholder: \"Search...\" } },\n}".into(),
            },
            UsageExample {
                label: "With tabs",
                code: "EqTopBar {\n    title: \"My App\",\n    tab_selector: rsx! { EqTab { tabs: tab_items, active: 0, stretch: true } },\n}".into(),
            },
        ],
        render_demo: || rsx! {
            DemoEqTopBar {}
        },
        render_gallery: || rsx! {
            GalleryEqTopBar {}
        },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqTopBar() -> Element {
    let mut title_str = use_signal(|| "Equidevium".to_string());
    let mut show_search = use_signal(|| false);
    let mut show_tabs = use_signal(|| false);

    let site_title: &'static str = match title_str().as_str() {
        "My App" => "My App",
        "Dashboard" => "Dashboard",
        "Acme Corp" => "Acme Corp",
        _ => "Equidevium",
    };

    let code = "EqTopBar {\n    title: \"My App\",\n    left_element: rsx! { /* back button */ },\n    right_element: rsx! { /* settings icon */ },\n    search_bar: rsx! { input { placeholder: \"Search...\" } },\n    tab_selector: rsx! { EqTab { ... } },\n}".to_string();

    let mut tab_active = use_signal(|| 0usize);
    let tabs = vec![
        crate::atoms::TabItem::new("All"),
        crate::atoms::TabItem::new("Starred"),
        crate::atoms::TabItem::new("Recent"),
    ];

    rsx! {
        DemoSection { title: "EqTopBar",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "title",
                    value: title_str(),
                    options: vec!["Equidevium", "My App", "Dashboard", "Acme Corp"],
                    onchange: move |v: String| title_str.set(v),
                }
                crate::playground::playground_helpers::PropToggle {
                    label: "show_search",
                    value: show_search(),
                    onchange: move |v: bool| show_search.set(v),
                }
                crate::playground::playground_helpers::PropToggle {
                    label: "show_tabs",
                    value: show_tabs(),
                    onchange: move |v: bool| show_tabs.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] overflow-hidden",
                EqTopBar {
                    title: site_title,
                    left_element: rsx! {
                        EqIcon { path: CARET_LEFT }
                    },
                    right_element: rsx! {
                        EqIcon { path: DOTS_SIX_VERTICAL }
                    },
                    search_bar: show_search().then(|| rsx! {
                        input {
                            class: "w-full p-2 rounded border border-[var(--color-card-border)] bg-[var(--color-card)] text-[var(--color-label-primary)]",
                            placeholder: "Search...",
                            r#type: "text",
                        }
                    }),
                    tab_selector: show_tabs().then(|| rsx! {
                        crate::atoms::EqTab {
                            tabs,
                            active: tab_active(),
                            stretch: true,
                            on_change: move |idx: usize| tab_active.set(idx),
                        }
                    }),
                }
            }
            StyleInfo {
                file: "eq_top_bar_styles.rs",
                styles: format_catalog(&s::catalog()),
            }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqTopBar() -> Element {
    let mut gallery_tab_active = use_signal(|| 0usize);

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Top Bar Gallery"
                }

                div { class: "space-y-2",
                    EqText { variant: TextVariant::Muted, "Basic" }
                    div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] overflow-hidden",
                        EqTopBar { title: "Equidevium" }
                    }
                }

                div { class: "space-y-2",
                    EqText { variant: TextVariant::Muted, "With search" }
                    div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] overflow-hidden",
                        EqTopBar {
                            title: "My App",
                            search_bar: rsx! {
                                input {
                                    class: "w-full p-2 rounded border border-[var(--color-card-border)] bg-[var(--color-card)] text-[var(--color-label-primary)]",
                                    placeholder: "Search...",
                                    r#type: "text",
                                }
                            },
                        }
                    }
                }

                div { class: "space-y-2",
                    EqText { variant: TextVariant::Muted, "With tabs" }
                    div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] overflow-hidden",
                        EqTopBar {
                            title: "Dashboard",
                            tab_selector: rsx! {
                                crate::atoms::EqTab {
                                    tabs: vec![
                                        crate::atoms::TabItem::new("All"),
                                        crate::atoms::TabItem::new("Starred"),
                                        crate::atoms::TabItem::new("Recent"),
                                    ],
                                    active: gallery_tab_active(),
                                    on_change: move |idx: usize| gallery_tab_active.set(idx),
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
