use dioxus::prelude::*;
use super::eq_text_styles as s;

/// Text variant — determines the HTML element and style applied.
#[derive(Clone, PartialEq, Default)]
pub enum TextVariant {
    H1,
    H2,
    H3,
    #[default]
    Body,
    Muted,
    Caption,
    Emphasis,
    Mono,
}

/// Atomic text component.
/// Renders the correct semantic HTML element based on the chosen variant.
#[component]
pub fn EqText(
    #[props(default)]
    variant: TextVariant,
    children: Element,
) -> Element {
    match variant {
        TextVariant::H1 => rsx! { h1 { class: s::H1, {children} } },
        TextVariant::H2 => rsx! { h2 { class: s::H2, {children} } },
        TextVariant::H3 => rsx! { h3 { class: s::H3, {children} } },
        TextVariant::Body => rsx! { p { class: s::BODY, {children} } },
        TextVariant::Muted => rsx! { p { class: s::MUTED, {children} } },
        TextVariant::Caption => rsx! { span { class: s::CAPTION, {children} } },
        TextVariant::Emphasis => rsx! { strong { class: s::EMPHASIS, {children} } },
        TextVariant::Mono => rsx! { code { class: s::MONO, {children} } },
    }
}
