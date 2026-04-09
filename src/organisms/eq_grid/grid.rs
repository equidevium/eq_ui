//! EqGrid — feature-rich, type-safe data grid organism.
//!
//! Inspired by AG Grid's architecture. Supports sorting, filtering,
//! pagination, row selection, custom cell renderers, value formatters,
//! density presets, loading/empty states, and full theme integration.

use super::body::render_body;
use super::column_def::EqColumnDef;
use super::header::render_header;
use super::pagination::render_pagination;
use super::quick_filter::render_quick_filter;
use super::styles as s;
use super::types::{GridDensity, RowSelection, SortDirection, SortState};
use crate::atoms::eq_icon_paths;
use crate::atoms::{EqIcon, IconSize};
use crate::theme::merge_classes;
use dioxus::prelude::*;
use std::collections::HashMap;

/// Feature-rich data grid organism.
///
/// Accepts generic row data and column definitions. Handles filtering,
/// sorting, pagination, row selection, and theming internally.
#[component]
pub fn EqGrid<T: Clone + PartialEq + 'static>(
    /// Row data to display.
    data: Vec<T>,
    /// Column definitions.
    columns: Vec<EqColumnDef<T>>,
    /// Enable pagination. Default: false.
    #[props(default = false)]
    paginate: bool,
    /// Rows per page when pagination is enabled.
    #[props(default = 25)]
    page_size: usize,
    /// Row selection mode.
    #[props(default)]
    row_selection: RowSelection,
    /// Grid density (row height preset).
    #[props(default)]
    density: GridDensity,
    /// Show alternating row backgrounds.
    #[props(default = true)]
    striped: bool,
    /// Show column divider borders.
    #[props(default = false)]
    column_borders: bool,
    /// Show loading overlay.
    #[props(default = false)]
    loading: bool,
    /// Show global quick-filter search bar.
    #[props(default = false)]
    quick_filter: bool,
    /// Message when data is empty.
    #[props(into, default = "No data to display".to_string())]
    empty_message: String,
    /// Callback when a row is clicked (provides row index in original data).
    #[props(default)]
    on_row_click: Option<EventHandler<usize>>,
    /// Optional class override.
    #[props(into, default)]
    class: String,
) -> Element {
    // ── Internal state ──────────────────────────────────────────

    let sort_state = use_signal(|| Vec::<SortState>::new());
    let mut current_page = use_signal(|| 0usize);
    let selected_row = use_signal(|| Option::<usize>::None);
    let quick_filter_text = use_signal(|| String::new());
    let column_filters = use_signal(|| HashMap::<&'static str, String>::new());

    // ── Density class ───────────────────────────────────────────

    let density_cls: &'static str = match density {
        GridDensity::Compact => s::DENSITY_COMPACT,
        GridDensity::Normal => s::DENSITY_NORMAL,
        GridDensity::Comfortable => s::DENSITY_COMFORTABLE,
    };

    // ── Filter → Sort → Paginate pipeline ───────────────────────

    // Step 1: Collect all row indices.
    let all_indices: Vec<usize> = (0..data.len()).collect();

    // Step 2: Apply column filters (AND logic — row must match ALL active filters).
    let col_filters = column_filters.read();
    let filtered_indices: Vec<usize> = if col_filters.is_empty() && quick_filter_text.read().is_empty() {
        all_indices
    } else {
        all_indices
            .into_iter()
            .filter(|&idx| {
                // Column filters: AND logic
                for (&col_id, filter_text) in col_filters.iter() {
                    if filter_text.is_empty() {
                        continue;
                    }
                    let needle = filter_text.to_lowercase();
                    if let Some(col) = columns.iter().find(|c| c.id == col_id) {
                        let val = (col.value_getter)(&data[idx]).to_lowercase();
                        if !val.contains(&needle) {
                            return false;
                        }
                    }
                }

                // Quick filter: OR logic — any column value contains the text
                let qf = quick_filter_text.read();
                if !qf.is_empty() {
                    let needle = qf.to_lowercase();
                    let matches_any = columns.iter().any(|col| {
                        let val = (col.value_getter)(&data[idx]).to_lowercase();
                        val.contains(&needle)
                    });
                    if !matches_any {
                        return false;
                    }
                }

                true
            })
            .collect()
    };
    drop(col_filters);

    // Step 3: Sort the filtered indices (supports multi-column chained sort).
    let sorted_indices: Vec<usize> = {
        let mut indices = filtered_indices;
        let sorts = sort_state.read();

        if !sorts.is_empty() {
            indices.sort_by(|&a, &b| {
                for sort in sorts.iter() {
                    if sort.direction == SortDirection::None {
                        continue;
                    }

                    if let Some(col) = columns.iter().find(|c| c.id == sort.column_id) {
                        let ord = if let Some(cmp) = col.comparator {
                            cmp(&data[a], &data[b])
                        } else {
                            let va = (col.value_getter)(&data[a]);
                            let vb = (col.value_getter)(&data[b]);
                            va.cmp(&vb)
                        };

                        let ord = if sort.direction == SortDirection::Asc {
                            ord
                        } else {
                            ord.reverse()
                        };

                        if ord != std::cmp::Ordering::Equal {
                            return ord;
                        }
                    }
                }

                std::cmp::Ordering::Equal
            });
        }

        indices
    };

    // Step 4: Paginate.
    let total_rows = sorted_indices.len();
    let total_pages = if paginate && page_size > 0 {
        (total_rows + page_size - 1) / page_size
    } else {
        1
    };

    // Clamp page when filtered/sorted data shrinks.
    let page = if current_page() >= total_pages && total_pages > 0 {
        current_page.set(total_pages - 1);
        total_pages - 1
    } else {
        current_page()
    };

    let visible_indices: Vec<usize> = if paginate {
        let start = page * page_size;
        let end = (start + page_size).min(total_rows);
        sorted_indices[start..end].to_vec()
    } else {
        sorted_indices
    };

    let row_start = page * page_size + 1;
    let row_end = (row_start - 1 + visible_indices.len()).max(row_start);

    // ── Render ──────────────────────────────────────────────────

    let wrapper_cls = merge_classes(s::GRID_WRAPPER, &class);

    rsx! {
        div { class: "{wrapper_cls}",

            // Quick filter bar (above the table)
            if quick_filter {
                {render_quick_filter(quick_filter_text, current_page)}
            }

            // Grid container (relative for loading overlay positioning)
            div { class: "relative {s::GRID_CONTAINER}",

                // Loading overlay
                if loading {
                    div { class: s::LOADING_OVERLAY,
                        EqIcon {
                            path: eq_icon_paths::SPINNER,
                            class: s::LOADING_SPINNER,
                        }
                    }
                }

                if data.is_empty() && !loading {
                    // Empty state
                    div { class: s::EMPTY_STATE, "{empty_message}" }
                } else {
                    // Table
                    table { class: s::TABLE,
                        {render_header(&columns, sort_state, current_page, column_filters, density_cls)}
                        {
                            render_body(
                                &data,
                                &columns,
                                &visible_indices,
                                density_cls,
                                striped,
                                column_borders,
                                row_selection,
                                selected_row,
                                &on_row_click,
                            )
                        }
                    }
                }
            }

            // Pagination bar
            if paginate && total_rows > 0 {
                {
                    render_pagination(
                        page,
                        total_pages,
                        total_rows,
                        row_start,
                        row_end,
                        current_page,
                    )
                }
            }
        }
    }
}
