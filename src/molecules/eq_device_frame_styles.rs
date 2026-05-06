//! Style constants for EqDeviceFrame.
//!
//! All dimensions are tuned for the iPhone 16 / 16 Pro family.
//! TODO: validate against Apple's Human Interface Guidelines.

// ── Outer device shell ──────────────────────────────────────────────

/// Outer wrapper - bezel + side buttons surround the screen area.
/// `box-content` so the padding adds *outside* the screen dims.
pub const SHELL: &str =
    "relative inline-block rounded-[3rem] bg-neutral-900 \
     shadow-2xl shadow-black/40 ring-1 ring-black/30 \
     p-[12px] box-content";

// ── Screen area ─────────────────────────────────────────────────────

/// Screen surface - clipped, rounded interior of the device.
/// Width and height are applied inline based on the `DeviceModel`.
pub const SCREEN: &str =
    "relative overflow-hidden rounded-[2.5rem] \
     bg-[var(--color-primary-dark)] \
     flex flex-col";

// ── Dynamic Island ──────────────────────────────────────────────────

/// Static black pill positioned at the top centre of the screen.
/// Width and height are applied inline based on the `DeviceModel`.
pub const DYNAMIC_ISLAND: &str =
    "absolute top-[11px] left-1/2 -translate-x-1/2 z-20 \
     rounded-full bg-black";

// ── Status bar ──────────────────────────────────────────────────────

/// Status bar row - flanks the Dynamic Island with time / indicators.
pub const STATUS_BAR: &str =
    "relative z-10 h-[54px] px-[28px] \
     flex items-center justify-between \
     text-[14px] font-semibold tracking-tight \
     text-[var(--color-label-primary)]";

/// Tabular numerals for the time so digits don't shift.
pub const STATUS_BAR_TIME: &str = "tabular-nums";

/// Right-aligned indicator cluster.
pub const STATUS_BAR_RIGHT: &str = "flex items-center gap-[6px]";

// ── Body / children area ────────────────────────────────────────────

/// Where caller-provided children render. Scrolls if too tall.
pub const BODY: &str = "flex-1 overflow-auto relative";

// ── Home indicator ──────────────────────────────────────────────────

/// Bottom horizontal pill (the "swipe-up" handle).
pub const HOME_INDICATOR: &str =
    "absolute bottom-[8px] left-1/2 -translate-x-1/2 z-20 \
     h-[5px] w-[134px] rounded-full \
     bg-[var(--color-label-primary)]/85";

// ── Painted side buttons (decorative) ───────────────────────────────

/// Common base for all painted side buttons.
pub const SIDE_BUTTON_BASE: &str =
    "absolute bg-neutral-700 rounded-sm pointer-events-none";

/// Action button (left side, top).
pub const ACTION_BUTTON: &str = "left-[-2px] top-[88px] h-[36px] w-[3px]";

/// Volume up button (left side, middle).
pub const VOLUME_UP: &str = "left-[-2px] top-[140px] h-[60px] w-[3px]";

/// Volume down button (left side, lower).
pub const VOLUME_DOWN: &str = "left-[-2px] top-[210px] h-[60px] w-[3px]";

/// Power / side button (right side).
pub const POWER_BUTTON: &str = "right-[-2px] top-[160px] h-[80px] w-[3px]";

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("SHELL", SHELL),
        ("SCREEN", SCREEN),
        ("DYNAMIC_ISLAND", DYNAMIC_ISLAND),
        ("STATUS_BAR", STATUS_BAR),
        ("STATUS_BAR_TIME", STATUS_BAR_TIME),
        ("STATUS_BAR_RIGHT", STATUS_BAR_RIGHT),
        ("BODY", BODY),
        ("HOME_INDICATOR", HOME_INDICATOR),
        ("SIDE_BUTTON_BASE", SIDE_BUTTON_BASE),
        ("ACTION_BUTTON", ACTION_BUTTON),
        ("VOLUME_UP", VOLUME_UP),
        ("VOLUME_DOWN", VOLUME_DOWN),
        ("POWER_BUTTON", POWER_BUTTON),
    ]
}
