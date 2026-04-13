//! Shared helpers for the EqPlayground and per-component demos.
//!
//! This module is feature-gated behind `playground` so it adds zero
//! overhead to production builds.  It provides:
//!
//! - **DemoSection** - titled wrapper for each demo area
//! - **CodeBlock** - Gruvbox-themed Rust syntax-highlighted code block
//! - **StyleInfo** - collapsible style token viewer with Tailwind highlighting
//! - **PropSelect / PropInput / PropToggle** - uniform controls for live prop editing
//! - Syntax highlighting utilities (Rust + Tailwind style tokens)
//!
//! All per-component `render_demo` / `render_gallery` functions should
//! import helpers from here rather than duplicating them.

use dioxus::prelude::*;
use crate::atoms::{EqText, TextVariant};

// ── DemoSection ────────────────────────────────────────────────────

/// Titled section wrapper used inside demo panels.
#[component]
pub fn DemoSection(title: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "p-8 space-y-6",
            h2 { class: "text-2xl font-semibold text-[var(--color-label-primary)] border-b border-[var(--color-card-border)] pb-2",
                "{title}"
            }
            {children}
        }
    }
}

// ── Gruvbox Dark palette ───────────────────────────────────────────

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

// ── Rust keywords ──────────────────────────────────────────────────

const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn",
    "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in",
    "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

// ── HTML escape utilities ──────────────────────────────────────────

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

// ── Rust syntax highlighter ────────────────────────────────────────

/// Tokenize a Rust code string into syntax-highlighted HTML spans
/// using the Gruvbox Dark palette.
pub fn highlight_rust(code: &str) -> String {
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

// ── Style token highlighter ────────────────────────────────────────

/// Tokenize a styles definition string into highlighted HTML.
/// Constant names (SCREAMING_CAPS) render in yellow, colons in grey,
/// and quoted Tailwind class strings render in light blue.
pub fn highlight_styles(input: &str) -> String {
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

        // Everything else
        push_escaped(&mut out, ch);
        i += 1;
    }

    out
}

// ── CodeBlock component ────────────────────────────────────────────

/// Renders a Gruvbox-themed code block with Rust syntax highlighting.
#[component]
pub fn CodeBlock(code: String) -> Element {
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

// ── StyleInfo component ────────────────────────────────────────────

/// Collapsible panel showing which style tokens a component uses,
/// with Gruvbox syntax highlighting for Tailwind classes.
#[component]
pub fn StyleInfo(file: &'static str, styles: String) -> Element {
    let highlighted = highlight_styles(&styles);
    rsx! {
        details {
            class: "mt-4 rounded-lg overflow-hidden",
            style: "border: 1px solid #6b2020;",
            summary {
                class: "px-4 py-2 cursor-pointer text-xs font-semibold tracking-wider select-none transition",
                style: format!("background:{};color:{};", GRV_BG_SOFT, GRV_FG),
                "Default Styles - {file}"
            }
            pre {
                class: "p-4 overflow-x-auto text-xs leading-relaxed font-mono",
                style: format!("background:{};color:{};", GRV_BG, GRV_FG),
                code { dangerous_inner_html: "{highlighted}" }
            }
        }
    }
}

// ── Prop control shared styles ─────────────────────────────────────

/// Flex row layout for a single prop control.
pub const PROP_ROW: &str = "flex items-center gap-3";

/// Label style for prop controls (fixed width, secondary colour).
pub const PROP_LABEL: &str =
    "text-xs font-medium text-[var(--color-label-secondary)] w-20 shrink-0";

/// Input / select control style for prop controls.
pub const PROP_CONTROL: &str =
    "rounded-md bg-[var(--color-card)] text-[var(--color-label-primary)] \
     border border-[var(--color-card-border)] px-2 py-1 text-xs";

// ── PropSelect ─────────────────────────────────────────────────────

/// Styled select dropdown for live prop editing in demos.
#[component]
pub fn PropSelect(
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

// ── PropInput ──────────────────────────────────────────────────────

/// Styled text input for live prop editing in demos.
#[component]
pub fn PropInput(
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

// ── PropToggle ─────────────────────────────────────────────────────

/// Styled boolean toggle (true/false dropdown) for live prop editing.
#[component]
pub fn PropToggle(
    label: &'static str,
    value: bool,
    onchange: EventHandler<bool>,
) -> Element {
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

// ── Utility: format style catalog for StyleInfo ────────────────────

/// Takes the output of a `catalog()` function and formats it as a
/// human-readable string for display inside `StyleInfo`.
///
/// ```ignore
/// let text = format_catalog(&eq_button_styles::catalog());
/// rsx! { StyleInfo { file: "eq_button_styles.rs", styles: text } }
/// ```
pub fn format_catalog(tokens: &[(&str, &str)]) -> String {
    let mut out = String::new();
    for (name, value) in tokens {
        out.push_str(name);
        out.push_str(": \"");
        out.push_str(value);
        out.push_str("\"\n");
    }
    out
}
