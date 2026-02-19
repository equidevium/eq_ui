use dioxus::prelude::*;
use super::eq_card_styles::*;

#[component]
pub fn EqCard(children: Element) -> Element {
    rsx! {
        div {
            class: CARD,
            {children}
        }
    }
}

#[component]
pub fn EqCardHeader(children: Element) -> Element {
    rsx! {
        div {
            class: CARD_HEADER,
            {children}
        }
    }
}

#[component]
pub fn EqCardBody(children: Element) -> Element {
    rsx! {
        div {
            class: CARD_BODY,
            {children}
        }
    }
}

#[component]
pub fn EqCardFooter(children: Element) -> Element {
    rsx! {
        div {
            class: CARD_FOOTER,
            {children}
        }
    }
}
