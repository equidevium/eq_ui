use dioxus::prelude::*;
use super::eq_nav_item_styles as s;
use crate::theme::merge_classes;
use crate::atoms::{EqIcon, IconSize};

#[cfg(feature = "playground")]
use crate::atoms::eq_icon_paths;
#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropInput, PropToggle, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Navigation item molecule pairing an icon with a text label.
///
/// Designed to live inside EqHeader's `nav` prop, wrapped in `<li>`.
///
/// ```rust,ignore
/// use eq_ui::atoms::eq_icon_paths;
///
/// // Plain links (no router):
/// EqHeader {
///     nav: rsx! {
///         li { EqNavItem { icon: eq_icon_paths::MAGNIFYING_GLASS, label: "Search", href: "/search" } }
///         li { EqNavItem { label: "About", href: "/about" } }
///     },
/// }
///
/// // With Dioxus Router (use onclick + navigator):
/// EqHeader {
///     nav: rsx! {
///         li { EqNavItem {
///             icon: eq_icon_paths::MAGNIFYING_GLASS,
///             label: "Settings",
///             onclick: move |_| navigator().push(Route::AppSettings {}),
///         } }
///     },
/// }
/// ```
#[component]
pub fn EqNavItem(
    /// SVG path data for the icon (from `eq_icon_paths` constants).
    /// When empty, no icon is rendered — label-only mode.
    #[props(into, default)]
    icon: String,
    /// Visible text label.
    #[props(into)]
    label: String,
    /// Link target URL. Rendered as an `<a>` tag.
    /// For Dioxus Router integration, wrap the component in a `Link`
    /// instead and leave `href` empty.
    #[props(into, default)]
    href: String,
    /// When true, applies the active/highlighted visual state.
    #[props(default = false)]
    active: bool,
    /// Click handler — use with `navigator().push(Route::…)`
    /// for Dioxus Router integration without adding a router dependency.
    #[props(default)]
    on_click: Option<Callback<Event<MouseData>>>,
    /// Optional class override — extend or replace default styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let base = if active {
        format!("{} {}", s::NAV_ITEM, s::NAV_ITEM_ACTIVE)
    } else {
        s::NAV_ITEM.to_string()
    };
    let cls = merge_classes(&base, &class);

    let has_icon = !icon.is_empty();
    let has_href = !href.is_empty();
    let has_onclick = on_click.is_some();

    // Render as <a> when href is set, otherwise as a clickable <span>.
    if has_href {
        rsx! {
            a {
                class: "{cls}",
                href: "{href}",
                onclick: move |evt| {
                    if let Some(cb) = &on_click {
                        cb(evt);
                    }
                },
                if has_icon {
                    span { class: s::ICON_WRAP,
                        EqIcon { path: icon, size: IconSize::Sm }
                    }
                }
                span { class: s::LABEL, "{label}" }
            }
        }
    } else {
        rsx! {
            span {
                class: "{cls}",
                role: if has_onclick { "button" },
                tabindex: if has_onclick { "0" },
                onclick: move |evt| {
                    if let Some(cb) = &on_click {
                        cb(evt);
                    }
                },
                if has_icon {
                    span { class: s::ICON_WRAP,
                        EqIcon { path: icon, size: IconSize::Sm }
                    }
                }
                span { class: s::LABEL, "{label}" }
            }
        }
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-nav-item",
        name: "EqNavItem",
        category: ComponentCategory::Molecule,
        description: "Navigation item pairing an optional icon with a text label. \
                      Designed for use inside EqHeader nav or EqNavbar.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "With icon",
                code: "EqNavItem {\n    icon: eq_icon_paths::MAGNIFYING_GLASS,\n    label: \"Search\",\n    href: \"/search\",\n}".into(),
            },
            UsageExample {
                label: "Label only",
                code: "EqNavItem { label: \"About\", href: \"/about\" }".into(),
            },
            UsageExample {
                label: "Active state",
                code: "EqNavItem {\n    icon: eq_icon_paths::CHECK,\n    label: \"Dashboard\",\n    href: \"/\",\n    active: true,\n}".into(),
            },
            UsageExample {
                label: "Router onclick",
                code: "EqNavItem {\n    icon: eq_icon_paths::FUNNEL,\n    label: \"Settings\",\n    onclick: move |_| navigator().push(Route::AppSettings {}),\n}".into(),
            },
        ],
        render_demo: || rsx! { DemoEqNavItem {} },
        render_gallery: || rsx! { GalleryEqNavItem {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqNavItem() -> Element {
    let mut label = use_signal(|| "Home".to_string());
    let mut href = use_signal(|| "#".to_string());
    let mut active = use_signal(|| false);
    let mut show_icon = use_signal(|| true);

    let icon_path = if show_icon() {
        eq_icon_paths::CHECK.to_string()
    } else {
        String::new()
    };

    let code = r#"use eq_ui::atoms::eq_icon_paths;

// Inside EqHeader nav:
li { EqNavItem { icon: eq_icon_paths::CHECK, label: "Home", href: "/" } }
li { EqNavItem { label: "About", href: "/about" } }
li { EqNavItem { icon: eq_icon_paths::MAGNIFYING_GLASS, label: "Search", href: "/search", active: true } }"#
        .to_string();

    rsx! {
        DemoSection { title: "EqNavItem",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropInput {
                    label: "label",
                    value: label(),
                    placeholder: "Nav label",
                    onchange: move |v: String| label.set(v),
                }
                PropInput {
                    label: "href",
                    value: href(),
                    placeholder: "URL",
                    onchange: move |v: String| href.set(v),
                }
                PropToggle {
                    label: "active",
                    value: active(),
                    onchange: move |v: bool| active.set(v),
                }
                PropToggle {
                    label: "show icon",
                    value: show_icon(),
                    onchange: move |v: bool| show_icon.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                nav {
                    ul { class: "flex items-center gap-1",
                        li {
                            EqNavItem {
                                icon: icon_path,
                                label: label(),
                                href: href(),
                                active: active(),
                            }
                        }
                        li {
                            EqNavItem {
                                icon: eq_icon_paths::MAGNIFYING_GLASS.to_string(),
                                label: "Search",
                                href: "#",
                            }
                        }
                        li {
                            EqNavItem {
                                label: "About",
                                href: "#",
                            }
                        }
                    }
                }
            }
            StyleInfo { file: "eq_nav_item_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqNavItem() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "NavItem Gallery" }

                nav {
                    ul { class: "flex items-center gap-1",
                        li {
                            EqNavItem {
                                icon: eq_icon_paths::CHECK.to_string(),
                                label: "Dashboard",
                                href: "#",
                                active: true,
                            }
                        }
                        li {
                            EqNavItem {
                                icon: eq_icon_paths::FUNNEL.to_string(),
                                label: "Filters",
                                href: "#",
                            }
                        }
                        li {
                            EqNavItem {
                                icon: eq_icon_paths::MAGNIFYING_GLASS.to_string(),
                                label: "Search",
                                href: "#",
                            }
                        }
                        li {
                            EqNavItem {
                                label: "About",
                                href: "#",
                            }
                        }
                    }
                }
            }
        }
    }
}
