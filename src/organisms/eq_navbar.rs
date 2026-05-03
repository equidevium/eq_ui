use dioxus::prelude::*;
use crate::playground;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{CodeBlock, DemoSection, StyleInfo, format_catalog};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[playground(
    category = Organism,
    description = "Custom navbar wrapper that applies navbar-specific styling via external CSS.",
    examples = [
        ("Basic", "EqNavbar {\n    div { \"Navigation content here\" }\n}"),
        ("With items", "EqNavbar {\n    ul { class: \"flex gap-4\",\n        li { a { href: \"/\", \"Home\" } }\n        li { a { href: \"/about\", \"About\" } }\n    }\n}"),
    ],
    no_styles,
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqNavbar(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            {children}
        }
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqNavbar() -> Element {
    let code = "EqNavbar {\n    ul { class: \"flex gap-4\",\n        li { a { href: \"/\", \"Home\" } }\n        li { a { href: \"/about\", \"About\" } }\n        li { a { href: \"/services\", \"Services\" } }\n    }\n}".to_string();

    rsx! {
        DemoSection { title: "EqNavbar",
            EqText {
                variant: TextVariant::Muted,
                "EqNavbar is a wrapper component that applies custom navbar styling. It wraps your navigation content and loads navbar.css for styling."
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] overflow-hidden p-4",
                EqNavbar {
                    div { class: "flex gap-6",
                        a { href: "#", class: "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition", "Home" }
                        a { href: "#", class: "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition", "About" }
                        a { href: "#", class: "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition", "Contact" }
                    }
                }
            }
            StyleInfo { file: "navbar.css", styles: format_catalog(&[
                ("NAVBAR", "id=\"navbar\" - styled via navbar.css"),
            ]) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqNavbar() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Navbar Gallery" }

                div { class: "space-y-3",
                    EqNavbar {
                        div { class: "flex gap-6",
                            span { class: "text-sm font-semibold", "Home" }
                            span { class: "text-sm text-[var(--color-label-secondary)]", "About" }
                            span { class: "text-sm text-[var(--color-label-secondary)]", "Services" }
                        }
                    }
                }
            }
        }
    }
}
