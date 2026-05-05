use dioxus::prelude::*;
use crate::organisms::eq_bottom_bar::BarItem;
use crate::theme::{merge_classes, APP, CONTAINER_LAYOUT, MAIN_CONTENT, MAIN_INNER};

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{CodeBlock, DemoSection, StyleInfo, format_catalog};
#[cfg(feature = "playground")]
use crate::atoms::{TabItem, EqText, TextVariant, EqTab, EqScrollableSpace, EqIcon};
#[cfg(feature = "playground")]
use crate::atoms::eq_icon_paths::{CHECK, MAGNIFYING_GLASS, SQUARE, CARET_LEFT, DOTS_SIX_VERTICAL};
#[cfg(feature = "playground")]
use super::{EqTopBar, EqBottomBar};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Generic app shell layout.
/// The platform crate passes its own header, footer, and main content
/// (typically `Outlet::<Route>`) as Element props.
#[component]
pub fn EqAppShell(
    header: Element,
    footer: Element,
    children: Element,
    /// Optional class override - extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let cls = merge_classes(APP, &class);
    rsx! {
        div { id: "app", class: "{cls}",
            {header}

            main { class: "{MAIN_CONTENT} {MAIN_INNER}",
                div { class: CONTAINER_LAYOUT, {children} }
            }

            {footer}
        }
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-app-shell",
        name: "EqAppShell",
        category: ComponentCategory::Organism,
        description: "Full page layout wrapper with header, footer, and main content area.",
        style_tokens: || vec![
            ("APP", "min-h-screen bg-[var(--color-primary-dark)]"),
            ("CONTAINER_LAYOUT", "mx-auto max-w-6xl px-4"),
            ("MAIN_CONTENT", "flex-1"),
            ("MAIN_INNER", "py-10"),
        ],
        usage_examples: || vec![
            UsageExample {
                label: "Basic",
                code: "EqAppShell {\n    header: rsx! {\n        EqHeader { site_title: \"My App\",\n            nav: rsx! { li { \"Nav item\" } },\n        }\n    },\n    footer: rsx! { EqFooter {} },\n    div { \"Your page content here\" }\n}".into(),
            },
            UsageExample {
                label: "Mobile App",
                code: "use eq_ui::organisms::{EqTopBar, EqBottomBar};\nuse eq_ui::atoms::{TabItem, EqTab, EqText, TextVariant, EqScrollableSpace};\nuse eq_ui::atoms::eq_icon_paths::{SQUARE, MAGNIFYING_GLASS, CHECK};\n\nlet mut active_tab = use_signal(|| 0usize);\nlet mut top_tab_active = use_signal(|| 0usize);\n\nEqAppShell {\n    class: \"flex-col\",\n    header: rsx! {\n        EqTopBar {\n            title: \"My App\",\n            left_element: rsx! {\n                button { class: \"px-3 py-1 text-xs rounded-full bg-[var(--color-button-primary)] text-[var(--color-button-primary-text)] font-medium\", \"▶ Start\" }\n            },\n            right_element: rsx! {\n                button { class: \"px-3 py-1 text-xs rounded-full bg-[var(--color-button-danger)] text-[var(--color-button-danger-text)] font-medium\", \"■ End\" }\n            },\n            tab_selector: rsx! {\n                EqTab {\n                    tabs: vec![\n                        TabItem::new(\"Active\"),\n                        TabItem::new(\"Completed\").badge(3),\n                        TabItem::new(\"Archived\"),\n                    ],\n                    active: top_tab_active(),\n                    on_change: move |idx: usize| top_tab_active.set(idx),\n                }\n            },\n        }\n    },\n    footer: rsx! {\n        EqBottomBar {\n            tabs: vec![\n                TabItem::new(\"Home\").icon(SQUARE),\n                TabItem::new(\"Search\").icon(MAGNIFYING_GLASS),\n                TabItem::new(\"Profile\").icon(CHECK),\n            ],\n            active: active_tab(),\n            on_change: move |idx: usize| active_tab.set(idx),\n        }\n    },\n    EqScrollableSpace {\n        class: \"p-4 pb-20 space-y-3\",\n        for card in 0..12 {\n            div {\n                class: \"rounded-xl bg-[var(--color-card)] border border-[var(--color-card-border)] p-4 space-y-2\",\n                EqText { variant: TextVariant::Emphasis, \"Card #{card + 1}\" }\n                EqText { variant: TextVariant::Body, \"Content for card #{card + 1} goes here.\" }\n            }\n        }\n    }\n}".into(),
            },
        ],
        render_demo: || rsx! {
            DemoEqAppShell {}
        },
        render_gallery: || rsx! {
            GalleryEqAppShell {}
        },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqAppShell() -> Element {
    let code = "EqAppShell {\n    header: rsx! {\n        EqHeader { site_title: \"My App\",\n            nav: rsx! { li { \"Nav item\" } },\n        }\n    },\n    footer: rsx! { EqFooter {} },\n\n    // Page content as children\n    div { \"Your page content here\" }\n}".to_string();

    let mut top_bar_active_tab = use_signal(|| 0usize);
    let mut bottom_bar_active_item = use_signal(|| 0usize);

    let top_bar_tabs = vec![
        TabItem::new("Active"),
        TabItem::new("Completed").badge(3),
        TabItem::new("Archived"),
    ];

    let bottom_bar_items = vec![
        BarItem::new("Home").icon(SQUARE),
        BarItem::new("Search").icon(MAGNIFYING_GLASS),
        BarItem::new("Profile").icon(CHECK),
    ];

    rsx! {
        DemoSection { title: "EqAppShell",
            EqText { variant: TextVariant::Muted,
                "EqAppShell wraps header + footer + children into a full page layout. The playground itself uses EqAppShell."
            }

            // Mobile app layout demo
            div { class: "mt-4 space-y-2",
                EqText { variant: TextVariant::Emphasis, "Mobile App Layout" }
                br {}
                // Fixed-height phone frame — simulates a real device viewport
                div { class: "mx-auto border-[3px] mt-2 border-[var(--color-card-border)] rounded-3xl overflow-hidden shadow-lg w-[375px] h-[640px] flex flex-col",

                    // Top bar with start/end buttons and tabs
                    EqTopBar {
                        title: "My App",
                        left_element: rsx! {
                            EqIcon { path: CARET_LEFT }
                        },
                        right_element: rsx! {
                            EqIcon { path: DOTS_SIX_VERTICAL }
                        },
                        tab_selector: rsx! {
                            EqTab {
                                tabs: top_bar_tabs,
                                active: top_bar_active_tab(),
                                stretch: true,
                                on_change: move |idx: usize| top_bar_active_tab.set(idx),
                            }
                        },
                    }

                    // Scrollable card area with themed scrollbar via EqScrollableSpace
                    EqScrollableSpace { class: "p-4 pb-20 space-y-3",
                        for card in 0..12 {
                            div { class: "rounded-xl bg-[var(--color-card)] border border-[var(--color-card-border)] p-4 space-y-2",
                                EqText {
                                    variant: TextVariant::Caption,
                                    class: "font-semibold",
                                    "Card #{card + 1}"
                                }
                                EqText { variant: TextVariant::Body, "Content for card #{card + 1}." }
                            }
                        }
                    }

                    // Bottom tab bar
                    EqBottomBar {
                        items: bottom_bar_items,
                        active: bottom_bar_active_item(),
                        on_change: move |idx: usize| bottom_bar_active_item.set(idx),
                    }
                }
            }

            StyleInfo {
                file: "theme.rs (shared)",
                styles: format_catalog(
                    &[
                        ("APP", "min-h-screen bg-[var(--color-primary-dark)]"),
                        ("CONTAINER_LAYOUT", "mx-auto max-w-6xl px-4"),
                        ("MAIN_CONTENT", "flex-1"),
                        ("MAIN_INNER", "py-10"),
                    ],
                ),
            }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqAppShell() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "App Shell Gallery"
                }
                div { class: "space-y-3 text-sm text-[var(--color-label-secondary)]",
                    "EqAppShell is a full-page layout. The playground itself uses this component."
                }
            }
        }
    }
}
