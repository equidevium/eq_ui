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
}

/// Row height / cell padding preset.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum GridDensity {
    Compact,
    #[default]
    Normal,
    Comfortable,
}
