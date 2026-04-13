//! Playground Guide - in-app developer documentation.
//!
//! Provides a rich, rendered guide that appears in the playground sidebar
//! under the "Guide" category. Covers playground navigation, the
//! `ComponentDescriptor` pattern, step-by-step instructions for adding
//! new components, the style catalog convention, and feature gating.

use dioxus::prelude::*;
use crate::atoms::{EqText, TextVariant, EqDivider};
use crate::playground::playground_helpers::{CodeBlock, DemoSection, highlight_rust};
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// ── Descriptor ────────────────────────────────────────────────────

pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "playground-guide",
        name: "Getting Started",
        category: ComponentCategory::Guide,
        description: "Learn how the EqPlayground works, how to navigate it, \
                      and how to add your own components.",
        style_tokens: || vec![],
        usage_examples: || vec![
            UsageExample {
                label: "Launch the playground",
                code: "use eq_ui::{all_component_descriptors, EqPlayground};\n\n\
                       fn App() -> Element {\n\
                       \x20   rsx! {\n\
                       \x20       EqPlayground {\n\
                       \x20           descriptors: all_component_descriptors(),\n\
                       \x20       }\n\
                       \x20   }\n\
                       }".into(),
            },
            UsageExample {
                label: "Add custom components",
                code: "let mut descs = all_component_descriptors();\n\
                       descs.push(my_widget::descriptor());\n\n\
                       rsx! {\n\
                       \x20   EqPlayground { descriptors: descs }\n\
                       }".into(),
            },
        ],
        render_demo: || rsx! { GuideContent {} },
        render_gallery: || rsx! { GuideContent {} },
    }
}

// ── Reusable section helpers ──────────────────────────────────────

/// A styled heading within the guide.
#[component]
fn GuideHeading(text: String) -> Element {
    rsx! {
        h3 { class: "text-lg font-semibold text-[var(--color-label-primary)] mt-6 mb-2",
            "{text}"
        }
    }
}

/// A styled sub-heading within the guide.
#[component]
fn GuideSubheading(text: String) -> Element {
    rsx! {
        h4 { class: "text-base font-medium text-[var(--color-label-primary)] mt-4 mb-1.5",
            "{text}"
        }
    }
}

/// Body paragraph.
#[component]
fn GuideParagraph(children: Element) -> Element {
    rsx! {
        div { class: "text-sm leading-relaxed text-[var(--color-label-secondary)] mb-3",
            {children}
        }
    }
}

/// Inline code span for use in prose.
#[component]
fn InlineCode(text: String) -> Element {
    rsx! {
        code { class: "px-1.5 py-0.5 rounded text-xs font-mono bg-[var(--color-surface-elevated)] text-[var(--color-accent-primary)]",
            "{text}"
        }
    }
}

/// A numbered step in an instructions list.
#[component]
fn Step(number: u8, title: String, children: Element) -> Element {
    rsx! {
        div { class: "flex gap-3 mb-4",
            // Number badge
            div { class: "flex-shrink-0 w-7 h-7 rounded-full bg-[var(--color-accent-primary)] \
                          text-[var(--color-primary-dark)] flex items-center justify-center \
                          text-xs font-bold mt-0.5",
                "{number}"
            }
            div { class: "flex-1",
                div { class: "text-sm font-semibold text-[var(--color-label-primary)] mb-1",
                    "{title}"
                }
                div { class: "text-sm leading-relaxed text-[var(--color-label-secondary)]",
                    {children}
                }
            }
        }
    }
}

/// A highlighted tip/note box.
#[component]
fn Tip(children: Element) -> Element {
    rsx! {
        div { class: "flex gap-2 px-4 py-3 rounded-lg border border-[var(--color-accent-primary)]/30 \
                      bg-[var(--color-accent-primary)]/5 my-3",
            span { class: "text-[var(--color-accent-primary)] text-sm flex-shrink-0 mt-0.5", "💡" }
            div { class: "text-sm text-[var(--color-label-secondary)] leading-relaxed",
                {children}
            }
        }
    }
}

/// A file path reference.
#[component]
fn FilePath(path: String) -> Element {
    rsx! {
        code { class: "px-2 py-1 rounded text-xs font-mono bg-[var(--color-primary-dark)] \
                       text-[var(--color-accent-secondary)] border border-[var(--color-card-border)]",
            "{path}"
        }
    }
}

// ── Main guide content ────────────────────────────────────────────

#[component]
fn GuideContent() -> Element {
    // ── Code samples ──
    let descriptor_code = r#"use crate::playground::playground_types::{
    ComponentDescriptor, ComponentCategory, UsageExample,
};
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, StyleInfo, PropSelect,
    PropToggle, format_catalog,
};

pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-my-widget",
        name: "EqMyWidget",
        category: ComponentCategory::Atom,
        description: "A short description of what this component does.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Basic",
                code: "EqMyWidget { value: 42 }".into(),
            },
        ],
        render_demo: || rsx! { DemoEqMyWidget {} },
        render_gallery: || rsx! { GalleryEqMyWidget {} },
    }
}"#;

    let styles_code = r#"//! Style constants for EqMyWidget - pure Tailwind.

pub const BASE: &str = "relative flex items-center ...";
pub const ACTIVE: &str = "bg-[var(--color-accent-primary)] ...";
pub const SM: &str = "px-2 py-1 text-xs";
pub const MD: &str = "px-4 py-2 text-sm";
pub const LG: &str = "px-6 py-3 text-base";

pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("BASE", BASE),
        ("ACTIVE", ACTIVE),
        ("SM", SM),
        ("MD", MD),
        ("LG", LG),
    ]
}"#;

    let demo_code = r#"#[cfg(feature = "playground")]
#[component]
fn DemoEqMyWidget() -> Element {
    let mut value = use_signal(|| 42usize);
    let mut variant_str = use_signal(|| "Default".to_string());

    let code = "EqMyWidget { value: 42 }".to_string();

    rsx! {
        DemoSection { title: "EqMyWidget",
            // Prop controls
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                PropSelect {
                    label: "variant",
                    value: variant_str(),
                    options: vec!["Default", "Alt"],
                    onchange: move |v: String| variant_str.set(v),
                }
            }

            // Live preview
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                EqMyWidget { value: value() }
            }

            StyleInfo {
                file: "eq_my_widget_styles.rs",
                styles: format_catalog(&s::catalog()),
            }
            CodeBlock { code }
        }
    }
}"#;

    let gallery_code = r#"#[cfg(feature = "playground")]
#[component]
fn GalleryEqMyWidget() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Variant Gallery"
                }
                // Show each variant side by side
                EqMyWidget { variant: MyVariant::Default }
                EqMyWidget { variant: MyVariant::Alt }
            }
        }
    }
}"#;

    let mod_registration = r#"// In atoms/mod.rs:
pub mod eq_my_widget;
pub mod eq_my_widget_styles;

pub use eq_my_widget::{EqMyWidget, MyVariant};"#;

    let lib_registration = r#"// In lib.rs → all_component_descriptors():
atoms::eq_my_widget::descriptor(),"#;

    let launch_code = r#"use eq_ui::{all_component_descriptors, EqPlayground};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        EqPlayground {
            descriptors: all_component_descriptors(),
        }
    }
}"#;

    let custom_inject_code = r#"use eq_ui::{all_component_descriptors, EqPlayground};

fn App() -> Element {
    let mut descs = all_component_descriptors();
    // Add your custom components
    descs.push(my_widget::descriptor());
    descs.push(my_other_widget::descriptor());

    rsx! {
        EqPlayground {
            descriptors: descs,
            site_title: "My Design System",
            copyright_holder: "My Company",
        }
    }
}"#;

    let cargo_toml_code = r#"[features]
default = []
playground = []

# In Dioxus.toml:
[features]
default = ["playground"]"#;

    let feature_gate_code = r#"// Component file - only demo code is gated:
use super::eq_my_widget_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

// These imports are only needed for the playground demo
#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, StyleInfo, PropSelect,
    PropToggle, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{
    ComponentDescriptor, ComponentCategory, UsageExample,
};

// ── The component itself (ALWAYS compiled) ──
#[component]
pub fn EqMyWidget(/* props */) -> Element {
    // ...production code...
}

// ── Playground-only code ──
#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor { /* ... */ }

#[cfg(feature = "playground")]
#[component]
fn DemoEqMyWidget() -> Element { /* ... */ }

#[cfg(feature = "playground")]
#[component]
fn GalleryEqMyWidget() -> Element { /* ... */ }"#;

    rsx! {
        DemoSection { title: "Playground Guide",

            // ─────────────────────────────────────────────────────
            // SECTION 1: Overview
            // ─────────────────────────────────────────────────────

            GuideHeading { text: "What is EqPlayground?".to_string() }

            GuideParagraph {
                "EqPlayground is a self-contained interactive component showcase built \
                 into the eq_ui library. Every component carries its own demo metadata \
                 via a "
                InlineCode { text: "ComponentDescriptor".to_string() }
                " struct, which the playground collects and renders as a navigable \
                 two-panel interface - a sidebar tree on the left and a live demo \
                 panel on the right."
            }

            GuideParagraph {
                "The playground is fully theme-aware: use the theme switcher in the \
                 header to preview all components across the complete set of built-in \
                 themes. Every component demo includes interactive prop controls, \
                 a live preview, style token introspection, and usage code samples."
            }

            EqDivider {}

            // ─────────────────────────────────────────────────────
            // SECTION 2: Navigation
            // ─────────────────────────────────────────────────────

            GuideHeading { text: "Navigating the Playground".to_string() }

            GuideParagraph {
                "The sidebar tree is organised following atomic design principles. \
                 Components are grouped into expandable categories:"
            }

            div { class: "space-y-2 mb-4 ml-4",
                div { class: "flex items-center gap-2 text-sm text-[var(--color-label-secondary)]",
                    span { class: "font-semibold text-[var(--color-accent-primary)]", "Guide" }
                    " - Documentation like this page"
                }
                div { class: "flex items-center gap-2 text-sm text-[var(--color-label-secondary)]",
                    span { class: "font-semibold text-[var(--color-accent-primary)]", "Atoms" }
                    " - Smallest building blocks (Text, Button, Input, Icon, Tab, ...)"
                }
                div { class: "flex items-center gap-2 text-sm text-[var(--color-label-secondary)]",
                    span { class: "font-semibold text-[var(--color-accent-primary)]", "Molecules" }
                    " - Composed from atoms (Card, Carousel, Tree, Accordion, ...)"
                }
                div { class: "flex items-center gap-2 text-sm text-[var(--color-label-secondary)]",
                    span { class: "font-semibold text-[var(--color-accent-primary)]", "Organisms" }
                    " - Complex compositions (Header, Grid, AppShell, ...)"
                }
                div { class: "flex items-center gap-2 text-sm text-[var(--color-label-secondary)]",
                    span { class: "font-semibold text-[var(--color-accent-primary)]", "Theming" }
                    " - Theme showcase and CSS custom property viewer"
                }
            }

            GuideParagraph {
                "Click any component in the sidebar to load its demo. Each demo panel contains:"
            }

            div { class: "space-y-1.5 mb-4 ml-4 text-sm text-[var(--color-label-secondary)]",
                div { "• Interactive prop controls - toggle, select, and tweak props in real time" }
                div { "• Live preview - see the component update instantly as you change props" }
                div { "• Style tokens - inspect all Tailwind class constants from the styles module" }
                div { "• Code block - copy-paste ready usage examples with syntax highlighting" }
            }

            EqDivider {}

            // ─────────────────────────────────────────────────────
            // SECTION 3: Architecture
            // ─────────────────────────────────────────────────────

            GuideHeading { text: "Architecture: The ComponentDescriptor Pattern".to_string() }

            GuideParagraph {
                "Every component in eq_ui exports a "
                InlineCode { text: "descriptor()".to_string() }
                " function (feature-gated behind "
                InlineCode { text: "playground".to_string() }
                ") that returns a "
                InlineCode { text: "ComponentDescriptor".to_string() }
                ". This struct tells the playground everything it needs to render \
                 the component's demo:"
            }

            div { class: "space-y-1.5 mb-4 ml-4 text-sm text-[var(--color-label-secondary)]",
                div {
                    InlineCode { text: "id".to_string() }
                    " - URL-safe identifier for routing (e.g. "
                    InlineCode { text: "\"eq-button\"".to_string() }
                    ")"
                }
                div {
                    InlineCode { text: "name".to_string() }
                    " - Display name in the tree (e.g. "
                    InlineCode { text: "\"EqButton\"".to_string() }
                    ")"
                }
                div {
                    InlineCode { text: "category".to_string() }
                    " - Determines tree grouping: "
                    InlineCode { text: "Atom".to_string() }
                    ", "
                    InlineCode { text: "Molecule".to_string() }
                    ", "
                    InlineCode { text: "Organism".to_string() }
                    ", or "
                    InlineCode { text: "Theming".to_string() }
                }
                div {
                    InlineCode { text: "description".to_string() }
                    " - One-line summary shown in the demo header"
                }
                div {
                    InlineCode { text: "style_tokens".to_string() }
                    " - Closure returning the style catalog "
                    InlineCode { text: "Vec<(&str, &str)>".to_string() }
                }
                div {
                    InlineCode { text: "usage_examples".to_string() }
                    " - Code snippets displayed in the demo"
                }
                div {
                    InlineCode { text: "render_demo".to_string() }
                    " - Function returning the interactive demo "
                    InlineCode { text: "Element".to_string() }
                }
                div {
                    InlineCode { text: "render_gallery".to_string() }
                    " - Function returning the static variant gallery "
                    InlineCode { text: "Element".to_string() }
                }
            }

            CodeBlock { code: descriptor_code.to_string() }

            EqDivider {}

            // ─────────────────────────────────────────────────────
            // SECTION 4: Adding a New Component (Step by Step)
            // ─────────────────────────────────────────────────────

            GuideHeading { text: "Adding a New Component".to_string() }

            GuideParagraph {
                "Follow these steps to add a new component to eq_ui with full \
                 playground support. We'll use a fictional "
                InlineCode { text: "EqMyWidget".to_string() }
                " atom as the example."
            }

            // Step 1
            Step { number: 1, title: "Create the styles module".to_string(),
                "Create "
                FilePath { path: "src/atoms/eq_my_widget_styles.rs".to_string() }
                " with pure Tailwind constants. Every styles module must export a "
                InlineCode { text: "pub fn catalog()".to_string() }
                " that returns all tokens as "
                InlineCode { text: "Vec<(&'static str, &'static str)>".to_string() }
                " so the playground can introspect them."
            }

            CodeBlock { code: styles_code.to_string() }

            // Step 2
            Step { number: 2, title: "Create the component file".to_string(),
                "Create "
                FilePath { path: "src/atoms/eq_my_widget.rs".to_string() }
                " with the component itself (always compiled), plus the feature-gated "
                InlineCode { text: "descriptor()".to_string() }
                ", "
                InlineCode { text: "DemoEqMyWidget".to_string() }
                ", and "
                InlineCode { text: "GalleryEqMyWidget".to_string() }
                " components."
            }

            GuideSubheading { text: "The descriptor function".to_string() }
            CodeBlock { code: descriptor_code.to_string() }

            GuideSubheading { text: "The interactive demo".to_string() }
            GuideParagraph {
                "The demo component should use "
                InlineCode { text: "DemoSection".to_string() }
                " as a wrapper, provide prop controls via "
                InlineCode { text: "PropSelect".to_string() }
                " / "
                InlineCode { text: "PropToggle".to_string() }
                " / "
                InlineCode { text: "PropInput".to_string() }
                ", show a live preview, and end with "
                InlineCode { text: "StyleInfo".to_string() }
                " and "
                InlineCode { text: "CodeBlock".to_string() }
                "."
            }
            CodeBlock { code: demo_code.to_string() }

            GuideSubheading { text: "The gallery".to_string() }
            GuideParagraph {
                "The gallery should show all variants and sizes in a compact layout \
                 so users can compare them at a glance."
            }
            CodeBlock { code: gallery_code.to_string() }

            // Step 3
            Step { number: 3, title: "Register the module".to_string(),
                "Add the module declarations and re-exports to the appropriate "
                InlineCode { text: "mod.rs".to_string() }
                " file (e.g. "
                FilePath { path: "src/atoms/mod.rs".to_string() }
                ")."
            }

            CodeBlock { code: mod_registration.to_string() }

            // Step 4
            Step { number: 4, title: "Register the descriptor".to_string(),
                "Add the "
                InlineCode { text: "descriptor()".to_string() }
                " call to the "
                InlineCode { text: "all_component_descriptors()".to_string() }
                " function in "
                FilePath { path: "src/lib.rs".to_string() }
                "."
            }

            CodeBlock { code: lib_registration.to_string() }

            // Step 5
            Step { number: 5, title: "Test it".to_string(),
                "Run the playground example to verify your component appears in the sidebar \
                 and renders correctly across all themes."
            }

            Tip {
                "Remember to run "
                InlineCode { text: "cargo check --features playground".to_string() }
                " to catch any compilation errors before launching the full app."
            }

            EqDivider {}

            // ─────────────────────────────────────────────────────
            // SECTION 5: Feature Gating
            // ─────────────────────────────────────────────────────

            GuideHeading { text: "Feature Gating".to_string() }

            GuideParagraph {
                "All playground-related code is gated behind the "
                InlineCode { text: "playground".to_string() }
                " Cargo feature. This means production builds carry zero demo overhead - \
                 only the component itself and its styles are compiled."
            }

            GuideParagraph {
                "The pattern is simple: the component and its styles are always compiled. \
                 The descriptor, demo, and gallery functions are wrapped in "
                InlineCode { text: "#[cfg(feature = \"playground\")]".to_string() }
                ". Playground-specific imports (helpers, types) are also gated."
            }

            CodeBlock { code: feature_gate_code.to_string() }

            Tip {
                "The "
                InlineCode { text: "playground".to_string() }
                " feature is configured in both "
                FilePath { path: "Cargo.toml".to_string() }
                " and "
                FilePath { path: "Dioxus.toml".to_string() }
                ":"
            }

            CodeBlock { code: cargo_toml_code.to_string() }

            EqDivider {}

            // ─────────────────────────────────────────────────────
            // SECTION 6: Styles Convention
            // ─────────────────────────────────────────────────────

            GuideHeading { text: "The Style Catalog Convention".to_string() }

            GuideParagraph {
                "Every component has a companion "
                InlineCode { text: "_styles.rs".to_string() }
                " module that exports named Tailwind constants. This is the single \
                 source of truth for all class strings used by the component."
            }

            GuideParagraph {
                "The convention requires a "
                InlineCode { text: "pub fn catalog()".to_string() }
                " function that returns every constant as a "
                InlineCode { text: "(&'static str, &'static str)".to_string() }
                " tuple. The playground's "
                InlineCode { text: "StyleInfo".to_string() }
                " helper renders this catalog so developers can inspect the exact \
                 Tailwind classes at a glance."
            }

            div { class: "space-y-1.5 mb-4 ml-4 text-sm text-[var(--color-label-secondary)]",
                div { "• Use ALL_CAPS for constant names" }
                div {
                    "• Reference theme variables via "
                    InlineCode { text: "var(--color-*)".to_string() }
                    " for theme compatibility"
                }
                div { "• Group constants by purpose: base, variants, sizes, modifiers" }
                div { "• Document each constant with a doc comment" }
                div {
                    "• Buttons are the sole exception - they use "
                    FilePath { path: "buttons.css".to_string() }
                    " because of "
                    InlineCode { text: "@property".to_string() }
                    " animations"
                }
            }

            CodeBlock { code: styles_code.to_string() }

            EqDivider {}

            // ─────────────────────────────────────────────────────
            // SECTION 7: Launching & Customising
            // ─────────────────────────────────────────────────────

            GuideHeading { text: "Launching the Playground".to_string() }

            GuideParagraph {
                "The playground is an example binary. The entry point is minimal - \
                 just collect descriptors and pass them to "
                InlineCode { text: "EqPlayground".to_string() }
                ":"
            }

            CodeBlock { code: launch_code.to_string() }

            GuideSubheading { text: "Injecting custom components".to_string() }

            GuideParagraph {
                "External consumers of eq_ui can extend the playground with their \
                 own components. Just push additional descriptors before passing them in. \
                 You can also customise the site title and copyright holder:"
            }

            CodeBlock { code: custom_inject_code.to_string() }

            Tip {
                "The "
                InlineCode { text: "EqPlayground".to_string() }
                " component is entirely self-contained - it includes its own CSS links, \
                 theme provider, and theme renderer. You don't need to set up anything \
                 else in your App component."
            }

            EqDivider {}

            // ─────────────────────────────────────────────────────
            // SECTION 8: Quick reference
            // ─────────────────────────────────────────────────────

            GuideHeading { text: "Quick Reference".to_string() }

            div { class: "rounded-lg border border-[var(--color-card-border)] overflow-hidden mb-4",
                // Table header
                div { class: "grid grid-cols-2 gap-px bg-[var(--color-card-border)]",
                    div { class: "px-4 py-2 bg-[var(--color-grid-header-bg)] text-xs font-semibold \
                                  text-[var(--color-label-primary)] uppercase tracking-wider",
                        "File"
                    }
                    div { class: "px-4 py-2 bg-[var(--color-grid-header-bg)] text-xs font-semibold \
                                  text-[var(--color-label-primary)] uppercase tracking-wider",
                        "Purpose"
                    }
                }
                // Table rows
                div { class: "grid grid-cols-2 gap-px bg-[var(--color-card-border)]",
                    div { class: "px-4 py-2 bg-[var(--color-card)] text-xs font-mono text-[var(--color-label-secondary)]",
                        "eq_my_widget.rs"
                    }
                    div { class: "px-4 py-2 bg-[var(--color-card)] text-sm text-[var(--color-label-secondary)]",
                        "Component + descriptor + demo + gallery"
                    }

                    div { class: "px-4 py-2 bg-[var(--color-card)] text-xs font-mono text-[var(--color-label-secondary)]",
                        "eq_my_widget_styles.rs"
                    }
                    div { class: "px-4 py-2 bg-[var(--color-card)] text-sm text-[var(--color-label-secondary)]",
                        "Tailwind constants + catalog()"
                    }

                    div { class: "px-4 py-2 bg-[var(--color-card)] text-xs font-mono text-[var(--color-label-secondary)]",
                        "atoms/mod.rs"
                    }
                    div { class: "px-4 py-2 bg-[var(--color-card)] text-sm text-[var(--color-label-secondary)]",
                        "Module declarations + pub use exports"
                    }

                    div { class: "px-4 py-2 bg-[var(--color-card)] text-xs font-mono text-[var(--color-label-secondary)]",
                        "lib.rs"
                    }
                    div { class: "px-4 py-2 bg-[var(--color-card)] text-sm text-[var(--color-label-secondary)]",
                        "Register descriptor in all_component_descriptors()"
                    }

                    div { class: "px-4 py-2 bg-[var(--color-card)] text-xs font-mono text-[var(--color-label-secondary)]",
                        "Cargo.toml"
                    }
                    div { class: "px-4 py-2 bg-[var(--color-card)] text-sm text-[var(--color-label-secondary)]",
                        "playground feature flag"
                    }
                }
            }

            Tip {
                "Browse any existing component (like "
                InlineCode { text: "EqProgress".to_string() }
                " or "
                InlineCode { text: "EqTab".to_string() }
                ") as a real-world reference. They follow this exact pattern."
            }
        }
    }
}
