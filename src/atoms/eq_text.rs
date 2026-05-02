use dioxus::prelude::*;
use super::eq_text_styles as s;
use crate::theme::merge_classes;
use crate::{PlaygroundEnum, playground};

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropInput, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Text variant - determines the HTML element and style applied.
#[derive(Clone, PartialEq, Default, PlaygroundEnum)]
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
///
/// Use `class` to extend or replace the default styles .
#[playground(
    category = Atom,
    description = "Semantic text component with variants for headings, body, muted, \
                   caption, emphasis, and monospace text.",
    examples = [
        ("Heading", "EqText { variant: TextVariant::H1, \"Heading 1\" }"),
        ("Body text", "EqText { variant: TextVariant::Body, \"Body text - the default variant.\" }"),
        ("Muted", "EqText { variant: TextVariant::Muted, \"Muted - secondary colour.\" }"),
    ],
)]
#[component]
pub fn EqText(
    #[props(default)]
    variant: TextVariant,
    #[props(into, default)]
    class: String,
    children: Element,
) -> Element {
    let base = match &variant {
        TextVariant::H1       => s::H1,
        TextVariant::H2       => s::H2,
        TextVariant::H3       => s::H3,
        TextVariant::Body     => s::BODY,
        TextVariant::Muted    => s::MUTED,
        TextVariant::Caption  => s::CAPTION,
        TextVariant::Emphasis => s::EMPHASIS,
        TextVariant::Mono     => s::MONO,
    };
    let cls = merge_classes(base, &class);

    match variant {
        TextVariant::H1       => rsx! { h1     { class: "{cls}", {children} } },
        TextVariant::H2       => rsx! { h2     { class: "{cls}", {children} } },
        TextVariant::H3       => rsx! { h3     { class: "{cls}", {children} } },
        TextVariant::Body     => rsx! { p      { class: "{cls}", {children} } },
        TextVariant::Muted    => rsx! { p      { class: "{cls}", {children} } },
        TextVariant::Caption  => rsx! { span   { class: "{cls}", {children} } },
        TextVariant::Emphasis => rsx! { strong { class: "{cls}", {children} } },
        TextVariant::Mono     => rsx! { code   { class: "{cls}", {children} } },
    }
}
