//! EqButton — themed button atom with gradient color transitions.
//!
//! Five variants (Primary, Ghost, Outline, Card, Danger) and three
//! sizes (Sm, Md, Lg). Primary uses @property-animated gradient stops
//! that smoothly morph between two palettes on hover. Renders a native
//! `<button>` element for accessibility.

use super::eq_button_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

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
    /// Destructive action — red background.
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
/// Content is passed via children — text, icons, or any combination.
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
    /// Button content — text, icons, or any element.
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
