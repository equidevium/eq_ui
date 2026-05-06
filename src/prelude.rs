//! Common imports for consumers and doctest preambles.
//!
//! Bring everything most apps need in one line:
//!
//! ```rust,no_run
//! use eq_ui::prelude::*;
//! ```
//!
//! This re-exports `dioxus::prelude::*` plus the eq_ui types you reach
//! for in nearly every file: theme assets, the theme provider and
//! switcher, and the most-used atoms / molecules / organisms.
//!
//! It does not re-export every component. Less common ones (EqGrid,
//! EqDeviceFrame, EqMobileAppShell, etc.) are accessed via their
//! module paths so you can be explicit about heavy types.

pub use dioxus::prelude::*;

// CSS assets
pub use crate::{UI_BUTTONS_CSS, UI_INDEX_CSS, UI_TAILWIND_CSS};

// Theming
pub use crate::eq_theme::EqTheme;

// Common atoms
pub use crate::atoms::{
    AspectRatio, AtomImageSize, ButtonSize, ButtonVariant, EqAvatar, EqButton, EqCheckbox, EqDivider,
    EqIcon, EqImage, EqInput, EqLabel, EqLink, EqProgress, EqText, IconSize, InputKind, ObjectFit,
    TextVariant,
};

// Common molecules
pub use crate::molecules::{EqCard, EqCardBody, EqCardFooter, EqCardHeader};

// Common organisms
pub use crate::organisms::{EqAppShell, EqFooter, EqHeader, EqPageSection};
