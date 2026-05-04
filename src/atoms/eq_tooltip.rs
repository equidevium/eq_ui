//! EqTooltip — hover/focus tooltip atom.
//!
//! Wraps any child element and shows a small text bubble on hover or
//! keyboard focus. Pure CSS positioning (no JS), four placement options,
//! and optional delay.
//!
//! ```rust,ignore
//! EqTooltip {
//!     text: "Save your changes",
//!     EqButton { on_click: move |_| {}, "Save" }
//! }
//! ```

use super::eq_tooltip_styles as s;
use crate::theme::merge_classes;
use crate::{PlaygroundEnum, playground};
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropInput, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant, EqButton, ButtonVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// ── Types ─────────────────────────────────────────────────────────

/// Placement of the tooltip relative to the trigger.
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
pub enum TooltipPosition {
    /// Above the trigger (default).
    #[default]
    Top,
    /// Below the trigger.
    Bottom,
    /// Left of the trigger.
    Left,
    /// Right of the trigger.
    Right,
}

// ── Component ─────────────────────────────────────────────────────

/// Hover/focus tooltip.
///
/// Wraps its children and displays a positioned text bubble on
/// `mouseenter` / `focusin` and hides on `mouseleave` / `focusout`.
///
/// **Accessibility** — the tooltip text is connected to the trigger
/// via `aria-describedby`, and the tooltip element uses `role="tooltip"`.
/// The tooltip is keyboard-accessible through focus.
#[playground(
    category = Atom,
    description = "Hover/focus tooltip with four positions, \
                   pure CSS positioning, and ARIA describedby.",
    examples = [
        ("Basic", "EqTooltip {\n    text: \"Save changes\",\n    EqButton { on_click: move |_| {}, \"Save\" }\n}"),
        ("Bottom", "EqTooltip {\n    text: \"More options\",\n    position: TooltipPosition::Bottom,\n    EqButton { on_click: move |_| {}, \"Options\" }\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqTooltip(
    /// Tooltip text content.
    #[props(into)]
    text: String,
    /// Placement relative to the trigger element.
    #[props(default)]
    position: TooltipPosition,
    /// Optional class override on the wrapper element.
    #[props(into, default)]
    class: String,
    /// The trigger element(s).
    children: Element,
) -> Element {
    let mut visible = use_signal(|| false);

    let pos_cls = match position {
        TooltipPosition::Top => s::POS_TOP,
        TooltipPosition::Bottom => s::POS_BOTTOM,
        TooltipPosition::Left => s::POS_LEFT,
        TooltipPosition::Right => s::POS_RIGHT,
    };

    let vis_cls = if visible() { s::VISIBLE } else { s::HIDDEN };
    let wrapper_cls = merge_classes(s::WRAPPER, &class);

    // Unique ID for aria-describedby.
    let tooltip_id = use_hook(|| {
        static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        format!("eq-tooltip-{}", COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    });

    rsx! {
        div {
            class: "{wrapper_cls}",
            "aria-describedby": "{tooltip_id}",
            onmouseenter: move |_| visible.set(true),
            onmouseleave: move |_| visible.set(false),
            onfocusin: move |_| visible.set(true),
            onfocusout: move |_| visible.set(false),

            {children}

            div {
                id: "{tooltip_id}",
                class: "{s::TOOLTIP} {pos_cls} {vis_cls}",
                role: "tooltip",
                "{text}"
            }
        }
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqTooltip() -> Element {
    let mut text = use_signal(|| "Tooltip text".to_string());
    let mut position_str = use_signal(|| "Top".to_string());

    let position = match position_str().as_str() {
        "Bottom" => TooltipPosition::Bottom,
        "Left" => TooltipPosition::Left,
        "Right" => TooltipPosition::Right,
        _ => TooltipPosition::Top,
    };

    let code = format!(
        r#"EqTooltip {{
    text: "{text}",
    position: TooltipPosition::{pos},
    EqButton {{ on_click: move |_| {{}}, "Hover me" }}
}}"#,
        text = text(),
        pos = position_str(),
    );

    rsx! {
        DemoSection { title: "EqTooltip",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropInput {
                    label: "text",
                    value: text(),
                    placeholder: "Tooltip content",
                    onchange: move |v: String| text.set(v),
                }
                PropSelect {
                    label: "position",
                    value: position_str(),
                    options: vec!["Top", "Bottom", "Left", "Right"],
                    onchange: move |v: String| position_str.set(v),
                }
            }

            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 space-y-6",
                // Main interactive tooltip
                div { class: "flex items-center justify-center py-8",
                    EqTooltip {
                        text: text(),
                        position,
                        EqButton { on_click: move |_| {}, "Hover me" }
                    }
                }

                // All positions
                div { class: "space-y-3",
                    EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "All positions" }
                    div { class: "flex items-center justify-center gap-6 py-8",
                        EqTooltip {
                            text: "Top tooltip",
                            position: TooltipPosition::Top,
                            EqButton { variant: ButtonVariant::Outline, on_click: move |_| {}, "Top" }
                        }
                        EqTooltip {
                            text: "Bottom tooltip",
                            position: TooltipPosition::Bottom,
                            EqButton { variant: ButtonVariant::Outline, on_click: move |_| {}, "Bottom" }
                        }
                        EqTooltip {
                            text: "Left tooltip",
                            position: TooltipPosition::Left,
                            EqButton { variant: ButtonVariant::Outline, on_click: move |_| {}, "Left" }
                        }
                        EqTooltip {
                            text: "Right tooltip",
                            position: TooltipPosition::Right,
                            EqButton { variant: ButtonVariant::Outline, on_click: move |_| {}, "Right" }
                        }
                    }
                }
            }

            StyleInfo { file: "eq_tooltip_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery ───────────────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqTooltip() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Tooltip Gallery" }

                div { class: "flex items-center gap-4 py-6 justify-center",
                    EqTooltip {
                        text: "Save your work",
                        EqButton { variant: ButtonVariant::Primary, on_click: move |_| {}, "Save" }
                    }
                    EqTooltip {
                        text: "Discard changes",
                        position: TooltipPosition::Bottom,
                        EqButton { variant: ButtonVariant::Danger, on_click: move |_| {}, "Delete" }
                    }
                    EqTooltip {
                        text: "View settings",
                        position: TooltipPosition::Right,
                        EqButton { variant: ButtonVariant::Ghost, on_click: move |_| {}, "Settings" }
                    }
                }
            }
        }
    }
}
