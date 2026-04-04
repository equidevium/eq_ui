//! Grid pagination bar with page navigation.

use super::styles as s;
use crate::atoms::eq_icon_paths;
use crate::atoms::{EqIcon, IconSize};
use dioxus::prelude::*;

/// Render the pagination bar below the table.
///
/// Shows a "Showing X–Y of Z" label on the left and a page-number
/// navigation strip with previous / next arrows on the right.
pub(super) fn render_pagination(
    page: usize,
    total_pages: usize,
    total_rows: usize,
    row_start: usize,
    row_end: usize,
    mut current_page: Signal<usize>,
) -> Element {
    let (window_start, window_end) = page_window(page, total_pages, 5);

    rsx! {
        div { class: s::PAGINATION_BAR,
            // Row info
            span { class: s::PAGINATION_INFO,
                "Showing {row_start}\u{2013}{row_end} of {total_rows}"
            }

            // Page navigation
            div { class: s::PAGINATION_NAV,
                // Previous
                button {
                    class: if page == 0 { s::PAGE_BTN_DISABLED } else { s::PAGE_BTN },
                    disabled: page == 0,
                    onclick: move |_| {
                        if page > 0 { current_page.set(page - 1); }
                    },
                    EqIcon { path: eq_icon_paths::CARET_LEFT, size: IconSize::Sm }
                }

                // Page numbers
                for p in window_start..window_end {
                    button {
                        key: "{p}",
                        class: if p == page { s::PAGE_BTN_ACTIVE } else { s::PAGE_BTN },
                        onclick: move |_| { current_page.set(p); },
                        "{p + 1}"
                    }
                }

                // Next
                button {
                    class: if page + 1 >= total_pages { s::PAGE_BTN_DISABLED } else { s::PAGE_BTN },
                    disabled: page + 1 >= total_pages,
                    onclick: move |_| {
                        if page + 1 < total_pages { current_page.set(page + 1); }
                    },
                    EqIcon { path: eq_icon_paths::CARET_RIGHT, size: IconSize::Sm }
                }
            }
        }
    }
}

/// Compute a sliding window of page numbers around the current page.
/// Returns (start, end) as a half-open range.
fn page_window(current: usize, total: usize, window_size: usize) -> (usize, usize) {
    if total <= window_size {
        return (0, total);
    }
    let half = window_size / 2;
    let start = if current <= half {
        0
    } else if current + half >= total {
        total - window_size
    } else {
        current - half
    };
    (start, start + window_size)
}
