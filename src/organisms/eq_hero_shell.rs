use dioxus::prelude::*;
use super::eq_hero_shell_styles as s;
use super::eq_hero_shell_styles::*;
use crate::theme::{merge_classes, CONTAINER_LAYOUT};

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{CodeBlock, DemoSection, PropInput, PropToggle, StyleInfo, format_catalog};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

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
    /// Optional class override - extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let base = format!("{HERO_SHELL} {HERO_SHELL_RELATIVE}");
    let cls = merge_classes(&base, &class);
    rsx! {
        section { class: "{cls}",
            if let Some(bg) = background {
                div { class: HERO_BG,
                    {bg}
                }
                div { class: HERO_OVERLAY }
            }
            div { class: "{CONTAINER_LAYOUT} {HERO_CONTENT}",
                h1 {
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

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-hero-shell",
        name: "EqHeroShell",
        category: ComponentCategory::Organism,
        description: "Full-width hero section with optional background image, customizable colors, and action buttons.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Basic",
                code: "EqHeroShell {\n    title: \"Welcome\",\n    subtitle: \"Build something great.\",\n}".into(),
            },
            UsageExample {
                label: "With background",
                code: "EqHeroShell {\n    title: \"Welcome\",\n    subtitle: \"Build something great.\",\n    background: rsx! {\n        EqImage {\n            src: \"hero.jpg\",\n            alt: \"Hero\",\n            size: AtomImageSize::Full,\n            aspect_ratio: AspectRatio::Ratio4_3,\n            object_fit: ObjectFit::Cover,\n        }\n    },\n}".into(),
            },
        ],
        render_demo: || rsx! { DemoEqHeroShell {} },
        render_gallery: || rsx! { GalleryEqHeroShell {} },
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
