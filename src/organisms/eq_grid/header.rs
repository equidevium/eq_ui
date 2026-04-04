//! Grid header (thead) rendering with sort indicators and column filters.

use super::column_def::EqColumnDef;
use super::styles as s;
use super::types::{ColumnAlign, SortDirection, SortState};
use crate::atoms::eq_icon_paths;
use crate::atoms::{EqIcon, IconSize};
use dioxus::prelude::*;
use std::collections::HashMap;

/// Render the sort indicator icon for a column header.
fn sort_icon(direction: SortDirection) -> Element {
    let (path, cls) = match direction {
        SortDirection::None => (eq_icon_paths::CARET_UP_DOWN, s::SORT_ICON),
        SortDirection::Asc => (eq_icon_paths::CARET_UP, s::SORT_ICON_ACTIVE),
        SortDirection::Desc => (eq_icon_paths::CARET_DOWN, s::SORT_ICON_ACTIVE),
    };
    rsx! { EqIcon { path: path, size: IconSize::Sm, class: cls } }
}

/// Render the `<thead>` block.
///
/// Handles sort-click cycling (None → Asc → Desc → None), resets the
/// current page to 0 on every sort change, and renders per-column
/// filter inputs for columns with `filterable: true`.
pub(super) fn render_header<T: Clone + PartialEq + 'static>(
    columns: &[EqColumnDef<T>],
    mut sort_state: Signal<Option<SortState>>,
    mut current_page: Signal<usize>,
    mut column_filters: Signal<HashMap<&'static str, String>>,
    density_cls: &'static str,
) -> Element {
    rsx! {
        thead { class: s::THEAD,
            tr {
                for col in columns.iter() {
                    {
                        let col_id = col.id;
                        let is_sortable = col.sortable;
                        let is_filterable = col.filterable;
                        let align_cls = match col.align {
                            ColumnAlign::Left => s::ALIGN_LEFT,
                            ColumnAlign::Center => s::ALIGN_CENTER,
                            ColumnAlign::Right => s::ALIGN_RIGHT,
                        };

                        let sort_cls = if is_sortable { s::TH_SORTABLE } else { "" };

                        let width_style = col.width
                            .map(|w| format!("width: {}px; min-width: {}px;", w, col.min_width))
                            .unwrap_or_else(|| format!("min-width: {}px;", col.min_width));

                        let current_sort_dir = sort_state.read().as_ref()
                            .filter(|ss| ss.column_id == col_id)
                            .map(|ss| ss.direction)
                            .unwrap_or(SortDirection::None);

                        let header_text = col.header;
                        let header_class = col.header_class;

                        let filter_value = column_filters.read()
                            .get(col_id)
                            .cloned()
                            .unwrap_or_default();

                        rsx! {
                            th {
                                key: "{col_id}",
                                class: "{s::TH} {density_cls} {align_cls} {sort_cls} {header_class}",
                                style: "{width_style}",
                                onclick: move |_| {
                                    if !is_sortable { return; }
                                    let new_dir = match &*sort_state.read() {
                                        Some(ss) if ss.column_id == col_id => {
                                            match ss.direction {
                                                SortDirection::None => SortDirection::Asc,
                                                SortDirection::Asc => SortDirection::Desc,
                                                SortDirection::Desc => SortDirection::None,
                                            }
                                        }
                                        _ => SortDirection::Asc,
                                    };
                                    if new_dir == SortDirection::None {
                                        sort_state.set(None);
                                    } else {
                                        sort_state.set(Some(SortState {
                                            column_id: col_id,
                                            direction: new_dir,
                                        }));
                                    }
                                    current_page.set(0);
                                },

                                // Header label + sort indicator
                                div { class: "flex items-center",
                                    span { "{header_text}" }
                                    if is_sortable {
                                        {sort_icon(current_sort_dir)}
                                    }
                                }

                                // Column filter input
                                if is_filterable {
                                    input {
                                        class: s::COLUMN_FILTER_INPUT,
                                        r#type: "text",
                                        placeholder: "Filter\u{2026}",
                                        value: "{filter_value}",
                                        // Stop click propagation so typing doesn't trigger sort
                                        onclick: move |evt: Event<MouseData>| { evt.stop_propagation(); },
                                        oninput: move |evt: Event<FormData>| {
                                            let val = evt.value();
                                            let mut filters = column_filters.write();
                                            if val.is_empty() {
                                                filters.remove(col_id);
                                            } else {
                                                filters.insert(col_id, val);
                                            }
                                            drop(filters);
                                            current_page.set(0);
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
