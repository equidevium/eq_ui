//! EqGrid - feature-rich, type-safe data grid organism.
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

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{CodeBlock, DemoSection, PropSelect, PropToggle, StyleInfo, format_catalog};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant, EqTab, TabItem, TabVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

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
    /// Receives `(Vec<usize>, String)` - selected indices and the new status.
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
    // ── Reorder props ────────────────────────────────────────────
    /// Enable row reordering via drag handles. A grip column appears
    /// as the leftmost column with drag-to-reorder support.
    #[props(default = false)]
    reorderable: bool,
    /// Fired when a row is reordered. Provides `(from_index, to_index)`
    /// as indices into the original data vec. The consumer is responsible
    /// for reordering the data.
    #[props(default)]
    on_reorder: Option<EventHandler<(usize, usize)>>,
    /// Optional class override.
    #[props(into, default)]
    class: String,
) -> Element {
    // ── Internal state ──────────────────────────────────────────

    let sort_state = use_signal(|| Vec::<SortState>::new());
    let mut current_page = use_signal(|| 0usize);
    let mut selected_row = use_signal(|| Option::<usize>::None);
    let mut selected_rows = use_signal(|| HashSet::<usize>::new());
    let quick_filter_text = use_signal(|| String::new());
    let column_filters = use_signal(|| HashMap::<&'static str, String>::new());
    let column_widths = use_signal(|| HashMap::<&'static str, f64>::new());
    let resize_active = use_signal(|| Option::<ResizeState>::None);
    let mut container_element: Signal<Option<MountedEvent>> = use_signal(|| None);
    let mut container_width = use_signal(|| 0.0_f64);
    let mut scroll_top = use_signal(|| 0.0_f64);
    let mut viewport_element: Signal<Option<MountedEvent>> = use_signal(|| None);
    // Actual row height - seeded with the density constant, then
    // refined after mount by measuring the first rendered data row.
    let measured_row_height = use_signal(|| density.row_height());
    // Drag-and-drop: shared context for passing payloads between grids.
    // The consumer must wrap both grids in a context provider:
    //   use_context_provider(|| Signal::new(Option::<GridDragPayload>::None));
    let drag_ctx: Option<Signal<Option<GridDragPayload>>> = try_consume_context();
    let mut drop_hover = use_signal(|| false);
    // Reorder state - grip handles and drag logic.
    let reorder_from: Signal<Option<usize>> = use_signal(|| None);
    let reorder_over: Signal<Option<usize>> = use_signal(|| None);

    // Clear selection when data length changes (e.g. rows moved via
    // drag-and-drop). Stale indices would point at wrong rows.
    let data_len = data.len();
    let mut prev_data_len = use_signal(|| data_len);
    if data_len != prev_data_len() {
        prev_data_len.set(data_len);
        selected_rows.write().clear();
        selected_row.set(None);
    }

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

    // Step 2: Apply column filters (AND logic - row must match ALL active filters).
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

                // Quick filter: OR logic - any column value contains the text
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
        (visible_indices.to_vec(), 0.0, 0.0, 0.0, 0, 0)
    };

    // ── Colgroup - shared column widths for split-table virtualisation ──

    let render_colgroup = |cols: &[EqColumnDef<T>],
                           widths: Signal<HashMap<&'static str, f64>>,
                           has_grip: bool,
                           has_checkbox: bool|
     -> Element {
        let w = widths.read();
        rsx! {
            colgroup {
                if has_grip {
                    col { style: "width: 32px;" }
                }
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
                    // Virtualized layout - fixed header table above a
                    // scrollable viewport that contains only the tbody.
                    // A shared colgroup keeps column widths in sync.
                    {
                        let has_cb = row_selection == RowSelection::Multi;
                        rsx! {
                            table { class: s::TABLE,
                                {render_colgroup(&columns, column_widths, reorderable, has_cb)}
                                {render_header(&columns, sort_state, current_page, column_filters, density_cls, row_selection, selected_rows, &visible_indices, &on_selection_change, column_widths, resize_active, reorderable)}
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
                                    {render_colgroup(&columns, column_widths, reorderable, has_cb)}
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
                                            reorderable,
                                            reorder_from,
                                            reorder_over,
                                            &on_reorder,
                                        )
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // Standard non-virtualized table.
                    table { class: s::TABLE,
                        {render_header(&columns, sort_state, current_page, column_filters, density_cls, row_selection, selected_rows, &visible_indices, &on_selection_change, column_widths, resize_active, reorderable)}
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
                                reorderable,
                                reorder_from,
                                reorder_over,
                                &on_reorder,
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

            // Virtualization info bar - visible range and total entries
            if virtualize && total_rows > 0 {
                div { class: s::VIRTUAL_INFO_BAR,
                    span { "Showing {virt_first}\u{2013}{virt_last} of {total_rows} entries" }
                    span { "{windowed_indices.len()} rows rendered" }
                }
            }
        }
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-grid",
        name: "EqGrid",
        category: ComponentCategory::Organism,
        description: "Feature-rich, type-safe data grid with sorting, filtering, pagination, row selection, and export.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Basic",
                code: "#[derive(Clone, PartialEq)]\nstruct Employee {\n    name: String,\n    role: String,\n    salary: f64,\n}\n\nlet columns = vec![\n    EqColumnDef::new(\"name\", \"Name\", |e| e.name.clone())\n        .filterable(true),\n];\n\nEqGrid {\n    data: employees,\n    columns: columns,\n}".into(),
            },
            UsageExample {
                label: "With pagination",
                code: "EqGrid {\n    data: employees,\n    columns: columns,\n    navigation: GridNavigation::Paginate,\n    page_size: 10,\n    row_selection: RowSelection::Single,\n    striped: true,\n}".into(),
            },
        ],
        render_demo: || rsx! { DemoEqGrid {} },
        render_gallery: || rsx! { GalleryEqGrid {} },
    }
}

// ── Playground demo data (feature-gated) ────────────────────────────

#[cfg(feature = "playground")]
#[derive(Clone, PartialEq)]
struct DemoEmployee {
    index: usize,
    name: String,
    role: String,
    department: String,
    salary: f64,
    status: String,
}

#[cfg(feature = "playground")]
fn demo_employees() -> Vec<DemoEmployee> {
    let base = vec![
        ("Ada Lovelace", "Engineer", "R&D", 95000.0, "Active"),
        ("Grace Hopper", "Architect", "R&D", 120000.0, "Active"),
        ("Alan Turing", "Researcher", "Science", 105000.0, "Inactive"),
        ("Linus Torvalds", "Lead", "Engineering", 150000.0, "Active"),
        ("Margaret Hamilton", "Director", "Engineering", 140000.0, "Active"),
        ("Dennis Ritchie", "Engineer", "Systems", 98000.0, "Inactive"),
        ("Barbara Liskov", "Professor", "Science", 130000.0, "Active"),
        ("Ken Thompson", "Engineer", "Systems", 102000.0, "Active"),
        ("Bjarne Stroustrup", "Architect", "Languages", 115000.0, "Active"),
        ("Guido van Rossum", "Lead", "Languages", 125000.0, "Inactive"),
        ("Hedy Lamarr", "Inventor", "R&D", 88000.0, "Active"),
        ("Tim Berners-Lee", "Architect", "Web", 135000.0, "Active"),
        ("John McCarthy", "Researcher", "AI", 110000.0, "Inactive"),
        ("Frances Allen", "Engineer", "Compilers", 99000.0, "Active"),
        ("Donald Knuth", "Professor", "Algorithms", 142000.0, "Active"),
    ];
    base.into_iter()
        .enumerate()
        .map(|(i, (n, r, d, s, st))| DemoEmployee {
            index: i + 1, name: n.into(), role: r.into(), department: d.into(),
            salary: s, status: st.into(),
        })
        .collect()
}

#[cfg(feature = "playground")]
fn demo_employees_large(count: usize) -> Vec<DemoEmployee> {
    let base = demo_employees();
    (0..count)
        .map(|i| {
            let src = &base[i % base.len()];
            DemoEmployee {
                index: i + 1,
                name: src.name.clone(),
                role: src.role.clone(),
                department: src.department.clone(),
                salary: src.salary + (i as f64 * 100.0),
                status: src.status.clone(),
            }
        })
        .collect()
}

#[cfg(feature = "playground")]
fn demo_columns() -> Vec<EqColumnDef<DemoEmployee>> {
    vec![
        EqColumnDef::new("idx", "#", |e: &DemoEmployee| e.index.to_string())
            .sortable(false)
            .resizable(false)
            .align(super::types::ColumnAlign::Right)
            .width(50)
            .min_width(40),
        EqColumnDef::new("name", "Name", |e: &DemoEmployee| e.name.clone())
            .filterable(true)
            .min_width(140),
        EqColumnDef::new("role", "Role", |e: &DemoEmployee| e.role.clone())
            .filterable(true)
            .min_width(100),
        EqColumnDef::new("dept", "Department", |e: &DemoEmployee| e.department.clone())
            .filterable(true)
            .min_width(100),
        EqColumnDef::new("salary", "Salary", |e: &DemoEmployee| e.salary.to_string())
            .with_formatter(|e: &DemoEmployee| format!("${:.0}", e.salary))
            .align(super::types::ColumnAlign::Right)
            .comparator(|a: &DemoEmployee, b: &DemoEmployee| a.salary.partial_cmp(&b.salary).unwrap_or(std::cmp::Ordering::Equal))
            .width(120)
            .min_width(80),
        EqColumnDef::new("status", "Status", |e: &DemoEmployee| e.status.clone())
            .with_renderer(|e: &DemoEmployee| {
                let (label, color) = match e.status.as_str() {
                    "Active" => ("Active", "text-[var(--color-success)]"),
                    "On Leave" => ("On Leave", "text-amber-400"),
                    _ => ("Inactive", "text-[var(--color-error)]"),
                };
                rsx! { span { class: "{color} font-medium text-xs", "{label}" } }
            })
            .sortable(false)
            .resizable(false)
            .align(super::types::ColumnAlign::Center)
            .min_width(80),
    ]
}

// ── Drag & Drop demo data ─────────────────────────────────────────

#[cfg(feature = "playground")]
#[derive(Clone, PartialEq)]
struct DndPerson {
    index: usize,
    name: String,
    role: String,
}

#[cfg(feature = "playground")]
fn team_a_data() -> Vec<DndPerson> {
    vec![
        DndPerson { index: 1, name: "Ada Lovelace".into(), role: "Engineer".into() },
        DndPerson { index: 2, name: "Grace Hopper".into(), role: "Architect".into() },
        DndPerson { index: 3, name: "Alan Turing".into(), role: "Researcher".into() },
        DndPerson { index: 4, name: "Linus Torvalds".into(), role: "Lead".into() },
        DndPerson { index: 5, name: "Margaret Hamilton".into(), role: "Director".into() },
    ]
}

#[cfg(feature = "playground")]
fn team_b_data() -> Vec<DndPerson> {
    vec![
        DndPerson { index: 1, name: "Dennis Ritchie".into(), role: "Engineer".into() },
        DndPerson { index: 2, name: "Barbara Liskov".into(), role: "Professor".into() },
        DndPerson { index: 3, name: "Ken Thompson".into(), role: "Engineer".into() },
    ]
}

#[cfg(feature = "playground")]
fn dnd_columns() -> Vec<EqColumnDef<DndPerson>> {
    vec![
        EqColumnDef::new("idx", "#", |e: &DndPerson| e.index.to_string())
            .sortable(false)
            .resizable(false)
            .align(super::types::ColumnAlign::Right)
            .width(40)
            .min_width(40),
        EqColumnDef::new("name", "Name", |e: &DndPerson| e.name.clone())
            .min_width(120),
        EqColumnDef::new("role", "Role", |e: &DndPerson| e.role.clone())
            .min_width(80),
    ]
}

// ── Drag & Drop sub-demo ──────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqGridDragDrop() -> Element {
    // Shared drag context - both grids read/write through this signal.
    let _drag_ctx: Signal<Option<GridDragPayload>> =
        use_context_provider(|| Signal::new(Option::<GridDragPayload>::None));

    let mut team_a = use_signal(|| team_a_data());
    let mut team_b = use_signal(|| team_b_data());
    let mut status = use_signal(|| String::new());

    // Re-index helper: updates the `index` field to match position.
    let reindex = |data: &mut Vec<DndPerson>| {
        for (i, person) in data.iter_mut().enumerate() {
            person.index = i + 1;
        }
    };

    let code = "// Wrap both grids in a shared drag context provider:\n\
        use_context_provider(|| Signal::new(Option::<GridDragPayload>::None));\n\
        \n\
        // Source grid - drag_id enables dragging selected rows\n\
        EqGrid {\n\
        \x20   data: team_a(),\n\
        \x20   columns: columns(),\n\
        \x20   row_selection: RowSelection::Multi,\n\
        \x20   drag_id: \"team-a\",\n\
        }\n\
        \n\
        // Target grid - drop_target + on_drop_receive\n\
        EqGrid {\n\
        \x20   data: team_b(),\n\
        \x20   columns: columns(),\n\
        \x20   row_selection: RowSelection::Multi,\n\
        \x20   drop_target: true,\n\
        \x20   on_drop_receive: move |payload: GridDragPayload| {\n\
        \x20       // Move rows from source to target\n\
        \x20   },\n\
        }".to_string();

    rsx! {
        div { class: "space-y-4",
            EqText { variant: TextVariant::Muted,
                "Select rows in Team A using the checkboxes, then drag any selected row to Team B. \
                 The rows move between the two grids."
            }

            if !status.read().is_empty() {
                div { class: "text-xs text-[var(--color-accent-primary)] bg-[var(--color-card)]/20 rounded px-3 py-1.5",
                    "{status}"
                }
            }

            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                // Team A - drag source
                div { class: "space-y-2",
                    EqText { variant: TextVariant::Emphasis, "Team A (drag source)" }
                    EqGrid {
                        data: team_a(),
                        columns: dnd_columns(),
                        row_selection: RowSelection::Multi,
                        density: GridDensity::Compact,
                        striped: true,
                        drag_id: "team-a",
                        drop_target: true,
                        on_drop_receive: {
                            let mut team_a = team_a;
                            let mut team_b = team_b;
                            let mut status = status;
                            move |payload: GridDragPayload| {
                                if payload.source_id == "team-b" {
                                    let mut b = team_b.write();
                                    let mut a = team_a.write();
                                    let mut moved = Vec::new();
                                    for &idx in payload.indices.iter().rev() {
                                        if idx < b.len() {
                                            moved.push(b.remove(idx));
                                        }
                                    }
                                    moved.reverse();
                                    let count = moved.len();
                                    a.extend(moved);
                                    reindex(&mut a);
                                    reindex(&mut b);
                                    drop(a);
                                    drop(b);
                                    status.set(format!("Moved {} row(s) from Team B to Team A", count));
                                }
                            }
                        },
                    }
                }

                // Team B - drop target
                div { class: "space-y-2",
                    EqText { variant: TextVariant::Emphasis, "Team B (drop target)" }
                    EqGrid {
                        data: team_b(),
                        columns: dnd_columns(),
                        row_selection: RowSelection::Multi,
                        density: GridDensity::Compact,
                        striped: true,
                        drag_id: "team-b",
                        drop_target: true,
                        on_drop_receive: {
                            let mut team_a = team_a;
                            let mut team_b = team_b;
                            let mut status = status;
                            move |payload: GridDragPayload| {
                                if payload.source_id == "team-a" {
                                    let mut a = team_a.write();
                                    let mut b = team_b.write();
                                    let mut moved = Vec::new();
                                    for &idx in payload.indices.iter().rev() {
                                        if idx < a.len() {
                                            moved.push(a.remove(idx));
                                        }
                                    }
                                    moved.reverse();
                                    let count = moved.len();
                                    b.extend(moved);
                                    reindex(&mut a);
                                    reindex(&mut b);
                                    drop(a);
                                    drop(b);
                                    status.set(format!("Moved {} row(s) from Team A to Team B", count));
                                }
                            }
                        },
                    }
                }
            }

            CodeBlock { code }
        }
    }
}

// ── Reorder sub-demo ──────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqGridReorder() -> Element {
    let mut data = use_signal(|| {
        vec![
            DemoEmployee { index: 1, name: "Ada Lovelace".into(), role: "Engineer".into(), department: "R&D".into(), salary: 95000.0, status: "Active".into() },
            DemoEmployee { index: 2, name: "Grace Hopper".into(), role: "Architect".into(), department: "R&D".into(), salary: 120000.0, status: "Active".into() },
            DemoEmployee { index: 3, name: "Alan Turing".into(), role: "Researcher".into(), department: "Science".into(), salary: 105000.0, status: "Inactive".into() },
            DemoEmployee { index: 4, name: "Linus Torvalds".into(), role: "Lead".into(), department: "Engineering".into(), salary: 150000.0, status: "Active".into() },
            DemoEmployee { index: 5, name: "Margaret Hamilton".into(), role: "Director".into(), department: "Engineering".into(), salary: 140000.0, status: "Active".into() },
            DemoEmployee { index: 6, name: "Dennis Ritchie".into(), role: "Engineer".into(), department: "Systems".into(), salary: 98000.0, status: "Inactive".into() },
            DemoEmployee { index: 7, name: "Barbara Liskov".into(), role: "Professor".into(), department: "Science".into(), salary: 130000.0, status: "Active".into() },
            DemoEmployee { index: 8, name: "Ken Thompson".into(), role: "Engineer".into(), department: "Systems".into(), salary: 102000.0, status: "Active".into() },
        ]
    });

    let mut last_move = use_signal(|| String::new());

    let columns = vec![
        EqColumnDef::new("index", "#", |e: &DemoEmployee| e.index.to_string())
            .width(60),
        EqColumnDef::new("name", "Name", |e: &DemoEmployee| e.name.clone())
            .sortable(true),
        EqColumnDef::new("role", "Role", |e: &DemoEmployee| e.role.clone()),
        EqColumnDef::new("dept", "Department", |e: &DemoEmployee| e.department.clone()),
        EqColumnDef::new("salary", "Salary", |e: &DemoEmployee| format!("${}", e.salary as u64)),
    ];

    let code = r#"EqGrid {
    data: items(),
    columns: columns,
    reorderable: true,
    on_reorder: move |(from, to): (usize, usize)| {
        let mut vec = items.write();
        let row = vec.remove(from);
        vec.insert(to, row);
        for (i, e) in vec.iter_mut().enumerate() {
            e.index = i + 1;
        }
    },
}"#;

    rsx! {
        div { class: "space-y-4",
            EqText { variant: TextVariant::Body,
                "Drag the grip handle (\u{2807}) on the left edge of any row to reorder. \
                 Works with all navigation modes."
            }

            if !last_move.read().is_empty() {
                div {
                    class: "px-3 py-2 rounded-lg bg-[var(--color-card)] border border-[var(--color-card-border)] text-sm",
                    EqText { variant: TextVariant::Caption, "{last_move}" }
                }
            }

            EqGrid {
                data: data(),
                columns: columns,
                reorderable: true,
                striped: true,
                density: GridDensity::Normal,
                on_reorder: move |(from, to): (usize, usize)| {
                    let mut vec = data.write();
                    let row = vec.remove(from);
                    vec.insert(to, row);
                    for (i, e) in vec.iter_mut().enumerate() {
                        e.index = i + 1;
                    }
                    drop(vec);
                    let d = data.read();
                    let name = &d[to].name;
                    last_move.set(format!("Moved \"{}\" from position {} to {}", name, from + 1, to + 1));
                },
            }

            CodeBlock { code: code.to_string() }
        }
    }
}

// ── Interactive demo ───────────────────────────────────────────────


#[cfg(feature = "playground")]
#[component]
fn DemoEqGrid() -> Element {
    let mut tab_idx = use_signal(|| 0usize); // 0=Data Grid, 1=Drag & Drop, 2=Reorder
    let mut nav_idx = use_signal(|| 1usize); // 0=Standard, 1=Paginate, 2=Virtualize
    let mut striped = use_signal(|| true);
    let mut col_borders = use_signal(|| false);
    let mut quick_filter = use_signal(|| true);
    let mut density_idx = use_signal(|| 1usize); // 0=Compact, 1=Normal, 2=Comfortable
    let mut selection_idx = use_signal(|| 1usize); // 0=None, 1=Single
    let mut page_size_idx = use_signal(|| 0usize); // 0=5, 1=10, 2=25
    let mut reorderable = use_signal(|| false);

    let navigation = match nav_idx() {
        0 => GridNavigation::Standard,
        2 => GridNavigation::Virtualize,
        _ => GridNavigation::Paginate,
    };

    let density = match density_idx() {
        0 => GridDensity::Compact,
        2 => GridDensity::Comfortable,
        _ => GridDensity::Normal,
    };

    let selection = match selection_idx() {
        1 => RowSelection::Single,
        2 => RowSelection::Multi,
        _ => RowSelection::None,
    };

    let mut employees = use_signal(|| demo_employees());
    // When virtualization is active, switch to a large dataset.
    use_effect(move || {
        let virt = nav_idx() == 2;
        if virt && employees.read().len() < 100 {
            employees.set(demo_employees_large(500));
        } else if !virt && employees.read().len() > 100 {
            employees.set(demo_employees());
        }
    });
    let mut selection_count = use_signal(|| 0usize);
    let mut bulk_status = use_signal(|| String::new());
    let mut export_preview = use_signal(|| String::new());
    let mut clipboard_preview = use_signal(|| String::new());

    let page_size = match page_size_idx() {
        1 => 10,
        2 => 25,
        _ => 5,
    };

    let code = "#[derive(Clone, PartialEq)]\n\
        struct Employee {\n\
        \x20   name: String,\n\
        \x20   role: String,\n\
        \x20   salary: f64,\n\
        }\n\
        \n\
        let columns = vec![\n\
        \x20   EqColumnDef::new(\"name\", \"Name\", |e| e.name.clone())\n\
        \x20       .filterable(true),\n\
        \x20   EqColumnDef::new(\"role\", \"Role\", |e| e.role.clone())\n\
        \x20       .filterable(true),\n\
        \x20   EqColumnDef::new(\"salary\", \"Salary\", |e| e.salary.to_string())\n\
        \x20       .with_formatter(|e| format!(\"${:.0}\", e.salary))\n\
        \x20       .align(ColumnAlign::Right),\n\
        ];\n\
        \n\
        EqGrid {\n\
        \x20   data: employees,\n\
        \x20   columns: columns,\n\
        \x20   navigation: GridNavigation::Paginate,\n\
        \x20   page_size: 10,\n\
        \x20   row_selection: RowSelection::Single,\n\
        \x20   quick_filter: true,\n\
        \x20   striped: true,\n\
        }".to_string();

    rsx! {
        DemoSection { title: "EqGrid",
            // ── Tab bar ──
            EqTab {
                tabs: vec![
                    TabItem::new("Data Grid"),
                    TabItem::new("Drag & Drop"),
                    TabItem::new("Reorder"),
                ],
                variant: TabVariant::Card,
                active: tab_idx(),
                on_change: move |idx: usize| tab_idx.set(idx),
            }

            // ── Tab content ──
            match tab_idx() {
                1 => rsx! { DemoEqGridDragDrop {} },
                2 => rsx! { DemoEqGridReorder {} },
                _ => rsx! {
            // Prop controls
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                div { class: "grid grid-cols-2 md:grid-cols-3 gap-3",
                    PropSelect {
                        label: "navigation",
                        value: match nav_idx() { 0 => "Standard", 2 => "Virtualize", _ => "Paginate" }.to_string(),
                        options: vec!["Standard", "Paginate", "Virtualize"],
                        onchange: move |v: String| nav_idx.set(match v.as_str() { "Standard" => 0, "Virtualize" => 2, _ => 1 }),
                    }
                    PropToggle {
                        label: "striped",
                        value: striped(),
                        onchange: move |v: bool| striped.set(v),
                    }
                    PropToggle {
                        label: "column_borders",
                        value: col_borders(),
                        onchange: move |v: bool| col_borders.set(v),
                    }
                    PropToggle {
                        label: "quick_filter",
                        value: quick_filter(),
                        onchange: move |v: bool| quick_filter.set(v),
                    }
                    PropSelect {
                        label: "density",
                        value: match density_idx() { 0 => "Compact", 2 => "Comfortable", _ => "Normal" }.to_string(),
                        options: vec!["Compact", "Normal", "Comfortable"],
                        onchange: move |v: String| density_idx.set(match v.as_str() { "Compact" => 0, "Comfortable" => 2, _ => 1 }),
                    }
                    PropSelect {
                        label: "row_selection",
                        value: match selection_idx() { 1 => "Single", 2 => "Multi", _ => "None" }.to_string(),
                        options: vec!["None", "Single", "Multi"],
                        onchange: move |v: String| selection_idx.set(match v.as_str() { "Single" => 1, "Multi" => 2, _ => 0 }),
                    }
                    PropSelect {
                        label: "page_size",
                        value: match page_size_idx() { 1 => "10", 2 => "25", _ => "5" }.to_string(),
                        options: vec!["5", "10", "25"],
                        onchange: move |v: String| page_size_idx.set(match v.as_str() { "10" => 1, "25" => 2, _ => 0 }),
                    }
                    PropToggle {
                        label: "reorderable",
                        value: reorderable(),
                        onchange: move |v: bool| reorderable.set(v),
                    }
                }
            }

            // Live preview
            // Selection feedback + bulk action status
            if selection == RowSelection::Multi {
                div { class: "text-sm text-[var(--color-label-secondary)] py-1",
                    "{selection_count()} row(s) selected"
                }
                if !bulk_status.read().is_empty() {
                    div { class: "text-xs text-[var(--color-accent-primary)] bg-[var(--color-card)]/20 rounded px-3 py-1.5 mb-1",
                        "{bulk_status}"
                    }
                }
            }

            EqGrid {
                data: employees(),
                columns: demo_columns(),
                navigation: navigation,
                page_size: page_size,
                row_selection: selection,
                density: density,
                striped: striped(),
                column_borders: col_borders(),
                quick_filter: quick_filter(),
                reorderable: reorderable(),
                on_reorder: move |(from, to): (usize, usize)| {
                    let mut data = employees.write();
                    let row = data.remove(from);
                    data.insert(to, row);
                    for (i, e) in data.iter_mut().enumerate() {
                        e.index = i + 1;
                    }
                },
                on_selection_change: move |rows: Vec<usize>| {
                    selection_count.set(rows.len());
                },
                // Bulk actions - these actually mutate the data signal
                on_delete: move |rows: Vec<usize>| {
                    let count = rows.len();
                    let mut data = employees.write();
                    // Remove in reverse order so indices stay valid.
                    for &idx in rows.iter().rev() {
                        if idx < data.len() {
                            data.remove(idx);
                        }
                    }
                    drop(data);
                    selection_count.set(0);
                    bulk_status.set(format!("Deleted {} row(s)", count));
                },
                export: true,
                on_export: move |payload: (ExportFormat, Vec<u8>)| {
                    let (fmt, bytes) = payload;
                    let label = match fmt {
                        ExportFormat::Csv => "CSV",
                        ExportFormat::Json => "JSON",
                        ExportFormat::Txt => "TXT",
                        ExportFormat::Ods => "ODS",
                    };
                    // Show text content for text formats, byte count for binary.
                    let preview = match fmt {
                        ExportFormat::Ods => format!("[Binary ODS: {} bytes]", bytes.len()),
                        _ => String::from_utf8(bytes.clone()).unwrap_or_else(|_| format!("[{} bytes]", bytes.len())),
                    };
                    clipboard_preview.set(String::new());
                    export_preview.set(preview);
                    bulk_status.set(format!("Exported {} ({} bytes)", label, bytes.len()));
                },
                on_clipboard: move |content: String| {
                    let len = content.len();
                    export_preview.set(String::new());
                    clipboard_preview.set(content.clone());
                    // Write to the system clipboard via document::eval (cross-platform).
                    let escaped = content.replace('\\', "\\\\").replace('`', "\\`");
                    let js = format!("navigator.clipboard.writeText(`{}`)", escaped);
                    let _ = document::eval(&js);
                    bulk_status.set(format!("Copied to clipboard ({} chars)", len));
                },
                status_column: "status",
                status_options: vec!["Active".into(), "Inactive".into(), "On Leave".into()],
                on_status_change: move |payload: (Vec<usize>, String)| {
                    let (rows, new_status) = payload;
                    let count = rows.len();
                    let mut data = employees.write();
                    for &idx in &rows {
                        if idx < data.len() {
                            data[idx].status = new_status.clone();
                        }
                    }
                    drop(data);
                    bulk_status.set(format!("Changed {} row(s) to '{}'", count, new_status));
                },
                aggregation_columns: vec!["salary"],
            }

            // Export preview
            if !export_preview.read().is_empty() {
                div { class: "mt-3 rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                    div { class: "flex items-center justify-between px-3 py-1.5 bg-[var(--color-grid-header-bg)] border-b border-[var(--color-card-border)]",
                        span { class: "text-xs font-semibold text-[var(--color-label-primary)]", "Export Preview" }
                        button {
                            class: "text-xs text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] cursor-pointer",
                            onclick: move |_| export_preview.set(String::new()),
                            "Close"
                        }
                    }
                    pre { class: "px-3 py-2 text-xs text-[var(--color-label-primary)] bg-[var(--color-primary-dark)] overflow-x-auto max-h-64 overflow-y-auto whitespace-pre font-mono",
                        "{export_preview}"
                    }
                }
            }

            // Clipboard preview
            if !clipboard_preview.read().is_empty() {
                div { class: "mt-3 rounded-lg border border-[var(--color-card-border)] overflow-hidden",
                    div { class: "flex items-center justify-between px-3 py-1.5 bg-[var(--color-grid-header-bg)] border-b border-[var(--color-card-border)]",
                        span { class: "text-xs font-semibold text-[var(--color-label-primary)]", "Clipboard Preview" }
                        button {
                            class: "text-xs text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] cursor-pointer",
                            onclick: move |_| clipboard_preview.set(String::new()),
                            "Close"
                        }
                    }
                    pre { class: "px-3 py-2 text-xs text-[var(--color-label-primary)] bg-[var(--color-primary-dark)] overflow-x-auto max-h-64 overflow-y-auto whitespace-pre font-mono",
                        "{clipboard_preview}"
                    }
                }
            }

            StyleInfo { file: "eq_grid/styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
                } // end _ => rsx!
            } // end match
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqGrid() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Grid Gallery" }

                div { class: "space-y-3",
                    EqGrid {
                        data: demo_employees(),
                        columns: demo_columns(),
                        navigation: GridNavigation::Paginate,
                        page_size: 5,
                        row_selection: RowSelection::Single,
                        striped: true,
                    }
                }
            }
        }
    }
}
