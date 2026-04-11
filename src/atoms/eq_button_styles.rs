//! Style constants for EqButton.
//!
//! Variant and size classes reference the CSS rules defined in
//! `assets/theme/buttons.css`. The base `.btn` class provides the
//! shared reset (flex layout, focus ring, active scale, disabled
//! opacity) while variant classes add color, gradient, and hover
//! behaviour.

// ── Base ───────────────────────────────────────────────────────────

/// Shared button reset — layout, focus ring, active feedback, disabled state.
pub const BASE: &str = "btn";

// ── Variants ───────────────────────────────────────────────────────

/// Gradient background with animated position shift on hover.
pub const PRIMARY: &str = "btn-primary";

/// Transparent background, secondary text. Subtle hover fill.
pub const GHOST: &str = "btn-ghost";

/// Bordered with gradient hover reveal and border color shift.
pub const OUTLINE: &str = "btn-outline";

/// Card-styled with glow shadow and lift on hover.
pub const CARD: &str = "btn-card";

/// Destructive action — red background with brightness shift on hover.
pub const DANGER: &str = "btn-danger";

// ── Modifiers ─────────────────────────────────────────────────────

/// Disables the gradient and uses a flat solid background instead.
pub const NO_GRADIENT: &str = "btn-no-gradient";

/// Disables the gradient color transition — hover snaps instantly.
pub const NO_TRANSITION: &str = "btn-no-transition";

// ── Sizes ──────────────────────────────────────────────────────────

pub const SM: &str = "btn-sm";
pub const MD: &str = "btn-md";
pub const LG: &str = "btn-lg";
