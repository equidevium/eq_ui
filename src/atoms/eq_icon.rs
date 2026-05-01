use dioxus::prelude::*;
use super::eq_icon_styles as s;
use crate::theme::merge_classes;
use crate::{PreviewEnum, preview};

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropToggle, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Icon size variant.
#[derive(Clone, PartialEq, Default, PreviewEnum)]
pub enum IconSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Atomic icon wrapper with two rendering modes:
///
/// **Path mode** - pass an SVG path data string via the `path` prop.
/// The component renders an inline `<svg>` with `fill="currentColor"`,
/// so color is controlled by the wrapper's text color class.
///
/// **Children mode** - pass any element (custom SVG, `<img>`, etc.)
/// as children. The wrapper applies consistent sizing and color.
#[preview(
    category = Atom,
    description = "Icon wrapper supporting SVG path data or custom SVG children. \
                   Configurable sizes with optional muted variant.",
    examples = [
        ("With path", "EqIcon { path: \"M12 4.5v15m7.5-7.5h-15\", size: IconSize::Sm }"),
        ("With SVG children", "EqIcon { size: IconSize::Md,\n    svg { /* custom svg */ }\n}"),
        ("Muted", "EqIcon { size: IconSize::Lg, muted: true,\n    svg { /* icon */ }\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqIcon(
    /// SVG path data (`d` attribute). When set, an inline `<svg>` is
    /// rendered automatically. Mutually exclusive with children.
    #[props(into, default)]
    path: String,
    #[props(default)]
    size: IconSize,
    #[props(default = false)]
    muted: bool,
    /// Accessible label for standalone icons (e.g. icon-only buttons).
    #[props(into, default)]
    aria_label: String,
    /// Optional CSS color override (e.g. "#3eb489", "var(--color-success)").
    #[props(into, default)]
    color: String,
    /// Optional class override - extend or replace default styles.
    #[props(into, default)]
    class: String,
    children: Option<Element>,
) -> Element {
    let size_class = match size {
        IconSize::Sm => s::SM,
        IconSize::Md => s::MD,
        IconSize::Lg => s::LG,
    };
    let has_color = !color.is_empty();
    let color_class = if has_color {
        "" // skip default color class when custom color is set
    } else if muted {
        s::MUTED
    } else {
        s::DEFAULT
    };
    let base = format!("{} {} {}", s::WRAPPER, size_class, color_class);
    let cls = merge_classes(&base, &class);
    let color_style = if has_color {
        format!("color: {};", color)
    } else {
        String::new()
    };

    let is_decorative = aria_label.is_empty();

    rsx! {
        span {
            class: "{cls}",
            style: "{color_style}",
            "aria-hidden": if is_decorative { "true" } else { "" },
            "aria-label": if !is_decorative { "{aria_label}" } else { "" },
            role: if !is_decorative { "img" } else { "" },
            if !path.is_empty() {
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 256 256",
                    fill: "currentColor",
                    class: "w-full h-full",
                    "aria-hidden": "true",
                    path { d: "{path}" }
                }
            } else if let Some(content) = children {
                {content}
            }
        }
    }
}

// ── Custom demo (SVG children needed) ────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqIcon() -> Element {
    let mut size_str = use_signal(|| "Md".to_string());
    let mut muted = use_signal(|| false);

    let size = match size_str().as_str() {
        "Sm" => IconSize::Sm,
        "Lg" => IconSize::Lg,
        _ => IconSize::Md,
    };

    let code = "EqIcon { size: IconSize::Sm,\n    svg { /* your SVG icon */ }\n}\n\nEqIcon { size: IconSize::Lg, muted: true,\n    svg { /* dimmed icon */ }\n}".to_string();

    rsx! {
        DemoSection { title: "EqIcon",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "size",
                    value: size_str(),
                    options: vec!["Sm", "Md", "Lg"],
                    onchange: move |v: String| size_str.set(v),
                }
                PropToggle {
                    label: "muted",
                    value: muted(),
                    onchange: move |v: bool| muted.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                EqIcon { size, muted: muted(),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "2",
                        stroke: "currentColor",
                        path { d: "M12 4.5v15m7.5-7.5h-15" }
                    }
                }
            }
            StyleInfo { file: "eq_icon_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Custom gallery ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqIcon() -> Element {
    rsx! {
        div { class: "flex items-center gap-6",
            EqText { variant: TextVariant::Emphasis, "All sizes" }
            for (label , s) in [("Sm", IconSize::Sm), ("Md", IconSize::Md), ("Lg", IconSize::Lg)] {
                div { class: "flex items-center gap-2",
                    EqIcon { size: s,
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke_width: "2",
                            stroke: "currentColor",
                            path { d: "M12 4.5v15m7.5-7.5h-15" }
                        }
                    }
                    EqText { variant: TextVariant::Caption, "{label}" }
                }
            }
        }
    }
}
