//! EqCta — call-to-action banner molecule.
//!
//! A prominent banner with title, optional description, and an action
//! slot (typically an `EqButton`). Two layout modes: `Inline` places
//! the action beside the text on wider viewports; `Centered` stacks
//! everything vertically with centred alignment.
//!
//! ```rust,ignore
//! EqCta {
//!     title: "Ready to get started?",
//!     description: "Sign up in seconds.",
//!     action: rsx! { EqButton { "Get Started" } },
//! }
//! ```

use super::eq_cta_styles as s;
use crate::theme::merge_classes;
use crate::{PlaygroundEnum, playground};
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropInput, PropSelect, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant, EqButton, ButtonVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Layout mode for the CTA banner.
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
pub enum CtaLayout {
    /// Title/description on the left, action on the right (responsive).
    #[default]
    Inline,
    /// Everything centred and stacked vertically.
    Centered,
}

/// Call-to-action banner molecule.
///
/// Renders a prominent section with a title, optional description, and
/// an action slot. Designed for landing pages, section footers, or
/// anywhere you need to drive a user towards an action.
#[playground(
    category = Molecule,
    description = "Call-to-action banner with title, description, and action slot. \
                   Supports inline (side-by-side) and centred layouts.",
    examples = [
        ("Inline", "EqCta {\n    title: \"Ready to get started?\",\n    description: \"Sign up in seconds.\",\n    action: rsx! { EqButton { \"Get Started\" } },\n}"),
        ("Centered", "EqCta {\n    title: \"Join the community\",\n    layout: CtaLayout::Centered,\n    action: rsx! { EqButton { \"Join Now\" } },\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqCta(
    /// Main heading text.
    #[props(into)]
    title: String,
    /// Optional supporting text below the title.
    #[props(into, default)]
    description: String,
    /// Action slot — typically an `EqButton` or link.
    action: Option<Element>,
    /// Layout mode.
    #[props(default)]
    layout: CtaLayout,
    /// Accessible label for screen readers. When set, the CTA becomes
    /// a named region (e.g. "Sign up, region").
    #[props(into, default)]
    aria_label: String,
    /// Optional class override — extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let has_label = !aria_label.is_empty();
    let has_description = !description.is_empty();

    let (wrapper_cls, action_cls) = match layout {
        CtaLayout::Inline => (s::CTA, s::ACTION),
        CtaLayout::Centered => (s::CTA_CENTERED, s::ACTION_CENTERED),
    };

    let cls = merge_classes(wrapper_cls, &class);

    rsx! {
        div {
            class: "{cls}",
            role: if has_label { "region" } else { "" },
            "aria-label": if has_label { "{aria_label}" } else { "" },

            div { class: "{s::TEXT_GROUP}",
                p { class: "{s::TITLE}", "{title}" }
                if has_description {
                    p { class: "{s::DESCRIPTION}", "{description}" }
                }
            }

            if let Some(action_content) = action {
                div { class: "{action_cls}",
                    {action_content}
                }
            }
        }
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqCta() -> Element {
    let mut title = use_signal(|| "Ready to get started?".to_string());
    let mut description =
        use_signal(|| "Create your first project in under a minute.".to_string());
    let mut layout_str = use_signal(|| "Inline".to_string());

    let layout = match layout_str().as_str() {
        "Centered" => CtaLayout::Centered,
        _ => CtaLayout::Inline,
    };

    let code = r#"use eq_ui::atoms::{EqButton, ButtonVariant};
use eq_ui::molecules::{EqCta, CtaLayout};

// Inline (default) — text left, action right
EqCta {
    title: "Ready to get started?",
    description: "Create your first project in under a minute.",
    action: rsx! {
        EqButton { variant: ButtonVariant::Primary, "Get Started" }
    },
}

// Centered
EqCta {
    title: "Join the community",
    layout: CtaLayout::Centered,
    action: rsx! {
        EqButton { variant: ButtonVariant::Primary, "Join Now" }
    },
}"#
    .to_string();

    rsx! {
        DemoSection { title: "EqCta",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "layout",
                    value: layout_str(),
                    options: vec!["Inline", "Centered"],
                    onchange: move |v: String| layout_str.set(v),
                }
                PropInput {
                    label: "title",
                    value: title(),
                    placeholder: "CTA title",
                    onchange: move |v: String| title.set(v),
                }
                PropInput {
                    label: "description",
                    value: description(),
                    placeholder: "Supporting text",
                    onchange: move |v: String| description.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                EqCta {
                    title: title(),
                    description: description(),
                    layout,
                    action: rsx! {
                        EqButton { variant: ButtonVariant::Primary, "Get Started" }
                    },
                }
            }
            StyleInfo { file: "eq_cta_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqCta() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Layout Modes" }

                div { class: "space-y-3",
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Inline (default)" }
                        EqCta {
                            title: "Upgrade your plan",
                            description: "Unlock advanced features and priority support.",
                            action: rsx! {
                                EqButton { variant: ButtonVariant::Primary, "Upgrade" }
                            },
                        }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Centered" }
                        EqCta {
                            title: "Join the community",
                            description: "Connect with other developers.",
                            layout: CtaLayout::Centered,
                            action: rsx! {
                                EqButton { variant: ButtonVariant::Primary, "Join Now" }
                            },
                        }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "No description" }
                        EqCta {
                            title: "Subscribe to our newsletter",
                            action: rsx! {
                                EqButton { variant: ButtonVariant::Outline, "Subscribe" }
                            },
                        }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "No action" }
                        EqCta {
                            title: "Coming soon",
                            description: "This feature is under development. Stay tuned!",
                        }
                    }
                }
            }
        }
    }
}
