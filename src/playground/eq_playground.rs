//! EqPlayground - reusable interactive component showcase organism.
//!
//! Takes a `Vec<ComponentDescriptor>` and renders a two-panel playground
//! with a sidebar tree (grouped by category) and a live demo panel.
//! Includes built-in theme switching across all 21 themes.
//!
//! ```rust,ignore
//! use eq_ui::{all_component_descriptors, EqPlayground};
//!
//! fn App() -> Element {
//!     rsx! {
//!         EqPlayground {
//!             descriptors: all_component_descriptors(),
//!         }
//!     }
//! }
//! ```
//!
//! External users can append their own descriptors:
//!
//! ```rust,ignore
//! let mut descs = all_component_descriptors();
//! descs.push(my_custom_component::descriptor());
//! rsx! { EqPlayground { descriptors: descs } }
//! ```

use super::eq_playground_styles as s;
use crate::atoms::{EqScrollableSpace, EqText, TextVariant};
use crate::eq_theme::EqTheme;
use crate::molecules::TreeNode;
use crate::organisms::{EqAppShell, EqFooter, EqHeader};
use crate::playground::playground_types::{ComponentCategory, ComponentDescriptor};
use crate::{UI_TAILWIND_CSS, UI_INDEX_CSS, UI_BUTTONS_CSS};
use dioxus::prelude::*;

// ── ThemeSwitcher (internal) ───────────────────────────────────────

/// Dropdown to switch between all built-in themes at runtime.
#[component]
fn ThemeSwitcher() -> Element {
    let mut theme = EqTheme::use_theme();

    rsx! {
        select {
            class: s::THEME_SELECT,
            value: format!("{:?}", *theme.read()),
            onchange: move |evt: Event<FormData>| {
                let new_theme = match evt.value().as_str() {
                    "Unghosty" => EqTheme::Unghosty,
                    "Burgundy" => EqTheme::Burgundy,
                    "Gold" => EqTheme::Gold,
                    "PurplePink" => EqTheme::PurplePink,
                    "Monochrome" => EqTheme::Monochrome,
                    "Watermelon" => EqTheme::Watermelon,
                    "Sunset" => EqTheme::Sunset,
                    "Ocean" => EqTheme::Ocean,
                    "Spacetime" => EqTheme::Spacetime,
                    "Gruvbox" => EqTheme::Gruvbox,
                    "Monokai" => EqTheme::Monokai,
                    "Hellas" => EqTheme::Hellas,
                    "Egypt" => EqTheme::Egypt,
                    "Dometrain" => EqTheme::Dometrain,
                    "Catppuccin" => EqTheme::Catppuccin,
                    "Dracula" => EqTheme::Dracula,
                    "Nord" => EqTheme::Nord,
                    "OneDark" => EqTheme::OneDark,
                    "RosePine" => EqTheme::RosePine,
                    "SolarizedDark" => EqTheme::SolarizedDark,
                    "TokyoNight" => EqTheme::TokyoNight,
                    _ => EqTheme::Unghosty,
                };
                theme.set(new_theme);
            },

            for (name, _variant) in EqTheme::build_in_variants() {
                option { value: "{name}", "{name}" }
            }
        }
    }
}

// ── EqThemeRenderer (internal) ─────────────────────────────────────

/// Injects the active theme's CSS as an inline `<style>` element.
#[component]
fn EqThemeRenderer() -> Element {
    let theme = EqTheme::use_theme();

    let (key, css) = match &*theme.read() {
        EqTheme::Custom(css) => ("custom".to_string(), css.clone()),
        other => {
            let name = format!("{:?}", other);
            let content = other.css_content().unwrap_or("").to_string();
            (name, content)
        }
    };

    rsx! {
        style { key: "{key}", dangerous_inner_html: "{css}" }
    }
}

// ── Tree builder ───────────────────────────────────────────────────

/// Builds a `Vec<TreeNode>` from descriptors, grouped by category.
fn build_tree_from_descriptors(descriptors: &[ComponentDescriptor]) -> Vec<TreeNode> {
    // Collect categories in order
    let mut categories: Vec<ComponentCategory> = Vec::new();
    for d in descriptors {
        if !categories.contains(&d.category) {
            categories.push(d.category);
        }
    }
    categories.sort_by_key(|c| c.sort_order());

    categories
        .into_iter()
        .map(|cat| {
            let children: Vec<TreeNode> = descriptors
                .iter()
                .filter(|d| d.category == cat)
                .map(|d| TreeNode::new(d.id, d.name))
                .collect();
            TreeNode::new_with_children(
                // Use category label as branch ID (lowercase)
                match cat {
                    ComponentCategory::Guide => "guide",
                    ComponentCategory::Atom => "atoms",
                    ComponentCategory::Molecule => "molecules",
                    ComponentCategory::Organism => "organisms",
                    ComponentCategory::Theming => "theming",
                },
                cat.label(),
                children,
            )
        })
        .collect()
}

// ── Preview panel ──────────────────────────────────────────────────

/// Routes the selected tree node ID to the matching descriptor's demo.
#[component]
fn PreviewPanel(
    selected: Option<String>,
    descriptors: Vec<ComponentDescriptor>,
) -> Element {
    if let Some(ref id) = selected {
        // Find the descriptor with a matching id
        if let Some(desc) = descriptors.iter().find(|d| d.id == id.as_str()) {
            return (desc.render_demo)();
        }
    }

    // Empty state - no selection or ID not found
    rsx! {
        div { class: s::EMPTY_STATE,
            svg {
                class: s::EMPTY_ICON,
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "1.5",
                stroke: "currentColor",
                path { d: "M3.75 6A2.25 2.25 0 0 1 6 3.75h2.25A2.25 2.25 0 0 1 10.5 6v2.25a2.25 2.25 0 0 1-2.25 2.25H6a2.25 2.25 0 0 1-2.25-2.25V6ZM3.75 15.75A2.25 2.25 0 0 1 6 13.5h2.25a2.25 2.25 0 0 1 2.25 2.25V18a2.25 2.25 0 0 1-2.25 2.25H6A2.25 2.25 0 0 1 3.75 18v-2.25ZM13.5 6a2.25 2.25 0 0 1 2.25-2.25H18A2.25 2.25 0 0 1 20.25 6v2.25A2.25 2.25 0 0 1 18 10.5h-2.25a2.25 2.25 0 0 1-2.25-2.25V6ZM13.5 15.75a2.25 2.25 0 0 1 2.25-2.25H18a2.25 2.25 0 0 1 2.25 2.25V18A2.25 2.25 0 0 1 18 20.25h-2.25a2.25 2.25 0 0 1-2.25-2.25v-2.25Z" }
            }
            EqText { variant: TextVariant::Muted, "Select a component from the sidebar" }
        }
    }
}

// ── EqPlayground ───────────────────────────────────────────────────

/// Reusable interactive playground organism.
///
/// Renders a full-page component showcase with:
/// - Sidebar tree grouped by category (Atoms / Molecules / Organisms)
/// - Live demo panel with interactive prop controls
/// - Theme switcher in the header
/// - Responsive mobile sidebar with hamburger toggle
///
/// # Props
///
/// - `descriptors` - collected via `all_component_descriptors()`, optionally
///   extended with custom entries.
/// - `site_title` - branding text in the header (defaults to "EqPlayground").
/// - `copyright_holder` - footer copyright line.
#[component]
pub fn EqPlayground(
    /// Component descriptors to render in the playground.
    descriptors: Vec<ComponentDescriptor>,
    /// Header brand text.
    #[props(default = "EqPlayground")]
    site_title: &'static str,
    /// Footer copyright holder.
    #[props(default = "Equidevium")]
    copyright_holder: &'static str,
) -> Element {
    let _theme = EqTheme::use_theme_provider();
    let mut selected = use_signal(|| Option::<String>::None);
    let mut sidebar_open = use_signal(|| false);

    let tree = build_tree_from_descriptors(&descriptors);

    rsx! {
        document::Link { rel: "stylesheet", href: UI_TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: UI_INDEX_CSS }
        document::Link { rel: "stylesheet", href: UI_BUTTONS_CSS }

        EqThemeRenderer {}

        EqAppShell {
            header: rsx! {
                EqHeader {
                    site_title: site_title,
                    nav: rsx! {
                        // Hamburger button - mobile only
                        li { class: s::HAMBURGER,
                            button {
                                class: s::HAMBURGER_BTN,
                                onclick: move |_| sidebar_open.set(!sidebar_open()),
                                svg {
                                    class: s::HAMBURGER_ICON,
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke_width: "2",
                                    stroke: "currentColor",
                                    if sidebar_open() {
                                        path { d: "M6 18 18 6M6 6l12 12" }
                                    } else {
                                        path { d: "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" }
                                    }
                                }
                            }
                        }
                        li { ThemeSwitcher {} }
                    },
                }
            },
            footer: rsx! {
                EqFooter { copyright_holder: copyright_holder }
            },

            // Two-panel layout
            div { class: s::LAYOUT,

                // Mobile overlay backdrop
                if sidebar_open() {
                    div {
                        class: s::MOBILE_BACKDROP,
                        onclick: move |_| sidebar_open.set(false),
                    }
                }

                // Sidebar
                aside {
                    class: if sidebar_open() { s::SIDEBAR_MOBILE_OPEN } else { s::SIDEBAR_MOBILE_CLOSED },
                    EqScrollableSpace {
                        crate::molecules::EqTree {
                            nodes: tree,
                            selected: selected(),
                            on_select: move |id: String| {
                                selected.set(Some(id));
                                sidebar_open.set(false);
                            },
                            show_count: true,
                        }
                    }
                }

                // Right preview panel
                div { class: s::PREVIEW_PANEL,
                    PreviewPanel {
                        selected: selected(),
                        descriptors: descriptors,
                    }
                }
            }
        }
    }
}
