use super::eq_image_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropToggle, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Image size preset.
#[derive(Clone, PartialEq, Default)]
pub enum AtomImageSize {
    Sm,
    #[default]
    Md,
    Lg,
    Full,
}

/// Aspect ratio constraint.
#[derive(Clone, PartialEq, Default)]
pub enum AspectRatio {
    Ratio16_9,
    Ratio4_3,
    Square,
    #[default]
    Free,
}

/// How the image fills its container.
#[derive(Clone, PartialEq, Default)]
pub enum ObjectFit {
    #[default]
    Cover,
    Contain,
    Fill,
}

/// Atomic image component.
/// Renders a styled `<img>` inside a sized wrapper with lazy loading.
/// Accepts both Dioxus `Asset` (via `asset!()`) and URL strings as `src`.
#[component]
pub fn EqImage(
    /// Image source - Asset or URL string.
    #[props(into)]
    src: String,
    /// Alt text (required for accessibility).
    #[props(into)]
    alt: String,
    /// Size preset.
    #[props(default)]
    size: AtomImageSize,
    /// Aspect ratio constraint.
    #[props(default)]
    aspect_ratio: AspectRatio,
    /// How the image fills the container.
    #[props(default)]
    object_fit: ObjectFit,
    /// Apply rounded corners.
    #[props(default = false)]
    rounded: bool,
    /// Optional class override - extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let size_class = match size {
        AtomImageSize::Sm => s::SM,
        AtomImageSize::Md => s::MD,
        AtomImageSize::Lg => s::LG,
        AtomImageSize::Full => s::FULL,
    };
    let ratio_class = match aspect_ratio {
        AspectRatio::Ratio16_9 => s::RATIO_16_9,
        AspectRatio::Ratio4_3 => s::RATIO_4_3,
        AspectRatio::Square => s::RATIO_SQUARE,
        AspectRatio::Free => s::RATIO_FREE,
    };
    let fit_class = match object_fit {
        ObjectFit::Cover => s::OBJECT_COVER,
        ObjectFit::Contain => s::OBJECT_CONTAIN,
        ObjectFit::Fill => s::OBJECT_FILL,
    };
    let rounded_class = if rounded { s::ROUNDED } else { "" };

    let wrapper_base = format!("{} {} {} {}", s::WRAPPER, size_class, ratio_class, rounded_class);
    let wrapper_cls = merge_classes(&wrapper_base, &class);

    rsx! {
        div { class: "{wrapper_cls}",
            img {
                class: "{s::IMAGE_ELEMENT} {fit_class} {rounded_class}",
                src: "{src}",
                alt: "{alt}",
                loading: "lazy",
            }
        }
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-image",
        name: "EqImage",
        category: ComponentCategory::Atom,
        description: "Atomic image component with lazy loading, aspect ratio constraints, \
                      object-fit control, and optional rounded corners.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Basic",
                code: "EqImage {\n    src: \"https://example.com/photo.jpg\",\n    alt: \"A scenic view\",\n}".into(),
            },
            UsageExample {
                label: "With aspect ratio",
                code: "EqImage {\n    src: \"photo.jpg\",\n    alt: \"Image\",\n    aspect_ratio: AspectRatio::Ratio16_9,\n    rounded: true,\n}".into(),
            },
        ],
        render_demo: || rsx! { DemoEqImage {} },
        render_gallery: || rsx! { GalleryEqImage {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqImage() -> Element {
    let mut size_str = use_signal(|| "Md".to_string());
    let mut ratio_str = use_signal(|| "Free".to_string());
    let mut fit_str = use_signal(|| "Cover".to_string());
    let mut rounded = use_signal(|| false);

    let size = match size_str().as_str() {
        "Sm" => AtomImageSize::Sm,
        "Lg" => AtomImageSize::Lg,
        "Full" => AtomImageSize::Full,
        _ => AtomImageSize::Md,
    };
    let aspect_ratio = match ratio_str().as_str() {
        "Ratio16_9" => AspectRatio::Ratio16_9,
        "Ratio4_3" => AspectRatio::Ratio4_3,
        "Square" => AspectRatio::Square,
        _ => AspectRatio::Free,
    };
    let object_fit = match fit_str().as_str() {
        "Contain" => ObjectFit::Contain,
        "Fill" => ObjectFit::Fill,
        _ => ObjectFit::Cover,
    };

    let code = r#"EqImage {
    src: "https://example.com/photo.jpg",
    alt: "A scenic view",
    size: AtomImageSize::Md,
    aspect_ratio: AspectRatio::Ratio16_9,
    rounded: true,
}

EqImage {
    src: "avatar.png",
    alt: "User avatar",
    size: AtomImageSize::Sm,
    aspect_ratio: AspectRatio::Square,
    object_fit: ObjectFit::Cover,
}"#
    .to_string();

    rsx! {
        DemoSection { title: "EqImage",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "size",
                    value: size_str(),
                    options: vec!["Sm", "Md", "Lg", "Full"],
                    onchange: move |v: String| size_str.set(v),
                }
                PropSelect {
                    label: "aspect_ratio",
                    value: ratio_str(),
                    options: vec!["Free", "Ratio16_9", "Ratio4_3", "Square"],
                    onchange: move |v: String| ratio_str.set(v),
                }
                PropSelect {
                    label: "object_fit",
                    value: fit_str(),
                    options: vec!["Cover", "Contain", "Fill"],
                    onchange: move |v: String| fit_str.set(v),
                }
                PropToggle {
                    label: "rounded",
                    value: rounded(),
                    onchange: move |v: bool| rounded.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 max-w-lg",
                EqImage {
                    src: "https://picsum.photos/seed/eq-preview/800/600",
                    alt: "Preview image",
                    size,
                    aspect_ratio,
                    object_fit,
                    rounded: rounded(),
                }
            }
            StyleInfo { file: "eq_image_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqImage() -> Element {
    rsx! {
        div { class: "space-y-4",
            // Size gallery
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Sizes" }

                div { class: "space-y-3",
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Small" }
                        EqImage {
                            src: "https://picsum.photos/seed/eq-sm/200/200",
                            alt: "Small image",
                            size: AtomImageSize::Sm,
                        }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Medium" }
                        EqImage {
                            src: "https://picsum.photos/seed/eq-md/300/300",
                            alt: "Medium image",
                            size: AtomImageSize::Md,
                        }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Large" }
                        EqImage {
                            src: "https://picsum.photos/seed/eq-lg/400/400",
                            alt: "Large image",
                            size: AtomImageSize::Lg,
                        }
                    }
                }
            }

            // Aspect ratio gallery
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Aspect Ratios" }

                div { class: "space-y-3",
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "16:9" }
                        EqImage {
                            src: "https://picsum.photos/seed/eq-16-9/800/450",
                            alt: "16:9 image",
                            size: AtomImageSize::Md,
                            aspect_ratio: AspectRatio::Ratio16_9,
                        }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Square" }
                        EqImage {
                            src: "https://picsum.photos/seed/eq-square/300/300",
                            alt: "Square image",
                            size: AtomImageSize::Md,
                            aspect_ratio: AspectRatio::Square,
                        }
                    }
                }
            }
        }
    }
}
