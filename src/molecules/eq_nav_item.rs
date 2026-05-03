use dioxus::prelude::*;
use super::eq_nav_item_styles as s;
use crate::theme::merge_classes;
use crate::atoms::{EqIcon, IconSize};
use crate::{PlaygroundEnum, playground};

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

/// Size variant for EqNavItem.
#[derive(Clone, PartialEq, Default, PlaygroundEnum)]
pub enum NavItemSize {
    #[default]
    Sm,
    Md,
    Lg,
}

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
#[playground(
    category = Molecule,
    description = "Navigation item pairing an optional icon with a text label. \
                   Designed for use inside EqHeader nav or EqNavbar.",
    examples = [
        ("With icon", "EqNavItem {\n    icon: eq_icon_paths::MAGNIFYING_GLASS,\n    label: \"Search\",\n    href: \"/search\",\n}"),
        ("Label only", "EqNavItem { label: \"About\", href: \"/about\" }"),
        ("Active state", "EqNavItem {\n    icon: eq_icon_paths::CHECK,\n    label: \"Dashboard\",\n    href: \"/\",\n    active: true,\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
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
    /// Size of the nav item — controls text size, padding, icon size.
    #[props(default)]
    size: NavItemSize,
    /// Optional icon color override (CSS color value, e.g. "#3eb489").
    /// When empty, the icon inherits the text color.
    #[props(into, default)]
    icon_color: String,
    /// Click handler — use with `navigator().push(Route::…)`
    /// for Dioxus Router integration without adding a router dependency.
    #[props(default)]
    on_click: Option<Callback<Event<MouseData>>>,
    /// Optional class override — extend or replace default styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let size_cls = match size {
        NavItemSize::Sm => s::NAV_ITEM_SM,
        NavItemSize::Md => s::NAV_ITEM_MD,
        NavItemSize::Lg => s::NAV_ITEM_LG,
    };
    let icon_wrap_cls = match size {
        NavItemSize::Sm => s::ICON_SM,
        NavItemSize::Md => s::ICON_MD,
        NavItemSize::Lg => s::ICON_LG,
    };
    let icon_size = match size {
        NavItemSize::Sm => IconSize::Sm,
        NavItemSize::Md => IconSize::Md,
        NavItemSize::Lg => IconSize::Lg,
    };

    let base = if active {
        format!("{} {} {}", s::NAV_ITEM_BASE, size_cls, s::NAV_ITEM_ACTIVE)
    } else {
        format!("{} {}", s::NAV_ITEM_BASE, size_cls)
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
                    span { class: icon_wrap_cls,
                        EqIcon { path: icon, size: icon_size, color: icon_color.clone() }
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
                    span { class: icon_wrap_cls,
                        EqIcon { path: icon, size: icon_size, color: icon_color.clone() }
                    }
                }
                span { class: s::LABEL, "{label}" }
            }
        }
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
