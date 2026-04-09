//! Grid body (tbody) rendering with row selection and cell formatting.

use super::column_def::EqColumnDef;
use super::styles as s;
use super::types::{ColumnAlign, RowSelection};
use crate::atoms::eq_icon_paths;
use crate::atoms::{EqIcon, IconSize};
use dioxus::prelude::*;
use std::collections::HashSet;

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
) -> Element {
    let border_cls = if column_borders {
        "border-r border-[var(--color-grid-border)] last:border-r-0"
    } else {
        ""
    };

    rsx! {
        tbody {
            for &data_idx in visible_indices.iter() {
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
                        cls
                    };

                    let on_click = on_row_click.clone();
                    let on_sel = on_selection_change.clone();

                    rsx! {
                        tr {
                            key: "{data_idx}",
                            class: "{row_cls}",
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

                            // Checkbox cell for Multi selection
                            if row_selection == RowSelection::Multi {
                                td {
                                    class: "{s::TD} {s::CHECKBOX_CELL} {density_cls}",
                                    if is_selected {
                                        EqIcon { path: eq_icon_paths::CHECK, size: IconSize::Sm, class: s::CHECKBOX_ICON_CHECKED }
                                    } else {
                                        span { class: s::CHECKBOX_ICON, "\u{25A1}" }
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
                                            {cell_content}
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
}
