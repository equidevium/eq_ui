//! EqGrid — feature-rich, type-safe data grid organism.
//!
//! Inspired by AG Grid's architecture. Supports sorting, filtering,
//! pagination, row selection, custom cell renderers, value formatters,
//! density presets, loading/empty states, and full theme integration.

use super::body::render_body;
use super::bulk_actions::render_bulk_actions;
use super::column_def::EqColumnDef;
use super::header::render_header;
use super::pagination::render_pagination;
use super::quick_filter::render_quick_filter;
use super::styles as s;
use super::types::{ExportFormat, GridDensity, GridDragPayload, GridNavigation, ResizeState, RowSelection, SortDirection, SortState};
use crate::atoms::eq_icon_paths;
use crate::atoms::EqIcon;
use crate::theme::merge_classes;
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};

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
    /// Navigation mode: Standard (all rows), Paginate (page controls),
    /// or Virtualize (virtual scroll). Default: Standard.
    #[props(default)]
    navigation: GridNavigation,
    /// Rows per page (Paginate) or visible rows in the viewport
    /// (Virtualize). Ignored when navigation is Standard.
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
    /// Callback when the set of selected rows changes (Multi selection mode).
    /// Provides a sorted `Vec<usize>` of selected row indices.
    #[props(default)]
    on_selection_change: Option<EventHandler<Vec<usize>>>,
    // ── Bulk action props ──────────────────────────────────────
    /// Callback for the Delete bulk action. When provided, a Delete button
    /// appears in the action bar. Receives sorted indices of selected rows.
    #[props(default)]
    on_delete: Option<EventHandler<Vec<usize>>>,
    /// Enable the Export dropdown in the bulk action bar.
    #[props(default = false)]
    export: bool,
    /// Callback when data is exported. Receives `(ExportFormat, Vec<u8>)`.
    /// For text formats the bytes are UTF-8; for ODS they are raw ZIP bytes.
    #[props(default)]
    on_export: Option<EventHandler<(ExportFormat, Vec<u8>)>>,
    /// Callback for clipboard copy. Receives the content as a `String`.
    /// The consumer is responsible for writing to the platform clipboard.
    #[props(default)]
    on_clipboard: Option<EventHandler<String>>,
    /// Column ID of the "status" column for the Change Status action.
    #[props(default)]
    status_column: Option<&'static str>,
    /// Valid status values shown in the Change Status dropdown.
    #[props(default)]
    status_options: Vec<String>,
    /// Callback when status is changed on selected rows.
    /// Receives `(Vec<usize>, String)` — selected indices and the new status.
    #[props(default)]
    on_status_change: Option<EventHandler<(Vec<usize>, String)>>,
    /// Column IDs to aggregate when rows are selected.
    /// Numeric columns show a sum; non-numeric columns show a count.
    #[props(default)]
    aggregation_columns: Vec<&'static str>,
    /// Extensible slot for consumer-defined custom bulk action buttons.
    #[props(default)]
    bulk_actions: Option<Element>,
    // ── Drag-and-drop props ───────────────────────────────────
    /// Enables dragging selected rows out of this grid. The string
    /// identifies this grid as the drag source.
    #[props(default)]
    drag_id: Option<&'static str>,
    /// When true, this grid accepts drops from other grids.
    #[props(default = false)]
    drop_target: bool,
    /// Fired when rows are dropped onto this grid. Provides the source
    /// grid ID and the dragged row indices so the consumer can move data.
    #[props(default)]
    on_drop_receive: Option<EventHandler<GridDragPayload>>,
    /// Optional class override.
    #[props(into, default)]
    class: String,
) -> Element {
    // ── Internal state ──────────────────────────────────────────

    let sort_state = use_signal(|| Vec::<SortState>::new());
    let mut current_page = use_signal(|| 0usize);
    let selected_row = use_signal(|| Option::<usize>::None);
    let selected_rows = use_signal(|| HashSet::<usize>::new());
    let quick_filter_text = use_signal(|| String::new());
    let column_filters = use_signal(|| HashMap::<&'static str, String>::new());
    let column_widths = use_signal(|| HashMap::<&'static str, f64>::new());
    let resize_active = use_signal(|| Option::<ResizeState>::None);
    let mut container_element: Signal<Option<MountedEvent>> = use_signal(|| None);
    let mut container_width = use_signal(|| 0.0_f64);
    let mut scroll_top = use_signal(|| 0.0_f64);
    let mut viewport_element: Signal<Option<MountedEvent>> = use_signal(|| None);
    // Actual row height — seeded with the density constant, then
    // refined after mount by measuring the first rendered data row.
    let measured_row_height = use_signal(|| density.row_height());
    // Drag-and-drop: shared context for passing payloads between grids.
    // The consumer must wrap both grids in a context provider:
    //   use_context_provider(|| Signal::new(Option::<GridDragPayload>::None));
    let drag_ctx: Option<Signal<Option<GridDragPayload>>> = try_consume_context();
    let mut drop_hover = use_signal(|| false);

    // Re-measure container width whenever the mounted element changes.
    use_effect(move || {
        if let Some(el) = container_element() {
            spawn(async move {
                if let Ok(rect) = el.get_client_rect().await {
                    container_width.set(rect.width());
                }
            });
        }
    });

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

    // Step 4: Paginate / Virtualize.
    let paginate = navigation == GridNavigation::Paginate;
    let virtualize = navigation == GridNavigation::Virtualize;

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

    // When virtualization is active, it replaces pagination as the
    // navigation mechanism. All sorted/filtered rows are fed into
    // the virtual scroll engine; pagination is bypassed.
    let visible_indices: Vec<usize> = if virtualize {
        sorted_indices
    } else if paginate {
        let start = page * page_size;
        let end = (start + page_size).min(total_rows);
        sorted_indices[start..end].to_vec()
    } else {
        sorted_indices
    };

    let row_start = if virtualize { 1 } else { page * page_size + 1 };
    let row_end = (row_start - 1 + visible_indices.len()).max(row_start);

    // ── Virtual scroll windowing ────────────────────────────────

    let row_height = if virtualize { measured_row_height() } else { 0.0 };
    let buffer = 3_usize; // extra rows above and below the viewport

    // `virt_first` / `virt_last` track the visible row range (1-based)
    // shown in the virtualization info footer.
    let (windowed_indices, top_spacer, bottom_spacer, viewport_height, virt_first, virt_last) = if virtualize {
        let total_vis = visible_indices.len();
        let vp_h = (page_size as f64) * row_height;

        if total_vis == 0 {
            (Vec::new(), 0.0, 0.0, vp_h, 0usize, 0usize)
        } else {
            let first = ((scroll_top() / row_height).floor() as usize).min(total_vis - 1);
            let last = (first + page_size).min(total_vis);

            // Expand by buffer, clamped to bounds.
            let win_start = first.saturating_sub(buffer);
            let win_end = (last + buffer).min(total_vis);

            let top_h = (win_start as f64) * row_height;
            let bottom_h = ((total_vis - win_end) as f64) * row_height;

            (visible_indices[win_start..win_end].to_vec(), top_h, bottom_h, vp_h, first + 1, last)
        }
    } else {
        (visible_indices.clone(), 0.0, 0.0, 0.0, 0, 0)
    };

    // ── Colgroup — shared column widths for split-table virtualisation ──

    let render_colgroup = |cols: &[EqColumnDef<T>],
                           widths: Signal<HashMap<&'static str, f64>>,
                           has_checkbox: bool|
     -> Element {
        let w = widths.read();
        rsx! {
            colgroup {
                if has_checkbox {
                    col { style: "width: 40px;" }
                }
                for col in cols.iter() {
                    {
                        let width_style = if let Some(&runtime_w) = w.get(col.id) {
                            format!("width: {:.0}px;", runtime_w)
                        } else if let Some(initial) = col.width {
                            format!("width: {}px;", initial)
                        } else {
                            String::new()
                        };
                        rsx! { col { style: "{width_style}" } }
                    }
                }
            }
        }
    };

    // ── Render ──────────────────────────────────────────────────

    let wrapper_cls = merge_classes(s::GRID_WRAPPER, &class);

    rsx! {
        div { class: "{wrapper_cls}",

            // Transparent full-viewport overlay during column resize drag.
            // Captures mousemove / mouseup so the drag works even when the
            // cursor leaves the header area.
            if resize_active.read().is_some() {
                div {
                    class: s::RESIZE_OVERLAY,
                    onmousemove: {
                        let mut column_widths = column_widths;
                        let resize_active = resize_active;
                        let columns = columns.clone();
                        move |evt: Event<MouseData>| {
                            if let Some(ref state) = *resize_active.read() {
                                let delta = evt.page_coordinates().x - state.start_x;
                                let col_id = state.column_id;

                                // Compute the minimum width for this column.
                                let min_w = columns.iter()
                                    .find(|c| c.id == col_id)
                                    .map(|c| c.min_width as f64)
                                    .unwrap_or(50.0);

                                let mut new_w = (state.start_width + delta).max(min_w);

                                // Clamp so total table width does not exceed container.
                                let cw = container_width();
                                if cw > 0.0 {
                                    let widths = column_widths.read();
                                    let checkbox_w = if row_selection == RowSelection::Multi { 40.0 } else { 0.0 };
                                    let others_total: f64 = columns.iter()
                                        .filter(|c| c.id != col_id)
                                        .map(|c| {
                                            widths.get(c.id).copied()
                                                .unwrap_or_else(|| c.width.map(|w| w as f64)
                                                    .unwrap_or(c.min_width as f64))
                                        })
                                        .sum();
                                    let max_w = (cw - others_total - checkbox_w).max(min_w);
                                    new_w = new_w.min(max_w);
                                }

                                column_widths.write().insert(col_id, new_w);
                            }
                        }
                    },
                    onmouseup: {
                        let mut resize_active = resize_active;
                        move |_| {
                            resize_active.set(None);
                        }
                    },
                }
            }

            // Quick filter bar (above the table)
            if quick_filter {
                {render_quick_filter(quick_filter_text, current_page)}
            }

            // Grid container (relative for loading overlay positioning)
            div {
                class: if drop_hover() { format!("relative {} {}", s::GRID_CONTAINER, s::DROP_TARGET_ACTIVE) } else { format!("relative {}", s::GRID_CONTAINER) },
                onmounted: move |evt: MountedEvent| {
                    container_element.set(Some(evt));
                },
                // Drag-and-drop: start drag on selected rows.
                ondragstart: {
                    let drag_id = drag_id;
                    let selected_rows = selected_rows;
                    let drag_ctx = drag_ctx;
                    move |_| {
                        if let (Some(id), Some(mut ctx)) = (drag_id, drag_ctx) {
                            let indices: Vec<usize> = {
                                let mut v: Vec<usize> = selected_rows.read().iter().copied().collect();
                                v.sort();
                                v
                            };
                            if !indices.is_empty() {
                                ctx.set(Some(GridDragPayload { source_id: id, indices }));
                            }
                        }
                    }
                },
                ondragover: move |evt: Event<DragData>| {
                    if drop_target && drag_ctx.is_some() {
                        evt.prevent_default();
                        drop_hover.set(true);
                    }
                },
                ondragleave: move |_| {
                    drop_hover.set(false);
                },
                ondrop: {
                    let on_drop_receive = on_drop_receive.clone();
                    let drag_ctx = drag_ctx;
                    move |evt: Event<DragData>| {
                        evt.prevent_default();
                        drop_hover.set(false);
                        if let Some(mut ctx) = drag_ctx {
                            if let Some(payload) = ctx().take() {
                                if let Some(ref handler) = on_drop_receive {
                                    handler.call(payload);
                                }
                            }
                            ctx.set(None);
                        }
                    }
                },

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
                } else if virtualize {
                    // Virtualized layout — fixed header table above a
                    // scrollable viewport that contains only the tbody.
                    // A shared colgroup keeps column widths in sync.
                    {
                        let has_cb = row_selection == RowSelection::Multi;
                        rsx! {
                            table { class: s::TABLE,
                                {render_colgroup(&columns, column_widths, has_cb)}
                                {render_header(&columns, sort_state, current_page, column_filters, density_cls, row_selection, selected_rows, &visible_indices, &on_selection_change, column_widths, resize_active)}
                            }
                            div {
                                class: s::VIRTUAL_VIEWPORT,
                                style: "height: {viewport_height:.0}px; overflow-y: auto;",
                                onmounted: move |evt: MountedEvent| {
                                    viewport_element.set(Some(evt));
                                },
                                onscroll: move |_| {
                                    if let Some(el) = viewport_element() {
                                        spawn(async move {
                                            if let Ok(offset) = el.get_scroll_offset().await {
                                                scroll_top.set(offset.y);
                                            }
                                        });
                                    }
                                },
                                table { class: s::TABLE,
                                    {render_colgroup(&columns, column_widths, has_cb)}
                                    {
                                        render_body(
                                            &data,
                                            &columns,
                                            &windowed_indices,
                                            density_cls,
                                            striped,
                                            column_borders,
                                            row_selection,
                                            selected_row,
                                            selected_rows,
                                            &on_row_click,
                                            &on_selection_change,
                                            column_widths,
                                            top_spacer,
                                            bottom_spacer,
                                            row_height,
                                            Some(measured_row_height),
                                            drag_id.is_some(),
                                        )
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // Standard non-virtualized table.
                    table { class: s::TABLE,
                        {render_header(&columns, sort_state, current_page, column_filters, density_cls, row_selection, selected_rows, &visible_indices, &on_selection_change, column_widths, resize_active)}
                        {
                            render_body(
                                &data,
                                &columns,
                                &windowed_indices,
                                density_cls,
                                striped,
                                column_borders,
                                row_selection,
                                selected_row,
                                selected_rows,
                                &on_row_click,
                                &on_selection_change,
                                column_widths,
                                0.0,
                                0.0,
                                0.0,
                                None,
                                drag_id.is_some(),
                            )
                        }
                    }
                }
            }

            // Bulk action bar (visible when Multi selection has rows selected)
            if row_selection == RowSelection::Multi && selected_rows.read().len() > 0 {
                {
                    let count = selected_rows.read().len();
                    render_bulk_actions(
                        &columns,
                        &data,
                        selected_rows,
                        count,
                        &on_delete,
                        export,
                        &on_export,
                        &on_clipboard,
                        status_column,
                        &status_options,
                        &on_status_change,
                        &aggregation_columns,
                        &bulk_actions,
                    )
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

            // Virtualization info bar — visible range and total entries
            if virtualize && total_rows > 0 {
                div { class: s::VIRTUAL_INFO_BAR,
                    span { "Showing {virt_first}\u{2013}{virt_last} of {total_rows} entries" }
                    span { "{windowed_indices.len()} rows rendered" }
                }
            }
        }
    }
}
