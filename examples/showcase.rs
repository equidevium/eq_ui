use dioxus::prelude::*;
use eq_ui::atoms::*;
use eq_ui::molecules::*;
use eq_ui::organisms::*;
use eq_ui::{UI_BUTTONS_CSS, UI_INDEX_CSS, UI_TAILWIND_CSS};
use eq_ui::eq_theme::EqTheme;

fn main() {
    dioxus::launch(App);
}

#[component]
fn ThemeSwitcher() -> Element {
    let mut theme = EqTheme::use_theme();

    rsx! {
        select {
            class: "rounded-md bg-[var(--color-card)] text-[var(--color-label-primary)] border border-[var(--color-card-border)] px-2 py-1 text-sm",
            value: format!("{:?}", *theme.read()),
            onchange: move |evt: Event<FormData>| {
                let new_theme = match evt.value().as_str() {
                    "Unghosty"      => EqTheme::Unghosty,
                    "Burgundy"      => EqTheme::Burgundy,
                    "Gold"          => EqTheme::Gold,
                    "PurplePink"    => EqTheme::PurplePink,
                    "Monochrome"    => EqTheme::Monochrome,
                    "Watermelon"    => EqTheme::Watermelon,
                    "Sunset"        => EqTheme::Sunset,
                    "Ocean"         => EqTheme::Ocean,
                    "Spacetime"     => EqTheme::Spacetime,
                    "Gruvbox"       => EqTheme::Gruvbox,
                    "Monokai"       => EqTheme::Monokai,
                    "Hellas"        => EqTheme::Hellas,
                    "Egypt"         => EqTheme::Egypt,
                    "Dometrain"     => EqTheme::Dometrain,
                    "Catppuccin"    => EqTheme::Catppuccin,
                    "Dracula"       => EqTheme::Dracula,
                    "Nord"          => EqTheme::Nord,
                    "OneDark"       => EqTheme::OneDark,
                    "RosePine"      => EqTheme::RosePine,
                    "SolarizedDark" => EqTheme::SolarizedDark,
                    "TokyoNight"    => EqTheme::TokyoNight,
                    _             => EqTheme::Unghosty,
                };
                theme.set(new_theme);
            },

            for (name, _variant) in EqTheme::build_in_variants() {
                option { value: "{name}", "{name}" }
            }
        }
    }
}

#[component]
pub fn EqThemeRenderer() -> Element {
    let theme = EqTheme::use_theme();

    let (key, css) = match &*theme.read() {
        EqTheme::Custom(css) => ("custom".to_string(), css.clone()),
        other => {
            let name = format!("{:?}", other);
            let content = other.css_content().unwrap_or("").to_string();
            (name, content)
        }
    };

    rsx! {
        style { key: "{key}", dangerous_inner_html: "{css}" }
    }
}

// Read CSS from wherever — file, string, user input, etc. TODO:: this still does not work yet.
// Inside a component's event handler or effect:
// EqTheme::set_custom_theme(r#"
//     :root {
//         --color-primary-dark: #1a0a0a;
//         --color-accent-primary: #ff6600;
//     }
// "#.to_string());

// Set a built-in theme directly
// EqTheme::set_theme(EqTheme::Ocean);

// Or in a component on mount:
// #[component]
// fn MyPage() -> Element {
//     use_effect(|| {
//         EqTheme::set_theme(EqTheme::Burgundy);
//     });
//     rsx! { /* ... */ }
// }

#[component]
fn App() -> Element {

    let _theme = EqTheme::use_theme_provider();

    rsx! {
        document::Link { rel: "stylesheet", href: UI_TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: UI_INDEX_CSS }
        document::Link { rel: "stylesheet", href: UI_BUTTONS_CSS }

        EqThemeRenderer {}

        EqAppShell {
            header: rsx! {
                EqHeader {
                    site_title: "eq_ui showcase (most of the samples to be produced for the docs site later on :P )",

                    nav: rsx! {
                        li { a { href: "#atoms", class: "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition", "Atoms" } }
                        li { a { href: "#molecules", class: "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition", "Molecules" } }
                        li { a { href: "#organisms", class: "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition", "Organisms" } }
                        li { ThemeSwitcher {} }
                    },
                }
            },
            footer: rsx! { EqFooter {} },

            // children go inside CONTAINER_LAYOUT automatically
            div { class: "space-y-16",
                ShowcaseAtoms {}
                ShowcaseMolecules {}
                ShowcaseOrganisms {}
            }
        }
    }
}


// ── Helpers ──────────────────────────────────────────────────────────

#[component]
fn Section(title: &'static str, id: Option<&'static str>, children: Element) -> Element {
    rsx! {
        section { id: id.unwrap_or_default(), class: "space-y-6 scroll-mt-20",
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
            h3 { class: "text-lg font-medium text-[var(--color-label-bold)]", "{name}" }
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
        Section { title: "Atoms", id: "atoms",

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

            ComponentBlock { name: "EqImage",
                div { class: "space-y-6",
                    div { class: "space-y-2",
                        span { class: "text-sm text-[var(--color-label-secondary)]",
                            "Small — default ratio, cover"
                        }
                        EqImage {
                            src: "https://picsum.photos/seed/eq-sm/400/300",
                            alt: "Small placeholder",
                            size: AtomImageSize::Sm,
                        }
                    }
                    div { class: "space-y-2",
                        span { class: "text-sm text-[var(--color-label-secondary)]",
                            "Medium — 16:9, rounded"
                        }
                        EqImage {
                            src: "https://picsum.photos/seed/eq-md/800/450",
                            alt: "Medium 16:9 placeholder",
                            size: AtomImageSize::Md,
                            aspect_ratio: AspectRatio::Ratio16_9,
                            rounded: true,
                        }
                    }
                    div { class: "space-y-2",
                        span { class: "text-sm text-[var(--color-label-secondary)]",
                            "Large — square, contain"
                        }
                        EqImage {
                            src: "https://picsum.photos/seed/eq-lg/600/600",
                            alt: "Large square placeholder",
                            size: AtomImageSize::Lg,
                            aspect_ratio: AspectRatio::Square,
                            object_fit: ObjectFit::Contain,
                            rounded: true,
                        }
                    }
                    div { class: "space-y-2",
                        span { class: "text-sm text-[var(--color-label-secondary)]",
                            "Full width — 4:3, cover"
                        }
                        EqImage {
                            src: "https://picsum.photos/seed/eq-full/1200/900",
                            alt: "Full width placeholder",
                            size: AtomImageSize::Full,
                            aspect_ratio: AspectRatio::Ratio4_3,
                            object_fit: ObjectFit::Cover,
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
                        span { class: "text-sm text-[var(--color-label-secondary)]",
                            "Sm"
                        }
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
                        span { class: "text-sm text-[var(--color-label-secondary)]",
                            "Md (default)"
                        }
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
                        span { class: "text-sm text-[var(--color-label-secondary)]",
                            "Lg"
                        }
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
                        span { class: "text-sm text-[var(--color-label-secondary)]",
                            "Lg muted"
                        }
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
        Section { title: "Molecules", id: "molecules",

            ComponentBlock { name: "EqImageCard — Caption Below",
                EqImageCard {
                    src: "https://picsum.photos/seed/eq-card1/800/500",
                    alt: "Mountain landscape",
                    mode: CaptionMode::Below,
                    size: AtomImageSize::Lg,
                    aspect_ratio: AspectRatio::Ratio16_9,
                    rounded: true,
                    title: "Alpine Meadow",
                    description: "A serene landscape captured during the golden hour.",
                    attribution: "Photo by Jane Doe",
                }
            }

            ComponentBlock { name: "EqImageCard — Overlay",
                EqImageCard {
                    src: "https://picsum.photos/seed/eq-card2/800/500",
                    alt: "Ocean waves",
                    mode: CaptionMode::Overlay,
                    size: AtomImageSize::Lg,
                    aspect_ratio: AspectRatio::Ratio16_9,
                    rounded: true,
                    title: "Ocean Power",
                    description: "The raw energy of nature captured in motion.",
                }
            }

            ComponentBlock { name: "EqCarousel",
                div { class: "max-w-lg",
                    EqCarousel {
                        slides: vec![
                            rsx! {
                                EqImageCard {
                                    src: "https://picsum.photos/seed/carousel1/800/450",
                                    alt: "Slide one",
                                    mode: CaptionMode::Overlay,
                                    size: AtomImageSize::Full,
                                    aspect_ratio: AspectRatio::Ratio16_9,
                                    rounded: true,
                                    title: "First Slide",
                                    description: "A beautiful mountain landscape.",
                                }
                            },
                            rsx! {
                                EqImageCard {
                                    src: "https://picsum.photos/seed/carousel2/800/450",
                                    alt: "Slide two",
                                    mode: CaptionMode::Overlay,
                                    size: AtomImageSize::Full,
                                    aspect_ratio: AspectRatio::Ratio16_9,
                                    rounded: true,
                                    title: "Second Slide",
                                    description: "Waves crashing on the shore.",
                                }
                            },
                            rsx! {
                                EqImageCard {
                                    src: "https://picsum.photos/seed/carousel3/800/450",
                                    alt: "Slide three",
                                    mode: CaptionMode::Overlay,
                                    size: AtomImageSize::Full,
                                    aspect_ratio: AspectRatio::Ratio16_9,
                                    rounded: true,
                                    title: "Third Slide",
                                    description: "A dense forest at dawn.",
                                }
                            },
                        ],
                    }
                }
            }

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
        Section { title: "Organisms", id: "organisms",

            ComponentBlock { name: "EqHeader",
                div { class: "space-y-4",
                    div { class: "rounded-lg overflow-hidden border border-[var(--color-card-border)]",
                        EqHeader {
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

            ComponentBlock { name: "EqHeroShell with background image",
                EqHeroShell {
                    title: "Hero Shell Title",
                    subtitle: "A tagline or subtitle goes here.",
                    background: rsx! {
                        EqImage {
                            src: "https://picsum.photos/seed/eq-full/1200/900",
                            alt: "Full width placeholder",
                            size: AtomImageSize::Full,
                            aspect_ratio: AspectRatio::Ratio4_3,
                            object_fit: ObjectFit::Cover,
                        }
                    }
                }
            }

            ComponentBlock { name: "EqHeroShell with background image and custom colors",
                EqHeroShell {
                    title: "Hero Shell Title",
                    subtitle: "A tagline or subtitle goes here.",
                    title_color: "#ff6b6b",
                    subtitle_color: "#ffd93d",
                    background: rsx! {
                        EqImage {
                            src: "https://picsum.photos/seed/eq-full/1200/900",
                            alt: "Full width placeholder",
                            size: AtomImageSize::Full,
                            aspect_ratio: AspectRatio::Ratio4_3,
                            object_fit: ObjectFit::Cover,
                        }
                    }
                }
            }

            ComponentBlock { name: "EqHeroShell with background image and custom colors from CSS variables from a different theme",
                EqHeroShell {
                    title: "Hero Shell Title",
                    subtitle: "A tagline or subtitle goes here.",
                    title_color: "#ff6b6b",
                    subtitle_color: "var(--color-label-bold)",
                    background: rsx! {
                        EqImage {
                            src: "https://picsum.photos/seed/eq-full/1200/900",
                            alt: "Full width placeholder",
                            size: AtomImageSize::Full,
                            aspect_ratio: AspectRatio::Ratio4_3,
                            object_fit: ObjectFit::Cover,
                        }
                    }
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
