//! EqToast — toast notification molecule.
//!
//! A toast system with two parts:
//! - **`ToastData`** — describes a single notification (severity, title,
//!   message, duration).
//! - **`EqToastList`** — renders a stack of toasts at a fixed viewport
//!   position, auto-dismissing each after its duration expires.
//!
//! The caller manages a `Signal<Vec<ToastData>>` and pushes new entries.
//! `EqToastList` handles rendering and cleanup.
//!
//! ```rust,ignore
//! let mut toasts = use_signal(Vec::<ToastData>::new);
//!
//! EqButton {
//!     on_click: move |_| toasts.write().push(
//!         ToastData::success("Saved", "Your changes were saved.")
//!     ),
//!     "Show Toast"
//! }
//!
//! EqToastList { toasts }
//! ```

use super::eq_toast_styles as s;
use crate::{PlaygroundEnum, playground};
use dioxus::document;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant, EqButton, ButtonVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// ── Types ─────────────────────────────────────────────────────────

/// Severity level controlling toast color and icon.
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
pub enum ToastSeverity {
    /// Neutral information.
    #[default]
    Info,
    /// Positive confirmation.
    Success,
    /// Non-blocking caution.
    Warning,
    /// Error or failure.
    Error,
}

/// Position anchor for the toast stack.
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
pub enum ToastPosition {
    #[default]
    TopRight,
    TopLeft,
    TopCenter,
    BottomRight,
    BottomLeft,
    BottomCenter,
}

/// Describes a single toast notification.
#[derive(Clone, PartialEq)]
pub struct ToastData {
    /// Unique identifier (auto-generated via counter).
    pub id: u64,
    /// Severity level.
    pub severity: ToastSeverity,
    /// Short title (bold).
    pub title: String,
    /// Optional longer message.
    pub message: String,
    /// Auto-dismiss duration in milliseconds. 0 = no auto-dismiss.
    pub duration_ms: u64,
}

impl ToastData {
    /// Internal counter for unique IDs.
    fn next_id() -> u64 {
        static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    /// Create an info toast.
    pub fn info(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            id: Self::next_id(),
            severity: ToastSeverity::Info,
            title: title.into(),
            message: message.into(),
            duration_ms: 4000,
        }
    }

    /// Create a success toast.
    pub fn success(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            id: Self::next_id(),
            severity: ToastSeverity::Success,
            title: title.into(),
            message: message.into(),
            duration_ms: 3000,
        }
    }

    /// Create a warning toast.
    pub fn warning(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            id: Self::next_id(),
            severity: ToastSeverity::Warning,
            title: title.into(),
            message: message.into(),
            duration_ms: 5000,
        }
    }

    /// Create an error toast.
    pub fn error(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            id: Self::next_id(),
            severity: ToastSeverity::Error,
            title: title.into(),
            message: message.into(),
            duration_ms: 6000,
        }
    }

    /// Builder: set custom duration (0 = sticky).
    pub fn duration(mut self, ms: u64) -> Self {
        self.duration_ms = ms;
        self
    }
}

// ── SVG icon paths per severity ─────────────────────────────────────

fn severity_icon(severity: ToastSeverity) -> &'static str {
    match severity {
        // Info circle
        ToastSeverity::Info => "M11.25 11.25l.041-.02a.75.75 0 0 1 1.063.852l-.708 2.836a.75.75 0 0 0 1.063.853l.041-.021M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9-3.75h.008v.008H12V8.25Z",
        // Check circle
        ToastSeverity::Success => "M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
        // Exclamation triangle
        ToastSeverity::Warning => "M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126ZM12 15.75h.007v.008H12v-.008Z",
        // X circle
        ToastSeverity::Error => "m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
    }
}

// ── EqToastList component ───────────────────────────────────────────

/// Renders a positioned stack of toast notifications.
///
/// Manages auto-dismiss timers internally. Each toast fades in on
/// mount and is removed from the signal after its duration expires.
///
/// **Accessibility** — the container uses `role="status"` and
/// `aria-live="polite"` so screen readers announce new toasts
/// without interrupting the current task.
#[playground(
    category = Molecule,
    description = "Toast notification stack with severity levels (info, success, warning, error), \
                   auto-dismiss, six position anchors, and manual close.",
    examples = [
        ("Push a toast", "let mut toasts = use_signal(Vec::<ToastData>::new);\n\ntoasts.write().push(ToastData::success(\"Saved\", \"Changes saved.\"));\n\nEqToastList { toasts }"),
        ("Sticky error", "toasts.write().push(\n    ToastData::error(\"Failed\", \"Could not connect.\").duration(0)\n);"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqToastList(
    /// Signal holding the current toast entries.
    /// The component reads and mutates this signal (removing expired toasts).
    toasts: Signal<Vec<ToastData>>,
    /// Viewport anchor position.
    #[props(default)]
    position: ToastPosition,
) -> Element {
    let pos_cls = match position {
        ToastPosition::TopRight => s::POS_TOP_RIGHT,
        ToastPosition::TopLeft => s::POS_TOP_LEFT,
        ToastPosition::TopCenter => s::POS_TOP_CENTER,
        ToastPosition::BottomRight => s::POS_BOTTOM_RIGHT,
        ToastPosition::BottomLeft => s::POS_BOTTOM_LEFT,
        ToastPosition::BottomCenter => s::POS_BOTTOM_CENTER,
    };

    rsx! {
        div {
            class: "{s::CONTAINER} {pos_cls}",
            role: "status",
            "aria-live": "polite",
            "aria-relevant": "additions",

            for toast in toasts().iter().cloned() {
                EqToastItem {
                    key: "{toast.id}",
                    toast: toast,
                    toasts: toasts,
                }
            }
        }
    }
}

// ── Single toast renderer ───────────────────────────────────────────

#[component]
fn EqToastItem(
    toast: ToastData,
    toasts: Signal<Vec<ToastData>>,
) -> Element {
    let id = toast.id;
    let severity = toast.severity;
    let duration_ms = toast.duration_ms;

    let severity_cls = match severity {
        ToastSeverity::Info => s::INFO,
        ToastSeverity::Success => s::SUCCESS,
        ToastSeverity::Warning => s::WARNING,
        ToastSeverity::Error => s::ERROR,
    };

    let icon_path = severity_icon(severity);
    let has_message = !toast.message.is_empty();

    // Auto-dismiss timer via JS setTimeout promise.
    let mut toasts_dismiss = toasts;
    use_effect(move || {
        if duration_ms > 0 {
            spawn(async move {
                let js = format!(
                    "await new Promise(r => setTimeout(r, {duration_ms}))"
                );
                let _ = document::eval(&js).await;
                toasts_dismiss.write().retain(|t| t.id != id);
            });
        }
    });

    rsx! {
        div {
            class: "{s::TOAST} {severity_cls} {s::TOAST_ENTER}",
            role: "alert",

            // Icon
            svg {
                class: "{s::ICON}",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "1.5",
                stroke: "currentColor",
                width: "20",
                height: "20",
                "aria-hidden": "true",
                path { d: "{icon_path}" }
            }

            // Content
            div { class: "{s::CONTENT}",
                p { class: "{s::TITLE}", "{toast.title}" }
                if has_message {
                    p { class: "{s::MESSAGE}", "{toast.message}" }
                }
            }

            // Close button
            button {
                class: "{s::CLOSE}",
                "aria-label": "Dismiss notification",
                onclick: move |_| {
                    toasts.write().retain(|t| t.id != id);
                },
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke_width: "2",
                    stroke: "currentColor",
                    width: "16",
                    height: "16",
                    "aria-hidden": "true",
                    path { d: "M6 18 18 6M6 6l12 12" }
                }
            }
        }
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqToastList() -> Element {
    let mut toasts = use_signal(Vec::<ToastData>::new);
    let mut position_str = use_signal(|| "TopRight".to_string());

    let position = match position_str().as_str() {
        "TopLeft" => ToastPosition::TopLeft,
        "TopCenter" => ToastPosition::TopCenter,
        "BottomRight" => ToastPosition::BottomRight,
        "BottomLeft" => ToastPosition::BottomLeft,
        "BottomCenter" => ToastPosition::BottomCenter,
        _ => ToastPosition::TopRight,
    };

    let code = r#"let mut toasts = use_signal(Vec::<ToastData>::new);

// Push toasts from anywhere
toasts.write().push(ToastData::info("Info", "Something happened."));
toasts.write().push(ToastData::success("Saved", "Changes saved."));
toasts.write().push(ToastData::warning("Caution", "Check your input."));
toasts.write().push(ToastData::error("Error", "Something went wrong."));

// Sticky toast (no auto-dismiss)
toasts.write().push(
    ToastData::error("Critical", "Connection lost.").duration(0)
);

// Render the stack
EqToastList { toasts, position: ToastPosition::TopRight }"#
        .to_string();

    rsx! {
        DemoSection { title: "EqToastList",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "position",
                    value: position_str(),
                    options: vec!["TopRight", "TopLeft", "TopCenter", "BottomRight", "BottomLeft", "BottomCenter"],
                    onchange: move |v: String| position_str.set(v),
                }
            }

            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6",
                div { class: "flex flex-wrap gap-3",
                    EqButton {
                        on_click: move |_| toasts.write().push(
                            ToastData::info("Info", "Something happened.")
                        ),
                        "Info"
                    }
                    EqButton {
                        variant: ButtonVariant::Primary,
                        on_click: move |_| toasts.write().push(
                            ToastData::success("Saved", "Your changes were saved successfully.")
                        ),
                        "Success"
                    }
                    EqButton {
                        variant: ButtonVariant::Outline,
                        on_click: move |_| toasts.write().push(
                            ToastData::warning("Warning", "Please check your input.")
                        ),
                        "Warning"
                    }
                    EqButton {
                        variant: ButtonVariant::Danger,
                        on_click: move |_| toasts.write().push(
                            ToastData::error("Error", "Something went wrong. Please try again.")
                        ),
                        "Error"
                    }
                    EqButton {
                        variant: ButtonVariant::Ghost,
                        on_click: move |_| toasts.write().push(
                            ToastData::error("Sticky", "This won't auto-dismiss.").duration(0)
                        ),
                        "Sticky"
                    }
                }
            }

            EqToastList { toasts, position }

            StyleInfo { file: "eq_toast_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqToastList() -> Element {
    let mut toasts = use_signal(Vec::<ToastData>::new);

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Toast Gallery" }

                EqText { variant: TextVariant::Muted, "Click buttons to fire toasts (top-right corner)" }

                div { class: "flex flex-wrap gap-3",
                    EqButton {
                        on_click: move |_| toasts.write().push(
                            ToastData::info("Notification", "You have 3 new messages.")
                        ),
                        "Info Toast"
                    }
                    EqButton {
                        variant: ButtonVariant::Primary,
                        on_click: move |_| toasts.write().push(
                            ToastData::success("Published", "Your article is now live.")
                        ),
                        "Success Toast"
                    }
                    EqButton {
                        variant: ButtonVariant::Danger,
                        on_click: move |_| toasts.write().push(
                            ToastData::error("Upload failed", "File exceeds 10 MB limit.")
                        ),
                        "Error Toast"
                    }
                }
            }

            EqToastList { toasts }
        }
    }
}
