//! Static iPhone chrome for showcasing mobile-only components.
//!
//! Renders an iPhone 16 or iPhone 16 Pro shell (Dynamic Island,
//! status bar, home indicator, painted side buttons) around its
//! children. The screen area sits at the model's real CSS viewport
//! size so children render at mobile resolution inside the playground.
//!
//! Pure presentation. The Dynamic Island doesn't expand, the painted
//! side buttons aren't interactive, and there are no callbacks for
//! hardware events.
//!
//! **Accessibility**: the outer wrapper carries `role="figure"` with
//! an `aria-label` naming the model. Decorative chrome is
//! `aria-hidden` so children keep their own semantics.
//!
//! TODO: verify the dimensions below against Apple's HIG or the
//! iPhone Simulator before v0.5 ships.
//!
//! - iPhone 16: 393 x 852
//! - iPhone 16 Pro: 402 x 874
//! - Dynamic Island pill: ~125 x 37
//!
//! ```no_run
//! use eq_ui::prelude::*;
//! use eq_ui::molecules::{EqDeviceFrame, DeviceModel};
//!
//! let _: Element = rsx! {
//!     EqDeviceFrame {
//!         model: DeviceModel::IPhone16,
//!         div { class: "p-6", "Mobile-only content here" }
//!     }
//! };
//! ```

use super::eq_device_frame_styles as s;
use crate::theme::merge_classes;
use crate::{PlaygroundEnum, playground};
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropToggle, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentCategory, ComponentDescriptor, UsageExample};

/// iPhone model variants supported by `EqDeviceFrame`.
///
/// Both render with the Dynamic Island; only the screen size differs.
#[derive(Clone, Copy, PartialEq, Default, Debug, PlaygroundEnum)]
pub enum DeviceModel {
    /// iPhone 16. 393 x 852 CSS px, 6.1" display.
    #[default]
    IPhone16,
    /// iPhone 16 Pro. 402 x 874 CSS px, 6.3" display.
    IPhone16Pro,
}

impl DeviceModel {
    /// CSS viewport dimensions in pixels: `(width, height)`.
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            Self::IPhone16 => (393, 852),
            Self::IPhone16Pro => (402, 874),
        }
    }

    /// Dynamic Island pill dimensions in pixels: `(width, height)`.
    pub fn dynamic_island_size(&self) -> (u32, u32) {
        match self {
            Self::IPhone16 => (124, 36),
            Self::IPhone16Pro => (126, 37),
        }
    }

    /// Human-readable name for this model.
    pub fn label(&self) -> &'static str {
        match self {
            Self::IPhone16 => "iPhone 16",
            Self::IPhone16Pro => "iPhone 16 Pro",
        }
    }
}

/// iPhone device chrome wrapper for showcasing mobile-only components.
#[playground(
    category = Molecule,
    description = "Static iPhone chrome (16 / 16 Pro) for showcasing mobile-only components. \
                   Pure presentation: no event callbacks, no Dynamic Island expansion.",
    examples = [
        ("Default (iPhone 16)", "EqDeviceFrame {\n    div { class: \"p-6\", \"Mobile content\" }\n}"),
        ("iPhone 16 Pro", "EqDeviceFrame {\n    model: DeviceModel::IPhone16Pro,\n    div { class: \"p-6\", \"Mobile content\" }\n}"),
        ("Without home indicator", "EqDeviceFrame {\n    show_home_indicator: false,\n    div { \"\" }\n}"),
        ("Custom shell color", "EqDeviceFrame {\n    shell_color: \"#1a1a1f\".into(),\n    div { \"\" }\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqDeviceFrame(
    /// Which iPhone model to render.
    #[props(default)]
    model: DeviceModel,
    /// Optional status bar content. When `None`, renders the default
    /// "9:41" bar with mock signal/wifi/battery icons.
    status_bar: Option<Element>,
    /// Whether to render the home indicator pill at the bottom.
    #[props(default = true)]
    show_home_indicator: bool,
    /// Optional shell color (any CSS color value). Defaults to dark gray.
    #[props(into, default)]
    shell_color: String,
    /// Optional class override on the outer shell. Empty string
    /// keeps defaults; prefix with `!` to fully replace.
    #[props(into, default)]
    class: String,
    /// Children render inside the screen area.
    children: Element,
) -> Element {
    let (screen_w, screen_h) = model.dimensions();
    let (di_w, di_h) = model.dynamic_island_size();

    let shell_inline_style = if shell_color.trim().is_empty() {
        String::new()
    } else {
        format!("background-color: {};", shell_color)
    };

    let screen_inline_style = format!("width: {}px; height: {}px;", screen_w, screen_h);
    let di_inline_style = format!("width: {}px; height: {}px;", di_w, di_h);

    let shell_cls = merge_classes(s::SHELL, &class);
    let aria_label = format!("{} preview", model.label());

    rsx! {
        div {
            class: "{shell_cls}",
            style: "{shell_inline_style}",
            role: "figure",
            "aria-label": "{aria_label}",

            // Painted side buttons (decorative).
            div {
                class: "{s::SIDE_BUTTON_BASE} {s::ACTION_BUTTON}",
                "aria-hidden": "true",
            }
            div {
                class: "{s::SIDE_BUTTON_BASE} {s::VOLUME_UP}",
                "aria-hidden": "true",
            }
            div {
                class: "{s::SIDE_BUTTON_BASE} {s::VOLUME_DOWN}",
                "aria-hidden": "true",
            }
            div {
                class: "{s::SIDE_BUTTON_BASE} {s::POWER_BUTTON}",
                "aria-hidden": "true",
            }

            // Screen surface.
            div { class: "{s::SCREEN}", style: "{screen_inline_style}",

                // Dynamic Island.
                div {
                    class: "{s::DYNAMIC_ISLAND}",
                    style: "{di_inline_style}",
                    "aria-hidden": "true",
                }

                // Status bar: caller override or default.
                if let Some(custom_bar) = status_bar {
                    {custom_bar}
                } else {
                    DefaultStatusBar {}
                }

                // Children area.
                div { class: "{s::BODY}", {children} }

                // Home indicator.
                if show_home_indicator {
                    div { class: "{s::HOME_INDICATOR}", "aria-hidden": "true" }
                }
            }
        }
    }
}

// ── Default status bar ──────────────────────────────────────────────

/// Default "9:41" status bar with mock signal/wifi/battery icons.
/// The whole row is aria-hidden.
#[component]
fn DefaultStatusBar() -> Element {
    rsx! {
        div { class: "{s::STATUS_BAR}", "aria-hidden": "true",

            span { class: "{s::STATUS_BAR_TIME}", "9:41" }

            div { class: "{s::STATUS_BAR_RIGHT}",
                // Signal bars.
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "18",
                    height: "12",
                    view_box: "0 0 18 12",
                    fill: "currentColor",
                    rect {
                        x: "0",
                        y: "8",
                        width: "3",
                        height: "4",
                        rx: "0.5",
                    }
                    rect {
                        x: "5",
                        y: "5",
                        width: "3",
                        height: "7",
                        rx: "0.5",
                    }
                    rect {
                        x: "10",
                        y: "2",
                        width: "3",
                        height: "10",
                        rx: "0.5",
                    }
                    rect {
                        x: "15",
                        y: "0",
                        width: "3",
                        height: "12",
                        rx: "0.5",
                    }
                }
                // Wifi arcs.
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "16",
                    height: "12",
                    view_box: "0 0 16 12",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.6",
                    stroke_linecap: "round",
                    path { d: "M1 4 Q8 -1 15 4" }
                    path { d: "M3 6.5 Q8 3 13 6.5" }
                    path { d: "M5 9 Q8 7 11 9" }
                    circle {
                        cx: "8",
                        cy: "11",
                        r: "0.7",
                        fill: "currentColor",
                        stroke: "none",
                    }
                }
                // Battery.
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "27",
                    height: "12",
                    view_box: "0 0 27 12",
                    fill: "none",
                    rect {
                        x: "0.5",
                        y: "0.5",
                        width: "23",
                        height: "11",
                        rx: "2.5",
                        stroke: "currentColor",
                        stroke_opacity: "0.5",
                    }
                    rect {
                        x: "2",
                        y: "2",
                        width: "20",
                        height: "8",
                        rx: "1",
                        fill: "currentColor",
                    }
                    rect {
                        x: "24.5",
                        y: "4",
                        width: "1.5",
                        height: "4",
                        rx: "0.4",
                        fill: "currentColor",
                        fill_opacity: "0.5",
                    }
                }
            }
        }
    }
}

// ── Smoke tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimensions_iphone16() {
        assert_eq!(DeviceModel::IPhone16.dimensions(), (393, 852));
    }

    #[test]
    fn dimensions_iphone16_pro() {
        assert_eq!(DeviceModel::IPhone16Pro.dimensions(), (402, 874));
    }

    #[test]
    fn iphone16_pro_is_larger_than_iphone16() {
        let (w16, h16) = DeviceModel::IPhone16.dimensions();
        let (wp, hp) = DeviceModel::IPhone16Pro.dimensions();
        assert!(wp > w16, "Pro should be wider");
        assert!(hp > h16, "Pro should be taller");
    }

    #[test]
    fn default_model_is_iphone16() {
        let m: DeviceModel = Default::default();

        assert_eq!(m, DeviceModel::IPhone16);
    }

    #[test]
    fn labels_distinct_and_nonempty() {
        let a = DeviceModel::IPhone16.label();
        let b = DeviceModel::IPhone16Pro.label();
        assert!(!a.is_empty());
        assert!(!b.is_empty());
        assert_ne!(a, b);
    }

    #[test]
    fn dynamic_island_size_nonzero() {
        for m in [DeviceModel::IPhone16, DeviceModel::IPhone16Pro] {
            let (w, h) = m.dynamic_island_size();
            assert!(
                w > 0 && h > 0,
                "DI size must be non-zero for {:?}",
                m.label()
            );
        }
    }
}

// ── Demo (custom; needs a model toggle) ────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqDeviceFrame() -> Element {
    let mut model_str = use_signal(|| "IPhone16".to_string());
    let mut show_home = use_signal(|| true);

    let model = match model_str().as_str() {
        "IPhone16Pro" => DeviceModel::IPhone16Pro,
        _ => DeviceModel::IPhone16,
    };

    let (w, h) = model.dimensions();

    let code = r#"EqDeviceFrame {
    model: DeviceModel::IPhone16,
    div { class: "p-6 space-y-3 text-center",
        h1 { class: "text-xl font-semibold", "My Mobile App" }
        p { class: "text-sm opacity-70", "Mobile-only components live here." }
    }
}"#
    .to_string();

    rsx! {
        DemoSection { title: "EqDeviceFrame",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                div { class: "grid grid-cols-2 gap-3",
                    PropSelect {
                        label: "model",
                        value: model_str(),
                        options: vec!["IPhone16", "IPhone16Pro"],
                        onchange: move |v: String| model_str.set(v),
                    }
                    PropToggle {
                        label: "show_home_indicator",
                        value: show_home(),
                        onchange: move |v: bool| show_home.set(v),
                    }
                }
            }

            div { class: "flex justify-center p-6",
                EqDeviceFrame { model, show_home_indicator: show_home(),
                    div { class: "p-6 space-y-3 text-center",
                        h1 { class: "text-xl font-semibold text-[var(--color-label-primary)]",
                            "My Mobile App"
                        }
                        p { class: "text-sm text-[var(--color-label-secondary)]",
                            "Mobile-only components live here."
                        }
                        p { class: "text-xs text-[var(--color-label-secondary)] mt-8 opacity-70",
                            "Resolution: {w} × {h}"
                        }
                    }
                }
            }

            StyleInfo {
                file: "eq_device_frame_styles.rs",
                styles: format_catalog(&s::catalog()),
            }
            CodeBlock { code }
        }
    }
}

// ── Gallery (both models, side-by-side) ─────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqDeviceFrame() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "iPhone 16 vs iPhone 16 Pro"
                }
            }

            div { class: "flex flex-wrap items-start gap-8 justify-center p-4",

                div { class: "flex flex-col items-center gap-2",
                    EqDeviceFrame { model: DeviceModel::IPhone16,
                        div { class: "p-6 text-center",
                            p { class: "text-lg font-semibold text-[var(--color-label-primary)]",
                                "iPhone 16"
                            }
                            p { class: "text-xs text-[var(--color-label-secondary)] mt-2",
                                "393 × 852"
                            }
                        }
                    }
                    EqText { variant: TextVariant::Caption, "iPhone 16" }
                }

                div { class: "flex flex-col items-center gap-2",
                    EqDeviceFrame { model: DeviceModel::IPhone16Pro,
                        div { class: "p-6 text-center",
                            p { class: "text-lg font-semibold text-[var(--color-label-primary)]",
                                "iPhone 16 Pro"
                            }
                            p { class: "text-xs text-[var(--color-label-secondary)] mt-2",
                                "402 × 874"
                            }
                        }
                    }
                    EqText { variant: TextVariant::Caption, "iPhone 16 Pro" }
                }
            }
        }
    }
}
