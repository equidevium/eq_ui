//! Theme Showcase - interactive palette viewer for all CSS custom properties.
//!
//! Displays every theme token as a colour swatch or gradient strip,
//! grouped by category (Core Darks, Labels, Gradients, Accents, etc.).
//! Switching themes via the header dropdown updates all swatches in real-time.

use crate::atoms::{EqText, TextVariant};
use crate::playground::playground_helpers::DemoSection;
use crate::playground::playground_types::{ComponentCategory, ComponentDescriptor, UsageExample};
use dioxus::prelude::*;

// ── Descriptor ─────────────────────────────────────────────────────

pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "theme-showcase",
        name: "Theme Showcase",
        category: ComponentCategory::Theming,
        description: "Interactive palette of all CSS custom properties in the active theme.",
        style_tokens: || vec![],
        usage_examples: || vec![
            UsageExample {
                label: "Switch themes",
                code: "Use the dropdown in the header to switch between\n\
                       all 21 built-in themes and see how each palette\n\
                       defines these tokens."
                    .into(),
            },
        ],
        render_demo: || rsx! { DemoThemeShowcase {} },
        render_gallery: || rsx! { DemoThemeShowcase {} },
    }
}

// ── Helper components ──────────────────────────────────────────────

/// Colour swatch - small square + label + variable name.
#[component]
fn ColorSwatch(label: &'static str, var_name: &'static str) -> Element {
    rsx! {
        div { class: "flex items-center gap-3",
            div {
                class: "size-10 rounded-md border border-[var(--color-card-border)] shrink-0",
                style: format!("background: var(--{var_name});"),
            }
            div { class: "flex flex-col",
                EqText { variant: TextVariant::Caption, class: "font-mono", "{label}" }
                EqText {
                    variant: TextVariant::Muted,
                    class: "font-mono text-[10px]",
                    "--{var_name}"
                }
            }
        }
    }
}

/// Gradient preview strip.
#[component]
fn GradientSwatch(label: &'static str, var_name: &'static str) -> Element {
    rsx! {
        div { class: "space-y-1",
            EqText { variant: TextVariant::Caption, class: "font-mono", "{label}" }
            div {
                class: "h-8 rounded-md border border-[var(--color-card-border)]",
                style: format!("background: var(--{var_name});"),
            }
            EqText { variant: TextVariant::Muted, class: "font-mono text-[10px]", "--{var_name}" }
        }
    }
}

/// Labeled section within the showcase.
#[component]
fn ShowcaseSection(title: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "space-y-3",
            EqText { variant: TextVariant::H3, "{title}" }
            div {
                class: "rounded-lg border border-white/10 p-4",
                style: "background: rgba(26, 26, 26, 0.85);",
                {children}
            }
        }
    }
}

/// Transition speed comparison - hover to see difference.
#[component]
fn TransitionDemo() -> Element {
    let mut fast_hover = use_signal(|| false);
    let mut normal_hover = use_signal(|| false);

    let fast_style = if fast_hover() {
        "transition: all var(--transition-fast); background: var(--color-accent-primary); border-color: var(--color-card-border-bright); transform: scale(1.05);"
    } else {
        "transition: all var(--transition-fast); background: var(--color-card); border-color: var(--color-card-border); transform: scale(1);"
    };
    let normal_style = if normal_hover() {
        "transition: all var(--transition-normal); background: var(--color-accent-primary); border-color: var(--color-card-border-bright); transform: scale(1.05);"
    } else {
        "transition: all var(--transition-normal); background: var(--color-card); border-color: var(--color-card-border); transform: scale(1);"
    };

    rsx! {
        div { class: "space-y-3",
            EqText { variant: TextVariant::Muted, class: "text-xs",
                "Hover over each box to see the transition speed difference."
            }
            div { class: "flex flex-wrap gap-4",
                div {
                    class: "px-6 py-3 rounded-md border cursor-pointer select-none",
                    style: "{fast_style}",
                    onmouseenter: move |_| fast_hover.set(true),
                    onmouseleave: move |_| fast_hover.set(false),
                    EqText { variant: TextVariant::Caption, class: "font-mono", "Fast (0.15s)" }
                }
                div {
                    class: "px-6 py-3 rounded-md border cursor-pointer select-none",
                    style: "{normal_style}",
                    onmouseenter: move |_| normal_hover.set(true),
                    onmouseleave: move |_| normal_hover.set(false),
                    EqText { variant: TextVariant::Caption, class: "font-mono", "Normal (0.25s)" }
                }
            }
            EqText { variant: TextVariant::Muted, class: "text-xs",
                "These tokens are used across the library: accordion chevrons, carousel slides, tree expand, and all button hovers."
            }
        }
    }
}

/// Interactive card demo - pure CSS hover/active transitions.
#[component]
fn InteractiveCardDemo() -> Element {
    rsx! {
        div { class: "flex gap-4",
            div { class: "card-interactive rounded-xl bg-[var(--color-card)]/60 p-6 cursor-pointer select-none",
                EqText { variant: TextVariant::Body, "Hover and click me" }
                EqText { variant: TextVariant::Muted, class: "mt-1",
                    "Lift on hover, press down on click"
                }
            }
        }
    }
}

// ── Main showcase ──────────────────────────────────────────────────

#[component]
fn DemoThemeShowcase() -> Element {
    let backdrop_style = "\
        background-color: #1a1a1a; \
        background-image: \
            linear-gradient(45deg, #222 25%, transparent 25%), \
            linear-gradient(-45deg, #222 25%, transparent 25%), \
            linear-gradient(45deg, transparent 75%, #222 75%), \
            linear-gradient(-45deg, transparent 75%, #222 75%); \
        background-size: 20px 20px; \
        background-position: 0 0, 0 10px, 10px -10px, -10px 0;";

    rsx! {
        DemoSection { title: "Theme Showcase",
            div { class: "rounded-xl p-6 space-y-6", style: "{backdrop_style}",
                EqText { variant: TextVariant::Muted,
                    "All CSS custom properties available in the current theme. Switch themes using the dropdown above to see how each palette defines these tokens."
                }

                // ── Core Darks ──
                ShowcaseSection { title: "Core Darks",
                    div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                        ColorSwatch { label: "Primary Dark", var_name: "color-primary-dark" }
                        ColorSwatch { label: "Secondary Dark", var_name: "color-secondary-dark" }
                        ColorSwatch { label: "Tertiary Dark", var_name: "color-tertiary-dark" }
                        ColorSwatch { label: "Hover Button", var_name: "color-hover-button" }
                        ColorSwatch { label: "Card", var_name: "color-card" }
                        ColorSwatch { label: "Card Border", var_name: "color-card-border" }
                        ColorSwatch { label: "Card Border Bright", var_name: "color-card-border-bright" }
                        ColorSwatch { label: "Card Shadow", var_name: "color-card-shadow" }
                        ColorSwatch { label: "Background", var_name: "color-background" }
                        ColorSwatch { label: "Primary", var_name: "color-primary" }
                    }
                }

                // ── Labels / Text ──
                ShowcaseSection { title: "Labels / Text",
                    div { class: "grid grid-cols-2 md:grid-cols-3 gap-4",
                        ColorSwatch { label: "Label Primary", var_name: "color-label-primary" }
                        ColorSwatch { label: "Label Secondary", var_name: "color-label-secondary" }
                        ColorSwatch { label: "Label Bold", var_name: "color-label-bold" }
                        ColorSwatch { label: "Label Muted", var_name: "color-label-muted" }
                        ColorSwatch { label: "Label Disabled", var_name: "color-label-disabled" }
                    }
                }

                // ── Gradients ──
                ShowcaseSection { title: "Gradients",
                    div { class: "space-y-4",
                        div { class: "grid grid-cols-3 gap-4",
                            ColorSwatch { label: "Gradient Start", var_name: "color-gradient-default-start" }
                            ColorSwatch { label: "Gradient Mid", var_name: "color-gradient-default-mid" }
                            ColorSwatch { label: "Gradient End", var_name: "color-gradient-default-end" }
                        }
                        GradientSwatch { label: "Tricolor Gradient", var_name: "gradient-primary-tricolor" }
                        GradientSwatch { label: "Background Gradient", var_name: "gradient-background" }
                        GradientSwatch { label: "Duocolor Gradient", var_name: "gradient-primary-duocolor" }
                    }
                }

                // ── Accent & Interaction ──
                ShowcaseSection { title: "Accent & Interaction",
                    div { class: "grid grid-cols-2 md:grid-cols-3 gap-4",
                        ColorSwatch { label: "Accent Primary", var_name: "color-accent-primary" }
                        ColorSwatch { label: "Accent Secondary", var_name: "color-accent-secondary" }
                        ColorSwatch { label: "Accent Muted", var_name: "color-accent-muted" }
                        ColorSwatch { label: "Focus Ring", var_name: "color-focus-ring" }
                        ColorSwatch { label: "Shadow Glow", var_name: "color-shadow-glow" }
                    }
                }

                // ── State / Feedback ──
                ShowcaseSection { title: "State / Feedback",
                    div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                        ColorSwatch { label: "Success", var_name: "color-success" }
                        ColorSwatch { label: "Warning", var_name: "color-warning" }
                        ColorSwatch { label: "Error", var_name: "color-error" }
                        ColorSwatch { label: "Info", var_name: "color-info" }
                    }
                }

                // ── Borders & Dividers ──
                ShowcaseSection { title: "Borders & Dividers",
                    div { class: "grid grid-cols-2 md:grid-cols-3 gap-4",
                        ColorSwatch { label: "Border Default", var_name: "color-border-default" }
                        ColorSwatch { label: "Border Subtle", var_name: "color-border-subtle" }
                        ColorSwatch { label: "Border Active", var_name: "color-border-active" }
                    }
                }

                // ── Input / Form ──
                ShowcaseSection { title: "Input / Form Elements",
                    div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                        ColorSwatch { label: "Input BG", var_name: "color-input-bg" }
                        ColorSwatch { label: "Input Border", var_name: "color-input-border" }
                        ColorSwatch { label: "Input Focus", var_name: "color-input-focus" }
                        ColorSwatch { label: "Placeholder", var_name: "color-input-placeholder" }
                    }
                }

                // ── Surfaces & Overlays ──
                ShowcaseSection { title: "Surfaces & Overlays",
                    div { class: "grid grid-cols-2 md:grid-cols-3 gap-4",
                        ColorSwatch { label: "Surface Elevated", var_name: "color-surface-elevated" }
                        ColorSwatch { label: "Surface Overlay", var_name: "color-surface-overlay" }
                        ColorSwatch { label: "Surface Tooltip", var_name: "color-surface-tooltip" }
                    }
                }

                // ── Code ──
                ShowcaseSection { title: "Code / Terminal",
                    div { class: "grid grid-cols-2 md:grid-cols-3 gap-4",
                        ColorSwatch { label: "Code BG", var_name: "color-code-bg" }
                        ColorSwatch { label: "Code Text", var_name: "color-code-text" }
                        ColorSwatch { label: "Code Comment", var_name: "color-code-comment" }
                        ColorSwatch { label: "Code Keyword", var_name: "color-code-keyword" }
                        ColorSwatch { label: "Code String", var_name: "color-code-string" }
                    }
                }

                // ── Scrollbar ──
                ShowcaseSection { title: "Scrollbar",
                    div { class: "grid grid-cols-2 gap-4",
                        ColorSwatch { label: "Thumb", var_name: "color-scrollbar-thumb" }
                        ColorSwatch { label: "Track", var_name: "color-scrollbar-track" }
                    }
                }

                // ── Buttons (live interactive) ──
                ShowcaseSection { title: "Button Variants",
                    div { class: "space-y-4",
                        div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                            ColorSwatch { label: "Btn Primary BG", var_name: "btn-primary-bg" }
                            ColorSwatch { label: "Btn Primary Hover", var_name: "btn-primary-hover" }
                            ColorSwatch { label: "Btn Primary Text", var_name: "btn-primary-text" }
                            ColorSwatch { label: "Btn Ghost Hover", var_name: "btn-ghost-hover" }
                            ColorSwatch { label: "Btn Outline Border", var_name: "btn-outline-border" }
                            ColorSwatch { label: "Btn Outline Hover", var_name: "btn-outline-hover-border" }
                            ColorSwatch { label: "Btn Outline Hover BG", var_name: "btn-outline-hover-bg" }
                            ColorSwatch { label: "Btn Danger BG", var_name: "btn-danger-bg" }
                        }
                        EqText {
                            variant: TextVariant::Caption,
                            class: "font-semibold uppercase tracking-wider mt-2",
                            "Live Preview"
                        }
                        div { class: "flex flex-wrap gap-3 items-center",
                            button { class: "btn btn-primary btn-md", "Primary" }
                            button { class: "btn btn-ghost btn-md", "Ghost" }
                            button { class: "btn btn-outline btn-md", "Outline" }
                            button { class: "btn btn-card btn-md", "Card" }
                            button { class: "btn btn-danger btn-md", "Danger" }
                        }
                        EqText {
                            variant: TextVariant::Caption,
                            class: "font-semibold uppercase tracking-wider mt-2",
                            "Sizes"
                        }
                        div { class: "flex flex-wrap gap-3 items-center",
                            button { class: "btn btn-primary btn-sm", "Small" }
                            button { class: "btn btn-primary btn-md", "Medium" }
                            button { class: "btn btn-primary btn-lg", "Large" }
                        }
                    }
                }

                // ── Transitions ──
                ShowcaseSection { title: "Transitions", TransitionDemo {} }

                // ── Interactive Card Demo ──
                ShowcaseSection { title: "Interactive Card (hover + click)", InteractiveCardDemo {} }
            }
        }
    }
}
