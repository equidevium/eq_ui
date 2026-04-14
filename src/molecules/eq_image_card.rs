use super::eq_image_card_styles as s;
use crate::atoms::{AspectRatio, AtomImageSize, EqImage, ObjectFit};
use crate::theme::merge_classes;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropInput, PropToggle, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Controls how the caption is displayed relative to the image.
#[derive(Clone, PartialEq, Default)]
pub enum CaptionMode {
    /// Caption text below the image (figure/figcaption).
    #[default]
    Below,
    /// Caption text overlaid on the image with a gradient backdrop.
    Overlay,
}

/// Internal helper - renders title, description, and attribution text.
#[component]
fn CaptionContent(
    title_class: &'static str,
    description_class: &'static str,
    attribution_class: &'static str,
    title: Option<String>,
    description: Option<String>,
    attribution: Option<String>,
) -> Element {
    rsx! {
        if let Some(t) = title {
            p { class: title_class, "{t}" }
        }
        if let Some(d) = description {
            p { class: description_class, "{d}" }
        }
        if let Some(a) = attribution {
            p { class: attribution_class, "{a}" }
        }
    }
}

/// Image card molecule.
/// Composes EqImage with title, description, and attribution text.
/// Supports two layout modes: caption below or overlay.
#[component]
pub fn EqImageCard(
    /// Image source - Asset or URL string.
    #[props(into)]
    src: String,
    /// Alt text (required for accessibility).
    #[props(into)]
    alt: String,
    /// Caption layout mode.
    #[props(default)]
    mode: CaptionMode,
    /// Image size preset.
    #[props(default)]
    size: AtomImageSize,
    /// Aspect ratio constraint.
    #[props(default)]
    aspect_ratio: AspectRatio,
    /// How the image fills its container.
    #[props(default)]
    object_fit: ObjectFit,
    /// Apply rounded corners.
    #[props(default = false)]
    rounded: bool,
    /// Optional title above description.
    #[props(into)]
    title: Option<String>,
    /// Optional description text.
    #[props(into)]
    description: Option<String>,
    /// Optional attribution / photo credit.
    #[props(into)]
    attribution: Option<String>,
    /// Accessible label for screen readers. Provides a concise
    /// description of the card as a whole (e.g. "Photo of Alpine Meadow
    /// by Jane Doe"). When set, the card wrapper gets `role="figure"`
    /// (overlay mode) or enhances the existing `<figure>` (below mode).
    #[props(into, default)]
    aria_label: String,
    /// Optional class override - extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let has_caption = title.is_some() || description.is_some() || attribution.is_some();
    let has_aria_label = !aria_label.is_empty();

    match mode {
        CaptionMode::Below => {
            let cls = merge_classes(s::CARD_WRAPPER, &class);
            rsx! {
            figure {
                class: "{cls}",
                "aria-label": if has_aria_label { "{aria_label}" } else { "" },
                EqImage { src, alt, size, aspect_ratio, object_fit, rounded }
                if has_caption {
                    figcaption { class: s::FIGCAPTION,
                        CaptionContent {
                            title_class: s::CAPTION_TITLE,
                            description_class: s::CAPTION_DESCRIPTION,
                            attribution_class: s::CAPTION_ATTRIBUTION,
                            title, description, attribution,
                        }
                    }
                }
            }
        }},
        CaptionMode::Overlay => {
            let cls = merge_classes(s::OVERLAY_CONTAINER, &class);
            rsx! {
            div {
                class: "{cls}",
                role: "figure",
                "aria-label": if has_aria_label { "{aria_label}" } else { "{alt}" },
                EqImage { src, alt, size, aspect_ratio, object_fit, rounded }
                if has_caption {
                    div { class: s::OVERLAY_GRADIENT,
                        "aria-hidden": "true",
                        div { class: s::OVERLAY_TEXT_WRAPPER,
                            CaptionContent {
                                title_class: s::OVERLAY_TITLE,
                                description_class: s::OVERLAY_DESCRIPTION,
                                attribution_class: s::OVERLAY_ATTRIBUTION,
                                title, description, attribution,
                            }
                        }
                    }
                }
            }
        }},
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-image-card",
        name: "EqImageCard",
        category: ComponentCategory::Molecule,
        description: "Image card with optional title, description, and attribution. \
                      Supports caption below or overlaid on the image.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Below caption",
                code: "EqImageCard {\n    src: \"photo.jpg\",\n    alt: \"Description\",\n    mode: CaptionMode::Below,\n    title: \"Card Title\",\n}".into(),
            },
            UsageExample {
                label: "Overlay caption",
                code: "EqImageCard {\n    src: \"photo.jpg\",\n    alt: \"Description\",\n    mode: CaptionMode::Overlay,\n    title: \"Card Title\",\n    description: \"A short description.\",\n}".into(),
            },
        ],
        render_demo: || rsx! { DemoEqImageCard {} },
        render_gallery: || rsx! { GalleryEqImageCard {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqImageCard() -> Element {
    let mut mode_str = use_signal(|| "Below".to_string());
    let mut size_str = use_signal(|| "Lg".to_string());
    let mut ratio_str = use_signal(|| "Ratio16_9".to_string());
    let mut fit_str = use_signal(|| "Cover".to_string());
    let mut rounded = use_signal(|| true);
    let mut title = use_signal(|| "Alpine Meadow".to_string());
    let mut description =
        use_signal(|| "A serene landscape captured during the golden hour.".to_string());
    let mut attribution = use_signal(|| "Photo by Jane Doe".to_string());

    let mode = match mode_str().as_str() {
        "Overlay" => CaptionMode::Overlay,
        _ => CaptionMode::Below,
    };
    let size = match size_str().as_str() {
        "Sm" => AtomImageSize::Sm,
        "Md" => AtomImageSize::Md,
        "Full" => AtomImageSize::Full,
        _ => AtomImageSize::Lg,
    };
    let aspect_ratio = match ratio_str().as_str() {
        "Free" => AspectRatio::Free,
        "Ratio4_3" => AspectRatio::Ratio4_3,
        "Square" => AspectRatio::Square,
        _ => AspectRatio::Ratio16_9,
    };
    let object_fit = match fit_str().as_str() {
        "Contain" => ObjectFit::Contain,
        "Fill" => ObjectFit::Fill,
        _ => ObjectFit::Cover,
    };

    let title_val = title();
    let desc_val = description();
    let attr_val = attribution();

    let code = "EqImageCard {\n    src: \"photo.jpg\",\n    alt: \"Description\",\n    mode: CaptionMode::Below,\n    size: AtomImageSize::Lg,\n    aspect_ratio: AspectRatio::Ratio16_9,\n    rounded: true,\n    title: \"Card Title\",\n    description: \"A short description.\",\n    attribution: \"Photo by Author\",\n}\n\nEqImageCard {\n    mode: CaptionMode::Overlay,\n    // ...\n}".to_string();

    rsx! {
        DemoSection { title: "EqImageCard",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "mode",
                    value: mode_str(),
                    options: vec!["Below", "Overlay"],
                    onchange: move |v: String| mode_str.set(v),
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
                PropInput {
                    label: "title",
                    value: title(),
                    placeholder: "Card title",
                    onchange: move |v: String| title.set(v),
                }
                PropInput {
                    label: "description",
                    value: description(),
                    placeholder: "Card description",
                    onchange: move |v: String| description.set(v),
                }
                PropInput {
                    label: "attribution",
                    value: attribution(),
                    placeholder: "Photo by...",
                    onchange: move |v: String| attribution.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 max-w-lg",
                EqImageCard {
                    src: "https://picsum.photos/seed/eq-card1/800/500",
                    alt: "Preview image card",
                    mode,
                    size,
                    aspect_ratio,
                    object_fit,
                    rounded: rounded(),
                    title: title_val,
                    description: desc_val,
                    attribution: attr_val,
                }
            }
            StyleInfo { file: "eq_image_card_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqImageCard() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Caption Modes" }

                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    div { class: "space-y-2",
                        EqText { variant: TextVariant::Muted, "Below" }
                        EqImageCard {
                            src: "https://picsum.photos/seed/gallery-below/400/300",
                            alt: "Below caption example",
                            mode: CaptionMode::Below,
                            rounded: true,
                            title: "Scenic View",
                            description: "A beautiful landscape.",
                        }
                    }
                    div { class: "space-y-2",
                        EqText { variant: TextVariant::Muted, "Overlay" }
                        EqImageCard {
                            src: "https://picsum.photos/seed/gallery-overlay/400/300",
                            alt: "Overlay caption example",
                            mode: CaptionMode::Overlay,
                            rounded: true,
                            title: "Sunset",
                            description: "Golden hour magic.",
                        }
                    }
                }
            }
        }
    }
}
