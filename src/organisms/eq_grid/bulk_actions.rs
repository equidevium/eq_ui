//! Bulk action bar rendered below the grid when rows are selected.
//!
//! Provides built-in actions (Delete, Export, Clipboard, Change Status)
//! plus an extensible slot for consumer-defined custom actions.

use super::column_def::EqColumnDef;
use super::export;
use super::styles as s;
use super::types::ExportFormat;
use dioxus::prelude::*;
use std::collections::HashSet;

/// Render the bulk action bar.
///
/// Only call this when `selected_count > 0`. The bar shows:
/// - Selection count label
/// - Delete button (if `on_delete` is provided)
/// - Export dropdown (if `export` is enabled)
/// - Clipboard button (if `on_clipboard` is provided)
/// - Change Status dropdown (if `status_column` + `on_status_change` are provided)
/// - Consumer-provided custom actions slot
/// - Aggregation panel (if `aggregation_columns` is non-empty)
#[allow(clippy::too_many_arguments)]
pub(super) fn render_bulk_actions<T: Clone + PartialEq + 'static>(
    columns: &[EqColumnDef<T>],
    data: &[T],
    selected_rows: Signal<HashSet<usize>>,
    selected_count: usize,
    // Action callbacks
    on_delete: &Option<EventHandler<Vec<usize>>>,
    export_enabled: bool,
    on_export: &Option<EventHandler<(ExportFormat, Vec<u8>)>>,
    on_clipboard: &Option<EventHandler<String>>,
    status_column: Option<&'static str>,
    status_options: &[String],
    on_status_change: &Option<EventHandler<(Vec<usize>, String)>>,
    aggregation_columns: &[&'static str],
    custom_actions: &Option<Element>,
) -> Element {
    let mut show_export_menu = use_signal(|| false);
    let mut show_status_menu = use_signal(|| false);
    let mut show_clipboard_menu = use_signal(|| false);

    // Pre-compute sorted indices for callbacks.
    let indices: Vec<usize> = {
        let set = selected_rows.read();
        let mut v: Vec<usize> = set.iter().copied().collect();
        v.sort();
        v
    };

    // Clone data needed by closures.
    let del_handler = on_delete.clone();
    let del_indices = indices.clone();

    let exp_handler = on_export.clone();
    let clip_handler = on_clipboard.clone();

    let status_handler = on_status_change.clone();

    // Build aggregation data if any aggregation columns are specified.
    let agg_data: Vec<(&'static str, String)> = if !aggregation_columns.is_empty() {
        aggregation_columns.iter().filter_map(|&agg_col_id| {
            let col = columns.iter().find(|c| c.id == agg_col_id)?;
            // Try to sum numeric values; fall back to count.
            let mut sum = 0.0f64;
            let mut all_numeric = true;
            for &idx in &indices {
                let val = (col.value_getter)(&data[idx]);
                if let Ok(n) = val.trim().parse::<f64>() {
                    sum += n;
                } else {
                    all_numeric = false;
                    break;
                }
            }
            let display = if all_numeric {
                format!("{:.2}", sum)
            } else {
                format!("{} values", indices.len())
            };
            Some((col.header, display))
        }).collect()
    } else {
        Vec::new()
    };

    rsx! {
        // ── Action bar ─────────────────────────────────────────
        div { class: s::BULK_BAR,
            // Selection count
            span { class: s::BULK_LABEL,
                "{selected_count} row(s) selected"
            }

            // Separator
            span { class: s::BULK_SEPARATOR }

            // Delete button
            if del_handler.is_some() {
                button {
                    class: s::BULK_BTN_DANGER,
                    onclick: move |_| {
                        if let Some(ref handler) = del_handler {
                            handler.call(del_indices.clone());
                        }
                    },
                    "Delete"
                }
            }

            // Export dropdown
            if export_enabled {
                div { class: "relative",
                    button {
                        class: s::BULK_BTN,
                        onclick: move |_| {
                            show_export_menu.set(!show_export_menu());
                            show_status_menu.set(false);
                            show_clipboard_menu.set(false);
                        },
                        "Export \u{25BE}"
                    }
                    if show_export_menu() {
                        {render_export_dropdown(
                            columns,
                            data,
                            &selected_rows,
                            &exp_handler,
                            show_export_menu,
                        )}
                    }
                }
            }

            // Clipboard dropdown
            if clip_handler.is_some() {
                div { class: "relative",
                    button {
                        class: s::BULK_BTN,
                        onclick: move |_| {
                            show_clipboard_menu.set(!show_clipboard_menu());
                            show_export_menu.set(false);
                            show_status_menu.set(false);
                        },
                        "Copy \u{25BE}"
                    }
                    if show_clipboard_menu() {
                        {render_clipboard_dropdown(
                            columns,
                            data,
                            &selected_rows,
                            &clip_handler,
                            show_clipboard_menu,
                        )}
                    }
                }
            }

            // Change Status dropdown
            if status_column.is_some() && status_handler.is_some() && !status_options.is_empty() {
                div { class: "relative",
                    button {
                        class: s::BULK_BTN,
                        onclick: move |_| {
                            show_status_menu.set(!show_status_menu());
                            show_export_menu.set(false);
                            show_clipboard_menu.set(false);
                        },
                        "Status \u{25BE}"
                    }
                    if show_status_menu() {
                        {render_status_dropdown(
                            &selected_rows,
                            status_options,
                            &status_handler,
                            show_status_menu,
                        )}
                    }
                }
            }

            // Consumer custom actions slot
            if let Some(custom) = custom_actions {
                span { class: s::BULK_SEPARATOR }
                {custom.clone()}
            }
        }

        // ── Aggregation panel ──────────────────────────────────
        if !agg_data.is_empty() {
            div { class: s::AGGREGATION_PANEL,
                for (label, value) in agg_data.iter() {
                    span { class: s::AGGREGATION_ITEM,
                        span { class: s::AGGREGATION_KEY, "{label}: " }
                        span { class: s::AGGREGATION_VALUE, "{value}" }
                    }
                }
            }
        }
    }
}

// ── Export dropdown ─────────────────────────────────────────────────

fn render_export_dropdown<T: Clone + PartialEq + 'static>(
    columns: &[EqColumnDef<T>],
    data: &[T],
    selected_rows: &Signal<HashSet<usize>>,
    on_export: &Option<EventHandler<(ExportFormat, Vec<u8>)>>,
    mut show_menu: Signal<bool>,
) -> Element {
    // Pre-generate content for each format.
    let indices: Vec<usize> = {
        let set = selected_rows.read();
        let mut v: Vec<usize> = set.iter().copied().collect();
        v.sort();
        v
    };

    let csv_content = export::export_csv(columns, data, &indices);
    let json_content = export::export_json(columns, data, &indices);
    let txt_content = export::export_txt(columns, data, &indices);
    let ods_content = export::export_ods(columns, data, &indices);

    let handler_csv = on_export.clone();
    let handler_json = on_export.clone();
    let handler_txt = on_export.clone();
    let handler_ods = on_export.clone();

    rsx! {
        div { class: s::BULK_DROPDOWN,
            button {
                class: s::BULK_DROPDOWN_ITEM,
                onclick: move |_| {
                    if let Some(ref h) = handler_csv {
                        h.call((ExportFormat::Csv, csv_content.clone().into_bytes()));
                    }
                    show_menu.set(false);
                },
                "CSV"
            }
            button {
                class: s::BULK_DROPDOWN_ITEM,
                onclick: move |_| {
                    if let Some(ref h) = handler_json {
                        h.call((ExportFormat::Json, json_content.clone().into_bytes()));
                    }
                    show_menu.set(false);
                },
                "JSON"
            }
            button {
                class: s::BULK_DROPDOWN_ITEM,
                onclick: move |_| {
                    if let Some(ref h) = handler_txt {
                        h.call((ExportFormat::Txt, txt_content.clone().into_bytes()));
                    }
                    show_menu.set(false);
                },
                "Text (TSV)"
            }
            button {
                class: s::BULK_DROPDOWN_ITEM,
                onclick: move |_| {
                    if let Some(ref h) = handler_ods {
                        h.call((ExportFormat::Ods, ods_content.clone()));
                    }
                    show_menu.set(false);
                },
                "ODS (LibreOffice)"
            }
        }
    }
}

// ── Clipboard dropdown ─────────────────────────────────────────────

fn render_clipboard_dropdown<T: Clone + PartialEq + 'static>(
    columns: &[EqColumnDef<T>],
    data: &[T],
    selected_rows: &Signal<HashSet<usize>>,
    on_clipboard: &Option<EventHandler<String>>,
    mut show_menu: Signal<bool>,
) -> Element {
    let indices: Vec<usize> = {
        let set = selected_rows.read();
        let mut v: Vec<usize> = set.iter().copied().collect();
        v.sort();
        v
    };

    let csv_content = export::export_csv(columns, data, &indices);
    let txt_content = export::export_txt(columns, data, &indices);

    let handler_csv = on_clipboard.clone();
    let handler_txt = on_clipboard.clone();

    rsx! {
        div { class: s::BULK_DROPDOWN,
            button {
                class: s::BULK_DROPDOWN_ITEM,
                onclick: move |_| {
                    if let Some(ref h) = handler_csv {
                        h.call(csv_content.clone());
                    }
                    show_menu.set(false);
                },
                "Copy as CSV"
            }
            button {
                class: s::BULK_DROPDOWN_ITEM,
                onclick: move |_| {
                    if let Some(ref h) = handler_txt {
                        h.call(txt_content.clone());
                    }
                    show_menu.set(false);
                },
                "Copy as Text"
            }
        }
    }
}

// ── Status dropdown ────────────────────────────────────────────────

fn render_status_dropdown(
    selected_rows: &Signal<HashSet<usize>>,
    status_options: &[String],
    on_status_change: &Option<EventHandler<(Vec<usize>, String)>>,
    mut show_menu: Signal<bool>,
) -> Element {
    let indices: Vec<usize> = {
        let set = selected_rows.read();
        let mut v: Vec<usize> = set.iter().copied().collect();
        v.sort();
        v
    };

    rsx! {
        div { class: s::BULK_DROPDOWN,
            for option in status_options.iter() {
                {
                    let opt = option.clone();
                    let handler = on_status_change.clone();
                    let idx = indices.clone();
                    rsx! {
                        button {
                            class: s::BULK_DROPDOWN_ITEM,
                            onclick: move |_| {
                                if let Some(ref h) = handler {
                                    h.call((idx.clone(), opt.clone()));
                                }
                                show_menu.set(false);
                            },
                            "{option}"
                        }
                    }
                }
            }
        }
    }
}
