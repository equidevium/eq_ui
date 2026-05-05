//! Bottom-anchored tab bar for mobile navigation.
//!
//! A horizontal row of icon + label items. One active at a time.
//! Items can carry a count or dot badge and can be disabled.
//!
//! Active state lives with the consumer: pass `active` (the id of the
//! selected item) and react to `on_change`. The component does not own
//! routing or selection state.
//!
//! Positioning is the consumer's job. Use `EqMobileAppShell` to anchor
//! this to the bottom of a mobile layout, or wrap it yourself with
//! `position: sticky` / `position: fixed` for a standalone app.
//!
//! ```rust,ignore
//! let mut active = use_signal(|| "home".to_string());
//!
//! EqBottomNav {
//!     items: vec![
//!         BottomNavItem::new("home", "Home", rsx! { /* icon */ }),
//!         BottomNavItem::new("inbox", "Inbox", rsx! { /* icon */ })
//!             .badge(BottomNavBadge::Count(3)),
//!         BottomNavItem::new("profile", "Profile", rsx! { /* icon */ }),
//!     ],
//!     active: active(),
//!     on_change: move |id| active.set(id),
//! }
//! ```

use super::eq_bottom_nav_styles as s;
use crate::playground;
use crate::theme::merge_classes;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::molecules::EqDeviceFrame;
#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{
    ComponentCategory, ComponentDescriptor, UsageExample,
};

/// Badge style on a `BottomNavItem`.
#[derive(Clone, PartialEq)]
pub enum BottomNavBadge {
    /// Show a count. Numbers above 99 render as "99+".
    Count(u32),
    /// Show a small dot with no number.
    Dot,
}

impl BottomNavBadge {
    /// Display string for a count badge. Numbers above 99 cap at "99+".
    /// Returns `None` for the dot variant since it has no label.
    pub fn count_label(&self) -> Option<String> {
        match self {
            Self::Count(n) if *n > 99 => Some("99+".to_string()),
            Self::Count(n) => Some(n.to_string()),
            Self::Dot => None,
        }
    }
}

/// Single item rendered inside `EqBottomNav`.
#[derive(Clone, PartialEq)]
pub struct BottomNavItem {
    /// Stable identifier returned via `on_change`.
    pub id: String,
    /// Visible label below the icon.
    pub label: String,
    /// Icon element (typically `EqIcon` or a raw `<svg>`).
    pub icon: Element,
    /// Optional badge in the icon's top-right corner.
    pub badge: Option<BottomNavBadge>,
    /// When `true`, the item renders muted and ignores taps.
    pub disabled: bool,
}

impl BottomNavItem {
    /// Build an item with no badge and disabled = false.
    pub fn new(id: impl Into<String>, label: impl Into<String>, icon: Element) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon,
            badge: None,
            disabled: false,
        }
    }

    pub fn badge(mut self, badge: BottomNavBadge) -> Self {
        self.badge = Some(badge);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Bottom-anchored tab bar.
#[playground(
    category = Organism,
    description = "Bottom-anchored mobile tab bar with icon + label items, badges, and active state.",
    examples = [
        ("Three items", "let mut active = use_signal(|| \"home\".to_string());\n\nEqBottomNav {\n    items: vec![\n        BottomNavItem::new(\"home\", \"Home\", rsx! { /* icon */ }),\n        BottomNavItem::new(\"inbox\", \"Inbox\", rsx! { /* icon */ }),\n        BottomNavItem::new(\"profile\", \"Profile\", rsx! { /* icon */ }),\n    ],\n    active: active(),\n    on_change: move |id| active.set(id),\n}"),
        ("With badges", "BottomNavItem::new(\"inbox\", \"Inbox\", rsx! { /* icon */ })\n    .badge(BottomNavBadge::Count(12))"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqBottomNav(
    /// Items to render, left to right.
    items: Vec<BottomNavItem>,
    /// Id of the active item. If no item matches, none is highlighted.
    #[props(into)]
    active: String,
    /// Fires with the tapped item's id. Disabled items do not fire.
    on_change: EventHandler<String>,
    /// Optional class override on the outer wrapper.
    #[props(into, default)]
    class: String,
) -> Element {
    let wrapper_cls = merge_classes(s::WRAPPER, &class);

    rsx! {
        nav {
            class: "{wrapper_cls}",
            role: "tablist",
            "aria-label": "Bottom navigation",

            for item in items.iter() {
                {
                    let id = item.id.clone();
                    let is_active = id == active;
                    let is_disabled = item.disabled;

                    let mut item_cls = String::from(s::ITEM);
                    if is_active {
                        item_cls.push(' ');
                        item_cls.push_str(s::ITEM_ACTIVE);
                    }
                    if is_disabled {
                        item_cls.push(' ');
                        item_cls.push_str(s::ITEM_DISABLED);
                    }

                    let aria_selected = if is_active { "true" } else { "false" };
                    let aria_disabled = if is_disabled { "true" } else { "false" };

                    let id_for_click = id.clone();
                    let icon = item.icon.clone();
                    let label = item.label.clone();
                    let badge = item.badge.clone();

                    rsx! {
                        button {
                            r#type: "button",
                            class: "{item_cls}",
                            role: "tab",
                            "aria-selected": "{aria_selected}",
                            "aria-disabled": "{aria_disabled}",
                            disabled: is_disabled,
                            onclick: move |_| {
                                if !is_disabled {
                                    on_change.call(id_for_click.clone());
                                }
                            },

                            span { class: "{s::ITEM_ICON}", "aria-hidden": "true",
                                {icon}

                                if let Some(b) = badge {
                                    match b {
                                        BottomNavBadge::Count(_) => rsx! {
                                            span {
                                                class: "{s::BADGE_BASE} {s::BADGE_COUNT}",
                                                {b.count_label().unwrap_or_default()}
                                            }
                                        },
                                        BottomNavBadge::Dot => rsx! {
                                            span { class: "{s::BADGE_BASE} {s::BADGE_DOT}" }
                                        },
                                    }
                                }
                            }

                            span { class: "{s::ITEM_LABEL}", "{label}" }
                        }
                    }
                }
            }
        }
    }
}

// ── Smoke tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_label_below_cap() {
        assert_eq!(BottomNavBadge::Count(0).count_label().unwrap(), "0");
        assert_eq!(BottomNavBadge::Count(1).count_label().unwrap(), "1");
        assert_eq!(BottomNavBadge::Count(99).count_label().unwrap(), "99");
    }

    #[test]
    fn count_label_above_cap() {
        assert_eq!(BottomNavBadge::Count(100).count_label().unwrap(), "99+");
        assert_eq!(BottomNavBadge::Count(9999).count_label().unwrap(), "99+");
    }

    #[test]
    fn dot_has_no_label() {
        assert!(BottomNavBadge::Dot.count_label().is_none());
    }

    #[test]
    fn item_builders_chain() {
        let item = BottomNavItem::new("a", "Alpha", rsx! { div {} })
            .badge(BottomNavBadge::Dot)
            .disabled(true);
        assert_eq!(item.id, "a");
        assert_eq!(item.label, "Alpha");
        assert!(item.disabled);
        assert!(matches!(item.badge, Some(BottomNavBadge::Dot)));
    }

    #[test]
    fn item_defaults() {
        let item = BottomNavItem::new("a", "Alpha", rsx! { div {} });
        assert!(item.badge.is_none());
        assert!(!item.disabled);
    }
}

// ── Demo (custom; needs interactive active state) ───────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqBottomNav() -> Element {
    let mut active = use_signal(|| "home".to_string());

    let code = r#"let mut active = use_signal(|| "home".to_string());

EqBottomNav {
    items: vec![
        BottomNavItem::new("home", "Home", rsx! { /* icon */ }),
        BottomNavItem::new("inbox", "Inbox", rsx! { /* icon */ })
            .badge(BottomNavBadge::Count(3)),
        BottomNavItem::new("profile", "Profile", rsx! { /* icon */ }),
    ],
    active: active(),
    on_change: move |id| active.set(id),
}"#
    .to_string();

    let items = vec![
        BottomNavItem::new("home", "Home", icon_home()),
        BottomNavItem::new("inbox", "Inbox", icon_inbox())
            .badge(BottomNavBadge::Count(3)),
        BottomNavItem::new("profile", "Profile", icon_user())
            .badge(BottomNavBadge::Dot),
        BottomNavItem::new("settings", "Settings", icon_gear())
            .disabled(true),
    ];

    rsx! {
        DemoSection { title: "EqBottomNav",
            EqText {
                variant: TextVariant::Muted,
                "Tap any item to flip the active state. The disabled item ignores taps."
            }

            div { class: "rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                EqBottomNav {
                    items: items.clone(),
                    active: active(),
                    on_change: move |id| active.set(id),
                }
            }

            div { class: "text-xs text-[var(--color-label-secondary)]",
                "Active id: "
                span { class: "font-mono text-[var(--color-label-primary)]", "{active()}" }
            }

            EqText {
                variant: TextVariant::Caption,
                class: "font-semibold uppercase tracking-wider",
                "In the mobile frame",
            }

            div { class: "flex justify-center",
                EqDeviceFrame {
                    div { class: "h-full w-full flex flex-col",
                        div {
                            class: "flex-1 p-4 text-sm text-[var(--color-label-secondary)] overflow-y-auto",
                            "Body content fills the screen above the bottom nav. The nav stays anchored at the bottom regardless of scroll."
                            for i in 1..=10 {
                                div {
                                    class: "p-3 rounded-md border border-[var(--color-card-border)] bg-[var(--color-card)] mt-2",
                                    "Item {i}"
                                }
                            }
                        }
                        EqBottomNav {
                            items: items.clone(),
                            active: active(),
                            on_change: move |id| active.set(id),
                        }
                    }
                }
            }

            StyleInfo {
                file: "eq_bottom_nav_styles.rs",
                styles: format_catalog(&s::catalog()),
            }
            CodeBlock { code }
        }
    }
}

// ── Gallery ─────────────────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqBottomNav() -> Element {
    let active = use_signal(|| "home".to_string());

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Bottom Nav Gallery",
                }

                div { class: "rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                    EqBottomNav {
                        items: vec![
                            BottomNavItem::new("home", "Home", icon_home()),
                            BottomNavItem::new("inbox", "Inbox", icon_inbox())
                                .badge(BottomNavBadge::Count(7)),
                            BottomNavItem::new("profile", "Profile", icon_user()),
                        ],
                        active: active(),
                        on_change: move |_| {},
                    }
                }

                div { class: "rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                    EqBottomNav {
                        items: vec![
                            BottomNavItem::new("home", "Home", icon_home()),
                            BottomNavItem::new("inbox", "Inbox", icon_inbox())
                                .badge(BottomNavBadge::Dot),
                            BottomNavItem::new("favs", "Favorites", icon_star()),
                            BottomNavItem::new("settings", "Settings", icon_gear())
                                .disabled(true),
                        ],
                        active: active(),
                        on_change: move |_| {},
                    }
                }
            }
        }
    }
}

// ── Demo / gallery icons ────────────────────────────────────────────
//
// Inline SVGs so the demo doesn't require any consumer-side icon
// wiring. These render at the size of the `ITEM_ICON` container.

#[cfg(feature = "playground")]
fn icon_home() -> Element {
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
fn icon_inbox() -> Element {
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
fn icon_user() -> Element {
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

#[cfg(feature = "playground")]
fn icon_gear() -> Element {
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
            circle { cx: "12", cy: "12", r: "3" }
            path { d: "M19.4 15a1.7 1.7 0 0 0 .3 1.8l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.7 1.7 0 0 0-1.8-.3 1.7 1.7 0 0 0-1 1.5V21a2 2 0 0 1-4 0v-.1a1.7 1.7 0 0 0-1.1-1.5 1.7 1.7 0 0 0-1.8.3l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.7 1.7 0 0 0 .3-1.8 1.7 1.7 0 0 0-1.5-1H3a2 2 0 0 1 0-4h.1a1.7 1.7 0 0 0 1.5-1.1 1.7 1.7 0 0 0-.3-1.8l-.1-.1a2 2 0 1 1 2.8-2.8l.1.1a1.7 1.7 0 0 0 1.8.3h.1a1.7 1.7 0 0 0 1-1.5V3a2 2 0 0 1 4 0v.1a1.7 1.7 0 0 0 1 1.5 1.7 1.7 0 0 0 1.8-.3l.1-.1a2 2 0 1 1 2.8 2.8l-.1.1a1.7 1.7 0 0 0-.3 1.8v.1a1.7 1.7 0 0 0 1.5 1H21a2 2 0 0 1 0 4h-.1a1.7 1.7 0 0 0-1.5 1Z" }
        }
    }
}

#[cfg(feature = "playground")]
fn icon_star() -> Element {
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
            path { d: "M12 2 15.1 8.3 22 9.3l-5 4.9 1.2 6.9L12 17.8 5.8 21l1.2-6.9-5-4.9 6.9-1Z" }
        }
    }
}
