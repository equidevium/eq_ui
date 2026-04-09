//! Grid header (thead) rendering with sort indicators and column filters.

use super::column_def::EqColumnDef;
use super::styles as s;
use super::types::{ColumnAlign, ResizeState, RowSelection, SortDirection, SortState};
use crate::atoms::eq_icon_paths;
use crate::atoms::{EqCheckbox, CheckboxState, EqIcon, IconSize};
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};

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
    row_selection: RowSelection,
    mut selected_rows: Signal<HashSet<usize>>,
    visible_indices: &[usize],
    on_selection_change: &Option<EventHandler<Vec<usize>>>,
    column_widths: Signal<HashMap<&'static str, f64>>,
    mut resize_active: Signal<Option<ResizeState>>,
    reorderable: bool,
) -> Element {
    let sort_count = sort_state.read().len();

    // Select All state: all visible rows selected?
    let all_selected = row_selection == RowSelection::Multi
        && !visible_indices.is_empty()
        && visible_indices.iter().all(|idx| selected_rows.read().contains(idx));
    let some_selected = row_selection == RowSelection::Multi
        && !all_selected
        && visible_indices.iter().any(|idx| selected_rows.read().contains(idx));

    // Copy visible indices for the closure (can't capture slice).
    let vis = visible_indices.to_vec();
    let on_sel = on_selection_change.clone();

    rsx! {
        thead { class: s::THEAD,
            tr {
                // Grip handle column (empty header cell)
                if reorderable {
                    th { class: "{s::TH} {s::GRIP_CELL} {density_cls}" }
                }
                // Select All checkbox column
                if row_selection == RowSelection::Multi {
                    {
                        let cb_state = if all_selected {
                            CheckboxState::Checked
                        } else if some_selected {
                            CheckboxState::Indeterminate
                        } else {
                            CheckboxState::Unchecked
                        };
                        rsx! {
                            th {
                                class: "{s::TH} {s::CHECKBOX_CELL} {density_cls}",
                                EqCheckbox {
                                    state: cb_state,
                                    on_change: move |_new: CheckboxState| {
                                        let mut set = selected_rows.write();
                                        if all_selected {
                                            for &idx in vis.iter() {
                                                set.remove(&idx);
                                            }
                                        } else {
                                            for &idx in vis.iter() {
                                                set.insert(idx);
                                            }
                                        }
                                        let sorted: Vec<usize> = {
                                            let mut v: Vec<usize> = set.iter().copied().collect();
                                            v.sort();
                                            v
                                        };
                                        drop(set);
                                        if let Some(ref handler) = on_sel {
                                            handler.call(sorted);
                                        }
                                    },
                                }
                            }
                        }
                    }
                }
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
                        let resize_cls = if col.resizable { s::TH_RESIZABLE } else { "" };
                        let is_resizable = col.resizable;
                        let col_min_width = col.min_width as f64;
                        let col_initial_width = col.width.map(|w| w as f64)
                            .unwrap_or(col_min_width.max(120.0));

                        // Runtime width takes priority, then column def, then flex.
                        let width_style = {
                            let widths = column_widths.read();
                            if let Some(&w) = widths.get(col_id) {
                                format!("width: {:.0}px; min-width: {}px;", w, col.min_width)
                            } else if let Some(w) = col.width {
                                format!("width: {}px; min-width: {}px;", w, col.min_width)
                            } else {
                                format!("min-width: {}px;", col.min_width)
                            }
                        };

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
                                class: "{s::TH} {density_cls} {align_cls} {sort_cls} {resize_cls} {header_class}",
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

                                // Resize drag handle on the right edge
                                if is_resizable {
                                    div {
                                        class: s::RESIZE_HANDLE,
                                        // Prevent sort click from firing when starting a resize.
                                        onclick: move |evt: Event<MouseData>| { evt.stop_propagation(); },
                                        onmousedown: {
                                            let column_widths = column_widths;
                                            move |evt: Event<MouseData>| {
                                                evt.stop_propagation();
                                                let current_w = {
                                                    let widths = column_widths.read();
                                                    widths.get(col_id).copied()
                                                        .unwrap_or(col_initial_width)
                                                };
                                                resize_active.set(Some(ResizeState {
                                                    column_id: col_id,
                                                    start_x: evt.page_coordinates().x,
                                                    start_width: current_w,
                                                }));
                                            }
                                        },
                                        // Double-click resets column to initial width.
                                        ondoubleclick: {
                                            let mut column_widths = column_widths;
                                            move |evt: Event<MouseData>| {
                                                evt.stop_propagation();
                                                column_widths.write().remove(col_id);
                                            }
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
