//! Shared types for the EqGrid organism.

/// Horizontal text alignment within a column's cells and header.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ColumnAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// Tri-state sort direction for a column.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum SortDirection {
    #[default]
    None,
    Asc,
    Desc,
}

/// Active sort state — which column and in which direction.
#[derive(Clone, Copy, PartialEq)]
pub struct SortState {
    pub column_id: &'static str,
    pub direction: SortDirection,
}

/// Controls how rows can be selected.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum RowSelection {
    #[default]
    None,
    Single,
    /// Checkbox column with Select All header. Exposes selection via callback.
    Multi,
}

/// Export format for bulk data export.
#[derive(Clone, Copy, PartialEq)]
pub enum ExportFormat {
    /// Comma-separated values.
    Csv,
    /// JSON array of objects.
    Json,
    /// Tab-separated plain text.
    Txt,
    /// OpenDocument Spreadsheet (LibreOffice-compatible).
    Ods,
}

/// Transient state while a column resize drag is in progress.
#[derive(Clone, PartialEq)]
pub struct ResizeState {
    /// Column being resized.
    pub column_id: &'static str,
    /// Mouse X position at drag start (pixels).
    pub start_x: f64,
    /// Column width at drag start (pixels).
    pub start_width: f64,
}

/// Controls how the grid navigates large datasets.
///
/// Only one mode is active at a time. `page_size` on the grid
/// determines how many rows are visible in both `Paginate` and
/// `Virtualize` modes.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum GridNavigation {
    /// All rows rendered. No pagination or scroll windowing.
    #[default]
    Standard,
    /// Traditional page-based navigation with prev/next controls.
    Paginate,
    /// Fixed-height viewport with virtual scrolling. Only the visible
    /// rows (plus a small buffer) are rendered in the DOM.
    Virtualize,
}

/// Payload passed through context when rows are dragged between grids.
#[derive(Clone, PartialEq, Default)]
pub struct GridDragPayload {
    /// Identifies the source grid (matches the `drag_id` prop).
    pub source_id: &'static str,
    /// Original row indices being dragged.
    pub indices: Vec<usize>,
}

/// Row height / cell padding preset.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum GridDensity {
    Compact,
    #[default]
    Normal,
    Comfortable,
}

impl GridDensity {
    /// Estimated row height in pixels for this density level.
    /// Used by the virtual scroll engine to compute viewport height
    /// and row positioning. These values must stay in sync with the
    /// padding defined in `styles.rs` density constants.
    pub const fn row_height(self) -> f64 {
        match self {
            Self::Compact => 32.0,
            Self::Normal => 44.0,
            Self::Comfortable => 56.0,
        }
    }
}
