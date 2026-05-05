//! Grid body (tbody) rendering with row selection and cell formatting.

use super::column_def::EqColumnDef;
use super::styles as s;
use super::types::{ColumnAlign, RowSelection};
use crate::atoms::eq_icon_paths;
use crate::atoms::{EqCheckbox, CheckboxState, EqIcon, IconSize};
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};

/// Render the `<tbody>` block.
///
/// Iterates `visible_indices` (already sorted and paginated) and
/// renders one `<tr>` per row with optional selection highlighting.
pub(super) fn render_body<T: Clone + PartialEq + 'static>(
    data: &[T],
    columns: &[EqColumnDef<T>],
    visible_indices: &[usize],
    density_cls: &'static str,
    striped: bool,
    column_borders: bool,
    row_selection: RowSelection,
    mut selected_row: Signal<Option<usize>>,
    mut selected_rows: Signal<HashSet<usize>>,
    on_row_click: &Option<EventHandler<usize>>,
    on_selection_change: &Option<EventHandler<Vec<usize>>>,
    column_widths: Signal<HashMap<&'static str, f64>>,
    top_spacer_height: f64,
    bottom_spacer_height: f64,
    row_height: f64,
    // When provided, the first data row measures its rendered height
    // and writes it into this signal so the viewport can self-correct.
    row_measure: Option<Signal<f64>>,
    // When true, selected rows get `draggable="true"`.
    row_draggable: bool,
    // Reorder grip handle support.
    reorderable: bool,
    mut reorder_from: Signal<Option<usize>>,
    mut reorder_over: Signal<Option<usize>>,
    on_reorder: &Option<EventHandler<(usize, usize)>>,
    // ARIA move announcements.
    announce_moves: bool,
    mut move_announcement: Signal<String>,
) -> Element {
    let border_cls = if column_borders {
        "border-r border-[var(--color-grid-border)] last:border-r-0"
    } else {
        ""
    };

    // Row height style - only applied when virtualized (row_height > 0).
    let row_h_style = if row_height > 0.0 {
        format!("height: {:.0}px; max-height: {:.0}px; overflow: hidden;", row_height, row_height)
    } else {
        String::new()
    };

    // Total column count for spacer colspan (include grip + checkbox columns).
    let col_count = columns.len()
        + if reorderable { 1 } else { 0 }
        + if row_selection == RowSelection::Multi { 1 } else { 0 };
    let col_span = format!("{col_count}");

    rsx! {
        tbody {
            // Top spacer - pushes visible rows into their correct scroll position.
            // Height is set on the <td> (not <tr>) because browsers ignore height on <tr>.
            if top_spacer_height > 0.0 {
                tr { "aria-hidden": "true",
                    td {
                        colspan: "{col_span}",
                        style: "height: {top_spacer_height:.0}px; padding: 0; border: none; line-height: 0;",
                    }
                }
            }
            for (vi, &data_idx) in visible_indices.iter().enumerate() {
                {
                    let row = &data[data_idx];
                    let is_selected = match row_selection {
                        RowSelection::Single => selected_row() == Some(data_idx),
                        RowSelection::Multi => selected_rows.read().contains(&data_idx),
                        RowSelection::None => false,
                    };

                    let row_cls = {
                        let mut cls = String::from(s::TR);
                        if striped {
                            cls.push(' ');
                            cls.push_str(s::TR_STRIPED);
                        }
                        if row_selection != RowSelection::None {
                            cls.push(' ');
                            cls.push_str(s::TR_HOVER);
                            cls.push(' ');
                            cls.push_str(s::TR_SELECTABLE);
                        }
                        if is_selected {
                            cls.push(' ');
                            cls.push_str(s::TR_SELECTED);
                        }
                        // Reorder insertion indicator
                        if reorderable {
                            if let Some(over_idx) = reorder_over() {
                                if over_idx == data_idx {
                                    if let Some(from_idx) = reorder_from() {
                                        if from_idx < data_idx {
                                            cls.push(' ');
                                            cls.push_str(s::REORDER_INSERT_BELOW);
                                        } else if from_idx > data_idx {
                                            cls.push(' ');
                                            cls.push_str(s::REORDER_INSERT_ABOVE);
                                        }
                                    }
                                }
                            }
                        }
                        cls
                    };

                    let on_click = on_row_click.clone();
                    let on_sel = on_selection_change.clone();
                    let measure_this = vi == 0 && row_measure.is_some();

                    let is_draggable = row_draggable && is_selected;

                    let on_reorder_handler = on_reorder.clone();

                    let has_selection = row_selection != RowSelection::None;

                    rsx! {
                        tr {
                            key: "{data_idx}",
                            class: "{row_cls}",
                            style: "{row_h_style}",
                            "aria-selected": if has_selection { if is_selected { "true" } else { "false" } } else { "" },
                            draggable: if is_draggable { "true" } else { "false" },
                            ondragover: move |evt: Event<DragData>| {
                                if reorderable && reorder_from().is_some() {
                                    evt.prevent_default();
                                    reorder_over.set(Some(data_idx));
                                }
                            },
                            ondragleave: move |_| {
                                if reorderable {
                                    let current = reorder_over();
                                    if current == Some(data_idx) {
                                        reorder_over.set(None);
                                    }
                                }
                            },
                            ondrop: {
                                let on_reorder_handler = on_reorder_handler.clone();
                                move |evt: Event<DragData>| {
                                    if reorderable {
                                        evt.prevent_default();
                                        evt.stop_propagation();
                                        if let Some(from) = reorder_from() {
                                            if from != data_idx {
                                                if announce_moves {
                                                    move_announcement.set(format!(
                                                        "Row moved from position {} to position {}",
                                                        from + 1,
                                                        data_idx + 1,
                                                    ));
                                                }
                                                if let Some(ref handler) = on_reorder_handler {
                                                    handler.call((from, data_idx));
                                                }
                                            }
                                        }
                                        reorder_from.set(None);
                                        reorder_over.set(None);
                                    }
                                }
                            },
                            onmounted: move |evt: MountedEvent| {
                                if measure_this {
                                    if let Some(mut sig) = row_measure {
                                        spawn(async move {
                                            if let Ok(rect) = evt.get_client_rect().await {
                                                let h = rect.height();
                                                if h > 0.0 && (h - sig()).abs() > 1.0 {
                                                    sig.set(h);
                                                }
                                            }
                                        });
                                    }
                                }
                            },
                            onclick: move |_| {
                                match row_selection {
                                    RowSelection::Single => {
                                        if selected_row() == Some(data_idx) {
                                            selected_row.set(None);
                                        } else {
                                            selected_row.set(Some(data_idx));
                                        }
                                    }
                                    RowSelection::Multi => {
                                        let mut set = selected_rows.write();
                                        if set.contains(&data_idx) {
                                            set.remove(&data_idx);
                                        } else {
                                            set.insert(data_idx);
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
                                    }
                                    RowSelection::None => {}
                                }
                                if let Some(ref handler) = on_click {
                                    handler.call(data_idx);
                                }
                            },

                            // Grip handle cell for row reordering
                            if reorderable {
                                td {
                                    class: "{s::TD} {s::GRIP_CELL} {density_cls}",
                                    draggable: "true",
                                    ondragstart: move |evt: Event<DragData>| {
                                        evt.stop_propagation();
                                        reorder_from.set(Some(data_idx));
                                        if announce_moves {
                                            move_announcement.set(format!(
                                                "Grabbed row at position {}. Use drop to reorder.",
                                                data_idx + 1,
                                            ));
                                        }
                                    },
                                    ondragend: move |_| {
                                        reorder_from.set(None);
                                        reorder_over.set(None);
                                    },
                                    EqIcon {
                                        path: eq_icon_paths::DOTS_SIX_VERTICAL,
                                        size: IconSize::Sm,
                                        class: s::GRIP_ICON,
                                    }
                                }
                            }

                            // Checkbox cell for Multi selection
                            if row_selection == RowSelection::Multi {
                                td {
                                    class: "{s::TD} {s::CHECKBOX_CELL} {density_cls}",
                                    EqCheckbox {
                                        state: if is_selected { CheckboxState::Checked } else { CheckboxState::Unchecked },
                                        on_change: {
                                            let on_sel = on_sel.clone();
                                            move |_new: CheckboxState| {
                                                let mut set = selected_rows.write();
                                                if set.contains(&data_idx) {
                                                    set.remove(&data_idx);
                                                } else {
                                                    set.insert(data_idx);
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
                                            }
                                        },
                                    }
                                }
                            }

                            for col in columns.iter() {
                                {
                                    let align_cls = match col.align {
                                        ColumnAlign::Left => s::ALIGN_LEFT,
                                        ColumnAlign::Center => s::ALIGN_CENTER,
                                        ColumnAlign::Right => s::ALIGN_RIGHT,
                                    };

                                    let width_style = {
                                        let widths = column_widths.read();
                                        if let Some(&w) = widths.get(col.id) {
                                            format!("width: {:.0}px; min-width: {}px;", w, col.min_width)
                                        } else if let Some(w) = col.width {
                                            format!("width: {}px; min-width: {}px;", w, col.min_width)
                                        } else {
                                            format!("min-width: {}px;", col.min_width)
                                        }
                                    };

                                    let cell_content = if let Some(renderer) = col.cell_renderer {
                                        renderer(row)
                                    } else {
                                        let text = col.value_formatter
                                            .map(|f| f(row))
                                            .unwrap_or_else(|| (col.value_getter)(row));
                                        rsx! { "{text}" }
                                    };

                                    rsx! {
                                        td {
                                            key: "{col.id}",
                                            class: "{s::TD} {density_cls} {align_cls} {border_cls} {col.cell_class}",
                                            style: "{width_style}",
                                            {cell_content}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Bottom spacer - fills remaining scroll height below visible rows.
            if bottom_spacer_height > 0.0 {
                tr { "aria-hidden": "true",
                    td {
                        colspan: "{col_span}",
                        style: "height: {bottom_spacer_height:.0}px; padding: 0; border: none; line-height: 0;",
                    }
                }
            }
        }
    }
}
