//! Style constants for EqTree.

/// The outermost tree container.
pub const TREE: &str = "flex flex-col gap-0.5 text-sm select-none";

/// A single tree node row (branch or leaf).
pub const NODE_ROW: &str =
    "flex items-center gap-1.5 px-2 py-1.5 rounded-md cursor-pointer transition-colors hover:bg-[var(--color-card)]/60";

/// Active/selected node highlight.
pub const NODE_ACTIVE: &str =
    "flex items-center gap-1.5 px-2 py-1.5 rounded-md cursor-pointer bg-[var(--color-primary)]/15 text-[var(--color-primary)]";

/// Chevron icon for branch nodes.
pub const CHEVRON: &str = "size-4 shrink-0 transition-transform duration-200";

/// Chevron rotated when expanded.
pub const CHEVRON_EXPANDED: &str = "rotate-90";

/// Leaf indent spacer (replaces chevron width).
pub const LEAF_SPACER: &str = "w-4 shrink-0";

/// Node label text.
pub const LABEL: &str = "truncate text-[var(--color-label)]";

/// Children container with left indentation.
pub const CHILDREN: &str = "ml-3";
