//! EqGrid organism — a feature-rich, type-safe data grid.
//!
//! Split into focused modules for maintainability:
//!
//! - **types** — shared enums and structs (ColumnAlign, SortDirection, etc.)
//! - **column_def** — `EqColumnDef<T>` struct and builder methods
//! - **styles** — co-located Tailwind class constants
//! - **header** — thead rendering with sort indicators
//! - **body** — tbody rendering with row selection and cell formatting
//! - **pagination** — page navigation bar
//! - **grid** — the `EqGrid` component that orchestrates everything

pub mod types;
pub mod column_def;
pub mod styles;

mod header;
mod body;
mod pagination;
mod quick_filter;
mod export;
mod bulk_actions;
mod grid;

pub use types::{ColumnAlign, SortDirection, SortState, RowSelection, GridDensity, ExportFormat};
pub use column_def::EqColumnDef;
pub use grid::EqGrid;
