//! EqButton - themed button atom with gradient color transitions.
//!
//! Five variants (Primary, Ghost, Outline, Card, Danger) and three
//! sizes (Sm, Md, Lg). Primary uses @property-animated gradient stops
//! that smoothly morph between two palettes on hover. Renders a native
//! `<button>` element for accessibility.

use super::eq_button_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropToggle, PropInput, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Visual variant of the button.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    /// Gradient background with color-morphing transition on hover.
    #[default]
    Primary,
    /// Transparent background, secondary text. Subtle hover fill.
    Ghost,
    /// Bordered with gradient hover reveal and border color shift.
    Outline,
    /// Card-styled with glow shadow and lift on hover.
    Card,
    /// Destructive action - red background.
    Danger,
}

/// Size preset for the button.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Themed button atom with gradient color transitions.
///
/// Maps to a Dioxus primitive `<button>`, covering five visual variants
/// and three size presets. The Primary variant uses CSS `@property` to
/// smoothly transition between two three-color gradients on hover.
///
/// Content is passed via children - text, icons, or any combination.
///
/// ```rust,ignore
/// EqButton {
///     variant: ButtonVariant::Primary,
///     size: ButtonSize::Lg,
///     on_click: move |_| do_something(),
///     "Save Changes"
/// }
/// ```
#[component]
pub fn EqButton(
    /// Visual variant.
    #[props(default)]
    variant: ButtonVariant,
    /// Size preset.
    #[props(default)]
    size: ButtonSize,
    /// Disables the button (dims and prevents interaction).
    #[props(default = false)]
    disabled: bool,
    /// Enable gradient background (default true). When false, uses a
    /// flat solid color from the theme.
    #[props(default = true)]
    gradient: bool,
    /// Enable gradient color transition on hover (default true).
    /// When false, the gradient snaps instantly to the hover palette.
    #[props(default = true)]
    animate: bool,
    /// Gradient angle in degrees (default 90 = horizontal left-to-right).
    /// Common values: 0 (bottom-to-top), 45 (diagonal), 90 (horizontal),
    /// 135 (diagonal down-right), 180 (top-to-bottom).
    #[props(default = 90)]
    angle: u16,
    /// Optional text color override. When non-empty, applied as an
    /// inline `color` style on the `<button>`, overriding the theme.
    #[props(into, default)]
    color: String,
    /// Fired on click.
    #[props(default)]
    on_click: Option<EventHandler<Event<MouseData>>>,
    /// Optional class override on the `<button>` element.
    #[props(into, default)]
    class: String,
    /// Button content - text, icons, or any element.
    children: Element,
) -> Element {
    let variant_cls = match variant {
        ButtonVariant::Primary => s::PRIMARY,
        ButtonVariant::Ghost => s::GHOST,
        ButtonVariant::Outline => s::OUTLINE,
        ButtonVariant::Card => s::CARD,
        ButtonVariant::Danger => s::DANGER,
    };

    let size_cls = match size {
        ButtonSize::Sm => s::SM,
        ButtonSize::Md => s::MD,
        ButtonSize::Lg => s::LG,
    };

    let gradient_cls = if gradient { "" } else { s::NO_GRADIENT };
    let animate_cls = if animate { "" } else { s::NO_TRANSITION };
    let base = format!(
        "{} {} {} {} {}",
        s::BASE, variant_cls, size_cls, gradient_cls, animate_cls
    );
    let cls = merge_classes(&base, &class);

    // Build inline style: optional color override + gradient angle
    let mut style_parts = Vec::new();
    if !color.is_empty() {
        style_parts.push(format!("color: {};", color));
    }
    if angle != 90 {
        style_parts.push(format!("--btn-angle: {}deg;", angle));
    }
    let inline_style = style_parts.join(" ");

    rsx! {
        button {
            class: "{cls}",
            style: "{inline_style}",
            disabled: disabled,
            onclick: move |evt| {
                if let Some(ref handler) = on_click {
                    handler.call(evt);
                }
            },
            {children}
        }
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-button",
        name: "EqButton",
        category: ComponentCategory::Atom,
        description: "Themed button atom with five visual variants and three size presets. \
                      Gradient variants use animated background-position shift on hover for smooth \
                      flowing effect. Renders native <button> for accessibility.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Primary button",
                code: "EqButton {\n    variant: ButtonVariant::Primary,\n    size: ButtonSize::Lg,\n    on_click: move |_| do_something(),\n    \"Save Changes\"\n}".into(),
            },
            UsageExample {
                label: "Solid (no gradient)",
                code: "EqButton {\n    gradient: false,\n    color: \"#fbbf24\",\n    \"Solid Button\"\n}".into(),
            },
            UsageExample {
                label: "Disabled state",
                code: "EqButton {\n    disabled: true,\n    \"Not Available\"\n}".into(),
            },
        ],
        render_demo: || rsx! { DemoEqButton {} },
        render_gallery: || rsx! { GalleryEqButton {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqButton() -> Element {
    let mut variant_str = use_signal(|| "Primary".to_string());
    let mut size_str = use_signal(|| "Md".to_string());
    let mut disabled = use_signal(|| false);
    let mut gradient = use_signal(|| true);
    let mut animate = use_signal(|| true);
    let mut angle_str = use_signal(|| "90".to_string());
    let mut color_str = use_signal(|| String::new());
    let mut click_count = use_signal(|| 0usize);

    let variant = match variant_str().as_str() {
        "Ghost" => ButtonVariant::Ghost,
        "Outline" => ButtonVariant::Outline,
        "Card" => ButtonVariant::Card,
        "Danger" => ButtonVariant::Danger,
        _ => ButtonVariant::Primary,
    };

    let size = match size_str().as_str() {
        "Sm" => ButtonSize::Sm,
        "Lg" => ButtonSize::Lg,
        _ => ButtonSize::Md,
    };

    let code = "use eq_ui::atoms::{EqButton, ButtonVariant, ButtonSize};\n\
        \n\
        EqButton {\n\
        \x20   variant: ButtonVariant::Primary,\n\
        \x20   size: ButtonSize::Lg,\n\
        \x20   on_click: move |_| do_something(),\n\
        \x20   \"Save Changes\"\n\
        }\n\
        \n\
        // Solid (no gradient) with custom text color\n\
        EqButton {\n\
        \x20   gradient: false,\n\
        \x20   color: \"#fbbf24\",\n\
        \x20   \"Solid Button\"\n\
        }\n\
        \n\
        // Disabled state\n\
        EqButton {\n\
        \x20   disabled: true,\n\
        \x20   \"Not Available\"\n\
        }".to_string();

    let variant_names: Vec<(&str, ButtonVariant)> = vec![
        ("Primary", ButtonVariant::Primary),
        ("Ghost", ButtonVariant::Ghost),
        ("Outline", ButtonVariant::Outline),
        ("Card", ButtonVariant::Card),
        ("Danger", ButtonVariant::Danger),
    ];

    let size_names: Vec<(&str, ButtonSize)> = vec![
        ("Sm", ButtonSize::Sm),
        ("Md", ButtonSize::Md),
        ("Lg", ButtonSize::Lg),
    ];

    rsx! {
        DemoSection { title: "EqButton",
            EqText { variant: TextVariant::Muted,
                "Themed button atom with five visual variants and three size presets. Gradient variants (Primary, Outline, Danger) use an animated background-position shift on hover for a smooth flowing effect."
            }

            // Prop controls
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                div { class: "grid grid-cols-2 md:grid-cols-4 gap-3",
                    PropSelect {
                        label: "variant",
                        value: variant_str(),
                        options: vec!["Primary", "Ghost", "Outline", "Card", "Danger"],
                        onchange: move |v: String| variant_str.set(v),
                    }
                    PropSelect {
                        label: "size",
                        value: size_str(),
                        options: vec!["Sm", "Md", "Lg"],
                        onchange: move |v: String| size_str.set(v),
                    }
                    PropToggle {
                        label: "disabled",
                        value: disabled(),
                        onchange: move |v: bool| disabled.set(v),
                    }
                    PropToggle {
                        label: "gradient",
                        value: gradient(),
                        onchange: move |v: bool| gradient.set(v),
                    }
                    PropToggle {
                        label: "animate",
                        value: animate(),
                        onchange: move |v: bool| animate.set(v),
                    }
                    PropSelect {
                        label: "angle",
                        value: angle_str(),
                        options: vec!["0", "45", "90", "135", "180"],
                        onchange: move |v: String| angle_str.set(v),
                    }
                    PropInput {
                        label: "color",
                        value: color_str(),
                        placeholder: "#ffffff or red",
                        onchange: move |v: String| color_str.set(v),
                    }
                }
            }

            // Live preview
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 flex flex-col items-center gap-3",
                EqButton {
                    variant: variant,
                    size: size,
                    disabled: disabled(),
                    gradient: gradient(),
                    animate: animate(),
                    angle: angle_str().parse::<u16>().unwrap_or(90),
                    color: color_str(),
                    on_click: move |_| click_count += 1,
                    "Click Me"
                }
                if click_count() > 0 {
                    {
                        let suffix = if click_count() != 1 { "s" } else { "" };
                        rsx! {
                            EqText { variant: TextVariant::Muted, class: "text-sm",
                                "Clicked {click_count} time{suffix}"
                            }
                        }
                    }
                }
            }

            // Variant gallery - all 5 variants × 3 sizes
            div { class: "space-y-4",
                EqText { variant: TextVariant::Emphasis, "All Variants × Sizes" }
                div { class: "space-y-3",
                    for (v_name , v) in variant_names.iter() {
                        div { class: "space-y-1",
                            EqText { variant: TextVariant::Caption, class: "font-medium text-sm", "{v_name}" }
                            div { class: "flex flex-wrap items-center gap-3",
                                for (s_name , s) in size_names.iter() {
                                    EqButton {
                                        variant: *v,
                                        size: *s,
                                        "{v_name} {s_name}"
                                    }
                                }
                            }
                        }
                    }
                }

                EqText { variant: TextVariant::Emphasis, "Disabled States" }
                div { class: "flex flex-wrap items-center gap-3",
                    for (v_name , v) in variant_names.iter() {
                        EqButton {
                            variant: *v,
                            disabled: true,
                            "{v_name}"
                        }
                    }
                }

                EqText { variant: TextVariant::Emphasis, "Solid (No Gradient)" }
                div { class: "flex flex-wrap items-center gap-3",
                    EqButton { gradient: false, "Default" }
                    EqButton { gradient: false, color: "#fbbf24", "Custom Color" }
                    EqButton { gradient: false, size: ButtonSize::Lg, "Solid Lg" }
                    EqButton { variant: ButtonVariant::Danger, gradient: false, "Danger Solid" }
                }
            }

            StyleInfo { file: "eq_button_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqButton() -> Element {
    rsx! {
        div { class: "space-y-4",
            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Variants" }
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 flex flex-wrap items-center gap-3",
                EqButton { variant: ButtonVariant::Primary, "Primary" }
                EqButton { variant: ButtonVariant::Ghost, "Ghost" }
                EqButton { variant: ButtonVariant::Outline, "Outline" }
                EqButton { variant: ButtonVariant::Card, "Card" }
                EqButton { variant: ButtonVariant::Danger, "Danger" }
            }

            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Sizes" }
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 flex flex-wrap items-center gap-3",
                EqButton { size: ButtonSize::Sm, "Small" }
                EqButton { size: ButtonSize::Md, "Medium" }
                EqButton { size: ButtonSize::Lg, "Large" }
            }

            EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "States" }
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                div { class: "flex flex-wrap items-center gap-3",
                    EqButton { "Enabled" }
                    EqButton { disabled: true, "Disabled" }
                }
                div { class: "flex flex-wrap items-center gap-3",
                    EqButton { gradient: true, "With Gradient" }
                    EqButton { gradient: false, "Solid" }
                }
            }
        }
    }
}
