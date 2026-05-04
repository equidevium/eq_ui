//! EqAvatar — user avatar atom.
//!
//! Displays a user avatar with three rendering modes:
//! 1. **Image** — shows the provided `src` URL.
//! 2. **Initials** — extracts up to two initials from `name` when no image
//!    is available (or if the image fails to load).
//! 3. **Icon fallback** — a generic person silhouette when neither image
//!    nor name is provided.
//!
//! Supports four sizes, an optional online/offline/busy status dot,
//! and an optional selection ring.
//!
//! ```rust,ignore
//! // Image avatar
//! EqAvatar { src: "https://example.com/photo.jpg", name: "Jane Doe" }
//!
//! // Initials fallback
//! EqAvatar { name: "Jane Doe", size: AvatarSize::Lg }
//!
//! // With status indicator
//! EqAvatar { name: "John", status: AvatarStatus::Online }
//! ```

use super::eq_avatar_styles as s;
use crate::theme::merge_classes;
use crate::{PlaygroundEnum, playground};
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropToggle, PropInput, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// ── Types ─────────────────────────────────────────────────────────

/// Size of the avatar.
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
pub enum AvatarSize {
    /// 32×32px.
    Sm,
    /// 40×40px (default).
    #[default]
    Md,
    /// 48×48px.
    Lg,
    /// 64×64px.
    Xl,
}

/// Online status indicator.
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
pub enum AvatarStatus {
    /// No status dot shown.
    #[default]
    None,
    /// Green dot.
    Online,
    /// Grey dot.
    Offline,
    /// Red dot.
    Busy,
}

// ── Helpers ───────────────────────────────────────────────────────

/// Extract up to 2 initials from a name string.
/// "Jane Doe" → "JD", "Alice" → "A", "" → "".
fn extract_initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

// ── SVG fallback icon (generic person silhouette) ─────────────────

/// Heroicons `user` outline (24×24).
const USER_ICON_PATH: &str =
    "M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 \
     20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 \
     0-5.216-.584-7.499-1.632Z";

// ── Component ─────────────────────────────────────────────────────

/// User avatar with image, initials fallback, and icon fallback.
///
/// **Accessibility** — renders `role="img"` with `aria-label` derived
/// from the `name` prop (or "User avatar" as default). The image, if
/// present, uses an empty `alt` to avoid double-announcement.
#[playground(
    category = Atom,
    description = "User avatar with image, initials fallback, status indicator, \
                   four sizes, and selection ring.",
    examples = [
        ("Image", "EqAvatar {\n    src: \"https://i.pravatar.cc/150?u=jane\",\n    name: \"Jane Doe\",\n}"),
        ("Initials", "EqAvatar {\n    name: \"Jane Doe\",\n    size: AvatarSize::Lg,\n}"),
        ("With status", "EqAvatar {\n    name: \"John\",\n    status: AvatarStatus::Online,\n}"),
        ("Icon fallback", "EqAvatar { size: AvatarSize::Xl }"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqAvatar(
    /// Image URL. When empty or if loading fails, falls back to
    /// initials or icon.
    #[props(into, default)]
    src: String,
    /// User's display name. Used for initials fallback and aria-label.
    #[props(into, default)]
    name: String,
    /// Size of the avatar.
    #[props(default)]
    size: AvatarSize,
    /// Online status indicator dot.
    #[props(default)]
    status: AvatarStatus,
    /// Show a ring around the avatar (e.g. selected state).
    #[props(default = false)]
    ring: bool,
    /// Optional class override on the wrapper element.
    #[props(into, default)]
    class: String,
) -> Element {
    let size_cls = match size {
        AvatarSize::Sm => s::SM,
        AvatarSize::Md => s::MD,
        AvatarSize::Lg => s::LG,
        AvatarSize::Xl => s::XL,
    };

    let ring_cls = if ring { s::RING } else { "" };
    let base_cls = merge_classes(&format!("{} {} {}", s::BASE, size_cls, ring_cls), &class);

    let aria = if name.is_empty() {
        "User avatar".to_string()
    } else {
        format!("{}'s avatar", name)
    };

    let initials = extract_initials(&name);
    let has_src = !src.is_empty();
    let has_initials = !initials.is_empty();

    // Track image load failure to fall back to initials/icon.
    let mut img_error = use_signal(|| false);

    let show_image = has_src && !img_error();

    let fallback_cls = if show_image {
        ""
    } else if has_initials {
        s::INITIALS
    } else {
        s::ICON_FALLBACK
    };

    // Status dot sizing.
    let status_size = match size {
        AvatarSize::Sm => s::STATUS_SM,
        AvatarSize::Md => s::STATUS_MD,
        AvatarSize::Lg => s::STATUS_LG,
        AvatarSize::Xl => s::STATUS_XL,
    };

    let status_color = match status {
        AvatarStatus::None => "",
        AvatarStatus::Online => s::STATUS_ONLINE,
        AvatarStatus::Offline => s::STATUS_OFFLINE,
        AvatarStatus::Busy => s::STATUS_BUSY,
    };

    rsx! {
        div {
            class: "relative inline-flex",

            div {
                class: "{base_cls} {fallback_cls}",
                role: "img",
                "aria-label": "{aria}",

                if show_image {
                    img {
                        class: "{s::IMAGE}",
                        src: "{src}",
                        alt: "",
                        onerror: move |_| img_error.set(true),
                    }
                } else if has_initials {
                    span { "{initials}" }
                } else {
                    // Generic user icon
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "1.5",
                        stroke: "currentColor",
                        width: "60%",
                        height: "60%",
                        "aria-hidden": "true",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: USER_ICON_PATH,
                        }
                    }
                }
            }

            // Status dot
            if status != AvatarStatus::None {
                span {
                    class: "{s::STATUS_DOT} {status_size} {status_color}",
                    "aria-hidden": "true",
                }
            }
        }
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqAvatar() -> Element {
    let mut src = use_signal(|| "https://i.pravatar.cc/150?u=demo".to_string());
    let mut name = use_signal(|| "Jane Doe".to_string());
    let mut size_str = use_signal(|| "Md".to_string());
    let mut status_str = use_signal(|| "None".to_string());
    let mut ring = use_signal(|| false);

    let size = match size_str().as_str() {
        "Sm" => AvatarSize::Sm,
        "Lg" => AvatarSize::Lg,
        "Xl" => AvatarSize::Xl,
        _ => AvatarSize::Md,
    };

    let status = match status_str().as_str() {
        "Online" => AvatarStatus::Online,
        "Offline" => AvatarStatus::Offline,
        "Busy" => AvatarStatus::Busy,
        _ => AvatarStatus::None,
    };

    let code = format!(
        r#"EqAvatar {{
    src: "{src}",
    name: "{name}",
    size: AvatarSize::{sz},
    status: AvatarStatus::{st},
    ring: {ring},
}}"#,
        src = src(),
        name = name(),
        sz = size_str(),
        st = status_str(),
        ring = ring(),
    );

    rsx! {
        DemoSection { title: "EqAvatar",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropInput {
                    label: "src",
                    value: src(),
                    placeholder: "Image URL",
                    onchange: move |v: String| src.set(v),
                }
                PropInput {
                    label: "name",
                    value: name(),
                    placeholder: "Display name",
                    onchange: move |v: String| name.set(v),
                }
                PropSelect {
                    label: "size",
                    value: size_str(),
                    options: vec!["Sm", "Md", "Lg", "Xl"],
                    onchange: move |v: String| size_str.set(v),
                }
                PropSelect {
                    label: "status",
                    value: status_str(),
                    options: vec!["None", "Online", "Offline", "Busy"],
                    onchange: move |v: String| status_str.set(v),
                }
                PropToggle {
                    label: "ring",
                    value: ring(),
                    onchange: move |v: bool| ring.set(v),
                }
            }

            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 space-y-6",
                // Main interactive avatar
                div { class: "flex items-center justify-center",
                    EqAvatar {
                        src: src(),
                        name: name(),
                        size,
                        status,
                        ring: ring(),
                    }
                }

                // Size comparison
                div { class: "space-y-3",
                    EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Size comparison" }
                    div { class: "flex items-end gap-4",
                        for (label, sz) in [("Sm", AvatarSize::Sm), ("Md", AvatarSize::Md), ("Lg", AvatarSize::Lg), ("Xl", AvatarSize::Xl)] {
                            div { class: "flex flex-col items-center gap-1",
                                EqAvatar { name: name(), size: sz, status }
                                EqText { variant: TextVariant::Muted, "{label}" }
                            }
                        }
                    }
                }

                // Fallback modes
                div { class: "space-y-3",
                    EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Fallback modes" }
                    div { class: "flex items-center gap-4",
                        div { class: "flex flex-col items-center gap-1",
                            EqAvatar { src: "https://i.pravatar.cc/150?u=demo", name: "Jane Doe", size: AvatarSize::Lg }
                            EqText { variant: TextVariant::Muted, "Image" }
                        }
                        div { class: "flex flex-col items-center gap-1",
                            EqAvatar { name: "Jane Doe", size: AvatarSize::Lg }
                            EqText { variant: TextVariant::Muted, "Initials" }
                        }
                        div { class: "flex flex-col items-center gap-1",
                            EqAvatar { size: AvatarSize::Lg }
                            EqText { variant: TextVariant::Muted, "Icon" }
                        }
                    }
                }

                // Status indicators
                div { class: "space-y-3",
                    EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Status indicators" }
                    div { class: "flex items-center gap-4",
                        for (label, st) in [("Online", AvatarStatus::Online), ("Offline", AvatarStatus::Offline), ("Busy", AvatarStatus::Busy)] {
                            div { class: "flex flex-col items-center gap-1",
                                EqAvatar { name: "AB", size: AvatarSize::Lg, status: st }
                                EqText { variant: TextVariant::Muted, "{label}" }
                            }
                        }
                    }
                }
            }

            StyleInfo { file: "eq_avatar_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery ───────────────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqAvatar() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Avatar Gallery" }

                // Row of various avatars
                div { class: "flex items-center gap-3 flex-wrap",
                    EqAvatar { src: "https://i.pravatar.cc/150?u=alice", name: "Alice", size: AvatarSize::Lg, status: AvatarStatus::Online }
                    EqAvatar { src: "https://i.pravatar.cc/150?u=bob", name: "Bob", size: AvatarSize::Lg, status: AvatarStatus::Busy }
                    EqAvatar { name: "Charlie Davis", size: AvatarSize::Lg, status: AvatarStatus::Offline }
                    EqAvatar { name: "Eve", size: AvatarSize::Lg }
                    EqAvatar { size: AvatarSize::Lg }
                    EqAvatar { src: "https://i.pravatar.cc/150?u=frank", name: "Frank", size: AvatarSize::Lg, ring: true }
                }

                // Small row
                div { class: "flex items-center gap-2",
                    for i in 0..6 {
                        EqAvatar {
                            src: "https://i.pravatar.cc/150?u=team{i}",
                            name: "Team Member",
                            size: AvatarSize::Sm,
                        }
                    }
                    EqText { variant: TextVariant::Muted, "+12 more" }
                }
            }
        }
    }
}
