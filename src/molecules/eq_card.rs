use dioxus::prelude::*;
use super::eq_card_styles::*;
use crate::theme::merge_classes;

/// Card container molecule.
///
/// Use `class` to extend or replace the default styles .
#[component]
pub fn EqCard(
    /// Optional class override — extend or replace default styles.
    #[props(into, default)]
    class: String,
    children: Element,
) -> Element {
    let cls = merge_classes(CARD, &class);
    rsx! {
        div {
            class: "{cls}",
            {children}
        }
    }
}

/// Card header section.
///
/// Use `class` to extend or replace the default styles .
#[component]
pub fn EqCardHeader(
    #[props(into, default)]
    class: String,
    children: Element,
) -> Element {
    let cls = merge_classes(CARD_HEADER, &class);
    rsx! {
        div {
            class: "{cls}",
            {children}
        }
    }
}

/// Card body section.
///
/// Use `class` to extend or replace the default styles .
#[component]
pub fn EqCardBody(
    #[props(into, default)]
    class: String,
    children: Element,
) -> Element {
    let cls = merge_classes(CARD_BODY, &class);
    rsx! {
        div {
            class: "{cls}",
            {children}
        }
    }
}

/// Card footer section.
///
/// Use `class` to extend or replace the default styles .
#[component]
pub fn EqCardFooter(
    #[props(into, default)]
    class: String,
    children: Element,
) -> Element {
    let cls = merge_classes(CARD_FOOTER, &class);
    rsx! {
        div {
            class: "{cls}",
            {children}
        }
    }
}
