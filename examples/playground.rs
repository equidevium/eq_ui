use dioxus::prelude::*;
use eq_ui::atoms::*;
use eq_ui::eq_theme::EqTheme;
use eq_ui::molecules::*;
use eq_ui::organisms::*;
use wasm_bindgen_futures::spawn_local;
use eq_ui::{UI_BUTTONS_CSS, UI_INDEX_CSS, UI_TAILWIND_CSS};

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
                    "Unghosty" => EqTheme::Unghosty,
                    "Burgundy" => EqTheme::Burgundy,
                    "Gold" => EqTheme::Gold,
                    "PurplePink" => EqTheme::PurplePink,
                    "Monochrome" => EqTheme::Monochrome,
                    "Watermelon" => EqTheme::Watermelon,
                    "Sunset" => EqTheme::Sunset,
                    "Ocean" => EqTheme::Ocean,
                    "Spacetime" => EqTheme::Spacetime,
                    "Gruvbox" => EqTheme::Gruvbox,
                    "Monokai" => EqTheme::Monokai,
                    "Hellas" => EqTheme::Hellas,
                    "Egypt" => EqTheme::Egypt,
                    "Dometrain" => EqTheme::Dometrain,
                    "Catppuccin" => EqTheme::Catppuccin,
                    "Dracula" => EqTheme::Dracula,
                    "Nord" => EqTheme::Nord,
                    "OneDark" => EqTheme::OneDark,
                    "RosePine" => EqTheme::RosePine,
                    "SolarizedDark" => EqTheme::SolarizedDark,
                    "TokyoNight" => EqTheme::TokyoNight,
                    _ => EqTheme::Unghosty,
                };
                theme.set(new_theme);
            },

            for (name , _variant) in EqTheme::build_in_variants() {
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
        TreeNode::new_with_children(
            "atoms",
            "Atoms",
            vec![
                TreeNode::new("text", "EqText"),
                TreeNode::new("label", "EqLabel"),
                TreeNode::new("link", "EqLink"),
                TreeNode::new("input", "EqInput"),
                TreeNode::new("image", "EqImage"),
                TreeNode::new("icon", "EqIcon"),
                TreeNode::new("scrollable-space", "EqScrollableSpace"),
                TreeNode::new("divider", "EqDivider"),
                TreeNode::new("video", "EqVideo"),
                TreeNode::new("checkbox", "EqCheckbox"),
            ],
        ),
        TreeNode::new_with_children(
            "molecules",
            "Molecules",
            vec![
                TreeNode::new("image-card", "EqImageCard"),
                TreeNode::new("carousel", "EqCarousel"),
                TreeNode::new("card", "EqCard"),
                TreeNode::new("tree", "EqTree"),
                TreeNode::new("accordion", "EqAccordion"),
            ],
        ),
        TreeNode::new_with_children(
            "organisms",
            "Organisms",
            vec![
                TreeNode::new("header", "EqHeader"),
                TreeNode::new("hero-shell", "EqHeroShell"),
                TreeNode::new("page-section", "EqPageSection"),
                TreeNode::new("footer", "EqFooter"),
                TreeNode::new("app-shell", "EqAppShell"),
                TreeNode::new("grid", "EqGrid"),
                TreeNode::new("grid-dnd", "EqGrid Drag & Drop"),
                TreeNode::new("grid-reorder", "EqGrid Reorder"),
            ],
        ),
        TreeNode::new_with_children(
            "theming",
            "Theming",
            vec![TreeNode::new("theme-showcase", "Theme Showcase")],
        ),
    ]
}

// ── App ─────────────────────────────────────────────────────────────

#[component]
fn App() -> Element {
    let _theme = EqTheme::use_theme_provider();
    let mut selected = use_signal(|| Option::<String>::None);
    let mut sidebar_open = use_signal(|| false);

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
                        // Hamburger button — mobile only
                        li { class: "md:hidden",
                            button {
                                class: "p-2 rounded-md text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] active:text-[var(--color-label-primary)] transition",
                                onclick: move |_| sidebar_open.set(!sidebar_open()),
                                svg {
                                    class: "size-5",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke_width: "2",
                                    stroke: "currentColor",
                                    if sidebar_open() {
                                        // X icon
                                        path { d: "M6 18 18 6M6 6l12 12" }
                                    } else {
                                        // Hamburger icon
                                        path { d: "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" }
                                    }
                                }
                            }
                        }
                        li { ThemeSwitcher {} }
                    },
                }
            },
            footer: rsx! {
                EqFooter {}
            },

            // Two-panel layout
            div { class: "flex min-h-[calc(100vh-8rem)] relative",

                // Mobile overlay backdrop
                if sidebar_open() {
                    div {
                        class: "fixed inset-0 z-30 bg-black/50 md:hidden",
                        onclick: move |_| sidebar_open.set(false),
                    }
                }

                // Sidebar — overlay on mobile, static on desktop
                aside {
                    class: if sidebar_open() {
                        "fixed inset-y-0 left-0 z-40 w-64 bg-[var(--color-primary-dark)] border-r border-[var(--color-card-border)] p-3 flex flex-col pt-16 md:pt-3 md:relative md:inset-auto md:z-auto"
                    } else {
                        "hidden md:flex w-64 shrink-0 border-r border-[var(--color-card-border)] p-3 flex-col"
                    },
                    EqScrollableSpace {
                        EqTree {
                            nodes: build_component_tree(),
                            selected: selected(),
                            on_select: move |id: String| {
                                selected.set(Some(id));
                                sidebar_open.set(false);
                            },
                            show_count: true,
                        }
                    }
                }

                // Right preview panel — full width on mobile
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
        Some("text") => rsx! {
            DemoEqText {}
            // Atoms
        },
        Some("label") => rsx! {
            DemoEqLabel {}
        },
        Some("link") => rsx! {
            DemoEqLink {}
        },
        Some("input") => rsx! {
            DemoEqInput {}
        },
        Some("image") => rsx! {
            DemoEqImage {}
        },
        Some("icon") => rsx! {
            DemoEqIcon {}
        },
        Some("scrollable-space") => rsx! {
            DemoEqScrollableSpace {}
        },
        Some("divider") => rsx! {
            DemoEqDivider {}
        },
        Some("video") => rsx! {
            DemoEqVideo {}
        },
        Some("checkbox") => rsx! {
            DemoEqCheckbox {}
        },

        // Molecules
        Some("image-card") => rsx! {
            DemoEqImageCard {}

            // Molecules
        },
        Some("carousel") => rsx! {
            DemoEqCarousel {}
        },
        Some("card") => rsx! {
            DemoEqCard {}
        },
        Some("tree") => rsx! {
            DemoEqTree {}
        },
        Some("accordion") => rsx! {
            DemoEqAccordion {}
        },

        // Organisms
        Some("header") => rsx! {
            DemoEqHeader {}

            // Organisms
        },
        Some("hero-shell") => rsx! {
            DemoEqHeroShell {}
        },
        Some("page-section") => rsx! {
            DemoEqPageSection {}
        },
        Some("footer") => rsx! {
            DemoEqFooter {}
        },
        Some("app-shell") => rsx! {
            DemoEqAppShell {}
        },
        Some("grid") => rsx! {
            DemoEqGrid {}
        },
        Some("grid-dnd") => rsx! {
            DemoEqGridDragDrop {}
        },
        Some("grid-reorder") => rsx! {
            DemoEqGridReorder {}
        },

        // Theming
        Some("theme-showcase") => rsx! {
            DemoThemeShowcase {}

            // Theming
        },

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

// ── Shared helpers ──────────────────────────────────────────────────

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

// ── Gruvbox Dark palette ────────────────────────────────────────────

const GRV_BG: &str = "#282828";
const GRV_BG_SOFT: &str = "#3c3836";
const GRV_FG: &str = "#ebdbb2";
const GRV_GREY: &str = "#928374";
const GRV_RED: &str = "#fb4934";
const GRV_GREEN: &str = "#b8bb26";
const GRV_YELLOW: &str = "#fabd2f";
const GRV_ORANGE: &str = "#fe8019";
const GRV_PURPLE: &str = "#d3869b";
const GRV_AQUA: &str = "#8ec07c";
const GRV_BLUE: &str = "#7cc6d4";

const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

/// Append a char to `buf`, HTML-escaping &, <, >.
fn push_escaped(buf: &mut String, ch: char) {
    match ch {
        '&' => buf.push_str("&amp;"),
        '<' => buf.push_str("&lt;"),
        '>' => buf.push_str("&gt;"),
        _ => buf.push(ch),
    }
}

/// HTML-escape a full string slice.
fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        push_escaped(&mut out, ch);
    }
    out
}

/// Tokenize a Rust code string into syntax-highlighted HTML spans.
fn highlight_rust(code: &str) -> String {
    let chars: Vec<char> = code.chars().collect();
    let len = chars.len();
    let mut out = String::with_capacity(code.len() * 2);
    let mut i = 0;

    while i < len {
        let ch = chars[i];

        // Line comments
        if ch == '/' && i + 1 < len && chars[i + 1] == '/' {
            out.push_str(&format!("<span style=\"color:{}\">", GRV_GREY));
            while i < len && chars[i] != '\n' {
                push_escaped(&mut out, chars[i]);
                i += 1;
            }
            out.push_str("</span>");
            continue;
        }

        // String literals
        if ch == '"' {
            out.push_str(&format!("<span style=\"color:{}\">", GRV_GREEN));
            push_escaped(&mut out, ch);
            i += 1;
            while i < len && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < len {
                    push_escaped(&mut out, chars[i]);
                    i += 1;
                    push_escaped(&mut out, chars[i]);
                    i += 1;
                } else {
                    push_escaped(&mut out, chars[i]);
                    i += 1;
                }
            }
            if i < len {
                push_escaped(&mut out, chars[i]);
                i += 1;
            }
            out.push_str("</span>");
            continue;
        }

        // Numbers
        if ch.is_ascii_digit() && (i == 0 || !chars[i - 1].is_alphanumeric()) {
            out.push_str(&format!("<span style=\"color:{}\">", GRV_PURPLE));
            while i < len
                && (chars[i].is_ascii_alphanumeric() || chars[i] == '.' || chars[i] == '_')
            {
                push_escaped(&mut out, chars[i]);
                i += 1;
            }
            out.push_str("</span>");
            continue;
        }

        // Identifiers, keywords, macros, types
        if ch.is_alphabetic() || ch == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();

            // Macro (word followed by !)
            if i < len && chars[i] == '!' {
                out.push_str(&format!(
                    "<span style=\"color:{}\">{}!</span>",
                    GRV_AQUA,
                    html_escape(&word)
                ));
                i += 1;
                continue;
            }

            // Keyword
            if RUST_KEYWORDS.contains(&word.as_str()) {
                out.push_str(&format!("<span style=\"color:{}\">", GRV_ORANGE));
                out.push_str(&html_escape(&word));
                out.push_str("</span>");
                continue;
            }

            // PascalCase type/component name
            if word.len() > 1
                && word.chars().next().unwrap().is_uppercase()
                && word.chars().any(|c| c.is_lowercase())
            {
                out.push_str(&format!("<span style=\"color:{}\">", GRV_YELLOW));
                out.push_str(&html_escape(&word));
                out.push_str("</span>");
                continue;
            }

            out.push_str(&html_escape(&word));
            continue;
        }

        // Path separator ::
        if ch == ':' && i + 1 < len && chars[i + 1] == ':' {
            out.push_str(&format!("<span style=\"color:{}\">::</span>", GRV_GREY));
            i += 2;
            continue;
        }

        // Attributes #
        if ch == '#' {
            out.push_str(&format!("<span style=\"color:{}\">", GRV_RED));
            push_escaped(&mut out, ch);
            i += 1;
            // Include [...] if present
            if i < len && chars[i] == '[' {
                let mut depth = 0;
                while i < len {
                    if chars[i] == '[' {
                        depth += 1;
                    }
                    if chars[i] == ']' {
                        depth -= 1;
                    }
                    push_escaped(&mut out, chars[i]);
                    i += 1;
                    if depth == 0 {
                        break;
                    }
                }
            }
            out.push_str("</span>");
            continue;
        }

        // Everything else
        push_escaped(&mut out, ch);
        i += 1;
    }

    out
}

/// Tokenize a styles definition string into highlighted HTML.
/// Constant names (SCREAMING_CAPS) render in yellow, colons in grey,
/// and quoted Tailwind class strings render in light blue.
fn highlight_styles(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut out = String::with_capacity(input.len() * 2);
    let mut i = 0;

    while i < len {
        let ch = chars[i];

        // Quoted string → light blue (Tailwind classes)
        if ch == '"' {
            out.push_str(&format!("<span style=\"color:{}\">", GRV_BLUE));
            push_escaped(&mut out, ch);
            i += 1;
            while i < len && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < len {
                    push_escaped(&mut out, chars[i]);
                    i += 1;
                    push_escaped(&mut out, chars[i]);
                    i += 1;
                } else {
                    push_escaped(&mut out, chars[i]);
                    i += 1;
                }
            }
            if i < len {
                push_escaped(&mut out, chars[i]);
                i += 1;
            }
            out.push_str("</span>");
            continue;
        }

        // SCREAMING_CAPS identifier → yellow
        if ch.is_ascii_uppercase() || ch == '_' {
            let start = i;
            while i < len
                && (chars[i].is_ascii_uppercase() || chars[i].is_ascii_digit() || chars[i] == '_')
            {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            // Only colorize if it looks like a constant (has letters, not just underscores)
            if word.chars().any(|c| c.is_ascii_uppercase()) {
                out.push_str(&format!("<span style=\"color:{}\">", GRV_YELLOW));
                out.push_str(&html_escape(&word));
                out.push_str("</span>");
            } else {
                out.push_str(&html_escape(&word));
            }
            continue;
        }

        // Colon → grey
        if ch == ':' {
            out.push_str(&format!("<span style=\"color:{}\">:</span>", GRV_GREY));
            i += 1;
            continue;
        }

        // Slash separator → grey
        if ch == '/' {
            out.push_str(&format!("<span style=\"color:{}\">/</span>", GRV_GREY));
            i += 1;
            continue;
        }

        // Everything else (whitespace, etc.)
        push_escaped(&mut out, ch);
        i += 1;
    }

    out
}

/// Renders a Gruvbox-themed code block with Rust syntax highlighting.
#[component]
fn CodeBlock(code: String) -> Element {
    let highlighted = highlight_rust(&code);

    rsx! {
        div { class: "mt-6 space-y-2",
            EqText {
                variant: TextVariant::Caption,
                class: "font-semibold uppercase tracking-wider",
                "Example Usage"
            }
            div {
                class: "rounded-lg overflow-hidden",
                style: "border: 1px solid #6b2020;",
                pre {
                    class: "p-4 overflow-x-auto text-xs leading-relaxed font-mono",
                    style: format!("background:{};color:{};", GRV_BG, GRV_FG),
                    code { dangerous_inner_html: "{highlighted}" }
                }
            }
        }
    }
}

/// Shows which CSS constants a component uses, with Gruvbox syntax highlighting.
#[component]
fn StyleInfo(file: &'static str, styles: String) -> Element {
    let highlighted = highlight_styles(&styles);
    rsx! {
        details {
            class: "mt-4 rounded-lg overflow-hidden",
            style: "border: 1px solid #6b2020;",
            summary {
                class: "px-4 py-2 cursor-pointer text-xs font-semibold tracking-wider select-none transition",
                style: format!("background:{};color:{};", GRV_BG_SOFT, GRV_FG),
                "Default Styles — {file}"
            }
            pre {
                class: "p-4 overflow-x-auto text-xs leading-relaxed font-mono",
                style: format!("background:{};color:{};", GRV_BG, GRV_FG),
                code { dangerous_inner_html: "{highlighted}" }
            }
        }
    }
}

// Shared style constants for playground prop controls.
const PROP_ROW: &str = "flex items-center gap-3";
const PROP_LABEL: &str = "text-xs font-medium text-[var(--color-label-secondary)] w-20 shrink-0";
const PROP_CONTROL: &str = "rounded-md bg-[var(--color-card)] text-[var(--color-label-primary)] border border-[var(--color-card-border)] px-2 py-1 text-xs";

/// Styled select dropdown for prop controls.
#[component]
fn PropSelect(
    label: &'static str,
    value: String,
    options: Vec<&'static str>,
    onchange: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: PROP_ROW,
            span { class: PROP_LABEL, "{label}" }
            select {
                class: PROP_CONTROL,
                value: "{value}",
                onchange: move |evt: Event<FormData>| onchange.call(evt.value()),
                for opt in options {
                    option { value: "{opt}", "{opt}" }
                }
            }
        }
    }
}

/// Styled text input for prop controls.
#[component]
fn PropInput(
    label: &'static str,
    value: String,
    placeholder: &'static str,
    onchange: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: PROP_ROW,
            span { class: PROP_LABEL, "{label}" }
            input {
                class: "flex-1 {PROP_CONTROL}",
                r#type: "text",
                value: "{value}",
                placeholder: "{placeholder}",
                oninput: move |evt: FormEvent| onchange.call(evt.value()),
            }
        }
    }
}

/// Styled boolean toggle for prop controls.
#[component]
fn PropToggle(label: &'static str, value: bool, onchange: EventHandler<bool>) -> Element {
    rsx! {
        div { class: PROP_ROW,
            span { class: PROP_LABEL, "{label}" }
            select {
                class: PROP_CONTROL,
                value: if value { "true" } else { "false" },
                onchange: move |evt: Event<FormData>| onchange.call(evt.value() == "true"),
                option { value: "false", "false" }
                option { value: "true", "true" }
            }
        }
    }
}

// ── Atom demos ──────────────────────────────────────────────────────

#[component]
fn DemoEqText() -> Element {
    let mut variant_str = use_signal(|| "Body".to_string());
    let mut content = use_signal(|| "The quick brown fox jumps over the lazy dog.".to_string());

    let variant = match variant_str().as_str() {
        "H1" => TextVariant::H1,
        "H2" => TextVariant::H2,
        "H3" => TextVariant::H3,
        "Muted" => TextVariant::Muted,
        "Caption" => TextVariant::Caption,
        "Emphasis" => TextVariant::Emphasis,
        "Mono" => TextVariant::Mono,
        _ => TextVariant::Body,
    };

    let styles = "H1:       \"text-3xl md:text-4xl font-semibold tracking-tight text-[var(--color-label-primary)]\"\nH2:       \"text-2xl font-semibold tracking-tight text-[var(--color-label-primary)]\"\nH3:       \"text-lg font-semibold text-[var(--color-label-primary)]\"\nBODY:     \"text-base leading-relaxed text-[var(--color-label-primary)]\"\nMUTED:    \"text-base leading-relaxed text-[var(--color-label-secondary)]\"\nCAPTION:  \"text-sm text-[var(--color-label-secondary)]\"\nEMPHASIS: \"text-[var(--color-label-bold)]\"\nMONO:     \"font-mono text-sm text-[var(--color-label-secondary)]\"".to_string();

    let code = r#"EqText { variant: TextVariant::H1, "Heading 1" }

EqText { variant: TextVariant::Body,
    "Body text — the default variant."
}

EqText { variant: TextVariant::Muted,
    "Muted — secondary colour."
}"#
    .to_string();

    rsx! {
        DemoSection { title: "EqText",
            // Interactive controls
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                span { class: "text-xs font-semibold uppercase tracking-wider text-[var(--color-label-secondary)]",
                    "Props"
                }
                PropSelect {
                    label: "variant",
                    value: variant_str(),
                    options: vec!["Body", "H1", "H2", "H3", "Muted", "Caption", "Emphasis", "Mono"],
                    onchange: move |v: String| variant_str.set(v),
                }
                PropInput {
                    label: "content",
                    value: content(),
                    placeholder: "Enter text content…",
                    onchange: move |v: String| content.set(v),
                }
            }

            // Live preview
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                EqText { variant, "{content}" }
            }

            // All variants gallery
            div { class: "space-y-3",
                EqText { variant: TextVariant::Emphasis, "All variants" }
                EqText { variant: TextVariant::H1, "Heading 1" }
                EqText { variant: TextVariant::H2, "Heading 2" }
                EqText { variant: TextVariant::H3, "Heading 3" }
                EqText { variant: TextVariant::Body, "Body text — the default variant." }
                EqText { variant: TextVariant::Muted, "Muted text — secondary colour." }
                EqText { variant: TextVariant::Caption, "Caption text" }
                EqText { variant: TextVariant::Emphasis, "Emphasis text" }
                EqText { variant: TextVariant::Mono, "Mono text — code snippets" }
            }

            StyleInfo { file: "eq_text_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

#[component]
fn DemoEqLabel() -> Element {
    let mut for_id_str = use_signal(|| "username".to_string());
    let mut content = use_signal(|| "Username".to_string());

    let for_id: &'static str = match for_id_str().as_str() {
        "email" => "email",
        "password" => "password",
        "(none)" => "",
        _ => "username",
    };

    let styles = "LABEL: \"text-sm font-medium text-[var(--color-label-primary)]\"".to_string();

    let code = r#"EqLabel { for_id: "username", "Username" }

EqLabel { "Label without for attribute" }"#
        .to_string();

    rsx! {
        DemoSection { title: "EqLabel",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "for_id",
                    value: for_id_str(),
                    options: vec!["username", "email", "password", "(none)"],
                    onchange: move |v: String| for_id_str.set(v),
                }
                PropInput {
                    label: "content",
                    value: content(),
                    placeholder: "Label text",
                    onchange: move |v: String| content.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                EqLabel { for_id, "{content}" }
            }
            StyleInfo { file: "eq_label_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

#[component]
fn DemoEqLink() -> Element {
    let mut href = use_signal(|| "https://example.com".to_string());
    let mut content = use_signal(|| "Click me".to_string());

    let styles = "LINK: \"text-[var(--color-label-primary)] underline hover:text-[var(--color-label-bold)] transition\"".to_string();

    let code = r##"EqLink { href: "#", "Internal link" }

EqLink { href: "https://example.com", "External link" }"##
        .to_string();

    rsx! {
        DemoSection { title: "EqLink",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropInput {
                    label: "href",
                    value: href(),
                    placeholder: "https://…",
                    onchange: move |v: String| href.set(v),
                }
                PropInput {
                    label: "content",
                    value: content(),
                    placeholder: "Link text",
                    onchange: move |v: String| content.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                EqLink { href: href(), "{content}" }
            }
            StyleInfo { file: "eq_link_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

#[component]
fn DemoEqInput() -> Element {
    let mut kind_str = use_signal(|| "Text".to_string());
    let mut placeholder_str = use_signal(|| "Type something…".to_string());
    let mut disabled = use_signal(|| false);
    let mut required = use_signal(|| false);
    let mut demo_value = use_signal(|| String::new());

    let kind = match kind_str().as_str() {
        "Email" => InputKind::Email,
        "Password" => InputKind::Password,
        "Textarea" => InputKind::Textarea,
        _ => InputKind::Text,
    };
    let placeholder: &'static str = match placeholder_str().as_str() {
        "you@example.com" => "you@example.com",
        "Enter password…" => "Enter password…",
        "Write a message…" => "Write a message…",
        "(none)" => "",
        _ => "Type something…",
    };

    let styles = "CONTROL:  \"w-full rounded-md border px-3 py-2 text-sm outline-none transition ...\"\nTEXTAREA: \"min-h-[120px] resize-y\"\nDISABLED: \"opacity-60 cursor-not-allowed\"".to_string();

    let code = r#"let mut value = use_signal(|| String::new());

EqInput {
    id: "email",
    kind: InputKind::Email,
    placeholder: "you@example.com",
    value: value(),
    oninput: move |e: FormEvent| value.set(e.value()),
}

EqInput {
    kind: InputKind::Textarea,
    placeholder: "Write a message…",
    value: value(),
    oninput: move |e: FormEvent| value.set(e.value()),
}

EqInput {
    placeholder: "Cannot edit",
    disabled: true,
    value: String::new(),
    oninput: move |_| {},
}"#
    .to_string();

    rsx! {
        DemoSection { title: "EqInput",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "kind",
                    value: kind_str(),
                    options: vec!["Text", "Email", "Password", "Textarea"],
                    onchange: move |v: String| kind_str.set(v),
                }
                PropSelect {
                    label: "placeholder",
                    value: placeholder_str(),
                    options: vec![
                        "Type something…",
                        "you@example.com",
                        "Enter password…",
                        "Write a message…",
                        "(none)",
                    ],
                    onchange: move |v: String| placeholder_str.set(v),
                }
                PropToggle {
                    label: "disabled",
                    value: disabled(),
                    onchange: move |v: bool| disabled.set(v),
                }
                PropToggle {
                    label: "required",
                    value: required(),
                    onchange: move |v: bool| required.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 max-w-md",
                EqInput {
                    kind,
                    placeholder,
                    disabled: disabled(),
                    required: required(),
                    value: demo_value(),
                    oninput: move |e: FormEvent| demo_value.set(e.value()),
                }
            }
            div { class: "space-y-4 max-w-md",
                EqText { variant: TextVariant::Emphasis, "All kinds" }
                for (label , k) in [
                    ("Text", InputKind::Text),
                    ("Email", InputKind::Email),
                    ("Password", InputKind::Password),
                    ("Textarea", InputKind::Textarea),
                ]
                {
                    div { class: "space-y-1",
                        EqLabel { "{label}" }
                        EqInput {
                            kind: k,
                            placeholder: "Example…",
                            value: String::new(),
                            oninput: move |_| {},
                        }
                    }
                }
            }
            StyleInfo { file: "eq_input_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

#[component]
fn DemoEqImage() -> Element {
    let mut size_str = use_signal(|| "Md".to_string());
    let mut ratio_str = use_signal(|| "Free".to_string());
    let mut fit_str = use_signal(|| "Cover".to_string());
    let mut rounded = use_signal(|| false);

    let size = match size_str().as_str() {
        "Sm" => AtomImageSize::Sm,
        "Lg" => AtomImageSize::Lg,
        "Full" => AtomImageSize::Full,
        _ => AtomImageSize::Md,
    };
    let aspect_ratio = match ratio_str().as_str() {
        "Ratio16_9" => AspectRatio::Ratio16_9,
        "Ratio4_3" => AspectRatio::Ratio4_3,
        "Square" => AspectRatio::Square,
        _ => AspectRatio::Free,
    };
    let object_fit = match fit_str().as_str() {
        "Contain" => ObjectFit::Contain,
        "Fill" => ObjectFit::Fill,
        _ => ObjectFit::Cover,
    };

    let styles = "WRAPPER: \"relative overflow-hidden\"\nSM/MD/LG/FULL: \"w-48\" / \"w-64\" / \"w-96\" / \"w-full\"\nRATIO_16_9: \"aspect-video\"\nRATIO_4_3: \"aspect-[4/3]\"\nRATIO_SQUARE: \"aspect-square\"\nOBJECT_COVER/CONTAIN/FILL: \"object-cover\" / ...\nIMAGE_ELEMENT: \"w-full h-full bg-[var(--color-card)]/40\"\nROUNDED: \"rounded-lg\"".to_string();

    let code = r#"EqImage {
    src: "https://example.com/photo.jpg",
    alt: "A scenic view",
    size: AtomImageSize::Md,
    aspect_ratio: AspectRatio::Ratio16_9,
    rounded: true,
}

EqImage {
    src: "avatar.png",
    alt: "User avatar",
    size: AtomImageSize::Sm,
    aspect_ratio: AspectRatio::Square,
    object_fit: ObjectFit::Cover,
}"#
    .to_string();

    rsx! {
        DemoSection { title: "EqImage",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "size",
                    value: size_str(),
                    options: vec!["Sm", "Md", "Lg", "Full"],
                    onchange: move |v: String| size_str.set(v),
                }
                PropSelect {
                    label: "aspect_ratio",
                    value: ratio_str(),
                    options: vec!["Free", "Ratio16_9", "Ratio4_3", "Square"],
                    onchange: move |v: String| ratio_str.set(v),
                }
                PropSelect {
                    label: "object_fit",
                    value: fit_str(),
                    options: vec!["Cover", "Contain", "Fill"],
                    onchange: move |v: String| fit_str.set(v),
                }
                PropToggle {
                    label: "rounded",
                    value: rounded(),
                    onchange: move |v: bool| rounded.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 max-w-lg",
                EqImage {
                    src: "https://picsum.photos/seed/eq-preview/800/600",
                    alt: "Preview image",
                    size,
                    aspect_ratio,
                    object_fit,
                    rounded: rounded(),
                }
            }
            StyleInfo { file: "eq_image_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

#[component]
fn DemoEqIcon() -> Element {
    let mut size_str = use_signal(|| "Md".to_string());
    let mut muted = use_signal(|| false);

    let size = match size_str().as_str() {
        "Sm" => IconSize::Sm,
        "Lg" => IconSize::Lg,
        _ => IconSize::Md,
    };

    let styles = "WRAPPER: \"inline-flex items-center justify-center shrink-0\"\nSM/MD/LG: \"size-4\" / \"size-5\" / \"size-6\"\nDEFAULT: \"text-[var(--color-label-primary)]\"\nMUTED: \"text-[var(--color-label-secondary)]\"".to_string();

    let code = "EqIcon { size: IconSize::Sm,\n    svg { /* your SVG icon */ }\n}\n\nEqIcon { size: IconSize::Lg, muted: true,\n    svg { /* dimmed icon */ }\n}".to_string();

    rsx! {
        DemoSection { title: "EqIcon",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "size",
                    value: size_str(),
                    options: vec!["Sm", "Md", "Lg"],
                    onchange: move |v: String| size_str.set(v),
                }
                PropToggle {
                    label: "muted",
                    value: muted(),
                    onchange: move |v: bool| muted.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                EqIcon { size, muted: muted(),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "2",
                        stroke: "currentColor",
                        path { d: "M12 4.5v15m7.5-7.5h-15" }
                    }
                }
            }
            div { class: "flex items-center gap-6",
                EqText { variant: TextVariant::Emphasis, "All sizes" }
                for (label , s) in [("Sm", IconSize::Sm), ("Md", IconSize::Md), ("Lg", IconSize::Lg)] {
                    div { class: "flex items-center gap-2",
                        EqIcon { size: s,
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                stroke: "currentColor",
                                path { d: "M12 4.5v15m7.5-7.5h-15" }
                            }
                        }
                        EqText { variant: TextVariant::Caption, "{label}" }
                    }
                }
            }
            StyleInfo { file: "eq_icon_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

#[component]
fn DemoEqScrollableSpace() -> Element {
    let mut item_count = use_signal(|| "20".to_string());
    let count: usize = item_count().parse().unwrap_or(20).min(200);

    let styles = "CONTAINER: \"overflow-y-auto flex-1 min-h-0\"\nSCROLLBAR: \"scrollbar-thin scrollbar-thumb-[var(--color-label-secondary)]/30 scrollbar-track-transparent\"".to_string();

    let code = "// Wrap in a flex-col container with fixed height\ndiv { class: \"h-48 flex flex-col\",\n    EqScrollableSpace {\n        // Content that overflows will scroll\n        for item in items {\n            p { \"{item}\" }\n        }\n    }\n}\n\n// Or with a custom max-height\nEqScrollableSpace { max_height: \"max-h-96\",\n    // ...children\n}".to_string();

    rsx! {
        DemoSection { title: "EqScrollableSpace",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropInput {
                    label: "item count",
                    value: item_count(),
                    placeholder: "20",
                    onchange: move |v: String| item_count.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                div { class: "w-80 h-48 flex flex-col border border-[var(--color-card-border)] rounded-lg",
                    EqScrollableSpace {
                        div { class: "p-4 space-y-3",
                            for i in 1..=count {
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
            StyleInfo { file: "eq_scrollable_space_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

#[component]
fn DemoEqDivider() -> Element {
    let mut variant_str = use_signal(|| "Solid".to_string());
    let mut weight_str = use_signal(|| "Normal".to_string());
    let mut spacing_str = use_signal(|| "Default".to_string());

    let variant = match variant_str().as_str() {
        "Dashed" => DividerVariant::Dashed,
        "Dotted" => DividerVariant::Dotted,
        "Spacer" => DividerVariant::Spacer,
        _ => DividerVariant::Solid,
    };
    let weight = match weight_str().as_str() {
        "Thick" => DividerWeight::Thick,
        "ExtraThick" => DividerWeight::ExtraThick,
        _ => DividerWeight::Normal,
    };
    let spacing = match spacing_str().as_str() {
        "Compact" => DividerSpacing::Compact,
        "Wide" => DividerSpacing::Wide,
        _ => DividerSpacing::Default,
    };

    let styles = "BASE: \"border-0 border-t border-[var(--color-card-border)]\"\nDASHED/DOTTED: \"border-dashed\" / \"border-dotted\"\nTHICK/EXTRA_THICK: \"border-t-2\" / \"border-t-4\"\nSPACER: \"border-0 my-4\"\nSPACING_COMPACT/DEFAULT/WIDE: \"my-2\" / \"my-4\" / \"my-8\"".to_string();

    let code = "EqDivider {}  // solid, normal weight\n\nEqDivider { variant: DividerVariant::Dashed }\n\nEqDivider {\n    variant: DividerVariant::Dashed,\n    weight: DividerWeight::Thick,\n}\n\nEqDivider {\n    variant: DividerVariant::Spacer,\n    spacing: DividerSpacing::Wide,\n}".to_string();

    rsx! {
        DemoSection { title: "EqDivider",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "variant",
                    value: variant_str(),
                    options: vec!["Solid", "Dashed", "Dotted", "Spacer"],
                    onchange: move |v: String| variant_str.set(v),
                }
                PropSelect {
                    label: "weight",
                    value: weight_str(),
                    options: vec!["Normal", "Thick", "ExtraThick"],
                    onchange: move |v: String| weight_str.set(v),
                }
                PropSelect {
                    label: "spacing",
                    value: spacing_str(),
                    options: vec!["Compact", "Default", "Wide"],
                    onchange: move |v: String| spacing_str.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 max-w-lg",
                div { class: "rounded-lg bg-[var(--color-card)]/30 p-2 text-sm text-[var(--color-label-secondary)]",
                    "Content above"
                }
                EqDivider { variant, weight, spacing }
                div { class: "rounded-lg bg-[var(--color-card)]/30 p-2 text-sm text-[var(--color-label-secondary)]",
                    "Content below"
                }
            }
            StyleInfo { file: "eq_divider_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

#[component]
fn DemoEqVideo() -> Element {
    let mut autoplay = use_signal(|| false);
    let mut muted = use_signal(|| true);
    let mut loop_video = use_signal(|| false);
    let mut controls = use_signal(|| true);
    let mut rounded = use_signal(|| true);
    let mut size_str = use_signal(|| "Full".to_string());
    let mut ratio_str = use_signal(|| "Ratio16_9".to_string());
    let mut show_poster = use_signal(|| true);

    let size = match size_str().as_str() {
        "Sm" => AtomImageSize::Sm,
        "Md" => AtomImageSize::Md,
        "Lg" => AtomImageSize::Lg,
        _ => AtomImageSize::Full,
    };
    let aspect_ratio = match ratio_str().as_str() {
        "Ratio4_3" => AspectRatio::Ratio4_3,
        "Square" => AspectRatio::Square,
        "Free" => AspectRatio::Free,
        _ => AspectRatio::Ratio16_9,
    };

    let poster_url = if show_poster() {
        "https://picsum.photos/seed/eq-video/1280/720".to_string()
    } else {
        String::new()
    };

    let styles = "WRAPPER: \"relative overflow-hidden\"\nSM/MD/LG/FULL: \"w-48\" / \"w-64\" / \"w-96\" / \"w-full\"\nRATIO_16_9: \"aspect-video\"\nRATIO_4_3: \"aspect-[4/3]\"\nVIDEO_ELEMENT: \"w-full h-full\"\nPOSTER_OVERLAY: \"absolute inset-0 z-10 cursor-pointer\"\nPLAY_CIRCLE: \"size-16 rounded-full bg-black/60 ...\"".to_string();

    let code = "EqVideo {\n    src: \"https://example.com/video.mp4\",\n    controls: true,\n    rounded: true,\n}\n\nEqVideo {\n    src: \"https://example.com/video.mp4\",\n    poster: \"https://example.com/thumb.jpg\",\n    muted: true,\n    loop_video: true,\n}".to_string();

    rsx! {
        DemoSection { title: "EqVideo",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "size",
                    value: size_str(),
                    options: vec!["Sm", "Md", "Lg", "Full"],
                    onchange: move |v: String| size_str.set(v),
                }
                PropSelect {
                    label: "ratio",
                    value: ratio_str(),
                    options: vec!["Ratio16_9", "Ratio4_3", "Square", "Free"],
                    onchange: move |v: String| ratio_str.set(v),
                }
                PropToggle {
                    label: "autoplay",
                    value: autoplay(),
                    onchange: move |v: bool| autoplay.set(v),
                }
                PropToggle {
                    label: "muted",
                    value: muted(),
                    onchange: move |v: bool| muted.set(v),
                }
                PropToggle {
                    label: "loop",
                    value: loop_video(),
                    onchange: move |v: bool| loop_video.set(v),
                }
                PropToggle {
                    label: "controls",
                    value: controls(),
                    onchange: move |v: bool| controls.set(v),
                }
                PropToggle {
                    label: "rounded",
                    value: rounded(),
                    onchange: move |v: bool| rounded.set(v),
                }
                PropToggle {
                    label: "poster",
                    value: show_poster(),
                    onchange: move |v: bool| show_poster.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] overflow-hidden p-4",
                EqVideo {
                    src: "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
                    poster: poster_url,
                    size,
                    aspect_ratio,
                    autoplay: autoplay(),
                    muted: muted(),
                    loop_video: loop_video(),
                    controls: controls(),
                    rounded: rounded(),
                }
            }
            StyleInfo { file: "eq_video_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

// ── Molecule demos ──────────────────────────────────────────────────

#[component]
fn DemoEqImageCard() -> Element {
    let mut mode_str = use_signal(|| "Below".to_string());
    let mut size_str = use_signal(|| "Lg".to_string());
    let mut ratio_str = use_signal(|| "Ratio16_9".to_string());
    let mut fit_str = use_signal(|| "Cover".to_string());
    let mut rounded = use_signal(|| true);
    let mut title = use_signal(|| "Alpine Meadow".to_string());
    let mut description =
        use_signal(|| "A serene landscape captured during the golden hour.".to_string());
    let mut attribution = use_signal(|| "Photo by Jane Doe".to_string());

    let mode = match mode_str().as_str() {
        "Overlay" => CaptionMode::Overlay,
        _ => CaptionMode::Below,
    };
    let size = match size_str().as_str() {
        "Sm" => AtomImageSize::Sm,
        "Md" => AtomImageSize::Md,
        "Full" => AtomImageSize::Full,
        _ => AtomImageSize::Lg,
    };
    let aspect_ratio = match ratio_str().as_str() {
        "Free" => AspectRatio::Free,
        "Ratio4_3" => AspectRatio::Ratio4_3,
        "Square" => AspectRatio::Square,
        _ => AspectRatio::Ratio16_9,
    };
    let object_fit = match fit_str().as_str() {
        "Contain" => ObjectFit::Contain,
        "Fill" => ObjectFit::Fill,
        _ => ObjectFit::Cover,
    };

    let title_val = title();
    let desc_val = description();
    let attr_val = attribution();

    let styles = "CARD_WRAPPER: \"space-y-3\"\nFIGCAPTION: \"space-y-2 px-1 py-2\"\nCAPTION_TITLE: \"text-lg font-semibold ...\"\nOVERLAY_CONTAINER: \"relative\"\nOVERLAY_GRADIENT: \"absolute inset-0 bg-gradient-to-t from-black/80 ...\"".to_string();

    let code = "EqImageCard {\n    src: \"photo.jpg\",\n    alt: \"Description\",\n    mode: CaptionMode::Below,\n    size: AtomImageSize::Lg,\n    aspect_ratio: AspectRatio::Ratio16_9,\n    rounded: true,\n    title: \"Card Title\",\n    description: \"A short description.\",\n    attribution: \"Photo by Author\",\n}\n\nEqImageCard {\n    mode: CaptionMode::Overlay,\n    // ...\n}".to_string();

    rsx! {
        DemoSection { title: "EqImageCard",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "mode",
                    value: mode_str(),
                    options: vec!["Below", "Overlay"],
                    onchange: move |v: String| mode_str.set(v),
                }
                PropSelect {
                    label: "size",
                    value: size_str(),
                    options: vec!["Sm", "Md", "Lg", "Full"],
                    onchange: move |v: String| size_str.set(v),
                }
                PropSelect {
                    label: "aspect_ratio",
                    value: ratio_str(),
                    options: vec!["Free", "Ratio16_9", "Ratio4_3", "Square"],
                    onchange: move |v: String| ratio_str.set(v),
                }
                PropSelect {
                    label: "object_fit",
                    value: fit_str(),
                    options: vec!["Cover", "Contain", "Fill"],
                    onchange: move |v: String| fit_str.set(v),
                }
                PropToggle {
                    label: "rounded",
                    value: rounded(),
                    onchange: move |v: bool| rounded.set(v),
                }
                PropInput {
                    label: "title",
                    value: title(),
                    placeholder: "Card title",
                    onchange: move |v: String| title.set(v),
                }
                PropInput {
                    label: "description",
                    value: description(),
                    placeholder: "Card description",
                    onchange: move |v: String| description.set(v),
                }
                PropInput {
                    label: "attribution",
                    value: attribution(),
                    placeholder: "Photo by…",
                    onchange: move |v: String| attribution.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 max-w-lg",
                EqImageCard {
                    src: "https://picsum.photos/seed/eq-card1/800/500",
                    alt: "Preview image card",
                    mode,
                    size,
                    aspect_ratio,
                    object_fit,
                    rounded: rounded(),
                    title: title_val,
                    description: desc_val,
                    attribution: attr_val,
                }
            }
            StyleInfo { file: "eq_image_card_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

#[component]
fn DemoEqCarousel() -> Element {
    let mut mode_str = use_signal(|| "Default".to_string());
    let mut gap_str = use_signal(|| "12".to_string());

    let mode = match mode_str().as_str() {
        "Peek" => CarouselMode::Peek,
        _ => CarouselMode::Default,
    };
    let gap_val: u32 = gap_str().parse().unwrap_or(12);

    let slides = vec![
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
        rsx! {
            EqImageCard {
                src: "https://picsum.photos/seed/carousel4/800/450",
                alt: "Slide four",
                mode: CaptionMode::Overlay,
                size: AtomImageSize::Full,
                aspect_ratio: AspectRatio::Ratio16_9,
                rounded: true,
                title: "Fourth Slide",
                description: "Sunset over the desert.",
            }
        },
    ];

    let styles = "CAROUSEL: \"relative w-full overflow-hidden\"\nSLIDE_STRIP: \"flex transition-transform duration-500 ease-in-out\"\nSLIDE: \"w-full shrink-0\"\nARROW_BASE: \"absolute top-1/2 -translate-y-1/2 z-20 ...\"\nDOTS: \"flex justify-center items-center gap-2 mt-4\"".to_string();

    let code = "// Default carousel with slide animation\nEqCarousel {\n    slides: vec![\n        rsx! { /* slide content */ },\n        rsx! { /* slide content */ },\n    ],\n}\n\n// Peek mode — shows neighbours with fade\nEqCarousel {\n    mode: CarouselMode::Peek,\n    gap: 24,  // default 12px\n    slides: vec![ /* ... */ ],\n}".to_string();

    rsx! {
        DemoSection { title: "EqCarousel",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "mode",
                    value: mode_str(),
                    options: vec!["Default", "Peek"],
                    onchange: move |v: String| mode_str.set(v),
                }
                PropInput {
                    label: "gap (px)",
                    value: gap_str(),
                    placeholder: "12",
                    onchange: move |v: String| gap_str.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                EqCarousel { mode, gap: gap_val, slides }
            }
            StyleInfo { file: "eq_carousel_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

#[component]
fn DemoEqCard() -> Element {
    let mut show_header = use_signal(|| true);
    let mut show_footer = use_signal(|| true);

    let styles = "CARD: \"rounded-xl border border-[var(--color-card-border)] bg-[var(--color-card)]/60 shadow-sm\"\nCARD_HEADER: \"px-6 py-4 border-b border-[var(--color-card-border)]\"\nCARD_BODY: \"px-6 py-4\"\nCARD_FOOTER: \"px-6 py-4 border-t border-[var(--color-card-border)] flex justify-end gap-2\"".to_string();

    let code = "EqCard {\n    EqCardHeader { \"Card Title\" }\n    EqCardBody { \"Card content goes here.\" }\n    EqCardFooter { \"Footer content\" }\n}\n\n// Minimal card\nEqCard {\n    EqCardBody { \"Body only — no header or footer.\" }\n}".to_string();

    rsx! {
        DemoSection { title: "EqCard",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropToggle {
                    label: "header",
                    value: show_header(),
                    onchange: move |v: bool| show_header.set(v),
                }
                PropToggle {
                    label: "footer",
                    value: show_footer(),
                    onchange: move |v: bool| show_footer.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 max-w-md",
                EqCard {
                    if show_header() {
                        EqCardHeader { "Card Title" }
                    }
                    EqCardBody { "Card body content goes here." }
                    if show_footer() {
                        EqCardFooter { "Footer content" }
                    }
                }
            }
            StyleInfo { file: "eq_card_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

#[component]
fn DemoEqTree() -> Element {
    let mut selected = use_signal(|| Option::<String>::None);
    let mut show_count = use_signal(|| true);

    let tree = vec![
        TreeNode::new_with_children(
            "demo-branch-a",
            "Branch A",
            vec![
                TreeNode::new("leaf-1", "Leaf 1"),
                TreeNode::new("leaf-2", "Leaf 2"),
                TreeNode::new_with_children(
                    "nested-branch",
                    "Nested Branch",
                    vec![TreeNode::new("deep-leaf", "Deep Leaf")],
                ),
            ],
        ),
        TreeNode::new_with_children(
            "demo-branch-b",
            "Branch B",
            vec![
                TreeNode::new("leaf-3", "Leaf 3"),
                TreeNode::new("leaf-4", "Leaf 4"),
            ],
        ),
    ];

    let styles = "TREE: \"flex flex-col gap-0.5 text-sm select-none\"\nNODE_ROW: \"flex items-center gap-1.5 px-2 py-1.5 rounded-md ...\"\nNODE_ACTIVE: \"... bg-[var(--color-primary)]/15 text-[var(--color-primary)]\"\nCHEVRON: \"size-4 shrink-0 transition-transform duration-200\"\nLABEL: \"truncate text-[var(--color-label)]\"\nCOUNT: \"ml-1 text-[var(--color-label-secondary)] opacity-60\"".to_string();

    let code = "let nodes = vec![\n    TreeNode::new_with_children(\"branch\", \"Branch\", vec![\n        TreeNode::new(\"leaf-1\", \"Leaf 1\"),\n        TreeNode::new(\"leaf-2\", \"Leaf 2\"),\n    ]),\n];\n\nEqTree {\n    nodes: nodes,\n    selected: selected(),\n    on_select: move |id: String| selected.set(Some(id)),\n    show_count: true,\n}".to_string();

    rsx! {
        DemoSection { title: "EqTree",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropToggle {
                    label: "show_count",
                    value: show_count(),
                    onchange: move |v: bool| show_count.set(v),
                }
            }
            div { class: "flex gap-6",
                div { class: "w-64 h-64 flex flex-col border border-[var(--color-card-border)] rounded-lg p-3",
                    EqScrollableSpace {
                        EqTree {
                            nodes: tree,
                            selected: selected(),
                            on_select: move |id: String| selected.set(Some(id)),
                            show_count: show_count(),
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
            StyleInfo { file: "eq_tree_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

#[component]
fn DemoEqAccordion() -> Element {
    let mut mode_str = use_signal(|| "Single".to_string());
    let mut panel_count_str = use_signal(|| "3".to_string());

    let mode = match mode_str().as_str() {
        "Multi" => AccordionMode::Multi,
        _ => AccordionMode::Single,
    };
    let panel_count: usize = panel_count_str().parse().unwrap_or(3).min(6).max(1);

    let sample_panels: Vec<(&str, &str, &str)> = vec![
        (
            "what-is",
            "What is eq_ui?",
            "A portable Dioxus 0.7 component library following atomic design principles. It ships atoms, molecules, and organisms with 21 built-in themes.",
        ),
        (
            "getting-started",
            "Getting started",
            "Add the crate to your Cargo.toml, include the Tailwind @source directive, and wire up the theme provider. Components are ready to use immediately.",
        ),
        (
            "theming",
            "How does theming work?",
            "Themes are CSS variable sets applied at the root. Switch themes at runtime with a single function call — all components update instantly.",
        ),
        (
            "customisation",
            "Can I customise styles?",
            "Every component exposes a class prop. Pass Tailwind utilities to extend defaults, or prefix with ! to replace them entirely.",
        ),
        (
            "accordion",
            "Is this an accordion?",
            "Yes. You are looking at a live EqAccordion right now. It supports single-expand and multi-expand modes.",
        ),
        (
            "animation",
            "How does the animation work?",
            "The body uses a CSS grid-rows transition between 0fr (collapsed) and 1fr (expanded), giving a smooth height animation without JavaScript measurement.",
        ),
    ];

    let items: Vec<AccordionItem> = sample_panels
        .into_iter()
        .take(panel_count)
        .map(|(id, title, body)| AccordionItem::new(id, rsx! { "{title}" }, rsx! { "{body}" }))
        .collect();

    let items_multi: Vec<AccordionItem> = vec![
        AccordionItem::new(
            "faq-1",
            rsx! { "Single mode" },
            rsx! { "Only one panel can be open at a time." },
        ),
        AccordionItem::new(
            "faq-2",
            rsx! { "Multi mode" },
            rsx! { "Multiple panels can be open simultaneously." },
        ),
    ];

    let styles = "ACCORDION: \"flex flex-col divide-y divide-[var(--color-card-border)]\"\nHEADER: \"flex items-center justify-between w-full gap-3 px-4 py-3 ...\"\nBODY: \"grid transition-[grid-template-rows] duration-200 ease-in-out\"\nBODY_OPEN: \"grid-rows-[1fr]\"\nBODY_CLOSED: \"grid-rows-[0fr]\"\nCONTENT: \"pb-4 text-sm text-[var(--color-label-secondary)]\"".to_string();

    let code = "let items = vec![\n    AccordionItem::new(\n        \"panel-1\",\n        rsx! { \"First panel\" },\n        rsx! { \"Content for the first panel.\" },\n    ),\n    AccordionItem::new(\n        \"panel-2\",\n        rsx! { \"Second panel\" },\n        rsx! { \"Content for the second panel.\" },\n    ),\n];\n\n// Single expand (default)\nEqAccordion { items: items.clone() }\n\n// Multi expand\nEqAccordion {\n    items: items,\n    mode: AccordionMode::Multi,\n}".to_string();

    rsx! {
        DemoSection { title: "EqAccordion",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "mode",
                    value: mode_str(),
                    options: vec!["Single", "Multi"],
                    onchange: move |v: String| mode_str.set(v),
                }
                PropInput {
                    label: "panels",
                    value: panel_count_str(),
                    placeholder: "3",
                    onchange: move |v: String| panel_count_str.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] overflow-hidden",
                EqAccordion { items, mode }
            }
            div { class: "space-y-4 mt-6",
                EqText { variant: TextVariant::Emphasis, "Mode comparison" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    div { class: "space-y-2",
                        EqText { variant: TextVariant::Caption, "Single (default)" }
                        div { class: "rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                            EqAccordion {
                                items: items_multi.clone(),
                                mode: AccordionMode::Single,
                            }
                        }
                    }
                    div { class: "space-y-2",
                        EqText { variant: TextVariant::Caption, "Multi" }
                        div { class: "rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                            EqAccordion {
                                items: items_multi,
                                mode: AccordionMode::Multi,
                            }
                        }
                    }
                }
            }
            StyleInfo { file: "eq_accordion_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

// ── Organism demos ──────────────────────────────────────────────────

#[component]
fn DemoEqHeader() -> Element {
    let mut title_str = use_signal(|| "Equidevium".to_string());

    let site_title: &'static str = match title_str().as_str() {
        "My App" => "My App",
        "Dashboard" => "Dashboard",
        "Acme Corp" => "Acme Corp",
        _ => "Equidevium",
    };

    let styles = "HEADER: \"sticky top-0 z-50 border-b ... bg-[var(--color-primary-dark)]/80 backdrop-blur\"\nHEADER_INNER: \"flex items-center justify-between py-4\"\nBRAND: \"text-lg font-semibold tracking-tight ...\"\nNAV_UL: \"flex gap-4 items-center list-none m-0 p-0\"".to_string();

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
            StyleInfo { file: "eq_header_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

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

    let styles = "HERO_SHELL: \"py-20 md:py-28 bg-[var(--gradient-background)]\"\nHERO_TITLE: \"text-4xl md:text-5xl font-semibold tracking-tight ...\"\nHERO_SUBTITLE: \"mt-4 max-w-2xl text-lg ...\"\nHERO_ACTIONS: \"mt-8 flex gap-4\"\nHERO_BG: \"absolute inset-0 w-full h-full\"\nHERO_OVERLAY: \"absolute inset-0 bg-black/50\"".to_string();

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
                        background: rsx! {
                            EqImage {
                                src: "https://picsum.photos/seed/eq-full/1200/900",
                                alt: "Background image",
                                size: AtomImageSize::Full,
                                aspect_ratio: AspectRatio::Ratio4_3,
                                object_fit: ObjectFit::Cover,
                            }
                        },
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
            StyleInfo { file: "eq_hero_shell_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

#[component]
fn DemoEqPageSection() -> Element {
    let mut title = use_signal(|| "Section Title".to_string());
    let mut description =
        use_signal(|| "A description of this section with some context.".to_string());

    let title_val: Option<String> = if title().is_empty() {
        None
    } else {
        Some(title())
    };
    let desc_val: Option<String> = if description().is_empty() {
        None
    } else {
        Some(description())
    };

    let styles = "SECTION_WRAP: \"py-12 md:py-16\"\nSECTION_TITLE: \"text-2xl md:text-3xl font-semibold tracking-tight\"\nSECTION_DESC: \"mt-2 w-full text-[var(--color-label-secondary)]\"\nSECTION_BODY: \"mt-8\"".to_string();

    let code = "EqPageSection {\n    title: \"Features\",\n    description: \"What makes this product special.\",\n}\n\nEqPageSection {\n    title: \"With Children\",\n    description: \"Extra content below.\",\n    div { \"Nested child content\" }\n}".to_string();

    rsx! {
        DemoSection { title: "EqPageSection",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropInput {
                    label: "title",
                    value: title(),
                    placeholder: "Section title",
                    onchange: move |v: String| title.set(v),
                }
                PropInput {
                    label: "description",
                    value: description(),
                    placeholder: "Section description",
                    onchange: move |v: String| description.set(v),
                }
            }
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] overflow-hidden",
                EqPageSection { title: title_val, description: desc_val,
                    div { class: "mt-4 p-4 rounded-lg bg-[var(--color-card)]/40",
                        "Child content inside a PageSection."
                    }
                }
            }
            StyleInfo { file: "eq_page_section_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

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

    let styles = "FOOTER: \"mt-16 border-t border-[var(--color-card-border)] bg-[var(--color-primary-dark)]\"\nFOOTER_INNER: \"mx-auto max-w-6xl px-4 py-12\"\nFOOTER_GRID: \"grid gap-10 md:grid-cols-3\"\nFOOTER_BOTTOM: \"mt-10 flex flex-col gap-2 ...\"".to_string();

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
            StyleInfo { file: "eq_footer_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

#[component]
fn DemoEqAppShell() -> Element {
    let styles = "APP: \"min-h-screen bg-[var(--color-primary-dark)] text-[var(--color-label-primary)]\"\nCONTAINER_LAYOUT: \"mx-auto max-w-6xl px-4\"\nMAIN_CONTENT: \"flex-1\"\nMAIN_INNER: \"py-10\"".to_string();

    let code = "EqAppShell {\n    header: rsx! {\n        EqHeader { site_title: \"My App\",\n            nav: rsx! { li { \"Nav item\" } },\n        }\n    },\n    footer: rsx! { EqFooter {} },\n\n    // Page content as children\n    div { \"Your page content here\" }\n}".to_string();

    rsx! {
        DemoSection { title: "EqAppShell",
            EqText { variant: TextVariant::Muted,
                "EqAppShell wraps header + footer + children into a full page layout. It is the outermost layout component — you are looking at a live example right now. This playground itself uses EqAppShell."
            }
            StyleInfo { file: "theme.rs (shared)", styles }
            CodeBlock { code }
        }
    }
}

// ── EqGrid Demo ─────────────────────────────────────────────────────

// ── EqCheckbox Demo ────────────────────────────────────────────────

#[component]
fn DemoEqCheckbox() -> Element {
    let mut state_idx = use_signal(|| 0usize); // 0=Unchecked, 1=Checked, 2=Indeterminate
    let mut disabled = use_signal(|| false);
    let mut size_str = use_signal(|| "Sm".to_string());
    let mut label_text = use_signal(|| String::new());

    let state = match state_idx() {
        1 => CheckboxState::Checked,
        2 => CheckboxState::Indeterminate,
        _ => CheckboxState::Unchecked,
    };

    let size = match size_str().as_str() {
        "Md" => IconSize::Md,
        "Lg" => IconSize::Lg,
        _ => IconSize::Sm,
    };

    let styles = "WRAPPER: \"inline-flex items-center gap-2 cursor-pointer select-none\"\n\
        WRAPPER_DISABLED: \"inline-flex items-center gap-2 cursor-not-allowed select-none opacity-50\"\n\
        ICON: \"size-5 shrink-0 text-[var(--color-label-secondary)] transition-colors\"\n\
        ICON_ACTIVE: \"size-5 shrink-0 text-[var(--color-accent-primary)] transition-colors\"\n\
        LABEL: \"text-sm text-[var(--color-label-primary)]\"".to_string();

    let code = "use eq_ui::atoms::{EqCheckbox, CheckboxState};\n\
        \n\
        let mut agreed = use_signal(|| CheckboxState::Unchecked);\n\
        \n\
        EqCheckbox {\n\
        \x20   state: agreed(),\n\
        \x20   label: \"I agree to the terms\",\n\
        \x20   on_change: move |next| agreed.set(next),\n\
        }\n\
        \n\
        // Indeterminate (e.g. header select-all with partial selection)\n\
        EqCheckbox {\n\
        \x20   state: CheckboxState::Indeterminate,\n\
        \x20   on_change: move |_| { /* select all */ },\n\
        }".to_string();

    rsx! {
        DemoSection { title: "EqCheckbox",
            // Prop controls
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                div { class: "grid grid-cols-2 md:grid-cols-3 gap-3",
                    PropSelect {
                        label: "state",
                        value: match state_idx() { 1 => "Checked", 2 => "Indeterminate", _ => "Unchecked" }.to_string(),
                        options: vec!["Unchecked", "Checked", "Indeterminate"],
                        onchange: move |v: String| state_idx.set(match v.as_str() { "Checked" => 1, "Indeterminate" => 2, _ => 0 }),
                    }
                    PropToggle {
                        label: "disabled",
                        value: disabled(),
                        onchange: move |v: bool| disabled.set(v),
                    }
                    PropSelect {
                        label: "size",
                        value: size_str(),
                        options: vec!["Sm", "Md", "Lg"],
                        onchange: move |v: String| size_str.set(v),
                    }
                    PropInput {
                        label: "label",
                        value: label_text(),
                        placeholder: "Optional label",
                        onchange: move |v: String| label_text.set(v),
                    }
                }
            }

            // Live preview
            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 flex items-center gap-4",
                EqCheckbox {
                    state: state,
                    disabled: disabled(),
                    size: size,
                    label: label_text(),
                    on_change: move |next: CheckboxState| {
                        state_idx.set(match next {
                            CheckboxState::Checked => 1,
                            CheckboxState::Indeterminate => 2,
                            CheckboxState::Unchecked => 0,
                        });
                    },
                }
            }

            // Variant gallery
            div { class: "space-y-4",
                EqText { variant: TextVariant::Emphasis, "All States" }
                div { class: "flex flex-wrap items-center gap-6",
                    for (label , st) in [
                        ("Unchecked", CheckboxState::Unchecked),
                        ("Checked", CheckboxState::Checked),
                        ("Indeterminate", CheckboxState::Indeterminate),
                    ] {
                        div { class: "flex items-center gap-2",
                            EqCheckbox { state: st, label: label }
                        }
                    }
                }

                EqText { variant: TextVariant::Emphasis, "Disabled" }
                div { class: "flex flex-wrap items-center gap-6",
                    for (label , st) in [
                        ("Unchecked", CheckboxState::Unchecked),
                        ("Checked", CheckboxState::Checked),
                        ("Indeterminate", CheckboxState::Indeterminate),
                    ] {
                        div { class: "flex items-center gap-2",
                            EqCheckbox { state: st, label: label, disabled: true }
                        }
                    }
                }

                EqText { variant: TextVariant::Emphasis, "Sizes" }
                div { class: "flex flex-wrap items-center gap-6",
                    for (label , sz) in [("Sm", IconSize::Sm), ("Md", IconSize::Md), ("Lg", IconSize::Lg)] {
                        div { class: "flex items-center gap-2",
                            EqCheckbox { state: CheckboxState::Checked, size: sz, label: label }
                        }
                    }
                }
            }

            StyleInfo { file: "eq_checkbox_styles.rs", styles }
            CodeBlock { code }
        }
    }
}

// ── EqGrid demo data ──────────────────────────────────────────────

#[derive(Clone, PartialEq)]
struct DemoEmployee {
    index: usize,
    name: String,
    role: String,
    department: String,
    salary: f64,
    status: String,
}

fn demo_employees() -> Vec<DemoEmployee> {
    let base = vec![
        ("Ada Lovelace", "Engineer", "R&D", 95000.0, "Active"),
        ("Grace Hopper", "Architect", "R&D", 120000.0, "Active"),
        ("Alan Turing", "Researcher", "Science", 105000.0, "Inactive"),
        ("Linus Torvalds", "Lead", "Engineering", 150000.0, "Active"),
        ("Margaret Hamilton", "Director", "Engineering", 140000.0, "Active"),
        ("Dennis Ritchie", "Engineer", "Systems", 98000.0, "Inactive"),
        ("Barbara Liskov", "Professor", "Science", 130000.0, "Active"),
        ("Ken Thompson", "Engineer", "Systems", 102000.0, "Active"),
        ("Bjarne Stroustrup", "Architect", "Languages", 115000.0, "Active"),
        ("Guido van Rossum", "Lead", "Languages", 125000.0, "Inactive"),
        ("Hedy Lamarr", "Inventor", "R&D", 88000.0, "Active"),
        ("Tim Berners-Lee", "Architect", "Web", 135000.0, "Active"),
        ("John McCarthy", "Researcher", "AI", 110000.0, "Inactive"),
        ("Frances Allen", "Engineer", "Compilers", 99000.0, "Active"),
        ("Donald Knuth", "Professor", "Algorithms", 142000.0, "Active"),
    ];
    base.into_iter()
        .enumerate()
        .map(|(i, (n, r, d, s, st))| DemoEmployee {
            index: i + 1, name: n.into(), role: r.into(), department: d.into(),
            salary: s, status: st.into(),
        })
        .collect()
}

/// Generate a large dataset by cycling the base employees with index suffixes.
fn demo_employees_large(count: usize) -> Vec<DemoEmployee> {
    let base = demo_employees();
    (0..count)
        .map(|i| {
            let src = &base[i % base.len()];
            DemoEmployee {
                index: i + 1,
                name: src.name.clone(),
                role: src.role.clone(),
                department: src.department.clone(),
                salary: src.salary + (i as f64 * 100.0),
                status: src.status.clone(),
            }
        })
        .collect()
}

fn demo_columns() -> Vec<EqColumnDef<DemoEmployee>> {
    vec![
        EqColumnDef::new("idx", "#", |e: &DemoEmployee| e.index.to_string())
            .sortable(false)
            .resizable(false)
            .align(ColumnAlign::Right)
            .width(50)
            .min_width(40),
        EqColumnDef::new("name", "Name", |e: &DemoEmployee| e.name.clone())
            .filterable(true)
            .min_width(140),
        EqColumnDef::new("role", "Role", |e: &DemoEmployee| e.role.clone())
            .filterable(true)
            .min_width(100),
        EqColumnDef::new("dept", "Department", |e: &DemoEmployee| e.department.clone())
            .filterable(true)
            .min_width(100),
        EqColumnDef::new("salary", "Salary", |e: &DemoEmployee| e.salary.to_string())
            .with_formatter(|e: &DemoEmployee| format!("${:.0}", e.salary))
            .align(ColumnAlign::Right)
            .comparator(|a: &DemoEmployee, b: &DemoEmployee| a.salary.partial_cmp(&b.salary).unwrap_or(std::cmp::Ordering::Equal))
            .width(120)
            .min_width(80),
        EqColumnDef::new("status", "Status", |e: &DemoEmployee| e.status.clone())
            .with_renderer(|e: &DemoEmployee| {
                let (label, color) = match e.status.as_str() {
                    "Active" => ("Active", "text-[var(--color-success)]"),
                    "On Leave" => ("On Leave", "text-amber-400"),
                    _ => ("Inactive", "text-[var(--color-error)]"),
                };
                rsx! { span { class: "{color} font-medium text-xs", "{label}" } }
            })
            .sortable(false)
            .resizable(false)
            .align(ColumnAlign::Center)
            .min_width(80),
    ]
}

#[component]
fn DemoEqGrid() -> Element {
    let mut nav_idx = use_signal(|| 1usize); // 0=Standard, 1=Paginate, 2=Virtualize
    let mut striped = use_signal(|| true);
    let mut col_borders = use_signal(|| false);
    let mut quick_filter = use_signal(|| true);
    let mut density_idx = use_signal(|| 1usize); // 0=Compact, 1=Normal, 2=Comfortable
    let mut selection_idx = use_signal(|| 1usize); // 0=None, 1=Single
    let mut page_size_idx = use_signal(|| 0usize); // 0=5, 1=10, 2=25
    let mut reorderable = use_signal(|| false);

    let navigation = match nav_idx() {
        0 => GridNavigation::Standard,
        2 => GridNavigation::Virtualize,
        _ => GridNavigation::Paginate,
    };

    let density = match density_idx() {
        0 => GridDensity::Compact,
        2 => GridDensity::Comfortable,
        _ => GridDensity::Normal,
    };

    let selection = match selection_idx() {
        1 => RowSelection::Single,
        2 => RowSelection::Multi,
        _ => RowSelection::None,
    };

    let mut employees = use_signal(|| demo_employees());
    // When virtualization is active, switch to a large dataset.
    use_effect(move || {
        let virt = nav_idx() == 2;
        if virt && employees.read().len() < 100 {
            employees.set(demo_employees_large(500));
        } else if !virt && employees.read().len() > 100 {
            employees.set(demo_employees());
        }
    });
    let mut selection_count = use_signal(|| 0usize);
    let mut bulk_status = use_signal(|| String::new());
    let mut export_preview = use_signal(|| String::new());
    let mut clipboard_preview = use_signal(|| String::new());

    let page_size = match page_size_idx() {
        1 => 10,
        2 => 25,
        _ => 5,
    };

    let styles = "GRID_WRAPPER: \"rounded-xl border ... overflow-hidden\"\n\
        TABLE: \"w-full border-collapse text-sm\"\n\
        TH: \"px-3 py-2 md:px-4 md:py-3 ... font-semibold\"\n\
        TD: \"px-3 py-2 md:px-4 md:py-3\"\n\
        TR_STRIPED: \"even:bg-[var(--color-card)]/5\"\n\
        QUICK_FILTER: \"px-3 py-2 md:px-4 md:py-3 ... flex items-center\"\n\
        COLUMN_FILTER_INPUT: \"w-full mt-1 px-2 py-1 text-xs rounded ...\"\n\
        PAGINATION_BAR: \"flex flex-col gap-2 md:flex-row ...\"".to_string();

    let code = "#[derive(Clone, PartialEq)]\n\
        struct Employee {\n\
        \x20   name: String,\n\
        \x20   role: String,\n\
        \x20   salary: f64,\n\
        }\n\
        \n\
        let columns = vec![\n\
        \x20   EqColumnDef::new(\"name\", \"Name\", |e| e.name.clone())\n\
        \x20       .filterable(true),\n\
        \x20   EqColumnDef::new(\"role\", \"Role\", |e| e.role.clone())\n\
        \x20       .filterable(true),\n\
        \x20   EqColumnDef::new(\"salary\", \"Salary\", |e| e.salary.to_string())\n\
        \x20       .with_formatter(|e| format!(\"${:.0}\", e.salary))\n\
        \x20       .align(ColumnAlign::Right),\n\
        ];\n\
        \n\
        EqGrid {\n\
        \x20   data: employees,\n\
        \x20   columns: columns,\n\
        \x20   navigation: GridNavigation::Paginate,\n\
        \x20   page_size: 10,\n\
        \x20   row_selection: RowSelection::Single,\n\
        \x20   quick_filter: true,\n\
        \x20   striped: true,\n\
        }".to_string();

    rsx! {
        DemoSection { title: "EqGrid",
            // Prop controls
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                div { class: "grid grid-cols-2 md:grid-cols-3 gap-3",
                    PropSelect {
                        label: "navigation",
                        value: match nav_idx() { 0 => "Standard", 2 => "Virtualize", _ => "Paginate" }.to_string(),
                        options: vec!["Standard", "Paginate", "Virtualize"],
                        onchange: move |v: String| nav_idx.set(match v.as_str() { "Standard" => 0, "Virtualize" => 2, _ => 1 }),
                    }
                    PropToggle {
                        label: "striped",
                        value: striped(),
                        onchange: move |v: bool| striped.set(v),
                    }
                    PropToggle {
                        label: "column_borders",
                        value: col_borders(),
                        onchange: move |v: bool| col_borders.set(v),
                    }
                    PropToggle {
                        label: "quick_filter",
                        value: quick_filter(),
                        onchange: move |v: bool| quick_filter.set(v),
                    }
                    PropSelect {
                        label: "density",
                        value: match density_idx() { 0 => "Compact", 2 => "Comfortable", _ => "Normal" }.to_string(),
                        options: vec!["Compact", "Normal", "Comfortable"],
                        onchange: move |v: String| density_idx.set(match v.as_str() { "Compact" => 0, "Comfortable" => 2, _ => 1 }),
                    }
                    PropSelect {
                        label: "row_selection",
                        value: match selection_idx() { 1 => "Single", 2 => "Multi", _ => "None" }.to_string(),
                        options: vec!["None", "Single", "Multi"],
                        onchange: move |v: String| selection_idx.set(match v.as_str() { "Single" => 1, "Multi" => 2, _ => 0 }),
                    }
                    PropSelect {
                        label: "page_size",
                        value: match page_size_idx() { 1 => "10", 2 => "25", _ => "5" }.to_string(),
                        options: vec!["5", "10", "25"],
                        onchange: move |v: String| page_size_idx.set(match v.as_str() { "10" => 1, "25" => 2, _ => 0 }),
                    }
                    PropToggle {
                        label: "reorderable",
                        value: reorderable(),
                        onchange: move |v: bool| reorderable.set(v),
                    }
                }
            }

            // Live preview
            // Selection feedback + bulk action status
            if selection == RowSelection::Multi {
                div { class: "text-sm text-[var(--color-label-secondary)] py-1",
                    "{selection_count()} row(s) selected"
                }
                if !bulk_status.read().is_empty() {
                    div { class: "text-xs text-[var(--color-accent-primary)] bg-[var(--color-card)]/20 rounded px-3 py-1.5 mb-1",
                        "{bulk_status}"
                    }
                }
            }

            EqGrid {
                data: employees(),
                columns: demo_columns(),
                navigation: navigation,
                page_size: page_size,
                row_selection: selection,
                density: density,
                striped: striped(),
                column_borders: col_borders(),
                quick_filter: quick_filter(),
                reorderable: reorderable(),
                on_reorder: move |(from, to): (usize, usize)| {
                    let mut data = employees.write();
                    let row = data.remove(from);
                    data.insert(to, row);
                    for (i, e) in data.iter_mut().enumerate() {
                        e.index = i + 1;
                    }
                },
                on_selection_change: move |rows: Vec<usize>| {
                    selection_count.set(rows.len());
                },
                // Bulk actions — these actually mutate the data signal
                on_delete: move |rows: Vec<usize>| {
                    let count = rows.len();
                    let mut data = employees.write();
                    // Remove in reverse order so indices stay valid.
                    for &idx in rows.iter().rev() {
                        if idx < data.len() {
                            data.remove(idx);
                        }
                    }
                    drop(data);
                    selection_count.set(0);
                    bulk_status.set(format!("Deleted {} row(s)", count));
                },
                export: true,
                on_export: move |payload: (ExportFormat, Vec<u8>)| {
                    let (fmt, bytes) = payload;
                    let label = match fmt {
                        ExportFormat::Csv => "CSV",
                        ExportFormat::Json => "JSON",
                        ExportFormat::Txt => "TXT",
                        ExportFormat::Ods => "ODS",
                    };
                    // Show text content for text formats, byte count for binary.
                    let preview = match fmt {
                        ExportFormat::Ods => format!("[Binary ODS: {} bytes]", bytes.len()),
                        _ => String::from_utf8(bytes.clone()).unwrap_or_else(|_| format!("[{} bytes]", bytes.len())),
                    };
                    clipboard_preview.set(String::new());
                    export_preview.set(preview);
                    bulk_status.set(format!("Exported {} ({} bytes)", label, bytes.len()));
                },
                on_clipboard: move |content: String| {
                    let len = content.len();
                    export_preview.set(String::new());
                    clipboard_preview.set(content.clone());
                    // Write to the system clipboard via the Web Clipboard API.
                    spawn_local(async move {
                        let window = web_sys::window().unwrap();
                        let clipboard = window.navigator().clipboard();
                        let _ = wasm_bindgen_futures::JsFuture::from(
                            clipboard.write_text(&content)
                        ).await;
                    });
                    bulk_status.set(format!("Copied to clipboard ({} chars)", len));
                },
                status_column: "status",
                status_options: vec!["Active".into(), "Inactive".into(), "On Leave".into()],
                on_status_change: move |payload: (Vec<usize>, String)| {
                    let (rows, new_status) = payload;
                    let count = rows.len();
                    let mut data = employees.write();
                    for &idx in &rows {
                        if idx < data.len() {
                            data[idx].status = new_status.clone();
                        }
                    }
                    drop(data);
                    bulk_status.set(format!("Changed {} row(s) to '{}'", count, new_status));
                },
                aggregation_columns: vec!["salary"],
            }

            // Export preview
            if !export_preview.read().is_empty() {
                div { class: "mt-3 rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                    div { class: "flex items-center justify-between px-3 py-1.5 bg-[var(--color-grid-header-bg)] border-b border-[var(--color-card-border)]",
                        span { class: "text-xs font-semibold text-[var(--color-label-primary)]", "Export Preview" }
                        button {
                            class: "text-xs text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] cursor-pointer",
                            onclick: move |_| export_preview.set(String::new()),
                            "Close"
                        }
                    }
                    pre { class: "px-3 py-2 text-xs text-[var(--color-label-primary)] bg-[var(--color-primary-dark)] overflow-x-auto max-h-64 overflow-y-auto whitespace-pre font-mono",
                        "{export_preview}"
                    }
                }
            }

            // Clipboard preview
            if !clipboard_preview.read().is_empty() {
                div { class: "mt-3 rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                    div { class: "flex items-center justify-between px-3 py-1.5 bg-[var(--color-grid-header-bg)] border-b border-[var(--color-card-border)]",
                        span { class: "text-xs font-semibold text-[var(--color-label-primary)]", "Clipboard Preview" }
                        button {
                            class: "text-xs text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] cursor-pointer",
                            onclick: move |_| clipboard_preview.set(String::new()),
                            "Close"
                        }
                    }
                    pre { class: "px-3 py-2 text-xs text-[var(--color-label-primary)] bg-[var(--color-primary-dark)] overflow-x-auto max-h-64 overflow-y-auto whitespace-pre font-mono",
                        "{clipboard_preview}"
                    }
                }
            }

            StyleInfo { file: "eq_grid/styles.rs", styles }
            CodeBlock { code }
        }
    }
}

// ── EqGrid Drag & Drop Demo ───────────────────────────────────────

#[derive(Clone, PartialEq)]
struct DndPerson {
    index: usize,
    name: String,
    role: String,
}

fn team_a_data() -> Vec<DndPerson> {
    vec![
        DndPerson { index: 1, name: "Ada Lovelace".into(), role: "Engineer".into() },
        DndPerson { index: 2, name: "Grace Hopper".into(), role: "Architect".into() },
        DndPerson { index: 3, name: "Alan Turing".into(), role: "Researcher".into() },
        DndPerson { index: 4, name: "Linus Torvalds".into(), role: "Lead".into() },
        DndPerson { index: 5, name: "Margaret Hamilton".into(), role: "Director".into() },
    ]
}

fn team_b_data() -> Vec<DndPerson> {
    vec![
        DndPerson { index: 1, name: "Dennis Ritchie".into(), role: "Engineer".into() },
        DndPerson { index: 2, name: "Barbara Liskov".into(), role: "Professor".into() },
        DndPerson { index: 3, name: "Ken Thompson".into(), role: "Engineer".into() },
    ]
}

fn dnd_columns() -> Vec<EqColumnDef<DndPerson>> {
    vec![
        EqColumnDef::new("idx", "#", |e: &DndPerson| e.index.to_string())
            .sortable(false)
            .resizable(false)
            .align(ColumnAlign::Right)
            .width(40)
            .min_width(40),
        EqColumnDef::new("name", "Name", |e: &DndPerson| e.name.clone())
            .min_width(120),
        EqColumnDef::new("role", "Role", |e: &DndPerson| e.role.clone())
            .min_width(80),
    ]
}

#[component]
fn DemoEqGridDragDrop() -> Element {
    // Shared drag context — both grids read/write through this signal.
    let _drag_ctx: Signal<Option<GridDragPayload>> =
        use_context_provider(|| Signal::new(Option::<GridDragPayload>::None));

    let mut team_a = use_signal(|| team_a_data());
    let mut team_b = use_signal(|| team_b_data());
    let mut status = use_signal(|| String::new());

    // Re-index helper: updates the `index` field to match position.
    let reindex = |data: &mut Vec<DndPerson>| {
        for (i, person) in data.iter_mut().enumerate() {
            person.index = i + 1;
        }
    };

    let code = "// Wrap both grids in a shared drag context provider:\n\
        use_context_provider(|| Signal::new(Option::<GridDragPayload>::None));\n\
        \n\
        // Source grid — drag_id enables dragging selected rows\n\
        EqGrid {\n\
        \x20   data: team_a(),\n\
        \x20   columns: columns(),\n\
        \x20   row_selection: RowSelection::Multi,\n\
        \x20   drag_id: \"team-a\",\n\
        }\n\
        \n\
        // Target grid — drop_target + on_drop_receive\n\
        EqGrid {\n\
        \x20   data: team_b(),\n\
        \x20   columns: columns(),\n\
        \x20   row_selection: RowSelection::Multi,\n\
        \x20   drop_target: true,\n\
        \x20   on_drop_receive: move |payload: GridDragPayload| {\n\
        \x20       // Move rows from source to target\n\
        \x20   },\n\
        }".to_string();

    rsx! {
        DemoSection { title: "EqGrid Drag & Drop",
            EqText { variant: TextVariant::Muted,
                "Select rows in Team A using the checkboxes, then drag any selected row to Team B. \
                 The rows move between the two grids."
            }

            if !status.read().is_empty() {
                div { class: "text-xs text-[var(--color-accent-primary)] bg-[var(--color-card)]/20 rounded px-3 py-1.5",
                    "{status}"
                }
            }

            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                // Team A — drag source
                div { class: "space-y-2",
                    EqText { variant: TextVariant::Emphasis, "Team A (drag source)" }
                    EqGrid {
                        data: team_a(),
                        columns: dnd_columns(),
                        row_selection: RowSelection::Multi,
                        density: GridDensity::Compact,
                        striped: true,
                        drag_id: "team-a",
                        // Also accept drops so you can drag back
                        drop_target: true,
                        on_drop_receive: {
                            let mut team_a = team_a;
                            let mut team_b = team_b;
                            let mut status = status;
                            move |payload: GridDragPayload| {
                                if payload.source_id == "team-b" {
                                    let mut b = team_b.write();
                                    let mut a = team_a.write();
                                    let mut moved = Vec::new();
                                    // Collect in reverse so indices stay valid.
                                    for &idx in payload.indices.iter().rev() {
                                        if idx < b.len() {
                                            moved.push(b.remove(idx));
                                        }
                                    }
                                    moved.reverse();
                                    let count = moved.len();
                                    a.extend(moved);
                                    reindex(&mut a);
                                    reindex(&mut b);
                                    drop(a);
                                    drop(b);
                                    status.set(format!("Moved {} row(s) from Team B to Team A", count));
                                }
                            }
                        },
                    }
                }

                // Team B — drop target
                div { class: "space-y-2",
                    EqText { variant: TextVariant::Emphasis, "Team B (drop target)" }
                    EqGrid {
                        data: team_b(),
                        columns: dnd_columns(),
                        row_selection: RowSelection::Multi,
                        density: GridDensity::Compact,
                        striped: true,
                        drag_id: "team-b",
                        drop_target: true,
                        on_drop_receive: {
                            let mut team_a = team_a;
                            let mut team_b = team_b;
                            let mut status = status;
                            move |payload: GridDragPayload| {
                                if payload.source_id == "team-a" {
                                    let mut a = team_a.write();
                                    let mut b = team_b.write();
                                    let mut moved = Vec::new();
                                    for &idx in payload.indices.iter().rev() {
                                        if idx < a.len() {
                                            moved.push(a.remove(idx));
                                        }
                                    }
                                    moved.reverse();
                                    let count = moved.len();
                                    b.extend(moved);
                                    reindex(&mut a);
                                    reindex(&mut b);
                                    drop(a);
                                    drop(b);
                                    status.set(format!("Moved {} row(s) from Team A to Team B", count));
                                }
                            }
                        },
                    }
                }
            }

            CodeBlock { code }
        }
    }
}

// ── Reorderable Grid Demo ──────────────────────────────────────────

#[component]
fn DemoEqGridReorder() -> Element {
    let mut data = use_signal(|| {
        vec![
            DemoEmployee { index: 1, name: "Ada Lovelace".into(), role: "Engineer".into(), department: "R&D".into(), salary: 95000.0, status: "Active".into() },
            DemoEmployee { index: 2, name: "Grace Hopper".into(), role: "Architect".into(), department: "R&D".into(), salary: 120000.0, status: "Active".into() },
            DemoEmployee { index: 3, name: "Alan Turing".into(), role: "Researcher".into(), department: "Science".into(), salary: 105000.0, status: "Inactive".into() },
            DemoEmployee { index: 4, name: "Linus Torvalds".into(), role: "Lead".into(), department: "Engineering".into(), salary: 150000.0, status: "Active".into() },
            DemoEmployee { index: 5, name: "Margaret Hamilton".into(), role: "Director".into(), department: "Engineering".into(), salary: 140000.0, status: "Active".into() },
            DemoEmployee { index: 6, name: "Dennis Ritchie".into(), role: "Engineer".into(), department: "Systems".into(), salary: 98000.0, status: "Inactive".into() },
            DemoEmployee { index: 7, name: "Barbara Liskov".into(), role: "Professor".into(), department: "Science".into(), salary: 130000.0, status: "Active".into() },
            DemoEmployee { index: 8, name: "Ken Thompson".into(), role: "Engineer".into(), department: "Systems".into(), salary: 102000.0, status: "Active".into() },
        ]
    });

    let mut last_move = use_signal(|| String::new());

    let columns = vec![
        EqColumnDef::new("index", "#", |e: &DemoEmployee| e.index.to_string())
            .width(60),
        EqColumnDef::new("name", "Name", |e: &DemoEmployee| e.name.clone())
            .sortable(true),
        EqColumnDef::new("role", "Role", |e: &DemoEmployee| e.role.clone()),
        EqColumnDef::new("dept", "Department", |e: &DemoEmployee| e.department.clone()),
        EqColumnDef::new("salary", "Salary", |e: &DemoEmployee| format!("${}", e.salary as u64)),
    ];

    let code = r#"EqGrid {
    data: items(),
    columns: columns,
    reorderable: true,
    on_reorder: move |(from, to): (usize, usize)| {
        let mut vec = items.write();
        let row = vec.remove(from);
        vec.insert(to, row);
        // Re-index after move
        for (i, e) in vec.iter_mut().enumerate() {
            e.index = i + 1;
        }
    },
}"#;

    rsx! {
        div { class: "space-y-6 p-4",
            EqText { variant: TextVariant::H2, "EqGrid Reorder" }
            EqText { variant: TextVariant::Body,
                "Drag the grip handle (\u{2807}) on the left edge of any row to reorder. \
                 Works with all navigation modes."
            }

            if !last_move.read().is_empty() {
                div {
                    class: "px-3 py-2 rounded-lg bg-[var(--color-card)] border border-[var(--color-card-border)] text-sm",
                    EqText { variant: TextVariant::Caption, "{last_move}" }
                }
            }

            EqGrid {
                data: data(),
                columns: columns,
                reorderable: true,
                striped: true,
                density: GridDensity::Normal,
                on_reorder: move |(from, to): (usize, usize)| {
                    let mut vec = data.write();
                    let row = vec.remove(from);
                    vec.insert(to, row);
                    for (i, e) in vec.iter_mut().enumerate() {
                        e.index = i + 1;
                    }
                    drop(vec);
                    let d = data.read();
                    let name = &d[to].name;
                    last_move.set(format!("Moved \"{}\" from position {} to {}", name, from + 1, to + 1));
                },
            }

            CodeBlock { code }
        }
    }
}

// ── Theme Showcase ─────────────────────────────────────────────────

/// A single color swatch — shows the color + its variable name.
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

/// A gradient preview strip.
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

/// A labeled section within the showcase.
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

/// Transition speed comparison — uses signals for hover state.
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

/// Interactive card demo — pure CSS hover/active transitions via `.card-interactive`.
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

#[component]
fn DemoThemeShowcase() -> Element {
    // Neutral checkerboard backdrop so swatches have consistent contrast
    // regardless of the active theme. Fixed #1a1a1a base with subtle
    // lighter squares (#222) at 20px intervals.
    // Hope this works as well on the deployed website as well. Fingers crossed.
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
                        ColorSwatch {
                            label: "Primary Dark",
                            var_name: "color-primary-dark",
                        }
                        ColorSwatch {
                            label: "Secondary Dark",
                            var_name: "color-secondary-dark",
                        }
                        ColorSwatch {
                            label: "Tertiary Dark",
                            var_name: "color-tertiary-dark",
                        }
                        ColorSwatch {
                            label: "Hover Button",
                            var_name: "color-hover-button",
                        }
                        ColorSwatch { label: "Card", var_name: "color-card" }
                        ColorSwatch {
                            label: "Card Border",
                            var_name: "color-card-border",
                        }
                        ColorSwatch {
                            label: "Card Border Bright",
                            var_name: "color-card-border-bright",
                        }
                        ColorSwatch {
                            label: "Card Shadow",
                            var_name: "color-card-shadow",
                        }
                        ColorSwatch { label: "Background", var_name: "color-background" }
                        ColorSwatch { label: "Primary", var_name: "color-primary" }
                    }
                }

                // ── Labels / Text ──
                ShowcaseSection { title: "Labels / Text",
                    div { class: "grid grid-cols-2 md:grid-cols-3 gap-4",
                        ColorSwatch {
                            label: "Label Primary",
                            var_name: "color-label-primary",
                        }
                        ColorSwatch {
                            label: "Label Secondary",
                            var_name: "color-label-secondary",
                        }
                        ColorSwatch { label: "Label Bold", var_name: "color-label-bold" }
                        ColorSwatch {
                            label: "Label Muted",
                            var_name: "color-label-muted",
                        }
                        ColorSwatch {
                            label: "Label Disabled",
                            var_name: "color-label-disabled",
                        }
                    }
                }

                // ── Gradients ──
                ShowcaseSection { title: "Gradients",
                    div { class: "space-y-4",
                        div { class: "grid grid-cols-3 gap-4",
                            ColorSwatch {
                                label: "Gradient Start",
                                var_name: "color-gradient-default-start",
                            }
                            ColorSwatch {
                                label: "Gradient Mid",
                                var_name: "color-gradient-default-mid",
                            }
                            ColorSwatch {
                                label: "Gradient End",
                                var_name: "color-gradient-default-end",
                            }
                        }
                        GradientSwatch {
                            label: "Tricolor Gradient",
                            var_name: "gradient-primary-tricolor",
                        }
                        GradientSwatch {
                            label: "Background Gradient",
                            var_name: "gradient-background",
                        }
                        GradientSwatch {
                            label: "Duocolor Gradient",
                            var_name: "gradient-primary-duocolor",
                        }
                    }
                }

                // ── Accent & Interaction ──
                ShowcaseSection { title: "Accent & Interaction",
                    div { class: "grid grid-cols-2 md:grid-cols-3 gap-4",
                        ColorSwatch {
                            label: "Accent Primary",
                            var_name: "color-accent-primary",
                        }
                        ColorSwatch {
                            label: "Accent Secondary",
                            var_name: "color-accent-secondary",
                        }
                        ColorSwatch {
                            label: "Accent Muted",
                            var_name: "color-accent-muted",
                        }
                        ColorSwatch { label: "Focus Ring", var_name: "color-focus-ring" }
                        ColorSwatch {
                            label: "Shadow Glow",
                            var_name: "color-shadow-glow",
                        }
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
                        ColorSwatch {
                            label: "Border Default",
                            var_name: "color-border-default",
                        }
                        ColorSwatch {
                            label: "Border Subtle",
                            var_name: "color-border-subtle",
                        }
                        ColorSwatch {
                            label: "Border Active",
                            var_name: "color-border-active",
                        }
                    }
                }

                // ── Input / Form ──
                ShowcaseSection { title: "Input / Form Elements",
                    div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                        ColorSwatch { label: "Input BG", var_name: "color-input-bg" }
                        ColorSwatch {
                            label: "Input Border",
                            var_name: "color-input-border",
                        }
                        ColorSwatch {
                            label: "Input Focus",
                            var_name: "color-input-focus",
                        }
                        ColorSwatch {
                            label: "Placeholder",
                            var_name: "color-input-placeholder",
                        }
                    }
                }

                // ── Surfaces & Overlays ──
                ShowcaseSection { title: "Surfaces & Overlays",
                    div { class: "grid grid-cols-2 md:grid-cols-3 gap-4",
                        ColorSwatch {
                            label: "Surface Elevated",
                            var_name: "color-surface-elevated",
                        }
                        ColorSwatch {
                            label: "Surface Overlay",
                            var_name: "color-surface-overlay",
                        }
                        ColorSwatch {
                            label: "Surface Tooltip",
                            var_name: "color-surface-tooltip",
                        }
                    }
                }

                // ── Code ──
                ShowcaseSection { title: "Code / Terminal",
                    div { class: "grid grid-cols-2 md:grid-cols-3 gap-4",
                        ColorSwatch { label: "Code BG", var_name: "color-code-bg" }
                        ColorSwatch { label: "Code Text", var_name: "color-code-text" }
                        ColorSwatch {
                            label: "Code Comment",
                            var_name: "color-code-comment",
                        }
                        ColorSwatch {
                            label: "Code Keyword",
                            var_name: "color-code-keyword",
                        }
                        ColorSwatch {
                            label: "Code String",
                            var_name: "color-code-string",
                        }
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
                            ColorSwatch {
                                label: "Btn Primary BG",
                                var_name: "btn-primary-bg",
                            }
                            ColorSwatch {
                                label: "Btn Primary Hover",
                                var_name: "btn-primary-hover",
                            }
                            ColorSwatch {
                                label: "Btn Primary Text",
                                var_name: "btn-primary-text",
                            }
                            ColorSwatch {
                                label: "Btn Ghost Hover",
                                var_name: "btn-ghost-hover",
                            }
                            ColorSwatch {
                                label: "Btn Outline Border",
                                var_name: "btn-outline-border",
                            }
                            ColorSwatch {
                                label: "Btn Outline Hover",
                                var_name: "btn-outline-hover-border",
                            }
                            ColorSwatch {
                                label: "Btn Outline Hover BG",
                                var_name: "btn-outline-hover-bg",
                            }
                            ColorSwatch {
                                label: "Btn Danger BG",
                                var_name: "btn-danger-bg",
                            }
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
            } // backdrop div
        }
    }
}
