use super::eq_divider_styles as s;
use crate::theme::merge_classes;
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
#[derive(Clone, PartialEq, Default)]
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
#[derive(Clone, PartialEq, Default)]
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
#[derive(Clone, PartialEq, Default)]
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

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-divider",
        name: "EqDivider",
        category: ComponentCategory::Atom,
        description: "Horizontal divider with configurable style (solid/dashed/dotted/spacer), \
                      thickness, and spacing presets.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Basic solid divider",
                code: "EqDivider {}".into(),
            },
            UsageExample {
                label: "Dashed variant",
                code: "EqDivider { variant: DividerVariant::Dashed }".into(),
            },
            UsageExample {
                label: "Thick with wide spacing",
                code: "EqDivider {\n    variant: DividerVariant::Solid,\n    weight: DividerWeight::Thick,\n    spacing: DividerSpacing::Wide,\n}".into(),
            },
        ],
        render_demo: || rsx! { DemoEqDivider {} },
        render_gallery: || rsx! { GalleryEqDivider {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqDivider() -> Element {
    let mut variant_str = use_signal(|| "Solid".to_string());
    let mut weight_str = use_signal(|| "Normal".to_string());
    let mut spacing_str = use_signal(|| "Default".to_string());

    let variant = match variant_str().as_str() {
        "Dashed" => DividerVariant::Dashed,
        "Dotted" => DividerVariant::Dotted,
        "Spacer" => DividerVariant::Spacer,
        _ => DividerVariant::Solid,
    };
    let weight = match weight_str().as_str() {
        "Thick" => DividerWeight::Thick,
        "ExtraThick" => DividerWeight::ExtraThick,
        _ => DividerWeight::Normal,
    };
    let spacing = match spacing_str().as_str() {
        "Compact" => DividerSpacing::Compact,
        "Wide" => DividerSpacing::Wide,
        _ => DividerSpacing::Default,
    };

    let code = "EqDivider {}  // solid, normal weight\n\nEqDivider { variant: DividerVariant::Dashed }\n\nEqDivider {\n    variant: DividerVariant::Dashed,\n    weight: DividerWeight::Thick,\n}\n\nEqDivider {\n    variant: DividerVariant::Spacer,\n    spacing: DividerSpacing::Wide,\n}".to_string();

    rsx! {
        DemoSection { title: "EqDivider",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "variant",
                    value: variant_str(),
                    options: vec!["Solid", "Dashed", "Dotted", "Spacer"],
                    onchange: move |v: String| variant_str.set(v),
                }
                PropSelect {
                    label: "weight",
                    value: weight_str(),
                    options: vec!["Normal", "Thick", "ExtraThick"],
                    onchange: move |v: String| weight_str.set(v),
                }
                PropSelect {
                    label: "spacing",
                    value: spacing_str(),
                    options: vec!["Compact", "Default", "Wide"],
                    onchange: move |v: String| spacing_str.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 max-w-lg",
                div { class: "rounded-lg bg-[var(--color-card)]/30 p-2 text-sm text-[var(--color-label-secondary)]",
                    "Content above"
                }
                EqDivider { variant, weight, spacing }
                div { class: "rounded-lg bg-[var(--color-card)]/30 p-2 text-sm text-[var(--color-label-secondary)]",
                    "Content below"
                }
            }
            StyleInfo { file: "eq_divider_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqDivider() -> Element {
    rsx! {
        div { class: "space-y-4",
            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Variants" }
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4 max-w-lg",
                div { class: "space-y-2",
                    EqText { variant: TextVariant::Muted, class: "text-xs font-medium uppercase", "Solid" }
                    EqDivider { variant: DividerVariant::Solid }
                }
                div { class: "space-y-2",
                    EqText { variant: TextVariant::Muted, class: "text-xs font-medium uppercase", "Dashed" }
                    EqDivider { variant: DividerVariant::Dashed }
                }
                div { class: "space-y-2",
                    EqText { variant: TextVariant::Muted, class: "text-xs font-medium uppercase", "Dotted" }
                    EqDivider { variant: DividerVariant::Dotted }
                }
                div { class: "space-y-2",
                    EqText { variant: TextVariant::Muted, class: "text-xs font-medium uppercase", "Thick" }
                    EqDivider { variant: DividerVariant::Solid, weight: DividerWeight::Thick }
                }
                div { class: "space-y-2",
                    EqText { variant: TextVariant::Muted, class: "text-xs font-medium uppercase", "Extra Thick" }
                    EqDivider { variant: DividerVariant::Solid, weight: DividerWeight::ExtraThick }
                }
            }
        }
    }
}
