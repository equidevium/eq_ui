//! Shared style tokens used across multiple components.
//!
//! When an actual atom/molecule component is created (e.g. EqButton),
//! its specific constants move from here to the component's `_styles.rs`.

// ── Layout ──────────────────────────────────────────────────────────
pub const APP: &str =
    "min-h-screen bg-[var(--color-primary-dark)] text-[var(--color-label-primary)]";
pub const CONTAINER_LAYOUT: &str = "mx-auto max-w-6xl px-4";
pub const MAIN_CONTENT: &str = "flex-1";
pub const MAIN_INNER: &str = "py-10";

// ── Not Found (web page) ───────────────────────────────────────────
pub const NOTFOUND: &str = "mx-auto max-w-2xl py-16";
pub const NOTFOUND_HEADING: &str = "text-2xl font-semibold";
pub const NOTFOUND_TEXT: &str = "mt-3 text-[var(--color-label-secondary)]";
pub const NOTFOUND_LINK: &str =
    "mt-6 inline-block text-[var(--color-label-primary)] underline hover:text-[var(--color-label-bold)]";

// ── Spacing ─────────────────────────────────────────────────────────
pub const STACK_SM: &str = "space-y-2";
pub const STACK_MD: &str = "space-y-4";
pub const STACK_LG: &str = "space-y-6";
pub const STACK_XL: &str = "space-y-10";
pub const SECTION_Y_SM: &str = "py-6";
pub const SECTION_Y_MD: &str = "py-10";
pub const SECTION_Y_LG: &str = "py-16";
pub const PAGE_Y: &str = "py-10";
pub const PAGE_Y_LG: &str = "py-16";
pub const GAP_SM: &str = "gap-2";
pub const GAP_MD: &str = "gap-4";
pub const GAP_LG: &str = "gap-6";

// ── Borders ─────────────────────────────────────────────────────────
pub const RADIUS_SM: &str = "rounded-sm";
pub const RADIUS_MD: &str = "rounded-md";
pub const RADIUS_LG: &str = "rounded-lg";
pub const RADIUS_XL: &str = "rounded-xl";
pub const BORDER: &str = "border";
pub const BORDER_2: &str = "border-2";
pub const BORDER_DEFAULT: &str = "border-[var(--color-card-border)]";
pub const BORDER_SUBTLE: &str = "border-[var(--color-card-border)]/60";
pub const BORDER_STRONG: &str = "border-[var(--color-card-border)]";
pub const BORDER_FOCUS: &str =
    "focus:border-[var(--color-hover-button)] focus:ring-2 focus:ring-[var(--color-hover-button)]/50";

// ── Surfaces ────────────────────────────────────────────────────────
pub const SURFACE: &str =
    "rounded-xl border border-[var(--color-card-border)] \
     bg-[var(--color-card)]/60";
pub const SURFACE_ELEVATED: &str = "shadow-sm";
pub const SURFACE_INTERACTIVE: &str =
    "transition hover:shadow-md hover:-translate-y-[1px]";

// ── Shadows ─────────────────────────────────────────────────────────
pub const SHADOW_NONE: &str = "shadow-none";
pub const SHADOW_SM: &str = "shadow-sm shadow-black/10";
pub const SHADOW_MD: &str = "shadow-md shadow-black/20";
pub const SHADOW_LG: &str = "shadow-lg shadow-black/30";
pub const SHADOW_FOCUS: &str = "focus:shadow-md focus:shadow-black/30";

// ── Buttons (move to eq_button_styles.rs when EqButton is created) ─
pub const BTN_BASE: &str =
    "inline-flex items-center justify-center \
     rounded-md font-medium \
     transition focus:outline-none focus-visible:ring-2 \
     disabled:opacity-50 disabled:pointer-events-none";
pub const BTN_PRIMARY: &str =
    "px-4 py-2 \
     bg-[var(--color-hover-button)] \
     text-[var(--color-label-primary)] \
     hover:opacity-90";
pub const BTN_GHOST: &str =
    "px-4 py-2 \
     bg-transparent \
     text-[var(--color-label-secondary)] \
     hover:text-[var(--color-label-primary)]";
pub const BTN_DANGER: &str =
    "px-4 py-2 \
     bg-red-600 text-white hover:bg-red-700";
pub const BTN_SM: &str = "px-3 py-1.5 text-sm";
pub const BTN_MD: &str = "px-4 py-2 text-sm";
pub const BTN_LG: &str = "px-5 py-3 text-base";

// ── CTAs (move to eq_cta_styles.rs when EqCta is created) ──────────
pub const CTA: &str =
    "rounded-xl border border-[var(--color-card-border)] \
     bg-[var(--color-tertiary-dark)]/60 \
     p-6 flex flex-col gap-4 md:flex-row md:items-center md:justify-between";
pub const CTA_TEXT_GROUP: &str = "space-y-1";
pub const CTA_TITLE: &str =
    "text-lg font-semibold text-[var(--color-label-primary)]";
pub const CTA_DESCRIPTION: &str =
    "text-sm text-[var(--color-label-secondary)]";
pub const CTA_ACTION: &str = "shrink-0";

// ── Section cards (move when component is created) ──────────────────
pub const SECTION_CARD: &str =
    "rounded-xl border border-[var(--color-card-border)] bg-[var(--color-card)]/60 p-6 shadow-sm";
pub const SECTION_CARD_SM: &str =
    "rounded-xl border border-[var(--color-card-border)] bg-[var(--color-card)]/60 p-4 shadow-sm";
pub const SECTION_HEADER: &str = "space-y-2";
pub const SECTION_TITLE: &str =
    "text-lg font-semibold text-[var(--color-label-primary)]";
pub const SECTION_BODY: &str =
    "text-[var(--color-label-secondary)] leading-relaxed";
pub const SECTION_STACK: &str = "space-y-6";

// ── Navs (move to eq_nav_styles.rs when EqNav is created) ──────────
pub const NAV: &str = "flex";
pub const NAV_LIST: &str = "flex items-center gap-4 list-none m-0 p-0";
pub const NAV_LINK: &str =
    "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition";
pub const NAV_LINK_ACTIVE: &str =
    "text-[var(--color-label-primary)] font-medium";

// ── Link groups (move when component is created) ────────────────────
pub const LINK_GROUP: &str = "space-y-3";
pub const LINK_GROUP_TITLE: &str =
    "text-sm font-semibold tracking-wide text-[var(--color-label-primary)]";
pub const LINK_GROUP_LIST: &str = "space-y-2";
pub const LINK_GROUP_LINK: &str =
    "text-sm text-[var(--color-label-secondary)] hover:text-[var(--color-label-primary)] transition";
