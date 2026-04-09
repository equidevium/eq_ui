//! Style constants for EqGrid and its internal sub-components.

// ── Wrapper ─────────────────────────────────────────────────────────

/// Outermost grid container — border, rounding, overflow clip.
pub const GRID_WRAPPER: &str = "rounded-xl border border-[var(--color-grid-border)] overflow-hidden bg-[var(--color-primary-dark)]";

/// Scrollable area that holds the table. Allows horizontal scroll on mobile.
pub const GRID_CONTAINER: &str = "overflow-x-auto";

// ── Table ───────────────────────────────────────────────────────────

/// The HTML `<table>` element.
pub const TABLE: &str = "w-full border-collapse table-fixed text-sm";

// ── Header ──────────────────────────────────────────────────────────

/// `<thead>` — sticky at top during vertical scroll.
pub const THEAD: &str = "bg-[var(--color-grid-header-bg)] sticky top-0 z-10";

/// Default `<th>` cell.
pub const TH: &str = "px-3 py-2 md:px-4 md:py-3 text-left font-semibold \
     text-[var(--color-grid-header-text)] \
     border-b border-[var(--color-grid-border)] select-none whitespace-nowrap \
     overflow-hidden";

/// Additional class appended when the column is sortable.
pub const TH_SORTABLE: &str = "cursor-pointer hover:bg-[var(--color-card)]/20 active:bg-[var(--color-card)]/20 transition-colors";

/// Text alignment helpers applied to both `<th>` and `<td>`.
pub const ALIGN_LEFT: &str = "text-left";
pub const ALIGN_CENTER: &str = "text-center";
pub const ALIGN_RIGHT: &str = "text-right";

// ── Rows ────────────────────────────────────────────────────────────

/// Base `<tr>` styling.
pub const TR: &str = "border-b border-[var(--color-grid-border)] transition-colors";

/// Hover/active feedback on rows.
pub const TR_HOVER: &str = "hover:bg-[var(--color-card)]/20 active:bg-[var(--color-card)]/20";

/// Alternating row backgrounds when striped mode is on.
pub const TR_STRIPED: &str = "even:bg-[var(--color-card)]/5";

/// Selected row highlight.
pub const TR_SELECTED: &str = "bg-[var(--color-primary)]/15";

/// Selectable row cursor.
pub const TR_SELECTABLE: &str = "cursor-pointer";

// ── Cells ───────────────────────────────────────────────────────────

/// Default `<td>` cell.
pub const TD: &str =
    "px-3 py-2 md:px-4 md:py-3 text-[var(--color-label-primary)] overflow-hidden text-ellipsis";

// ── Density variants (applied to both th and td) ────────────────────

pub const DENSITY_COMPACT: &str = "px-2 py-1 md:px-3 md:py-1.5 text-xs";
pub const DENSITY_NORMAL: &str = "px-3 py-2 md:px-4 md:py-3 text-sm";
pub const DENSITY_COMFORTABLE: &str = "px-4 py-3 md:px-5 md:py-4 text-sm";

// ── Sort indicators ─────────────────────────────────────────────────

/// Sort icon wrapper — inline next to header text.
pub const SORT_ICON: &str = "ml-1 align-middle text-[var(--color-label-secondary)]";

/// Sort icon when actively sorting this column.
pub const SORT_ICON_ACTIVE: &str = "ml-1 align-middle text-[var(--color-accent-primary)]";

/// Sort feedback indicator — green up arrow for ascending.
pub const SORT_FEEDBACK_ASC: &str = "ml-0.5 align-middle text-green-500";

/// Sort feedback indicator — red down arrow for descending.
pub const SORT_FEEDBACK_DESC: &str = "ml-0.5 align-middle text-red-500";

/// Sort feedback indicator — blue dash for columns not participating in sort.
pub const SORT_FEEDBACK_NONE: &str = "ml-0.5 align-middle text-blue-400";

/// Sort priority badge — small number shown during multi-column sort.
pub const SORT_PRIORITY: &str =
    "text-[10px] leading-none font-semibold text-[var(--color-accent-primary)]";

// ── Pagination ──────────────────────────────────────────────────────

/// Pagination bar container — below the table.
pub const PAGINATION_BAR: &str = "flex flex-col gap-2 md:flex-row md:items-center md:justify-between \
     px-3 py-2 md:px-4 md:py-3 border-t border-[var(--color-grid-border)] \
     text-sm text-[var(--color-label-secondary)]";

/// Info text ("Showing 1-25 of 150").
pub const PAGINATION_INFO: &str = "";

/// Navigation button group.
pub const PAGINATION_NAV: &str = "flex items-center gap-1";

/// Individual page button (inactive).
pub const PAGE_BTN: &str = "size-8 flex items-center justify-center rounded \
     text-[var(--color-label-secondary)] \
     hover:bg-[var(--color-card)]/30 active:bg-[var(--color-card)]/30 \
     transition-colors cursor-pointer";

/// Active page button.
pub const PAGE_BTN_ACTIVE: &str = "size-8 flex items-center justify-center rounded \
     bg-[var(--color-primary)]/20 text-[var(--color-accent-primary)] \
     font-semibold cursor-default";

/// Disabled page button (prev/next at boundaries).
pub const PAGE_BTN_DISABLED: &str = "size-8 flex items-center justify-center rounded \
     text-[var(--color-label-secondary)]/30 cursor-not-allowed";

// ── Quick filter ────────────────────────────────────────────────────

/// Quick filter bar above the table.
pub const QUICK_FILTER: &str = "px-3 py-2 md:px-4 md:py-3 border-b border-[var(--color-grid-border)] \
     flex items-center gap-2";

/// Search input element.
pub const QUICK_FILTER_INPUT: &str = "flex-1 bg-transparent text-sm text-[var(--color-label-primary)] \
     placeholder-[var(--color-input-placeholder)] \
     focus:outline-none";

// ── Column filter ───────────────────────────────────────────────────

/// Small filter input rendered below the header text.
pub const COLUMN_FILTER_INPUT: &str = "w-full mt-1 px-2 py-1 text-xs rounded \
     bg-[var(--color-input-bg)] border border-[var(--color-input-border)] \
     text-[var(--color-label-primary)] \
     placeholder-[var(--color-input-placeholder)] \
     focus:border-[var(--color-input-focus)] focus:outline-none";

// ── Loading overlay ─────────────────────────────────────────────────

/// Full overlay covering the grid container during loading.
pub const LOADING_OVERLAY: &str = "absolute inset-0 flex items-center justify-center \
     bg-[var(--color-primary-dark)]/70 z-20";

/// Spinning animation for the loading icon.
pub const LOADING_SPINNER: &str = "size-8 text-[var(--color-accent-primary)] animate-spin";

// ── Empty state ─────────────────────────────────────────────────────

/// Empty state message container.
pub const EMPTY_STATE: &str = "px-4 py-12 text-center text-[var(--color-label-secondary)]";

// ── Selection checkbox ──────────────────────────────────────────────

/// Checkbox column header/cell width constraint.
pub const CHECKBOX_CELL: &str = "w-10 text-center";

/// Checkbox icon size.
pub const CHECKBOX_ICON: &str = "size-4 mx-auto cursor-pointer text-[var(--color-label-secondary)] \
     hover:text-[var(--color-accent-primary)] active:text-[var(--color-accent-primary)] transition-colors";

/// Checked checkbox icon.
pub const CHECKBOX_ICON_CHECKED: &str =
    "size-4 mx-auto cursor-pointer text-[var(--color-accent-primary)]";

// ── Bulk action bar ────────────────────────────────────────────────

/// Container for the bulk action bar below the grid.
pub const BULK_BAR: &str = "flex flex-wrap items-center gap-2 px-3 py-2 md:px-4 md:py-3 \
     border-t border-[var(--color-grid-border)] \
     bg-[var(--color-grid-header-bg)]";

/// Selection count label in the bulk action bar.
pub const BULK_LABEL: &str = "text-sm font-medium text-[var(--color-label-primary)] mr-2";

/// Standard bulk action button.
pub const BULK_BTN: &str = "px-3 py-1.5 text-xs font-medium rounded \
     bg-[var(--color-card)] text-[var(--color-label-primary)] \
     border border-[var(--color-card-border)] \
     hover:bg-[var(--color-card)]/80 active:bg-[var(--color-card)]/60 \
     transition-colors cursor-pointer";

/// Destructive bulk action button (delete).
pub const BULK_BTN_DANGER: &str = "px-3 py-1.5 text-xs font-medium rounded \
     bg-red-500/10 text-red-400 \
     border border-red-500/30 \
     hover:bg-red-500/20 active:bg-red-500/30 \
     transition-colors cursor-pointer";

/// Dropdown menu container for export/status options.
pub const BULK_DROPDOWN: &str = "absolute bottom-full left-0 mb-1 min-w-[120px] \
     bg-[var(--color-primary-dark)] \
     border border-[var(--color-card-border)] rounded shadow-lg z-30";

/// Individual option within a dropdown.
pub const BULK_DROPDOWN_ITEM: &str = "block w-full text-left px-3 py-1.5 text-xs \
     text-[var(--color-label-primary)] \
     hover:bg-[var(--color-card)]/30 \
     transition-colors cursor-pointer";

/// Separator between action groups.
pub const BULK_SEPARATOR: &str = "w-px h-6 bg-[var(--color-card-border)]";

// ── Aggregation panel ──────────────────────────────────────────────

/// Container for the aggregation summary below the bulk action bar.
pub const AGGREGATION_PANEL: &str = "flex flex-wrap items-center gap-4 px-3 py-2 md:px-4 md:py-3 \
     border-t border-[var(--color-grid-border)] \
     text-sm text-[var(--color-label-secondary)]";

/// Individual aggregation value label.
pub const AGGREGATION_ITEM: &str = "flex items-center gap-1";

/// Aggregation column name.
pub const AGGREGATION_KEY: &str = "font-medium text-[var(--color-label-primary)]";

/// Aggregation computed value.
pub const AGGREGATION_VALUE: &str = "text-[var(--color-accent-primary)] font-semibold";

// ── Reorder grip handle ───────────────────────────────────────────

/// Narrow grip handle cell — first column when reorderable is on.
pub const GRIP_CELL: &str = "w-8 text-center cursor-grab active:cursor-grabbing select-none";

/// Grip icon — subtle by default, full opacity on hover.
pub const GRIP_ICON: &str = "size-4 mx-auto text-[var(--color-label-secondary)] \
     opacity-40 hover:opacity-100 transition-opacity";

/// Insertion indicator shown above the hovered row during reorder
/// (when dragging a row downward past this row).
pub const REORDER_INSERT_ABOVE: &str = "border-t-2 border-t-[var(--color-accent-primary)]";

/// Insertion indicator shown below the hovered row during reorder
/// (when dragging a row upward past this row).
pub const REORDER_INSERT_BELOW: &str = "border-b-2 border-b-[var(--color-accent-primary)]";

// ── Drag-and-drop ─────────────────────────────────────────────────

/// Visual feedback on the grid container when it is a valid drop target
/// and the user is hovering with a drag payload.
pub const DROP_TARGET_ACTIVE: &str = "ring-2 ring-[var(--color-accent-primary)]/60 \
     bg-[var(--color-accent-primary)]/5 transition-all";

// ── Virtual scroll info bar ────────────────────────────────────────

/// Info bar shown below the virtualized viewport — displays the
/// currently visible row range and total entry count.
pub const VIRTUAL_INFO_BAR: &str = "flex items-center justify-between \
     px-3 py-2 md:px-4 md:py-2 border-t border-[var(--color-grid-border)] \
     text-xs text-[var(--color-label-secondary)]";

// ── Virtual scroll viewport ────────────────────────────────────────

/// Fixed-height scrollable viewport used when virtualization is enabled.
/// Height is set dynamically via inline style based on `visible_rows × row_height`.
pub const VIRTUAL_VIEWPORT: &str = "overflow-y-auto relative";

// ── Column resize ──────────────────────────────────────────────────

/// Header cell wrapper — relative positioning so the resize handle can
/// be placed at the right edge.
pub const TH_RESIZABLE: &str = "relative";

/// Drag handle on the right edge of a resizable header cell.
pub const RESIZE_HANDLE: &str = "absolute top-0 right-0 w-1 h-full cursor-col-resize \
     hover:bg-[var(--color-accent-primary)]/40 \
     active:bg-[var(--color-accent-primary)]/60 \
     transition-colors z-10";

/// Transparent full-viewport overlay shown during a resize drag
/// to capture mouse events regardless of cursor position.
pub const RESIZE_OVERLAY: &str = "fixed inset-0 z-50 cursor-col-resize";
