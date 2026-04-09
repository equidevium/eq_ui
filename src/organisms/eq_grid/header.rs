//! Grid header (thead) rendering with sort indicators and column filters.

use super::column_def::EqColumnDef;
use super::styles as s;
use super::types::{ColumnAlign, SortDirection, SortState};
use crate::atoms::eq_icon_paths;
use crate::atoms::{EqIcon, IconSize};
use dioxus::prelude::*;
use std::collections::HashMap;

/// Render the sort indicator icon for a column header.
/// When `priority` is provided and > 0, a small number badge is shown
/// next to the icon to indicate multi-column sort order.
fn sort_icon(direction: SortDirection, priority: Option<usize>) -> Element {
    let (path, cls) = match direction {
        SortDirection::None => (eq_icon_paths::CARET_UP_DOWN, s::SORT_ICON),
        SortDirection::Asc => (eq_icon_paths::CARET_UP, s::SORT_ICON_ACTIVE),
        SortDirection::Desc => (eq_icon_paths::CARET_DOWN, s::SORT_ICON_ACTIVE),
    };
    rsx! {
        EqIcon { path: path, size: IconSize::Sm, class: cls }
        if let Some(p) = priority {
            span { class: s::SORT_PRIORITY, "{p}" }
        }
    }
}

/// Render a small colored feedback indicator showing the column's sort participation.
///
/// - **Asc** → green up arrow
/// - **Desc** → red down arrow
/// - **None** → blue dash
fn sort_feedback(direction: SortDirection) -> Element {
    let (path, cls) = match direction {
        SortDirection::Asc => (eq_icon_paths::CARET_UP, s::SORT_FEEDBACK_ASC),
        SortDirection::Desc => (eq_icon_paths::CARET_DOWN, s::SORT_FEEDBACK_DESC),
        SortDirection::None => (eq_icon_paths::MINUS, s::SORT_FEEDBACK_NONE),
    };
    rsx! {
        EqIcon { path: path, size: IconSize::Sm, class: cls }
    }
}

/// Render the `<thead>` block.
///
/// Handles sort-click cycling and multi-column sort via Shift+click.
///
/// - **Regular click**: replaces the entire sort with this column
///   (cycles None → Asc → Desc → None).
/// - **Shift+click**: appends or cycles this column within the
///   existing multi-sort list.
///
/// Resets the current page to 0 on every sort change. Renders per-column
/// filter inputs for columns with `filterable: true`.
pub(super) fn render_header<T: Clone + PartialEq + 'static>(
    columns: &[EqColumnDef<T>],
    mut sort_state: Signal<Vec<SortState>>,
    mut current_page: Signal<usize>,
    mut column_filters: Signal<HashMap<&'static str, String>>,
    density_cls: &'static str,
) -> Element {
    let sort_count = sort_state.read().len();

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

                        // Find this column's current sort direction and position.
                        let (current_sort_dir, sort_priority) = {
                            let sorts = sort_state.read();
                            let pos = sorts.iter().position(|ss| ss.column_id == col_id);
                            match pos {
                                Some(i) => (sorts[i].direction, if sort_count > 1 { Some(i + 1) } else { None }),
                                None => (SortDirection::None, None),
                            }
                        };

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
                                onclick: move |evt: Event<MouseData>| {
                                    if !is_sortable { return; }
                                    let shift = evt.modifiers().shift();

                                    if shift {
                                        // Shift+click: multi-sort — append or cycle within list.
                                        let mut sorts = sort_state.write();
                                        if let Some(pos) = sorts.iter().position(|ss| ss.column_id == col_id) {
                                            // Column already in sort list — cycle it.
                                            match sorts[pos].direction {
                                                SortDirection::None => sorts[pos].direction = SortDirection::Asc,
                                                SortDirection::Asc => sorts[pos].direction = SortDirection::Desc,
                                                SortDirection::Desc => { sorts.remove(pos); }
                                            }
                                        } else {
                                            // New column — append as Asc.
                                            sorts.push(SortState {
                                                column_id: col_id,
                                                direction: SortDirection::Asc,
                                            });
                                        }
                                    } else {
                                        // Regular click: single-sort — replace entire sort state.
                                        let current = sort_state.read()
                                            .iter()
                                            .find(|ss| ss.column_id == col_id)
                                            .map(|ss| ss.direction)
                                            .unwrap_or(SortDirection::None);

                                        let new_dir = match current {
                                            SortDirection::None => SortDirection::Asc,
                                            SortDirection::Asc => SortDirection::Desc,
                                            SortDirection::Desc => SortDirection::None,
                                        };

                                        if new_dir == SortDirection::None {
                                            sort_state.set(Vec::new());
                                        } else {
                                            sort_state.set(vec![SortState {
                                                column_id: col_id,
                                                direction: new_dir,
                                            }]);
                                        }
                                    }

                                    current_page.set(0);
                                },

                                // Header label + sort indicator
                                div { class: "flex items-center",
                                    span { "{header_text}" }
                                    if is_sortable {
                                        {sort_icon(current_sort_dir, sort_priority)}
                                        {sort_feedback(current_sort_dir)}
                                    }
                                }

                                // Column filter input
                                if is_filterable {
                                    input {
                                        class: s::COLUMN_FILTER_INPUT,
                                        r#type: "text",
                                        placeholder: "Filter\u{2026}",
                                        value: "{filter_value}",
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
