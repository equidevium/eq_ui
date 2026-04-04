//! Global quick-filter search bar rendered above the grid table.

use super::styles as s;
use crate::atoms::eq_icon_paths;
use crate::atoms::{EqIcon, IconSize};
use dioxus::prelude::*;

/// Render the quick-filter bar.
///
/// A single text input that filters all columns with OR logic
/// (a row matches if *any* column value contains the search text).
pub(super) fn render_quick_filter(
    mut quick_filter_text: Signal<String>,
    mut current_page: Signal<usize>,
) -> Element {
    rsx! {
        div { class: s::QUICK_FILTER,
            EqIcon {
                path: eq_icon_paths::MAGNIFYING_GLASS,
                size: IconSize::Sm,
                muted: true,
            }
            input {
                class: s::QUICK_FILTER_INPUT,
                r#type: "text",
                placeholder: "Search\u{2026}",
                value: "{quick_filter_text}",
                oninput: move |evt: Event<FormData>| {
                    quick_filter_text.set(evt.value());
                    current_page.set(0);
                },
            }
        }
    }
}
