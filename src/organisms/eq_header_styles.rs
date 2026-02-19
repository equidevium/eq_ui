//! Style constants for EqHeader component.
//! NAV_UL and NAV_A are pub so platform crates can style their nav items.

pub const HEADER: &str =
    "sticky top-0 z-50 border-b border-[var(--color-card-border)] bg-[var(--color-primary-dark)]/80 backdrop-blur";

pub const HEADER_INNER: &str = "flex items-center justify-between py-4";

pub const BRAND: &str =
    "text-lg font-semibold tracking-tight text-[var(--color-label-primary)]";

pub const NAV_UL: &str = "flex gap-4 items-center list-none m-0 p-0";

pub const NAV_A: &str =
    "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition";
