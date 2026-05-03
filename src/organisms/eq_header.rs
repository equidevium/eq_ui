use dioxus::prelude::*;
use super::eq_header_styles as s;
use crate::theme::{merge_classes, CONTAINER_LAYOUT};
use crate::playground;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{CodeBlock, DemoSection, PropSelect, StyleInfo, format_catalog};
#[cfg(feature = "playground")]
use crate::atoms::EqText;
#[cfg(feature = "playground")]
use crate::atoms::TextVariant;
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Portable header component.
/// Accepts nav content as an Element prop so the platform crate
/// can provide router-aware Links or plain `<a>` tags.
#[playground(
    category = Organism,
    description = "Sticky navigation header with configurable site title and nav items.",
    examples = [
        ("Basic", "EqHeader { site_title: \"My App\" }"),
        ("With navigation", "EqHeader {\n    site_title: \"My App\",\n    nav: rsx! {\n        li { a { href: \"/\", \"Home\" } }\n        li { a { href: \"/about\", \"About\" } }\n    },\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqHeader(
    #[props(default = "Equidevium")]
    site_title: &'static str,
    /// When false, the brand/site_title is hidden entirely.
    /// Useful when integrating with a router navbar that provides
    /// its own branding or when the header is nav-only.
    #[props(default = true)]
    show_brand: bool,
    /// When true (default), the brand is a clickable link to "/".
    /// Set to false to render the brand as plain text.
    #[props(default = true)]
    brand_link: bool,
    /// Navigation content - the caller provides `<li>` elements.
    /// EqHeader wraps them in `<nav><ul>` with correct styling.
    nav: Option<Element>,
    /// Optional class override - extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let cls = merge_classes(s::HEADER, &class);
    rsx! {
        header { class: "{cls}",
            div { class: "{CONTAINER_LAYOUT} {s::HEADER_INNER}",
                if show_brand {
                    h1 {
                        if brand_link {
                            a {
                                class: s::BRAND,
                                href: "/",
                                "{site_title}"
                            }
                        } else {
                            span {
                                class: s::BRAND,
                                "{site_title}"
                            }
                        }
                    }
                }
                if let Some(nav_content) = nav {
                    nav {
                        ul { class: s::NAV_UL,
                            {nav_content}
                        }
                    }
                }
            }
        }
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqHeader() -> Element {
    let mut title_str = use_signal(|| "Equidevium".to_string());

    let site_title: &'static str = match title_str().as_str() {
        "My App" => "My App",
        "Dashboard" => "Dashboard",
        "Acme Corp" => "Acme Corp",
        _ => "Equidevium",
    };

    let code = "EqHeader {\n    site_title: \"My App\",\n    nav: rsx! {\n        li { a { href: \"/\", \"Home\" } }\n        li { a { href: \"/about\", \"About\" } }\n    },\n}".to_string();

    rsx! {
        DemoSection { title: "EqHeader",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "site_title",
                    value: title_str(),
                    options: vec!["Equidevium", "My App", "Dashboard", "Acme Corp"],
                    onchange: move |v: String| title_str.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] overflow-hidden",
                EqHeader {
                    site_title,
                    nav: rsx! {
                        li {
                            a {
                                href: "#",
                                class: "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition",
                                "Home"
                            }
                        }
                        li {
                            a {
                                href: "#",
                                class: "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition",
                                "About"
                            }
                        }
                        li {
                            a {
                                href: "#",
                                class: "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition",
                                "Contact"
                            }
                        }
                    },
                }
            }
            StyleInfo { file: "eq_header_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqHeader() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Header Gallery" }

                div { class: "space-y-3",
                    EqHeader {
                        site_title: "Equidevium",
                        nav: rsx! {
                            li {
                                a {
                                    href: "#",
                                    class: "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition",
                                    "Home"
                                }
                            }
                            li {
                                a {
                                    href: "#",
                                    class: "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition",
                                    "Docs"
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}
