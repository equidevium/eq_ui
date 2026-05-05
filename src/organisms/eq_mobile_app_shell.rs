//! Three-region layout for mobile screens: fixed toolbar at the top,
//! scrollable middle, fixed bottom nav at the bottom.
//!
//! Distinct from `EqAppShell` because the structural concerns differ:
//! safe-area padding for the iOS notch and home indicator, fixed
//! positioning of the top and bottom regions, and a body area that
//! scrolls independently.
//!
//! Both fixed slots are optional. A detail page might have a toolbar
//! with no bottom nav; a full-bleed page might have neither. Safe-area
//! padding is applied regardless of slot content so children never
//! render under the status bar or home indicator.
//!
//! ```rust,ignore
//! EqMobileAppShell {
//!     toolbar: rsx! {
//!         EqToolbar { title: rsx! { "Inbox" } }
//!     },
//!     bottom_nav: rsx! {
//!         EqBottomNav {
//!             items: nav_items,
//!             active: active(),
//!             on_change: move |id| active.set(id),
//!         }
//!     },
//!     div { class: "p-4", "Page body here" }
//! }
//! ```

use super::eq_mobile_app_shell_styles as s;
use crate::playground;
use crate::theme::merge_classes;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant, EqButton, ButtonVariant};
#[cfg(feature = "playground")]
use crate::molecules::EqDeviceFrame;
#[cfg(feature = "playground")]
use crate::organisms::{EqToolbar, EqBottomNav, BottomNavItem, BottomNavBadge};
#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{
    ComponentCategory, ComponentDescriptor, UsageExample,
};

/// Mobile app layout with optional fixed toolbar, scrollable body, and
/// optional fixed bottom nav.
#[playground(
    category = Organism,
    description = "Mobile app layout with optional toolbar, scrollable body, and optional bottom nav. Pads for iOS safe areas.",
    examples = [
        ("Both slots", "EqMobileAppShell {\n    toolbar: rsx! { EqToolbar { title: rsx! { \"Inbox\" } } },\n    bottom_nav: rsx! { EqBottomNav { items, active, on_change } },\n    div { class: \"p-4\", \"Body\" }\n}"),
        ("Toolbar only", "EqMobileAppShell {\n    toolbar: rsx! { EqToolbar { title: rsx! { \"Detail\" } } },\n    div { \"Body\" }\n}"),
        ("Body only", "EqMobileAppShell {\n    div { class: \"p-4\", \"Full-bleed body\" }\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqMobileAppShell(
    /// Optional fixed top region. Typically `EqToolbar`.
    toolbar: Option<Element>,
    /// Optional fixed bottom region. Typically `EqBottomNav`.
    bottom_nav: Option<Element>,
    /// Page body. Fills the remaining height and scrolls.
    children: Element,
    /// Optional class override on the outer wrapper.
    #[props(into, default)]
    class: String,
) -> Element {
    let root_cls = merge_classes(s::ROOT, &class);

    rsx! {
        div { class: "{root_cls}",
            if let Some(node) = toolbar {
                div { class: "{s::TOOLBAR_REGION}",
                    {node}
                }
            }

            main { class: "{s::BODY}",
                {children}
            }

            if let Some(node) = bottom_nav {
                div { class: "{s::BOTTOM_NAV_REGION}",
                    {node}
                }
            }
        }
    }
}

// ── Demo (custom; composes EqDeviceFrame + EqToolbar + EqBottomNav) ─

#[cfg(feature = "playground")]
#[component]
fn DemoEqMobileAppShell() -> Element {
    let mut active = use_signal(|| "home".to_string());

    let code = r#"let mut active = use_signal(|| "home".to_string());

EqMobileAppShell {
    toolbar: rsx! {
        EqToolbar {
            title: rsx! { "Inbox" },
            end: rsx! { EqButton { variant: ButtonVariant::Ghost, "Edit" } },
        }
    },
    bottom_nav: rsx! {
        EqBottomNav {
            items: vec![
                BottomNavItem::new("home", "Home", rsx! { /* icon */ }),
                BottomNavItem::new("inbox", "Inbox", rsx! { /* icon */ })
                    .badge(BottomNavBadge::Count(3)),
                BottomNavItem::new("profile", "Profile", rsx! { /* icon */ }),
            ],
            active: active(),
            on_change: move |id| active.set(id),
        }
    },
    div { class: "p-4 space-y-3",
        // Page body
    }
}"#
    .to_string();

    let nav_items = vec![
        BottomNavItem::new("home", "Home", demo_icon_home()),
        BottomNavItem::new("inbox", "Inbox", demo_icon_inbox())
            .badge(BottomNavBadge::Count(3)),
        BottomNavItem::new("profile", "Profile", demo_icon_user()),
    ];

    rsx! {
        DemoSection { title: "EqMobileAppShell",
            EqText {
                variant: TextVariant::Muted,
                "The shell is rendered inside an EqDeviceFrame so you can see the mobile layout in context.",
            }

            div { class: "flex justify-center p-6",
                EqDeviceFrame {
                    div { class: "h-full w-full",
                        EqMobileAppShell {
                            toolbar: rsx! {
                                EqToolbar {
                                    title: rsx! { "Inbox" },
                                    end: rsx! {
                                        EqButton { variant: ButtonVariant::Ghost, "Edit" }
                                    },
                                }
                            },
                            bottom_nav: rsx! {
                                EqBottomNav {
                                    items: nav_items.clone(),
                                    active: active(),
                                    on_change: move |id| active.set(id),
                                }
                            },
                            div { class: "p-4 space-y-3",
                                for i in 1..=12 {
                                    div {
                                        class: "p-3 rounded-md border border-[var(--color-card-border)] bg-[var(--color-card)]",
                                        div { class: "text-sm font-semibold text-[var(--color-label-primary)]",
                                            "Message {i}"
                                        }
                                        div { class: "text-xs text-[var(--color-label-secondary)]",
                                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            StyleInfo {
                file: "eq_mobile_app_shell_styles.rs",
                styles: format_catalog(&s::catalog()),
            }
            CodeBlock { code }
        }
    }
}

// ── Gallery ─────────────────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqMobileAppShell() -> Element {
    let active = use_signal(|| "home".to_string());

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Mobile App Shell Gallery",
                }
            }

            div { class: "flex flex-wrap items-start gap-8 justify-center p-4",
                div { class: "flex flex-col items-center gap-2",
                    EqDeviceFrame {
                        div { class: "h-full w-full",
                            EqMobileAppShell {
                                toolbar: rsx! {
                                    EqToolbar { title: rsx! { "Toolbar only" } }
                                },
                                div { class: "p-4 text-sm text-[var(--color-label-secondary)]",
                                    "Detail page with no bottom nav."
                                }
                            }
                        }
                    }
                    EqText { variant: TextVariant::Caption, "Toolbar only" }
                }

                div { class: "flex flex-col items-center gap-2",
                    EqDeviceFrame {
                        div { class: "h-full w-full",
                            EqMobileAppShell {
                                bottom_nav: rsx! {
                                    EqBottomNav {
                                        items: vec![
                                            BottomNavItem::new("home", "Home", demo_icon_home()),
                                            BottomNavItem::new("inbox", "Inbox", demo_icon_inbox()),
                                            BottomNavItem::new("profile", "Profile", demo_icon_user()),
                                        ],
                                        active: active(),
                                        on_change: move |_| {},
                                    }
                                },
                                div { class: "p-4 text-sm text-[var(--color-label-secondary)]",
                                    "Full-bleed page with bottom nav only."
                                }
                            }
                        }
                    }
                    EqText { variant: TextVariant::Caption, "Bottom nav only" }
                }
            }
        }
    }
}

// ── Demo icons ──────────────────────────────────────────────────────

#[cfg(feature = "playground")]
fn demo_icon_home() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M3 9.5 12 3l9 6.5V21a1 1 0 0 1-1 1h-5v-7h-6v7H4a1 1 0 0 1-1-1Z" }
        }
    }
}

#[cfg(feature = "playground")]
fn demo_icon_inbox() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M22 13H16l-2 3h-4l-2-3H2" }
            path { d: "M5.5 5h13l3.5 8v6a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-6Z" }
        }
    }
}

#[cfg(feature = "playground")]
fn demo_icon_user() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            circle { cx: "12", cy: "8", r: "4" }
            path { d: "M4 21a8 8 0 0 1 16 0" }
        }
    }
}
