//! Column definition struct and builder methods for EqGrid.

use super::types::ColumnAlign;
use dioxus::prelude::*;
use std::cmp::Ordering;

/// Defines a single column in the grid.
///
/// Generic over the row data type `T`. Use the builder methods
/// to configure optional properties after construction.
pub struct EqColumnDef<T: Clone + PartialEq + 'static> {
    /// Unique column identifier.
    pub id: &'static str,
    /// Display text in the column header.
    pub header: &'static str,
    /// Extract a string value from the row for display, sorting, and filtering.
    pub value_getter: fn(&T) -> String,
    /// Optional display formatter — transforms the value for rendering.
    pub value_formatter: Option<fn(&T) -> String>,
    /// Optional custom cell renderer — returns an Element for full control.
    pub cell_renderer: Option<fn(&T) -> Element>,
    /// Enable sorting on this column.
    pub sortable: bool,
    /// Enable per-column text filtering in the header.
    pub filterable: bool,
    /// Initial width in pixels (None = flex).
    pub width: Option<u32>,
    /// Minimum width in pixels.
    pub min_width: u32,
    /// Text alignment within cells.
    pub align: ColumnAlign,
    /// Custom sort comparator — overrides default string comparison.
    pub comparator: Option<fn(&T, &T) -> Ordering>,
    /// Custom class applied to all cells in this column.
    pub cell_class: &'static str,
    /// Custom class applied to the header cell.
    pub header_class: &'static str,
}

impl<T: Clone + PartialEq + 'static> Clone for EqColumnDef<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            header: self.header,
            value_getter: self.value_getter,
            value_formatter: self.value_formatter,
            cell_renderer: self.cell_renderer,
            sortable: self.sortable,
            filterable: self.filterable,
            width: self.width,
            min_width: self.min_width,
            align: self.align,
            comparator: self.comparator,
            cell_class: self.cell_class,
            header_class: self.header_class,
        }
    }
}

impl<T: Clone + PartialEq + 'static> PartialEq for EqColumnDef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// ── Builder ─────────────────────────────────────────────────────────

impl<T: Clone + PartialEq + 'static> EqColumnDef<T> {
    /// Create a column with required fields. All optional fields use defaults.
    pub fn new(
        id: &'static str,
        header: &'static str,
        value_getter: fn(&T) -> String,
    ) -> Self {
        Self {
            id,
            header,
            value_getter,
            value_formatter: None,
            cell_renderer: None,
            sortable: true,
            filterable: false,
            width: None,
            min_width: 50,
            align: ColumnAlign::Left,
            comparator: None,
            cell_class: "",
            header_class: "",
        }
    }

    pub fn with_formatter(mut self, f: fn(&T) -> String) -> Self {
        self.value_formatter = Some(f);
        self
    }

    pub fn with_renderer(mut self, f: fn(&T) -> Element) -> Self {
        self.cell_renderer = Some(f);
        self
    }

    pub fn sortable(mut self, v: bool) -> Self {
        self.sortable = v;
        self
    }

    pub fn filterable(mut self, v: bool) -> Self {
        self.filterable = v;
        self
    }

    pub fn width(mut self, px: u32) -> Self {
        self.width = Some(px);
        self
    }

    pub fn min_width(mut self, px: u32) -> Self {
        self.min_width = px;
        self
    }

    pub fn align(mut self, a: ColumnAlign) -> Self {
        self.align = a;
        self
    }

    pub fn cell_class(mut self, c: &'static str) -> Self {
        self.cell_class = c;
        self
    }

    pub fn header_class(mut self, c: &'static str) -> Self {
        self.header_class = c;
        self
    }

    pub fn comparator(mut self, f: fn(&T, &T) -> Ordering) -> Self {
        self.comparator = Some(f);
        self
    }
}
