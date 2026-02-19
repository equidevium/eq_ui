use dioxus::prelude::*;
use eq_ui::atoms::*;
use eq_ui::molecules::*;
use eq_ui::organisms::*;
use eq_ui::{UI_TAILWIND_CSS, UI_INDEX_CSS, UI_COLORS_CSS, UI_BUTTONS_CSS};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: UI_TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: UI_INDEX_CSS }
        document::Link { rel: "stylesheet", href: UI_COLORS_CSS }
        document::Link { rel: "stylesheet", href: UI_BUTTONS_CSS }

        div { class: "min-h-screen bg-[var(--color-primary-dark)] text-[var(--color-label-primary)]",
            div { class: "mx-auto max-w-5xl px-4 py-8 space-y-16",
                PageHeader {}
                ShowcaseAtoms {}
                ShowcaseMolecules {}
                ShowcaseOrganisms {}
            }
        }
    }
}

// ── Page header ─────────────────────────────────────────────────────

#[component]
fn PageHeader() -> Element {
    rsx! {
        header { class: "space-y-2 pb-8 border-b border-[var(--color-card-border)]",
            h1 { class: "text-3xl font-bold text-[var(--color-label-primary)]",
                "eq_ui showcase"
            }
            p { class: "text-[var(--color-label-secondary)]",
                "Visual gallery of every component with prop variations."
            }
        }
    }
}

// ── Helpers ──────────────────────────────────────────────────────────

#[component]
fn Section(title: &'static str, children: Element) -> Element {
    rsx! {
        section { class: "space-y-6",
            h2 { class: "text-2xl font-semibold text-[var(--color-label-primary)] border-b border-[var(--color-card-border)] pb-2",
                "{title}"
            }
            {children}
        }
    }
}

#[component]
fn ComponentBlock(name: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "space-y-3",
            h3 { class: "text-lg font-medium text-[var(--color-label-bold)]",
                "{name}"
            }
            div { class: "rounded-xl border border-[var(--color-card-border)] bg-[var(--color-card)]/30 p-6 space-y-4",
                {children}
            }
        }
    }
}

// ── Atoms ────────────────────────────────────────────────────────────

#[component]
fn ShowcaseAtoms() -> Element {
    let mut demo_input = use_signal(|| String::new());
    let mut demo_email = use_signal(|| String::new());
    let mut demo_textarea = use_signal(|| String::new());

    rsx! {
        Section { title: "Atoms",

            ComponentBlock { name: "EqText",
                EqText { variant: TextVariant::H1, "Heading 1" }
                EqText { variant: TextVariant::H2, "Heading 2" }
                EqText { variant: TextVariant::H3, "Heading 3" }
                EqText { variant: TextVariant::Body,
                    "Body text — the default variant. Renders a <p> element."
                }
                EqText { variant: TextVariant::Muted,
                    "Muted text — secondary colour for less important content."
                }
                EqText { variant: TextVariant::Caption, "Caption text" }
                EqText { variant: TextVariant::Emphasis, "Emphasis text" }
                EqText { variant: TextVariant::Mono, "Mono text — code snippets" }
            }

            ComponentBlock { name: "EqLabel",
                div { class: "space-y-2",
                    EqLabel { for_id: "demo", "Form label (for=\"demo\")" }
                    EqLabel { "Label without for attribute" }
                }
            }

            ComponentBlock { name: "EqLink",
                div { class: "flex gap-6",
                    EqLink { href: "#".to_string(), "Default link" }
                    EqLink { href: "https://example.com".to_string(), "External link" }
                }
            }

            ComponentBlock { name: "EqInput",
                div { class: "space-y-4 max-w-md",
                    div { class: "space-y-1",
                        EqLabel { for_id: "text-demo", "Text input" }
                        EqInput {
                            id: "text-demo",
                            placeholder: "Type something…",
                            value: demo_input(),
                            oninput: move |e: FormEvent| demo_input.set(e.value()),
                        }
                    }
                    div { class: "space-y-1",
                        EqLabel { for_id: "email-demo", "Email input" }
                        EqInput {
                            kind: InputKind::Email,
                            id: "email-demo",
                            placeholder: "you@example.com",
                            value: demo_email(),
                            oninput: move |e: FormEvent| demo_email.set(e.value()),
                        }
                    }
                    div { class: "space-y-1",
                        EqLabel { for_id: "pw-demo", "Password input" }
                        EqInput {
                            kind: InputKind::Password,
                            id: "pw-demo",
                            placeholder: "••••••••",
                            value: String::new(),
                            oninput: move |_| {},
                        }
                    }
                    div { class: "space-y-1",
                        EqLabel { for_id: "ta-demo", "Textarea" }
                        EqInput {
                            kind: InputKind::Textarea,
                            id: "ta-demo",
                            placeholder: "Write a longer message…",
                            value: demo_textarea(),
                            oninput: move |e: FormEvent| demo_textarea.set(e.value()),
                        }
                    }
                    div { class: "space-y-1",
                        EqLabel { "Disabled input" }
                        EqInput {
                            placeholder: "Cannot edit this",
                            disabled: true,
                            value: String::new(),
                            oninput: move |_| {},
                        }
                    }
                }
            }

            ComponentBlock { name: "EqIcon",
                div { class: "flex items-center gap-6",
                    div { class: "flex items-center gap-2",
                        EqIcon { size: IconSize::Sm,
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                stroke: "currentColor",
                                path { d: "M12 4.5v15m7.5-7.5h-15" }
                            }
                        }
                        span { class: "text-sm text-[var(--color-label-secondary)]", "Sm" }
                    }
                    div { class: "flex items-center gap-2",
                        EqIcon { size: IconSize::Md,
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                stroke: "currentColor",
                                path { d: "M12 4.5v15m7.5-7.5h-15" }
                            }
                        }
                        span { class: "text-sm text-[var(--color-label-secondary)]", "Md (default)" }
                    }
                    div { class: "flex items-center gap-2",
                        EqIcon { size: IconSize::Lg,
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                stroke: "currentColor",
                                path { d: "M12 4.5v15m7.5-7.5h-15" }
                            }
                        }
                        span { class: "text-sm text-[var(--color-label-secondary)]", "Lg" }
                    }
                    div { class: "flex items-center gap-2",
                        EqIcon { size: IconSize::Lg, muted: true,
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                stroke: "currentColor",
                                path { d: "M12 4.5v15m7.5-7.5h-15" }
                            }
                        }
                        span { class: "text-sm text-[var(--color-label-secondary)]", "Lg muted" }
                    }
                }
            }
        }
    }
}

// ── Molecules ────────────────────────────────────────────────────────

#[component]
fn ShowcaseMolecules() -> Element {
    rsx! {
        Section { title: "Molecules",

            ComponentBlock { name: "EqCard",
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    EqCard {
                        EqCardHeader { "Card Title" }
                        EqCardBody { "A card with header, body, and footer sections." }
                        EqCardFooter { "Footer content" }
                    }
                    EqCard {
                        EqCardBody { "A minimal card — body only, no header or footer." }
                    }
                }
            }
        }
    }
}

// ── Organisms ────────────────────────────────────────────────────────

#[component]
fn ShowcaseOrganisms() -> Element {
    rsx! {
        Section { title: "Organisms",

            ComponentBlock { name: "EqHeader",
                div { class: "space-y-4",
                    div { class: "rounded-lg overflow-hidden border border-[var(--color-card-border)]",
                        EqHeader {
                            nav: rsx! {
                                li { a { href: "#", class: "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition", "Home" } }
                                li { a { href: "#", class: "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition", "About" } }
                                li { a { href: "#", class: "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition", "Contact" } }
                            }
                        }
                    }
                    p { class: "text-sm text-[var(--color-label-secondary)]",
                        "Header with nav links passed as an Element prop."
                    }
                }
            }

            ComponentBlock { name: "EqHeroShell",
                EqHeroShell {
                    title: "Hero Shell Title",
                    subtitle: "A tagline or subtitle goes here.",
                }
            }

            ComponentBlock { name: "EqPageSection",
                div { class: "space-y-4",
                    EqPageSection {
                        title: "Section Title",
                        description: "A description of this section with some context.",
                    }
                    EqPageSection {
                        title: "Section with Children",
                        description: "This section has extra content below.",
                        div { class: "mt-4 p-4 rounded-lg bg-[var(--color-card)]/40",
                            "Child content inside a PageSection."
                        }
                    }
                }
            }

            ComponentBlock { name: "EqFooter",
                div { class: "rounded-lg overflow-hidden border border-[var(--color-card-border)]",
                    EqFooter {}
                }
            }

            ComponentBlock { name: "EqAppShell",
                p { class: "text-sm text-[var(--color-label-secondary)]",
                    "EqAppShell wraps header + footer + children into a full page layout. It is the outermost layout component — see a live example in any consuming app."
                }
            }
        }
    }
}
