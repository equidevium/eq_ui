use dioxus::prelude::*;
use super::eq_footer_styles as s;
use crate::theme::merge_classes;
use crate::playground;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{CodeBlock, DemoSection, PropInput, PropSelect, StyleInfo, format_catalog};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Footer link representation
#[derive(Clone, PartialEq)]
pub struct FooterLink {
    pub label: &'static str,
    pub href: &'static str,
}

/// Footer link group for organized navigation
#[derive(Clone, PartialEq)]
pub struct FooterLinkGroup {
    pub title: &'static str,
    pub links: Vec<FooterLink>,
}

/// Default footer link groups
fn default_link_groups() -> Vec<FooterLinkGroup> {
    vec![
        FooterLinkGroup {
            title: "Company",
            links: vec![
                FooterLink { label: "About", href: "/about" },
                FooterLink { label: "Careers", href: "/careers" },
                FooterLink { label: "Press", href: "/press" },
            ],
        },
        FooterLinkGroup {
            title: "Resources",
            links: vec![
                FooterLink { label: "Documentation", href: "/docs" },
                FooterLink { label: "Blog", href: "/blog" },
                FooterLink { label: "Support", href: "/support" },
            ],
        },
        FooterLinkGroup {
            title: "Legal",
            links: vec![
                FooterLink { label: "Privacy", href: "/privacy" },
                FooterLink { label: "Terms", href: "/terms" },
                FooterLink { label: "Cookies", href: "/cookies" },
            ],
        },
    ]
}

/// Footer component containing site-wide footer content
#[playground(
    category = Organism,
    description = "Site-wide footer with link groups, copyright info, and customizable year/holder/tagline.",
    examples = [
        ("Basic", "EqFooter {}"),
        ("Custom", "EqFooter {\n    copyright_holder: \"Acme Corp\",\n    year: 2026,\n    tagline: \"Innovate. Build. Ship.\",\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqFooter(
    #[props(default = "Equidevium")] copyright_holder: &'static str,
    #[props(default = 2026)] year: u16,
    #[props(default = default_link_groups())] link_groups: Vec<FooterLinkGroup>,
    #[props(default = "Building the future, one line at a time.")] tagline: &'static str,
    /// Optional class override - extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let cls = merge_classes(s::FOOTER, &class);
    rsx! {
        footer { class: "{cls}",
            div { class: s::FOOTER_INNER,

                if !link_groups.is_empty() {
                    nav { class: s::FOOTER_GRID,
                        for group in link_groups.iter() {
                            div { key: "{group.title}",
                                h4 { class: s::FOOTER_GROUP_TITLE, "{group.title}" }
                                ul { class: s::FOOTER_LIST,
                                    for link in group.links.iter() {
                                        li { key: "{link.href}",
                                            a { class: s::FOOTER_LINK, href: "{link.href}", "{link.label}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: s::FOOTER_BOTTOM,
                    p { class: s::FOOTER_TAGLINE, "{tagline}" }
                    p { "© {year} {copyright_holder}. All rights reserved." }
                }
            }
        }
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqFooter() -> Element {
    let mut holder_str = use_signal(|| "Equidevium".to_string());
    let mut tagline_str = use_signal(|| "Building the future, one line at a time.".to_string());
    let mut year_str = use_signal(|| "2026".to_string());

    let copyright_holder: &'static str = match holder_str().as_str() {
        "Acme Corp" => "Acme Corp",
        "My Company" => "My Company",
        _ => "Equidevium",
    };
    let tagline: &'static str = match tagline_str().as_str() {
        "Innovate. Build. Ship." => "Innovate. Build. Ship.",
        "Making the web beautiful." => "Making the web beautiful.",
        _ => "Building the future, one line at a time.",
    };
    let year: u16 = year_str().parse().unwrap_or(2026);

    let code = "EqFooter {}\n\nEqFooter {\n    copyright_holder: \"Acme Corp\",\n    year: 2026,\n    tagline: \"Innovate. Build. Ship.\",\n}".to_string();

    rsx! {
        DemoSection { title: "EqFooter",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "holder",
                    value: holder_str(),
                    options: vec!["Equidevium", "Acme Corp", "My Company"],
                    onchange: move |v: String| holder_str.set(v),
                }
                PropSelect {
                    label: "tagline",
                    value: tagline_str(),
                    options: vec![
                        "Building the future, one line at a time.",
                        "Innovate. Build. Ship.",
                        "Making the web beautiful.",
                    ],
                    onchange: move |v: String| tagline_str.set(v),
                }
                PropInput {
                    label: "year",
                    value: year_str(),
                    placeholder: "2026",
                    onchange: move |v: String| year_str.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] overflow-hidden",
                EqFooter { copyright_holder, year, tagline }
            }
            StyleInfo { file: "eq_footer_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqFooter() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Footer Gallery" }

                div { class: "space-y-3",
                    EqFooter {}
                }
            }
        }
    }
}
