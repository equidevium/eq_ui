//! EqVirtualList — high-performance windowed list molecule.
//!
//! Only renders the items visible in the viewport (plus an overscan
//! buffer), enabling smooth scrolling over tens of thousands of rows
//! with minimal DOM footprint.
//!
//! ```rust,ignore
//! EqVirtualList {
//!     item_count: 10_000,
//!     item_size: 40.0,
//!     viewport_size: 400.0,
//!     render_item: move |idx: usize| rsx! {
//!         div { "Row {idx}" }
//!     },
//! }
//! ```

use super::eq_virtual_list_styles as s;
use crate::theme::merge_classes;
use crate::playground;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropInput, PropToggle, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// ── Orientation ──────────────────────────────────────────────────

/// Scroll direction of the virtual list.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum VirtualListDirection {
    /// Vertical scrolling (default).
    #[default]
    Vertical,
    /// Horizontal scrolling.
    Horizontal,
}

// ── Sticky header ────────────────────────────────────────────────

/// Descriptor for a sticky header that pins at a given item index.
#[derive(Clone, PartialEq)]
pub struct StickyHeader {
    /// The item index where this header appears.
    pub at_index: usize,
    /// Label or content identifier for the header.
    pub label: String,
}

impl StickyHeader {
    pub fn new(at_index: usize, label: impl Into<String>) -> Self {
        Self { at_index, label: label.into() }
    }
}

// ── Component ────────────────────────────────────────────────────

/// High-performance windowed list.
///
/// Renders only items visible in the viewport plus an overscan buffer.
/// Supports vertical or horizontal scrolling, fixed-size items,
/// scroll-to-index, and sticky section headers.
///
/// **How it works** — a sizer div is set to the total content size
/// (item_count × item_size) so the scrollbar thumb reflects the real
/// extent. Inside, an absolutely-positioned window div translates to
/// the current scroll offset and renders only the visible slice.
///
/// **Accessibility** — the viewport has `role="list"` and each item
/// is wrapped in `role="listitem"`.
#[playground(
    category = Molecule,
    description = "Windowed list rendering only visible items for \
                   smooth scrolling over thousands of rows. Fixed-size \
                   items, overscan buffer, scroll-to-index, sticky \
                   headers, vertical/horizontal modes.",
    examples = [
        ("Basic", "EqVirtualList {\n    item_count: 10_000,\n    item_size: 40.0,\n    viewport_size: 400.0,\n    render_item: move |idx: usize| rsx! {\n        div { \"Row {idx}\" }\n    },\n}"),
        ("Horizontal", "EqVirtualList {\n    direction: VirtualListDirection::Horizontal,\n    item_count: 500,\n    item_size: 120.0,\n    viewport_size: 600.0,\n    render_item: move |idx| rsx! {\n        div { class: \"p-4\", \"Col {idx}\" }\n    },\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqVirtualList(
    /// Total number of items in the list.
    item_count: usize,
    /// Height (vertical) or width (horizontal) of each item in pixels.
    item_size: f64,
    /// Height (vertical) or width (horizontal) of the viewport in pixels.
    viewport_size: f64,
    /// Render callback — receives the item index, returns an Element.
    render_item: Callback<usize, Element>,
    /// Scroll direction.
    #[props(default)]
    direction: VirtualListDirection,
    /// Number of extra items to render above/below (or left/right) the
    /// visible window. Reduces blank flashes during fast scrolling.
    #[props(default = 3)]
    overscan: usize,
    /// Programmatic scroll target — when set, the list scrolls to put
    /// this index at the top of the viewport.
    #[props(default)]
    scroll_to_index: Option<usize>,
    /// Sticky section headers. Each header pins at the top (vertical)
    /// or left (horizontal) edge when its section scrolls past.
    #[props(default)]
    sticky_headers: Vec<StickyHeader>,
    /// Optional render callback for sticky headers. Receives the
    /// StickyHeader's label. If not provided, a default styled div is used.
    #[props(default)]
    render_sticky_header: Option<Callback<String, Element>>,
    /// Optional class override on the viewport container.
    #[props(into, default)]
    class: String,
) -> Element {
    let is_horizontal = direction == VirtualListDirection::Horizontal;

    // Stable unique ID for the viewport element.
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let vp_id = use_hook(|| {
        format!("eq-vl-{}", COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    });

    // ── Scroll state ─────────────────────────────────────────────
    let mut scroll_offset = use_signal(|| 0.0_f64);
    let mut viewport_el: Signal<Option<MountedEvent>> = use_signal(|| None);

    // ── Scroll-to-index effect ───────────────────────────────────
    let scroll_idx = scroll_to_index;
    let vp_id_eff = vp_id.clone();
    use_effect(move || {
        if let Some(idx) = scroll_idx {
            let target = idx as f64 * item_size;
            let prop = if is_horizontal { "scrollLeft" } else { "scrollTop" };
            let js = format!(
                "const el = document.getElementById('{}'); if(el) el.{} = {};",
                vp_id_eff, prop, target
            );
            document::eval(&js);
        }
    });

    // ── Windowing math ───────────────────────────────────────────
    let total_size = item_count as f64 * item_size;
    let visible_count = (viewport_size / item_size).ceil() as usize;

    let first_visible = if item_size > 0.0 {
        (scroll_offset() / item_size).floor() as usize
    } else {
        0
    };
    let first_visible = first_visible.min(item_count.saturating_sub(1));

    let win_start = first_visible.saturating_sub(overscan);
    let win_end = (first_visible + visible_count + overscan).min(item_count);

    let offset_px = win_start as f64 * item_size;

    // ── Sticky header: find the active one ───────────────────────
    let active_sticky: Option<&StickyHeader> = sticky_headers.iter()
        .filter(|h| h.at_index <= first_visible)
        .last();

    // ── Styles ───────────────────────────────────────────────────
    let viewport_base = if is_horizontal { s::VIEWPORT_HORIZONTAL } else { s::VIEWPORT };
    let viewport_cls = merge_classes(viewport_base, &class);

    let sizer_cls = if is_horizontal { s::SIZER_HORIZONTAL } else { s::SIZER };
    let window_cls = if is_horizontal { s::WINDOW_HORIZONTAL } else { s::WINDOW };
    let item_cls = if is_horizontal { s::ITEM_HORIZONTAL } else { s::ITEM };
    let sticky_cls = if is_horizontal { s::STICKY_HEADER_HORIZONTAL } else { s::STICKY_HEADER };

    let viewport_style = if is_horizontal {
        format!("width: {viewport_size}px; height: {item_size}px;")
    } else {
        format!("height: {viewport_size}px;")
    };

    let sizer_style = if is_horizontal {
        format!("width: {total_size}px; height: 100%;")
    } else {
        format!("height: {total_size}px;")
    };

    let window_style = if is_horizontal {
        format!("transform: translateX({offset_px}px);")
    } else {
        format!("transform: translateY({offset_px}px);")
    };

    let item_style = if is_horizontal {
        format!("width: {item_size}px;")
    } else {
        format!("height: {item_size}px;")
    };

    rsx! {
        div {
            id: "{vp_id}",
            class: "{viewport_cls}",
            style: "{viewport_style}",
            role: "list",
            "aria-label": "Virtual list",
            onmounted: move |evt: MountedEvent| {
                viewport_el.set(Some(evt));
            },
            onscroll: move |_| {
                if let Some(el) = viewport_el() {
                    let is_hz = is_horizontal;
                    spawn(async move {
                        if let Ok(offset) = el.get_scroll_offset().await {
                            if is_hz {
                                scroll_offset.set(offset.x);
                            } else {
                                scroll_offset.set(offset.y);
                            }
                        }
                    });
                }
            },

            // Active sticky header
            if let Some(header) = active_sticky {
                {
                    let label = header.label.clone();
                    let label2 = label.clone();
                    rsx! {
                        div {
                            class: "{sticky_cls}",
                            if let Some(ref renderer) = render_sticky_header {
                                {renderer.call(label)}
                            } else {
                                div {
                                    class: "px-3 py-2 text-xs font-semibold uppercase tracking-wider \
                                            text-[var(--color-label-secondary)]",
                                    "{label2}"
                                }
                            }
                        }
                    }
                }
            }

            // Sizer — sets total scrollable extent
            div {
                class: "{sizer_cls}",
                style: "{sizer_style}",

                // Window — positioned at the current scroll offset
                div {
                    class: "{window_cls}",
                    style: "{window_style}",

                    for idx in win_start..win_end {
                        div {
                            key: "vl-{idx}",
                            class: "{item_cls}",
                            style: "{item_style}",
                            role: "listitem",
                            {render_item.call(idx)}
                        }
                    }
                }
            }
        }
    }
}

// ── Interactive demo ─────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqVirtualList() -> Element {
    let mut count_str = use_signal(|| "10000".to_string());
    let mut row_height_str = use_signal(|| "40".to_string());
    let mut viewport_str = use_signal(|| "400".to_string());
    let mut overscan_str = use_signal(|| "3".to_string());
    let mut horizontal = use_signal(|| false);
    let mut show_sticky = use_signal(|| false);
    let mut scroll_idx_str = use_signal(|| String::new());

    let count: usize = count_str().parse().unwrap_or(10_000).max(1);
    let row_h: f64 = row_height_str().parse().unwrap_or(40.0_f64).max(10.0);
    let vp: f64 = viewport_str().parse().unwrap_or(400.0_f64).max(100.0);
    let overscan: usize = overscan_str().parse().unwrap_or(3);
    let scroll_idx: Option<usize> = scroll_idx_str().parse().ok();

    let sticky_headers = if show_sticky() {
        // Create section headers every 50 items.
        (0..count).step_by(50)
            .map(|i| StickyHeader::new(i, format!("Section {}", i / 50 + 1)))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let direction = if horizontal() {
        VirtualListDirection::Horizontal
    } else {
        VirtualListDirection::Vertical
    };

    let code = r#"EqVirtualList {
    item_count: 10_000,
    item_size: 40.0,
    viewport_size: 400.0,
    overscan: 3,
    render_item: move |idx: usize| rsx! {
        div {
            class: "px-4 flex items-center border-b \
                    border-[var(--color-card-border)]/20",
            "Row {idx}"
        }
    },
}"#.to_string();

    rsx! {
        DemoSection { title: "EqVirtualList",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropInput {
                    label: "item_count",
                    value: count_str(),
                    placeholder: "10000",
                    onchange: move |v: String| count_str.set(v),
                }
                PropInput {
                    label: "item_size (px)",
                    value: row_height_str(),
                    placeholder: "40",
                    onchange: move |v: String| row_height_str.set(v),
                }
                PropInput {
                    label: "viewport_size (px)",
                    value: viewport_str(),
                    placeholder: "400",
                    onchange: move |v: String| viewport_str.set(v),
                }
                PropInput {
                    label: "overscan",
                    value: overscan_str(),
                    placeholder: "3",
                    onchange: move |v: String| overscan_str.set(v),
                }
                PropInput {
                    label: "scroll_to_index",
                    value: scroll_idx_str(),
                    placeholder: "(none)",
                    onchange: move |v: String| scroll_idx_str.set(v),
                }
                PropToggle {
                    label: "horizontal",
                    value: horizontal(),
                    onchange: move |v: bool| horizontal.set(v),
                }
                PropToggle {
                    label: "sticky headers (every 50)",
                    value: show_sticky(),
                    onchange: move |v: bool| show_sticky.set(v),
                }
            }

            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 space-y-2",
                EqText { variant: TextVariant::Muted,
                    "Rendering {count} items — only visible rows hit the DOM."
                }

                EqVirtualList {
                    item_count: count,
                    item_size: row_h,
                    viewport_size: vp,
                    overscan,
                    direction,
                    scroll_to_index: scroll_idx,
                    sticky_headers,
                    render_item: move |idx: usize| {
                        let bg = if idx % 2 == 0 { "bg-[var(--color-primary-dark)]/20" } else { "" };
                        rsx! {
                            div {
                                class: "px-4 flex items-center text-sm \
                                        text-[var(--color-label-primary)] {bg}",
                                style: "height: 100%; width: 100%;",
                                "Row {idx}"
                            }
                        }
                    },
                }
            }

            StyleInfo { file: "eq_virtual_list_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery ──────────────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqVirtualList() -> Element {
    let sticky_hdrs = (0..200).step_by(25)
        .map(|i| StickyHeader::new(i, format!("Group {}", i / 25 + 1)))
        .collect::<Vec<_>>();

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "VirtualList Gallery" }

                div { class: "flex items-start gap-4 flex-wrap",
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Vertical — 10k rows" }
                        EqVirtualList {
                            item_count: 10_000,
                            item_size: 36.0,
                            viewport_size: 220.0,
                            render_item: move |idx: usize| rsx! {
                                div {
                                    class: "px-3 flex items-center text-xs \
                                            text-[var(--color-label-primary)] \
                                            border-b border-[var(--color-card-border)]/20",
                                    style: "height: 100%;",
                                    "Item {idx}"
                                }
                            },
                        }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "With sticky headers" }
                        EqVirtualList {
                            item_count: 200,
                            item_size: 36.0,
                            viewport_size: 220.0,
                            sticky_headers: sticky_hdrs,
                            render_item: move |idx: usize| rsx! {
                                div {
                                    class: "px-3 flex items-center text-xs \
                                            text-[var(--color-label-primary)] \
                                            border-b border-[var(--color-card-border)]/20",
                                    style: "height: 100%;",
                                    "Row {idx}"
                                }
                            },
                        }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Horizontal — 500 cols" }
                        EqVirtualList {
                            direction: VirtualListDirection::Horizontal,
                            item_count: 500,
                            item_size: 120.0,
                            viewport_size: 360.0,
                            render_item: move |idx: usize| rsx! {
                                div {
                                    class: "flex items-center justify-center text-xs \
                                            text-[var(--color-label-primary)] \
                                            border-r border-[var(--color-card-border)]/20",
                                    style: "height: 100%; width: 100%;",
                                    "Col {idx}"
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
