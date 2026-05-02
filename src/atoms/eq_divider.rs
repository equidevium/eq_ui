use super::eq_divider_styles as s;
use crate::theme::merge_classes;
use crate::{PlaygroundEnum, playground};
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Visual style of the divider line.
#[derive(Clone, PartialEq, Default, PlaygroundEnum)]
pub enum DividerVariant {
    /// Solid continuous line (default).
    #[default]
    Solid,
    /// Dashed line (- - -).
    Dashed,
    /// Dotted line.
    Dotted,
    /// Invisible spacer - adds vertical space with no visible line.
    Spacer,
}

/// Thickness of the divider.
#[derive(Clone, PartialEq, Default, PlaygroundEnum)]
pub enum DividerWeight {
    /// Default thickness (1px).
    #[default]
    Normal,
    /// Thick (2px).
    Thick,
    /// Extra thick (4px).
    ExtraThick,
}

/// Vertical spacing around the divider.
#[derive(Clone, PartialEq, Default, PlaygroundEnum)]
pub enum DividerSpacing {
    /// Compact spacing (my-2).
    Compact,
    /// Default spacing (my-4).
    #[default]
    Default,
    /// Wide spacing (my-8).
    Wide,
}

/// A horizontal divider atom.
///
/// Renders an `<hr>` element with configurable style, thickness, and spacing.
#[playground(
    category = Atom,
    description = "Horizontal divider with configurable style (solid/dashed/dotted/spacer), \
                   thickness, and spacing presets.",
    examples = [
        ("Basic solid divider", "EqDivider {}"),
        ("Dashed variant", "EqDivider { variant: DividerVariant::Dashed }"),
        ("Thick with wide spacing", "EqDivider {\n    variant: DividerVariant::Solid,\n    weight: DividerWeight::Thick,\n    spacing: DividerSpacing::Wide,\n}"),
    ],
)]
#[component]
pub fn EqDivider(
    /// Visual style of the line.
    #[props(default)]
    variant: DividerVariant,
    /// Line thickness.
    #[props(default)]
    weight: DividerWeight,
    /// Vertical spacing around the divider.
    #[props(default)]
    spacing: DividerSpacing,
    /// Optional class override - extend or replace default styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let spacing_class = match spacing {
        DividerSpacing::Compact => s::SPACING_COMPACT,
        DividerSpacing::Default => s::SPACING_DEFAULT,
        DividerSpacing::Wide => s::SPACING_WIDE,
    };

    // Spacer is a special case - invisible, just spacing
    if variant == DividerVariant::Spacer {
        let base = format!("{} {}", s::SPACER, spacing_class);
        let cls = merge_classes(&base, &class);
        return rsx! {
            hr {
                class: "{cls}",
                role: "separator",
                "aria-orientation": "horizontal",
            }
        };
    }

    let variant_class = match variant {
        DividerVariant::Solid => "",
        DividerVariant::Dashed => s::DASHED,
        DividerVariant::Dotted => s::DOTTED,
        DividerVariant::Spacer => unreachable!(),
    };

    let weight_class = match weight {
        DividerWeight::Normal => "",
        DividerWeight::Thick => s::THICK,
        DividerWeight::ExtraThick => s::EXTRA_THICK,
    };

    let base = format!("{} {} {} {}", s::BASE, variant_class, weight_class, spacing_class);
    let cls = merge_classes(&base, &class);

    rsx! {
        hr {
            class: "{cls}",
            role: "separator",
            "aria-orientation": "horizontal",
        }
    }
}
