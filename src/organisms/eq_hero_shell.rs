use dioxus::prelude::*;
use super::eq_hero_shell_styles::*;
use crate::theme::{merge_classes, CONTAINER_LAYOUT};
use crate::playground;

#[cfg(feature = "playground")]
use super::eq_hero_shell_styles as s;
#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{CodeBlock, DemoSection, PropInput, PropToggle, StyleInfo, format_catalog};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Full-width hero section organism.
///
/// **Accessibility** – renders a `<section>` landmark with an auto-generated
/// `aria-labelledby` pointing at the `<h1>` title, so screen readers announce
/// it as a named region (e.g. "Welcome, region"). Decorative background and
/// overlay elements are hidden from the accessibility tree.
#[playground(
    category = Organism,
    description = "Full-width hero section with optional background image, customizable colors, and action buttons.",
    examples = [
        ("Basic", "EqHeroShell {\n    title: \"Welcome\",\n    subtitle: \"Build something great.\",\n}"),
        ("With background", "EqHeroShell {\n    title: \"Welcome\",\n    subtitle: \"Build something great.\",\n    background: rsx! {\n        EqImage {\n            src: \"hero.jpg\",\n            alt: \"Hero\",\n            size: AtomImageSize::Full,\n        }\n    },\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqHeroShell(
    #[props(into)]
    title: String,
    #[props(into)]
    subtitle: Option<String>,
    #[props(into)]
    title_color: Option<String>,
    #[props(into)]
    subtitle_color: Option<String>,
    actions: Option<Element>,
    background: Option<Element>,
    /// Accessible label for screen readers. When empty (default), the
    /// section is labelled by its `<h1>` title via `aria-labelledby`.
    #[props(into, default)]
    aria_label: String,
    /// Semantic role override. Common values:
    /// - `"banner"` for the primary hero of the page
    /// - empty (default) — the `<section>` landmark is used as-is
    #[props(into, default)]
    role: String,
    /// Optional class override - extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    // Stable unique ID for aria-labelledby linking.
    let hero_id = use_hook(|| {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("eq-hero-{id}")
    });

    let title_id = format!("{}-title", hero_id);
    let has_label = !aria_label.is_empty();
    let has_role = !role.is_empty();

    let base = format!("{HERO_SHELL} {HERO_SHELL_RELATIVE}");
    let cls = merge_classes(&base, &class);
    rsx! {
        section {
            class: "{cls}",
            role: if has_role { "{role}" } else { "" },
            "aria-label": if has_label { "{aria_label}" } else { "" },
            "aria-labelledby": if !has_label { "{title_id}" } else { "" },

            if let Some(bg) = background {
                div { class: HERO_BG, "aria-hidden": "true",
                    {bg}
                }
                div { class: HERO_OVERLAY, "aria-hidden": "true" }
            }
            div { class: "{CONTAINER_LAYOUT} {HERO_CONTENT}",
                h1 {
                    id: "{title_id}",
                    class: HERO_TITLE,
                    style: if let Some(ref c) = title_color { format!("color: {c}") } else { String::new() },
                    "{title}"
                }

                if let Some(subtitle) = subtitle {
                    p {
                        class: HERO_SUBTITLE,
                        style: if let Some(ref c) = subtitle_color { format!("color: {c}") } else { String::new() },
                        "{subtitle}"
                    }
                }

                if let Some(actions) = actions {
                    div { class: HERO_ACTIONS,
                        {actions}
                    }
                }
            }
        }
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqHeroShell() -> Element {
    let mut title = use_signal(|| "Hero Shell Title".to_string());
    let mut subtitle = use_signal(|| "A tagline or subtitle goes here.".to_string());
    let mut title_color = use_signal(|| String::new());
    let mut subtitle_color = use_signal(|| String::new());
    let mut show_bg = use_signal(|| false);

    let title_c_val = title_color();
    let subtitle_c_val = subtitle_color();
    let title_c: Option<String> = if title_c_val.is_empty() {
        None
    } else {
        Some(title_c_val.clone())
    };
    let subtitle_c: Option<String> = if subtitle_c_val.is_empty() {
        None
    } else {
        Some(subtitle_c_val.clone())
    };

    let code = "EqHeroShell {\n    title: \"Welcome\",\n    subtitle: \"Build something great.\",\n}\n\nEqHeroShell {\n    title: \"Welcome\",\n    subtitle: \"Build something great.\",\n    title_color: \"#ff6b6b\",\n    background: rsx! {\n        EqImage { src: \"hero.jpg\", alt: \"Hero\",\n            size: AtomImageSize::Full,\n            aspect_ratio: AspectRatio::Ratio4_3,\n            object_fit: ObjectFit::Cover }\n    },\n}".to_string();

    rsx! {
        DemoSection { title: "EqHeroShell",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropInput {
                    label: "title",
                    value: title(),
                    placeholder: "Hero title",
                    onchange: move |v: String| title.set(v),
                }
                PropInput {
                    label: "subtitle",
                    value: subtitle(),
                    placeholder: "Subtitle text",
                    onchange: move |v: String| subtitle.set(v),
                }
                PropInput {
                    label: "title_color",
                    value: title_c_val.clone(),
                    placeholder: "#ff6b6b (empty = theme)",
                    onchange: move |v: String| title_color.set(v),
                }
                PropInput {
                    label: "sub_color",
                    value: subtitle_c_val.clone(),
                    placeholder: "#ffd93d (empty = theme)",
                    onchange: move |v: String| subtitle_color.set(v),
                }
                PropToggle {
                    label: "background",
                    value: show_bg(),
                    onchange: move |v: bool| show_bg.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] overflow-hidden",
                if show_bg() {
                    EqHeroShell {
                        title: title(),
                        subtitle: subtitle(),
                        title_color: title_c.clone(),
                        subtitle_color: subtitle_c.clone(),
                    }
                } else {
                    EqHeroShell {
                        title: title(),
                        subtitle: subtitle(),
                        title_color: title_c.clone(),
                        subtitle_color: subtitle_c.clone(),
                    }
                }
            }
            StyleInfo { file: "eq_hero_shell_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqHeroShell() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Hero Shell Gallery" }

                div { class: "space-y-3",
                    EqHeroShell {
                        title: "Welcome to Equidevium".to_string(),
                        subtitle: Some("Build something amazing today.".to_string()),
                    }
                }
            }
        }
    }
}
