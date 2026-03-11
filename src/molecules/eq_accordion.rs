use super::eq_accordion_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

/// Controls whether multiple panels can be open simultaneously.
#[derive(Clone, PartialEq, Default)]
pub enum AccordionMode {
    /// Only one panel open at a time — opening a panel closes the others.
    #[default]
    Single,
    /// Multiple panels can be open simultaneously.
    Multi,
}

/// Describes a single panel inside the accordion.
#[derive(Clone, PartialEq)]
pub struct AccordionItem {
    /// Unique identifier for this panel.
    pub id: String,
    /// Header content (any element — text, icons, badges, etc.).
    pub header: Element,
    /// Body content revealed when the panel is expanded.
    pub body: Element,
}

impl AccordionItem {
    /// Shorthand constructor.
    pub fn new(
        id: impl Into<String>,
        header: Element,
        body: Element,
    ) -> Self {
        Self {
            id: id.into(),
            header,
            body,
        }
    }
}

/// Collapsible accordion molecule.
///
/// Renders a vertical stack of panels, each with a clickable header
/// that reveals or hides its body content with a smooth height transition.
///
/// Use `class` to extend or replace the default styles.
#[component]
pub fn EqAccordion(
    /// The panels to render.
    items: Vec<AccordionItem>,
    /// Expand behaviour.
    #[props(default)]
    mode: AccordionMode,
    /// Optional class override on the root container.
    #[props(into, default)]
    class: String,
) -> Element {
    // Track which panels are currently open by id.
    let mut open_ids = use_signal(Vec::<String>::new);

    let cls = merge_classes(s::ACCORDION, &class);

    rsx! {
        div { class: "{cls}",
            for item in items {
                {
                    let id = item.id.clone();
                    let is_open = open_ids().contains(&id);

                    let toggle_id = id.clone();
                    let toggle_mode = mode.clone();
                    let onclick = move |_| {
                        let mut current = open_ids();
                        if current.contains(&toggle_id) {
                            current.retain(|x| x != &toggle_id);
                        } else {
                            match toggle_mode {
                                AccordionMode::Single => {
                                    current.clear();
                                    current.push(toggle_id.clone());
                                }
                                AccordionMode::Multi => {
                                    current.push(toggle_id.clone());
                                }
                            }
                        }
                        open_ids.set(current);
                    };

                    let body_grid = if is_open {
                        format!("{} {}", s::BODY, s::BODY_OPEN)
                    } else {
                        format!("{} {}", s::BODY, s::BODY_CLOSED)
                    };
                    let chevron_cls = if is_open {
                        format!("{} {}", s::CHEVRON, s::CHEVRON_OPEN)
                    } else {
                        s::CHEVRON.to_string()
                    };

                    rsx! {
                        div {
                            key: "{id}",
                            class: s::PANEL,

                            // Header
                            button {
                                class: s::HEADER,
                                onclick: onclick,

                                div { class: s::HEADER_TEXT, {item.header} }

                                svg {
                                    class: "{chevron_cls}",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke_width: "2",
                                    stroke: "currentColor",
                                    path { d: "m19.5 8.25-7.5 7.5-7.5-7.5" }
                                }
                            }

                            // Body — animated via CSS grid rows
                            div {
                                class: "{body_grid}",
                                div { class: s::BODY_INNER,
                                    div { class: s::CONTENT, {item.body} }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
