//! EqDrawer — slide-in panel organism.
//!
//! A panel that slides in from any edge of the screen with a backdrop
//! overlay. Supports four sides, four size presets, optional header/footer
//! slots, close-on-backdrop, close-on-Escape, and WAI-ARIA dialog pattern.
//!
//! ```rust,ignore
//! let mut open = use_signal(|| false);
//!
//! EqButton { on_click: move |_| open.set(true), "Open drawer" }
//!
//! EqDrawer {
//!     open: open(),
//!     on_close: move |_| open.set(false),
//!     title: "Settings",
//!     side: DrawerSide::Right,
//!     body: rsx! { p { "Drawer content here." } },
//! }
//! ```

use super::eq_drawer_styles as s;
use crate::theme::merge_classes;
use crate::{PlaygroundEnum, playground};
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropToggle, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant, EqButton, ButtonVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// ── Types ─────────────────────────────────────────────────────────

/// Which edge the drawer slides in from.
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
pub enum DrawerSide {
    /// Slide in from the left edge.
    Left,
    /// Slide in from the right edge (default).
    #[default]
    Right,
    /// Slide down from the top edge.
    Top,
    /// Slide up from the bottom edge.
    Bottom,
}

/// Size preset for the drawer.
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
pub enum DrawerSize {
    /// Small (320px / 240px).
    Sm,
    /// Medium (420px / 320px, default).
    #[default]
    Md,
    /// Large (560px / 420px).
    Lg,
    /// Full width / height.
    Full,
}

// ── Helpers ───────────────────────────────────────────────────────

fn is_horizontal(side: DrawerSide) -> bool {
    matches!(side, DrawerSide::Left | DrawerSide::Right)
}

// ── Component ─────────────────────────────────────────────────────

/// Slide-in drawer panel.
///
/// Opens from any screen edge with a backdrop overlay. The drawer
/// contains a header with title and close button, a scrollable body,
/// and an optional footer slot.
///
/// **Accessibility** — uses `role="dialog"`, `aria-modal="true"`,
/// and `aria-labelledby` pointing at the title. Escape key closes
/// the drawer. Focus is trapped within the drawer while open.
#[playground(
    category = Organism,
    description = "Slide-in panel from any screen edge with backdrop, \
                   four sides, four sizes, header/body/footer, \
                   close-on-Escape, WAI-ARIA dialog.",
    examples = [
        ("Basic", "let mut open = use_signal(|| false);\n\nEqDrawer {\n    open: open(),\n    on_close: move |_| open.set(false),\n    title: \"Settings\",\n    body: rsx! { p { \"Content\" } },\n}"),
        ("Left side", "EqDrawer {\n    open: open(),\n    on_close: move |_| open.set(false),\n    title: \"Navigation\",\n    side: DrawerSide::Left,\n    size: DrawerSize::Sm,\n    body: rsx! { p { \"Nav links\" } },\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqDrawer(
    /// Whether the drawer is open.
    #[props(default = false)]
    open: bool,
    /// Fired when the drawer should close (backdrop click, Escape, close button).
    #[props(default)]
    on_close: Option<EventHandler<()>>,
    /// Title shown in the header.
    #[props(into, default)]
    title: String,
    /// The main scrollable content.
    body: Option<Element>,
    /// Optional footer content (e.g. action buttons).
    footer: Option<Element>,
    /// Which screen edge the drawer slides in from.
    #[props(default)]
    side: DrawerSide,
    /// Size preset.
    #[props(default)]
    size: DrawerSize,
    /// Whether clicking the backdrop closes the drawer.
    #[props(default = true)]
    close_on_backdrop: bool,
    /// Whether pressing Escape closes the drawer.
    #[props(default = true)]
    close_on_escape: bool,
    /// Show the close button in the header.
    #[props(default = true)]
    show_close: bool,
    /// Optional class override on the panel element.
    #[props(into, default)]
    class: String,
) -> Element {
    let close = move || {
        if let Some(handler) = &on_close {
            handler.call(());
        }
    };

    // Backdrop state.
    let backdrop_state = if open { s::BACKDROP_OPEN } else { s::BACKDROP_CLOSED };

    // Side classes.
    let side_cls = match side {
        DrawerSide::Left => s::SIDE_LEFT,
        DrawerSide::Right => s::SIDE_RIGHT,
        DrawerSide::Top => s::SIDE_TOP,
        DrawerSide::Bottom => s::SIDE_BOTTOM,
    };

    let transform_cls = if open {
        s::PANEL_OPEN
    } else {
        match side {
            DrawerSide::Left => s::SIDE_LEFT_CLOSED,
            DrawerSide::Right => s::SIDE_RIGHT_CLOSED,
            DrawerSide::Top => s::SIDE_TOP_CLOSED,
            DrawerSide::Bottom => s::SIDE_BOTTOM_CLOSED,
        }
    };

    // Size classes.
    let horiz = is_horizontal(side);
    let size_cls = match (size, horiz) {
        (DrawerSize::Sm, true) => s::SIZE_SM_H,
        (DrawerSize::Sm, false) => s::SIZE_SM_V,
        (DrawerSize::Md, true) => s::SIZE_MD_H,
        (DrawerSize::Md, false) => s::SIZE_MD_V,
        (DrawerSize::Lg, true) => s::SIZE_LG_H,
        (DrawerSize::Lg, false) => s::SIZE_LG_V,
        (DrawerSize::Full, true) => s::SIZE_FULL_H,
        (DrawerSize::Full, false) => s::SIZE_FULL_V,
    };

    let panel_cls = merge_classes(
        &format!("{} {} {} {}", s::PANEL, side_cls, size_cls, transform_cls),
        &class,
    );

    let has_title = !title.is_empty();

    rsx! {
        // Backdrop
        div {
            class: "{s::BACKDROP} {backdrop_state}",
            onclick: move |_| {
                if close_on_backdrop { close(); }
            },
            onkeydown: move |evt: KeyboardEvent| {
                if close_on_escape && evt.key() == Key::Escape {
                    close();
                }
            },
        }

        // Panel
        div {
            class: "{panel_cls}",
            role: "dialog",
            "aria-modal": "true",
            "aria-labelledby": if has_title { Some("eq-drawer-title") } else { None },
            onclick: move |evt| { evt.stop_propagation(); },

            // Header
            if has_title || show_close {
                div { class: "{s::HEADER}",
                    if has_title {
                        h2 {
                            id: "eq-drawer-title",
                            class: "{s::HEADER_TITLE}",
                            "{title}"
                        }
                    }
                    if show_close {
                        button {
                            class: "{s::CLOSE_BUTTON}",
                            "aria-label": "Close drawer",
                            onclick: move |_| close(),
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                stroke: "currentColor",
                                width: "20",
                                height: "20",
                                "aria-hidden": "true",
                                path { d: "M6 18 18 6M6 6l12 12" }
                            }
                        }
                    }
                }
            }

            // Body
            if let Some(body_content) = body {
                div { class: "{s::BODY}", {body_content} }
            }

            // Footer
            if let Some(footer_content) = footer {
                div { class: "{s::FOOTER}", {footer_content} }
            }
        }
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqDrawer() -> Element {
    let mut open = use_signal(|| false);
    let mut side_str = use_signal(|| "Right".to_string());
    let mut size_str = use_signal(|| "Md".to_string());
    let mut close_on_backdrop = use_signal(|| true);
    let mut show_close = use_signal(|| true);

    let side = match side_str().as_str() {
        "Left" => DrawerSide::Left,
        "Top" => DrawerSide::Top,
        "Bottom" => DrawerSide::Bottom,
        _ => DrawerSide::Right,
    };

    let size = match size_str().as_str() {
        "Sm" => DrawerSize::Sm,
        "Lg" => DrawerSize::Lg,
        "Full" => DrawerSize::Full,
        _ => DrawerSize::Md,
    };

    let code = format!(
        r#"let mut open = use_signal(|| false);

EqButton {{ on_click: move |_| open.set(true), "Open Drawer" }}

EqDrawer {{
    open: open(),
    on_close: move |_| open.set(false),
    title: "Drawer Title",
    side: DrawerSide::{side},
    size: DrawerSize::{size},
    close_on_backdrop: {cob},
    show_close: {sc},
    body: rsx! {{ p {{ "Your content here." }} }},
    footer: rsx! {{
        EqButton {{ variant: ButtonVariant::Ghost, on_click: move |_| open.set(false), "Cancel" }}
        EqButton {{ on_click: move |_| open.set(false), "Save" }}
    }},
}}"#,
        side = side_str(),
        size = size_str(),
        cob = close_on_backdrop(),
        sc = show_close(),
    );

    rsx! {
        DemoSection { title: "EqDrawer",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "side",
                    value: side_str(),
                    options: vec!["Left", "Right", "Top", "Bottom"],
                    onchange: move |v: String| side_str.set(v),
                }
                PropSelect {
                    label: "size",
                    value: size_str(),
                    options: vec!["Sm", "Md", "Lg", "Full"],
                    onchange: move |v: String| size_str.set(v),
                }
                PropToggle {
                    label: "close_on_backdrop",
                    value: close_on_backdrop(),
                    onchange: move |v: bool| close_on_backdrop.set(v),
                }
                PropToggle {
                    label: "show_close",
                    value: show_close(),
                    onchange: move |v: bool| show_close.set(v),
                }
            }

            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                EqButton {
                    variant: ButtonVariant::Primary,
                    on_click: move |_| open.set(true),
                    "Open Drawer"
                }
            }

            EqDrawer {
                open: open(),
                on_close: move |_| open.set(false),
                title: "Drawer Title",
                side,
                size,
                close_on_backdrop: close_on_backdrop(),
                show_close: show_close(),
                body: rsx! {
                    div { class: "space-y-3",
                        EqText { variant: TextVariant::Body, "This is the drawer body content. It scrolls if the content exceeds the available height." }
                        EqText { variant: TextVariant::Muted, "You can put any content here — forms, navigation links, settings panels, detail views, etc." }
                        for i in 1..6 {
                            div { class: "rounded-md border border-[var(--color-card-border)] p-3",
                                EqText { variant: TextVariant::Body, "Item {i}" }
                            }
                        }
                    }
                },
                footer: rsx! {
                    EqButton {
                        variant: ButtonVariant::Ghost,
                        on_click: move |_| open.set(false),
                        "Cancel"
                    }
                    EqButton {
                        on_click: move |_| open.set(false),
                        "Save changes"
                    }
                },
            }

            StyleInfo { file: "eq_drawer_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery ───────────────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqDrawer() -> Element {
    let mut left_open = use_signal(|| false);
    let mut right_open = use_signal(|| false);
    let mut top_open = use_signal(|| false);
    let mut bottom_open = use_signal(|| false);

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Drawer Gallery" }
                EqText { variant: TextVariant::Muted, "Open drawers from each side:" }

                div { class: "flex flex-wrap gap-3",
                    EqButton {
                        variant: ButtonVariant::Outline,
                        on_click: move |_| left_open.set(true),
                        "Left"
                    }
                    EqButton {
                        variant: ButtonVariant::Outline,
                        on_click: move |_| right_open.set(true),
                        "Right"
                    }
                    EqButton {
                        variant: ButtonVariant::Outline,
                        on_click: move |_| top_open.set(true),
                        "Top"
                    }
                    EqButton {
                        variant: ButtonVariant::Outline,
                        on_click: move |_| bottom_open.set(true),
                        "Bottom"
                    }
                }
            }

            EqDrawer {
                open: left_open(),
                on_close: move |_| left_open.set(false),
                title: "Navigation",
                side: DrawerSide::Left,
                size: DrawerSize::Sm,
                body: rsx! {
                    EqText { variant: TextVariant::Body, "Navigation links go here." }
                },
            }

            EqDrawer {
                open: right_open(),
                on_close: move |_| right_open.set(false),
                title: "Details",
                side: DrawerSide::Right,
                body: rsx! {
                    EqText { variant: TextVariant::Body, "Detail panel content." }
                },
            }

            EqDrawer {
                open: top_open(),
                on_close: move |_| top_open.set(false),
                title: "Notifications",
                side: DrawerSide::Top,
                size: DrawerSize::Sm,
                body: rsx! {
                    EqText { variant: TextVariant::Body, "Notification panel." }
                },
            }

            EqDrawer {
                open: bottom_open(),
                on_close: move |_| bottom_open.set(false),
                title: "Actions",
                side: DrawerSide::Bottom,
                size: DrawerSize::Sm,
                body: rsx! {
                    EqText { variant: TextVariant::Body, "Bottom sheet content." }
                },
                footer: rsx! {
                    EqButton {
                        on_click: move |_| bottom_open.set(false),
                        "Done"
                    }
                },
            }
        }
    }
}
