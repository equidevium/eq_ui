use dioxus::prelude::*;
use eq_ui::atoms::*;
use eq_ui::molecules::*;
use eq_ui::organisms::*;
use eq_ui::{UI_BUTTONS_CSS, UI_INDEX_CSS, UI_TAILWIND_CSS};
use eq_ui::eq_theme::EqTheme;

fn main() {
    dioxus::launch(App);
}

// ── Theme ───────────────────────────────────────────────────────────

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

// ── Component tree data ─────────────────────────────────────────────

fn build_component_tree() -> Vec<TreeNode> {
    vec![
        TreeNode::new_with_children("atoms", "Atoms", vec![
            TreeNode::new("text", "EqText"),
            TreeNode::new("label", "EqLabel"),
            TreeNode::new("link", "EqLink"),
            TreeNode::new("input", "EqInput"),
            TreeNode::new("image", "EqImage"),
            TreeNode::new("icon", "EqIcon"),
            TreeNode::new("scrollable-space", "EqScrollableSpace"),
            TreeNode::new("divider", "EqDivider"),
        ]),
        TreeNode::new_with_children("molecules", "Molecules", vec![
            TreeNode::new("image-card", "EqImageCard"),
            TreeNode::new("carousel", "EqCarousel"),
            TreeNode::new("card", "EqCard"),
            TreeNode::new("tree", "EqTree"),
        ]),
        TreeNode::new_with_children("organisms", "Organisms", vec![
            TreeNode::new("header", "EqHeader"),
            TreeNode::new("hero-shell", "EqHeroShell"),
            TreeNode::new("page-section", "EqPageSection"),
            TreeNode::new("footer", "EqFooter"),
            TreeNode::new("app-shell", "EqAppShell"),
        ]),
    ]
}

// ── App ─────────────────────────────────────────────────────────────

#[component]
fn App() -> Element {
    let _theme = EqTheme::use_theme_provider();
    let mut selected = use_signal(|| Option::<String>::None);

    rsx! {
        document::Link { rel: "stylesheet", href: UI_TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: UI_INDEX_CSS }
        document::Link { rel: "stylesheet", href: UI_BUTTONS_CSS }

        EqThemeRenderer {}

        EqAppShell {
            header: rsx! {
                EqHeader {
                    site_title: "EqPlayground",
                    nav: rsx! {
                        li { ThemeSwitcher {} }
                    },
                }
            },
            footer: rsx! { EqFooter {} },

            // Two-panel layout
            div { class: "flex min-h-[calc(100vh-8rem)]",
                // Left sidebar
                aside { class: "w-64 shrink-0 border-r border-[var(--color-card-border)] p-3 flex flex-col",
                    EqScrollableSpace {
                        EqTree {
                            nodes: build_component_tree(),
                            selected: selected(),
                            on_select: move |id: String| selected.set(Some(id)),
                        }
                    }
                }

                // Right preview panel
                div { class: "flex-1 overflow-y-auto",
                    PreviewPanel { selected: selected() }
                }
            }
        }
    }
}

// ── Preview router ──────────────────────────────────────────────────

#[component]
fn PreviewPanel(selected: Option<String>) -> Element {
    match selected.as_deref() {
        // Atoms
        Some("text")             => rsx! { DemoEqText {} },
        Some("label")            => rsx! { DemoEqLabel {} },
        Some("link")             => rsx! { DemoEqLink {} },
        Some("input")            => rsx! { DemoEqInput {} },
        Some("image")            => rsx! { DemoEqImage {} },
        Some("icon")             => rsx! { DemoEqIcon {} },
        Some("scrollable-space") => rsx! { DemoEqScrollableSpace {} },
        Some("divider")          => rsx! { DemoEqDivider {} },

        // Molecules
        Some("image-card") => rsx! { DemoEqImageCard {} },
        Some("carousel")   => rsx! { DemoEqCarousel {} },
        Some("card")        => rsx! { DemoEqCard {} },
        Some("tree")        => rsx! { DemoEqTree {} },

        // Organisms
        Some("header")       => rsx! { DemoEqHeader {} },
        Some("hero-shell")   => rsx! { DemoEqHeroShell {} },
        Some("page-section") => rsx! { DemoEqPageSection {} },
        Some("footer")       => rsx! { DemoEqFooter {} },
        Some("app-shell")    => rsx! { DemoEqAppShell {} },

        _ => rsx! {
            div { class: "flex flex-col items-center justify-center h-full min-h-[60vh] gap-4 text-[var(--color-label-secondary)]",
                svg {
                    class: "size-16 opacity-30",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke_width: "1.5",
                    stroke: "currentColor",
                    path { d: "M3.75 6A2.25 2.25 0 0 1 6 3.75h2.25A2.25 2.25 0 0 1 10.5 6v2.25a2.25 2.25 0 0 1-2.25 2.25H6a2.25 2.25 0 0 1-2.25-2.25V6ZM3.75 15.75A2.25 2.25 0 0 1 6 13.5h2.25a2.25 2.25 0 0 1 2.25 2.25V18a2.25 2.25 0 0 1-2.25 2.25H6A2.25 2.25 0 0 1 3.75 18v-2.25ZM13.5 6a2.25 2.25 0 0 1 2.25-2.25H18A2.25 2.25 0 0 1 20.25 6v2.25A2.25 2.25 0 0 1 18 10.5h-2.25a2.25 2.25 0 0 1-2.25-2.25V6ZM13.5 15.75a2.25 2.25 0 0 1 2.25-2.25H18a2.25 2.25 0 0 1 2.25 2.25V18A2.25 2.25 0 0 1 18 20.25h-2.25a2.25 2.25 0 0 1-2.25-2.25v-2.25Z" }
                }
                EqText { variant: TextVariant::Muted, "Select a component from the sidebar" }
            }
        },
    }
}

// ── Shared demo wrapper ─────────────────────────────────────────────

#[component]
fn DemoSection(title: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "p-8 space-y-6",
            h2 { class: "text-2xl font-semibold text-[var(--color-label-primary)] border-b border-[var(--color-card-border)] pb-2",
                "{title}"
            }
            {children}
        }
    }
}

// ── Atom demos ──────────────────────────────────────────────────────

#[component]
fn DemoEqText() -> Element {
    rsx! {
        DemoSection { title: "EqText",
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
    }
}

#[component]
fn DemoEqLabel() -> Element {
    rsx! {
        DemoSection { title: "EqLabel",
            div { class: "space-y-2",
                EqLabel { for_id: "demo", "Form label (for=\"demo\")" }
                EqLabel { "Label without for attribute" }
            }
        }
    }
}

#[component]
fn DemoEqLink() -> Element {
    rsx! {
        DemoSection { title: "EqLink",
            div { class: "flex gap-6",
                EqLink { href: "#".to_string(), "Default link" }
                EqLink { href: "https://example.com".to_string(), "External link" }
            }
        }
    }
}

#[component]
fn DemoEqInput() -> Element {
    let mut demo_input = use_signal(|| String::new());
    let mut demo_email = use_signal(|| String::new());
    let mut demo_textarea = use_signal(|| String::new());

    rsx! {
        DemoSection { title: "EqInput",
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
    }
}

#[component]
fn DemoEqImage() -> Element {
    rsx! {
        DemoSection { title: "EqImage",
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
    }
}

#[component]
fn DemoEqIcon() -> Element {
    rsx! {
        DemoSection { title: "EqIcon",
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

#[component]
fn DemoEqScrollableSpace() -> Element {
    rsx! {
        DemoSection { title: "EqScrollableSpace",
            p { class: "text-sm text-[var(--color-label-secondary)] mb-4",
                "A scrollable container with a fixed height. The content below overflows and scrolls."
            }
            div { class: "w-80 h-48 border border-[var(--color-card-border)] rounded-lg",
                EqScrollableSpace {
                    div { class: "p-4 space-y-3",
                        for i in 1..=20 {
                            p {
                                key: "{i}",
                                class: "text-sm text-[var(--color-label)]",
                                "Scrollable item {i}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DemoEqDivider() -> Element {
    rsx! {
        DemoSection { title: "EqDivider",
            div { class: "space-y-2 max-w-lg",
                span { class: "text-sm font-medium text-[var(--color-label-bold)]", "Solid (default)" }
                EqDivider {}

                span { class: "text-sm font-medium text-[var(--color-label-bold)]", "Dashed" }
                EqDivider { variant: DividerVariant::Dashed }

                span { class: "text-sm font-medium text-[var(--color-label-bold)]", "Dotted" }
                EqDivider { variant: DividerVariant::Dotted }

                span { class: "text-sm font-medium text-[var(--color-label-bold)]", "Thick" }
                EqDivider { weight: DividerWeight::Thick }

                span { class: "text-sm font-medium text-[var(--color-label-bold)]", "Extra Thick" }
                EqDivider { weight: DividerWeight::ExtraThick }

                span { class: "text-sm font-medium text-[var(--color-label-bold)]", "Dashed + Thick" }
                EqDivider { variant: DividerVariant::Dashed, weight: DividerWeight::Thick }

                span { class: "text-sm font-medium text-[var(--color-label-bold)]", "Spacer (invisible)" }
                div { class: "rounded-lg bg-[var(--color-card)]/30 p-2 text-sm text-[var(--color-label-secondary)]", "Content above" }
                EqDivider { variant: DividerVariant::Spacer, spacing: DividerSpacing::Wide }
                div { class: "rounded-lg bg-[var(--color-card)]/30 p-2 text-sm text-[var(--color-label-secondary)]", "Content below (wide spacer between)" }

                span { class: "text-sm font-medium text-[var(--color-label-bold)]", "Compact spacing" }
                EqDivider { spacing: DividerSpacing::Compact }

                span { class: "text-sm font-medium text-[var(--color-label-bold)]", "Wide spacing" }
                EqDivider { spacing: DividerSpacing::Wide }
            }
        }
    }
}

// ── Molecule demos ──────────────────────────────────────────────────

#[component]
fn DemoEqImageCard() -> Element {
    rsx! {
        DemoSection { title: "EqImageCard",
            div { class: "space-y-8",
                div { class: "space-y-2",
                    span { class: "text-sm font-medium text-[var(--color-label-bold)]", "Caption Below" }
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
                div { class: "space-y-2",
                    span { class: "text-sm font-medium text-[var(--color-label-bold)]", "Overlay" }
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
            }
        }
    }
}

#[component]
fn DemoEqCarousel() -> Element {
    rsx! {
        DemoSection { title: "EqCarousel",
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
    }
}

#[component]
fn DemoEqCard() -> Element {
    rsx! {
        DemoSection { title: "EqCard",
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

#[component]
fn DemoEqTree() -> Element {
    let mut selected = use_signal(|| Option::<String>::None);

    let tree = vec![
        TreeNode::new_with_children("demo-branch-a", "Branch A", vec![
            TreeNode::new("leaf-1", "Leaf 1"),
            TreeNode::new("leaf-2", "Leaf 2"),
            TreeNode::new_with_children("nested-branch", "Nested Branch", vec![
                TreeNode::new("deep-leaf", "Deep Leaf"),
            ]),
        ]),
        TreeNode::new_with_children("demo-branch-b", "Branch B", vec![
            TreeNode::new("leaf-3", "Leaf 3"),
            TreeNode::new("leaf-4", "Leaf 4"),
        ]),
    ];

    rsx! {
        DemoSection { title: "EqTree",
            div { class: "flex gap-6",
                div { class: "w-64 h-64 flex flex-col border border-[var(--color-card-border)] rounded-lg p-3",
                    EqScrollableSpace {
                        EqTree {
                            nodes: tree,
                            selected: selected(),
                            on_select: move |id: String| selected.set(Some(id)),
                        }
                    }
                }
                div { class: "flex-1 flex items-center justify-center rounded-lg border border-[var(--color-card-border)] p-6 min-h-[16rem]",
                    if let Some(id) = selected() {
                        EqText { variant: TextVariant::H3, "Selected: {id}" }
                    } else {
                        EqText { variant: TextVariant::Muted, "Click a leaf node" }
                    }
                }
            }
        }
    }
}

// ── Organism demos ──────────────────────────────────────────────────

#[component]
fn DemoEqHeader() -> Element {
    rsx! {
        DemoSection { title: "EqHeader",
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
    }
}

#[component]
fn DemoEqHeroShell() -> Element {
    rsx! {
        DemoSection { title: "EqHeroShell",
            div { class: "space-y-8",
                div { class: "space-y-2",
                    span { class: "text-sm font-medium text-[var(--color-label-bold)]", "Default" }
                    EqHeroShell {
                        title: "Hero Shell Title",
                        subtitle: "A tagline or subtitle goes here.",
                    }
                }

                div { class: "space-y-2",
                    span { class: "text-sm font-medium text-[var(--color-label-bold)]", "With background image" }
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

                div { class: "space-y-2",
                    span { class: "text-sm font-medium text-[var(--color-label-bold)]", "With custom colors" }
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

                div { class: "space-y-2",
                    span { class: "text-sm font-medium text-[var(--color-label-bold)]", "With CSS variable colors" }
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
            }
        }
    }
}

#[component]
fn DemoEqPageSection() -> Element {
    rsx! {
        DemoSection { title: "EqPageSection",
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
    }
}

#[component]
fn DemoEqFooter() -> Element {
    rsx! {
        DemoSection { title: "EqFooter",
            div { class: "rounded-lg overflow-hidden border border-[var(--color-card-border)]",
                EqFooter {}
            }
        }
    }
}

#[component]
fn DemoEqAppShell() -> Element {
    rsx! {
        DemoSection { title: "EqAppShell",
            p { class: "text-sm text-[var(--color-label-secondary)]",
                "EqAppShell wraps header + footer + children into a full page layout. It is the outermost layout component — you are looking at a live example right now. This playground itself uses EqAppShell."
            }
        }
    }
}
