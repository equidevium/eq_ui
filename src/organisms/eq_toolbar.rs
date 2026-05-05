//! Top-anchored mobile header with start / title / end slots and an
//! optional secondary row.
//!
//! Modeled on Ionic's toolbar. The primary row holds typical mobile
//! header content: a back/menu button on the left, a title in the
//! middle, action buttons on the right. The secondary row hosts
//! search, segmented controls, or a progress bar.
//!
//! All slots are optional. The component does not own state; the
//! consumer drives whatever lives in each slot.
//!
//! ```rust,ignore
//! EqToolbar {
//!     start: rsx! { EqButton { variant: ButtonVariant::Ghost, "Back" } },
//!     title: rsx! { "Inbox" },
//!     end: rsx! { EqButton { variant: ButtonVariant::Ghost, "Edit" } },
//!     secondary: rsx! {
//!         EqInput { placeholder: "Search", kind: InputKind::Text, oninput: move |_| {} }
//!     },
//! }
//! ```

use super::eq_toolbar_styles as s;
use crate::playground;
use crate::theme::merge_classes;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant, EqButton, ButtonVariant, EqInput, InputKind, EqProgress};
#[cfg(feature = "playground")]
use crate::molecules::EqDeviceFrame;
#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{
    ComponentCategory, ComponentDescriptor, UsageExample,
};

/// Mobile header / toolbar.
#[playground(
    category = Organism,
    description = "Mobile header with start / title / end slots and an optional secondary row for search, segments, or a progress bar.",
    examples = [
        ("Title only", "EqToolbar { title: rsx! { \"Inbox\" } }"),
        ("With actions", "EqToolbar {\n    start: rsx! { EqButton { variant: ButtonVariant::Ghost, \"Back\" } },\n    title: rsx! { \"Settings\" },\n    end: rsx! { EqButton { variant: ButtonVariant::Ghost, \"Save\" } },\n}"),
        ("With search row", "EqToolbar {\n    title: rsx! { \"Library\" },\n    secondary: rsx! {\n        EqInput { placeholder: \"Search\", kind: InputKind::Text }\n    },\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqToolbar(
    /// Optional leading slot (typically a back or menu button).
    start: Option<Element>,
    /// Optional title slot. Truncates if it overflows.
    title: Option<Element>,
    /// Optional trailing slot (typically action buttons).
    end: Option<Element>,
    /// Optional secondary row beneath the primary row. Hosts search,
    /// segmented controls, progress bars, or any other content.
    secondary: Option<Element>,
    /// Optional class override on the outer wrapper.
    #[props(into, default)]
    class: String,
) -> Element {
    let wrapper_cls = merge_classes(s::WRAPPER, &class);

    rsx! {
        div { class: "{wrapper_cls}", role: "banner",
            div { class: "{s::PRIMARY}",
                div { class: "{s::START}",
                    if let Some(node) = start { {node} }
                }
                div { class: "{s::TITLE}",
                    if let Some(node) = title { {node} }
                }
                div { class: "{s::END}",
                    if let Some(node) = end { {node} }
                }
            }

            if let Some(node) = secondary {
                div { class: "{s::SECONDARY}",
                    {node}
                }
            }
        }
    }
}

// ── Demo (custom; toggles secondary content) ────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqToolbar() -> Element {
    let mut secondary_kind = use_signal(|| "search".to_string());

    let code = r#"EqToolbar {
    start: rsx! { EqButton { variant: ButtonVariant::Ghost, "Back" } },
    title: rsx! { "Inbox" },
    end: rsx! { EqButton { variant: ButtonVariant::Ghost, "Edit" } },
    secondary: rsx! {
        EqInput { placeholder: "Search", kind: InputKind::Text, oninput: move |_| {} }
    },
}"#
    .to_string();

    let secondary_node: Option<Element> = match secondary_kind().as_str() {
        "search" => Some(rsx! {
            EqInput { placeholder: "Search", kind: InputKind::Text, oninput: move |_| {} }
        }),
        "progress" => Some(rsx! {
            EqProgress { value: 0.4 }
        }),
        _ => None,
    };

    // Built a second time so the rsx! Elements aren't shared across
    // both toolbar instances below.
    let secondary_node_framed: Option<Element> = match secondary_kind().as_str() {
        "search" => Some(rsx! {
            EqInput { placeholder: "Search", kind: InputKind::Text, oninput: move |_| {} }
        }),
        "progress" => Some(rsx! {
            EqProgress { value: 0.4 }
        }),
        _ => None,
    };

    rsx! {
        DemoSection { title: "EqToolbar",
            EqText {
                variant: TextVariant::Muted,
                "Pick what lives in the secondary row. Set to 'none' to hide the row entirely.",
            }

            div { class: "flex flex-wrap gap-2 text-xs",
                button {
                    r#type: "button",
                    class: "px-3 py-1 rounded-md border border-[var(--color-card-border)] cursor-pointer",
                    onclick: move |_| secondary_kind.set("search".to_string()),
                    "Search"
                }
                button {
                    r#type: "button",
                    class: "px-3 py-1 rounded-md border border-[var(--color-card-border)] cursor-pointer",
                    onclick: move |_| secondary_kind.set("progress".to_string()),
                    "Progress"
                }
                button {
                    r#type: "button",
                    class: "px-3 py-1 rounded-md border border-[var(--color-card-border)] cursor-pointer",
                    onclick: move |_| secondary_kind.set("none".to_string()),
                    "None"
                }
            }

            div { class: "rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                EqToolbar {
                    start: rsx! {
                        EqButton { variant: ButtonVariant::Ghost, "Back" }
                    },
                    title: rsx! { "Inbox" },
                    end: rsx! {
                        EqButton { variant: ButtonVariant::Ghost, "Edit" }
                    },
                    secondary: secondary_node,
                }
            }

            EqText {
                variant: TextVariant::Caption,
                class: "font-semibold uppercase tracking-wider",
                "In the mobile frame",
            }

            div { class: "flex justify-center",
                EqDeviceFrame {
                    div { class: "h-full w-full flex flex-col",
                        EqToolbar {
                            start: rsx! {
                                EqButton { variant: ButtonVariant::Ghost, "Back" }
                            },
                            title: rsx! { "Inbox" },
                            end: rsx! {
                                EqButton { variant: ButtonVariant::Ghost, "Edit" }
                            },
                            secondary: secondary_node_framed,
                        }
                        div {
                            class: "flex-1 p-4 space-y-2 text-sm text-[var(--color-label-secondary)] overflow-y-auto",
                            "Page body sits below the toolbar. Scroll to see how the toolbar holds its position at the top of the screen area."
                            for i in 1..=10 {
                                div {
                                    class: "p-3 rounded-md border border-[var(--color-card-border)] bg-[var(--color-card)] mt-2",
                                    "Row {i}"
                                }
                            }
                        }
                    }
                }
            }

            StyleInfo {
                file: "eq_toolbar_styles.rs",
                styles: format_catalog(&s::catalog()),
            }
            CodeBlock { code }
        }
    }
}

// ── Gallery ─────────────────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqToolbar() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Toolbar Gallery",
                }

                div { class: "rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                    EqToolbar { title: rsx! { "Title only" } }
                }

                div { class: "rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                    EqToolbar {
                        start: rsx! {
                            EqButton { variant: ButtonVariant::Ghost, "Back" }
                        },
                        title: rsx! { "Settings" },
                        end: rsx! {
                            EqButton { variant: ButtonVariant::Ghost, "Save" }
                        },
                    }
                }

                div { class: "rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                    EqToolbar {
                        title: rsx! { "Library" },
                        secondary: rsx! {
                            EqInput { placeholder: "Search", kind: InputKind::Text, oninput: move |_| {} }
                        },
                    }
                }

                div { class: "rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                    EqToolbar {
                        title: rsx! { "Uploading" },
                        secondary: rsx! { EqProgress { value: 0.65 } },
                    }
                }
            }
        }
    }
}
