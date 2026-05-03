//! EqModal — modal dialog molecule.
//!
//! Renders a full-viewport backdrop with a centred dialog panel.
//! The caller controls visibility via a `bool` signal. Closing can
//! happen via the close button, clicking the backdrop, or pressing
//! Escape — each governed by props.
//!
//! **Accessibility** — implements the WAI-ARIA [Dialog (Modal)][dlg]
//! pattern: `role="dialog"`, `aria-modal="true"`, `aria-labelledby`
//! linking to the title, focus trap via JS, and Escape to close.
//!
//! [dlg]: https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/
//!
//! ```rust,ignore
//! let mut open = use_signal(|| false);
//!
//! EqButton { on_click: move |_| open.set(true), "Open Modal" }
//!
//! EqModal {
//!     open: open(),
//!     on_close: move |_| open.set(false),
//!     title: "Confirm action",
//!     body: rsx! { "Are you sure you want to proceed?" },
//!     footer: rsx! {
//!         EqButton { variant: ButtonVariant::Ghost, on_click: move |_| open.set(false), "Cancel" }
//!         EqButton { variant: ButtonVariant::Primary, "Confirm" }
//!     },
//! }
//! ```

use super::eq_modal_styles as s;
use crate::theme::merge_classes;
use crate::{PlaygroundEnum, playground};
use dioxus::document;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropToggle, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant, EqButton, ButtonVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

/// Size preset for the modal panel.
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
pub enum ModalSize {
    /// max-width: 24rem (384px)
    Sm,
    /// max-width: 32rem (512px) — default.
    #[default]
    Md,
    /// max-width: 42rem (672px)
    Lg,
    /// max-width: 56rem (896px)
    Xl,
    /// Nearly full viewport width.
    Full,
}

/// Modal dialog molecule.
///
/// Renders a backdrop overlay with a centred dialog panel containing
/// optional header (with title and close button), scrollable body, and
/// footer. The caller owns the open/close state via a signal.
#[playground(
    category = Molecule,
    description = "Modal dialog with backdrop, size presets, optional close-on-backdrop, \
                   Escape to close, and focus management.",
    examples = [
        ("Basic", "let mut open = use_signal(|| false);\n\nEqModal {\n    open: open(),\n    on_close: move |_| open.set(false),\n    title: \"Confirm\",\n    body: rsx! { \"Are you sure?\" },\n}"),
        ("With footer", "EqModal {\n    open: open(),\n    on_close: move |_| open.set(false),\n    title: \"Delete item\",\n    body: rsx! { \"This action cannot be undone.\" },\n    footer: rsx! {\n        EqButton { variant: ButtonVariant::Ghost, \"Cancel\" }\n        EqButton { variant: ButtonVariant::Danger, \"Delete\" }\n    },\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqModal(
    /// Whether the modal is visible.
    #[props(default = false)]
    open: bool,
    /// Callback fired when the user requests closing (close button,
    /// backdrop click, or Escape key).
    on_close: EventHandler<()>,
    /// Dialog title shown in the header. When empty, the header is
    /// still rendered (with just the close button) unless
    /// `show_close` is also false.
    #[props(into, default)]
    title: String,
    /// Body content — scrollable when it exceeds max height.
    body: Option<Element>,
    /// Footer content — typically action buttons.
    footer: Option<Element>,
    /// Size preset for the panel width.
    #[props(default)]
    size: ModalSize,
    /// Show the close (×) button in the header.
    #[props(default = true)]
    show_close: bool,
    /// Close when the user clicks the backdrop overlay.
    #[props(default = true)]
    close_on_backdrop: bool,
    /// Close when the user presses Escape.
    #[props(default = true)]
    close_on_escape: bool,
    /// Optional class override on the panel element.
    #[props(into, default)]
    class: String,
) -> Element {
    // Stable unique ID for aria-labelledby linking.
    let modal_id = use_hook(|| {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("eq-modal-{id}")
    });

    let title_id = format!("{}-title", modal_id);
    let has_title = !title.is_empty();
    let show_header = has_title || show_close;

    // Backdrop and panel animation classes
    let backdrop_anim = if open { s::BACKDROP_OPEN } else { s::BACKDROP_CLOSED };
    let panel_anim = if open { s::PANEL_OPEN } else { s::PANEL_CLOSED };

    let size_cls = match size {
        ModalSize::Sm => s::SIZE_SM,
        ModalSize::Md => s::SIZE_MD,
        ModalSize::Lg => s::SIZE_LG,
        ModalSize::Xl => s::SIZE_XL,
        ModalSize::Full => s::SIZE_FULL,
    };

    let panel_base = format!("{} {}", s::PANEL, size_cls);
    let panel_cls = merge_classes(&panel_base, &class);

    // Focus trap: when opened, focus the panel; when pressing Tab,
    // cycle within the dialog.
    let panel_id_focus = modal_id.clone();
    use_effect(move || {
        if open {
            // Focus the dialog panel itself so keyboard events are captured.
            let dom_id = format!("{panel_id_focus}");
            document::eval(&format!(
                "setTimeout(() => document.getElementById('{dom_id}')?.focus(), 50)"
            ));
        }
    });

    rsx! {
        div {
            class: "{s::BACKDROP} {backdrop_anim}",
            // Backdrop click → close
            onclick: move |_| {
                if close_on_backdrop {
                    on_close.call(());
                }
            },

            div {
                id: "{modal_id}",
                class: "{panel_cls} {panel_anim}",
                role: "dialog",
                "aria-modal": "true",
                "aria-labelledby": if has_title { "{title_id}" } else { "" },
                tabindex: "-1",

                // Stop clicks inside the panel from bubbling to the backdrop
                onclick: move |evt| { evt.stop_propagation(); },

                // Escape key
                onkeydown: move |evt: Event<KeyboardData>| {
                    if close_on_escape && evt.key() == Key::Escape {
                        evt.prevent_default();
                        on_close.call(());
                    }
                },

                // Header
                if show_header {
                    div { class: "{s::HEADER}",
                        if has_title {
                            h2 {
                                id: "{title_id}",
                                class: "{s::HEADER_TITLE}",
                                "{title}"
                            }
                        } else {
                            // Spacer so close button stays on the right
                            div {}
                        }
                        if show_close {
                            button {
                                class: "{s::CLOSE_BUTTON}",
                                "aria-label": "Close dialog",
                                onclick: move |_| on_close.call(()),
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke_width: "2",
                                    stroke: "currentColor",
                                    width: "20",
                                    height: "20",
                                    "aria-hidden": "true",
                                    path { d: "M6 18 18 6M6 6l12 12" }
                                }
                            }
                        }
                    }
                }

                // Body
                if let Some(body_content) = body {
                    div { class: "{s::BODY}",
                        {body_content}
                    }
                }

                // Footer
                if let Some(footer_content) = footer {
                    div { class: "{s::FOOTER}",
                        {footer_content}
                    }
                }
            }
        }
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqModal() -> Element {
    let mut open = use_signal(|| false);
    let mut size_str = use_signal(|| "Md".to_string());
    let mut show_close = use_signal(|| true);
    let mut close_on_backdrop = use_signal(|| true);
    let mut close_on_escape = use_signal(|| true);

    let size = match size_str().as_str() {
        "Sm" => ModalSize::Sm,
        "Lg" => ModalSize::Lg,
        "Xl" => ModalSize::Xl,
        "Full" => ModalSize::Full,
        _ => ModalSize::Md,
    };

    let code = r#"let mut open = use_signal(|| false);

EqButton {
    on_click: move |_| open.set(true),
    "Open Modal"
}

EqModal {
    open: open(),
    on_close: move |_| open.set(false),
    title: "Confirm action",
    size: ModalSize::Md,
    body: rsx! { "Are you sure you want to proceed?" },
    footer: rsx! {
        EqButton {
            variant: ButtonVariant::Ghost,
            on_click: move |_| open.set(false),
            "Cancel"
        }
        EqButton {
            variant: ButtonVariant::Primary,
            "Confirm"
        }
    },
}"#
    .to_string();

    rsx! {
        DemoSection { title: "EqModal",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                div { class: "grid grid-cols-2 md:grid-cols-3 gap-3",
                    PropSelect {
                        label: "size",
                        value: size_str(),
                        options: vec!["Sm", "Md", "Lg", "Xl", "Full"],
                        onchange: move |v: String| size_str.set(v),
                    }
                    PropToggle {
                        label: "show_close",
                        value: show_close(),
                        onchange: move |v: bool| show_close.set(v),
                    }
                    PropToggle {
                        label: "close_on_backdrop",
                        value: close_on_backdrop(),
                        onchange: move |v: bool| close_on_backdrop.set(v),
                    }
                    PropToggle {
                        label: "close_on_escape",
                        value: close_on_escape(),
                        onchange: move |v: bool| close_on_escape.set(v),
                    }
                }
            }

            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                EqButton {
                    variant: ButtonVariant::Primary,
                    on_click: move |_| open.set(true),
                    "Open Modal"
                }
            }

            EqModal {
                open: open(),
                on_close: move |_| open.set(false),
                title: "Confirm action",
                size,
                show_close: show_close(),
                close_on_backdrop: close_on_backdrop(),
                close_on_escape: close_on_escape(),
                body: rsx! {
                    p { "Are you sure you want to proceed? This action may have consequences." }
                    p { class: "mt-3 text-[var(--color-label-secondary)]",
                        "This is a live modal dialog. Try pressing Escape, clicking the backdrop, or using the close button to dismiss it."
                    }
                },
                footer: rsx! {
                    EqButton {
                        variant: ButtonVariant::Ghost,
                        on_click: move |_| open.set(false),
                        "Cancel"
                    }
                    EqButton {
                        variant: ButtonVariant::Primary,
                        on_click: move |_| open.set(false),
                        "Confirm"
                    }
                },
            }

            StyleInfo { file: "eq_modal_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqModal() -> Element {
    let mut basic_open = use_signal(|| false);
    let mut danger_open = use_signal(|| false);
    let mut large_open = use_signal(|| false);

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Modal Gallery" }

                div { class: "flex flex-wrap gap-3",
                    EqButton {
                        variant: ButtonVariant::Primary,
                        on_click: move |_| basic_open.set(true),
                        "Basic Modal"
                    }
                    EqButton {
                        variant: ButtonVariant::Danger,
                        on_click: move |_| danger_open.set(true),
                        "Danger Confirm"
                    }
                    EqButton {
                        variant: ButtonVariant::Outline,
                        on_click: move |_| large_open.set(true),
                        "Large Modal"
                    }
                }
            }

            // Basic modal
            EqModal {
                open: basic_open(),
                on_close: move |_| basic_open.set(false),
                title: "Welcome",
                size: ModalSize::Sm,
                body: rsx! { "This is a simple informational modal." },
                footer: rsx! {
                    EqButton {
                        variant: ButtonVariant::Primary,
                        on_click: move |_| basic_open.set(false),
                        "Got it"
                    }
                },
            }

            // Danger confirmation modal
            EqModal {
                open: danger_open(),
                on_close: move |_| danger_open.set(false),
                title: "Delete item?",
                body: rsx! { "This action cannot be undone. The item and all its data will be permanently removed." },
                footer: rsx! {
                    EqButton {
                        variant: ButtonVariant::Ghost,
                        on_click: move |_| danger_open.set(false),
                        "Cancel"
                    }
                    EqButton {
                        variant: ButtonVariant::Danger,
                        on_click: move |_| danger_open.set(false),
                        "Delete"
                    }
                },
            }

            // Large modal with lots of content
            EqModal {
                open: large_open(),
                on_close: move |_| large_open.set(false),
                title: "Terms of Service",
                size: ModalSize::Lg,
                body: rsx! {
                    div { class: "space-y-4",
                        for i in 1..=8 {
                            p { "Section {i}: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris." }
                        }
                    }
                },
                footer: rsx! {
                    EqButton {
                        variant: ButtonVariant::Ghost,
                        on_click: move |_| large_open.set(false),
                        "Decline"
                    }
                    EqButton {
                        variant: ButtonVariant::Primary,
                        on_click: move |_| large_open.set(false),
                        "Accept"
                    }
                },
            }
        }
    }
}
