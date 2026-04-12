use dioxus::prelude::*;
use crate::theme::{merge_classes, APP, CONTAINER_LAYOUT, MAIN_CONTENT, MAIN_INNER};

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{CodeBlock, DemoSection, StyleInfo, format_catalog};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
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
                div { class: CONTAINER_LAYOUT,
                    {children}
                }
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
        ],
        render_demo: || rsx! { DemoEqAppShell {} },
        render_gallery: || rsx! { GalleryEqAppShell {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqAppShell() -> Element {
    let code = "EqAppShell {\n    header: rsx! {\n        EqHeader { site_title: \"My App\",\n            nav: rsx! { li { \"Nav item\" } },\n        }\n    },\n    footer: rsx! { EqFooter {} },\n\n    // Page content as children\n    div { \"Your page content here\" }\n}".to_string();

    rsx! {
        DemoSection { title: "EqAppShell",
            EqText { variant: TextVariant::Muted,
                "EqAppShell wraps header + footer + children into a full page layout. It is the outermost layout component - you are looking at a live example right now. This playground itself uses EqAppShell."
            }
            StyleInfo { file: "theme.rs (shared)", styles: format_catalog(&[
                ("APP", "min-h-screen bg-[var(--color-primary-dark)]"),
                ("CONTAINER_LAYOUT", "mx-auto max-w-6xl px-4"),
                ("MAIN_CONTENT", "flex-1"),
                ("MAIN_INNER", "py-10"),
            ]) }
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
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "App Shell Gallery" }

                div { class: "space-y-3 text-sm text-[var(--color-label-secondary)]",
                    "EqAppShell is a full-page layout. The playground itself uses this component."
                }
            }
        }
    }
}
