use dioxus::prelude::*;
use super::eq_footer_styles as s;

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
#[component]
pub fn EqFooter(
    #[props(default = "Equidevium")] copyright_holder: &'static str,
    #[props(default = 2026)] year: u16,
    #[props(default = default_link_groups())] link_groups: Vec<FooterLinkGroup>,
    #[props(default = "Building the future, one line at a time.")] tagline: &'static str,
) -> Element {
    rsx! {
        footer { class: s::FOOTER,
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
