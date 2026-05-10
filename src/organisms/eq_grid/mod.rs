//! EqGrid organism - a feature-rich, type-safe data grid.
//!
//! Split into focused modules for maintainability:
//!
//! - **types** - shared enums and structs (ColumnAlign, SortDirection, etc.)
//! - **column_def** - `EqColumnDef<T>` struct and builder methods
//! - **styles** - co-located Tailwind class constants
//! - **header** - thead rendering with sort indicators
//! - **body** - tbody rendering with row selection and cell formatting
//! - **pagination** - page navigation bar
//! - **grid** - the `EqGrid` component that orchestrates everything

pub mod column_def;
pub mod styles;
pub mod types;

mod body;
mod bulk_actions;
mod export;
pub mod grid;
mod header;
mod pagination;
mod quick_filter;

pub use column_def::EqColumnDef;
pub use grid::EqGrid;
pub use types::{
    ColumnAlign, ExportFormat, GridDensity, GridDragPayload, GridNavigation, RowSelection,
    SortDirection, SortState,
};
