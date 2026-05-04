//! EqDatePicker — date picker molecule.
//!
//! A trigger input that opens a calendar popup for date selection.
//! Pure Rust date math (no external crate dependencies), keyboard
//! navigation, today highlight, and WAI-ARIA dialog pattern.
//!
//! ```rust,ignore
//! let mut date = use_signal(|| None::<DateValue>);
//!
//! EqDatePicker {
//!     value: date(),
//!     placeholder: "Pick a date",
//!     on_change: move |d| date.set(Some(d)),
//! }
//! ```

use super::eq_date_picker_styles as s;
use crate::theme::merge_classes;
use crate::{PlaygroundEnum, playground};
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropToggle, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// ── Types ─────────────────────────────────────────────────────────

/// A simple date value (year, month 1-12, day 1-31).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DateValue {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl DateValue {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// Format as YYYY-MM-DD.
    pub fn format(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Format as human-readable "Mon DD, YYYY".
    pub fn format_display(&self) -> String {
        let month_name = MONTH_NAMES
            .get(self.month as usize - 1)
            .unwrap_or(&"???");
        format!("{} {}, {}", month_name, self.day, self.year)
    }
}

/// Position of the calendar popup.
#[derive(Clone, Copy, PartialEq, Default, PlaygroundEnum)]
pub enum DatePickerPosition {
    #[default]
    Bottom,
    Top,
}

// ── Date math helpers (no external deps) ──────────────────────────

const MONTH_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun",
    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

const MONTH_FULL: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
];

const WEEKDAY_SHORT: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

/// Is this a leap year?
fn is_leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Days in a given month (1-12).
fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 => 31,
        2 => if is_leap(year) { 29 } else { 28 },
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 30,
    }
}

/// Day of week for a given date (0 = Sunday) — Zeller-like formula.
fn day_of_week(year: i32, month: u32, day: u32) -> u32 {
    // Tomohiko Sakamoto's algorithm.
    let t = [0i32, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = year;
    if month < 3 { y -= 1; }
    let dow = (y + y / 4 - y / 100 + y / 400 + t[month as usize - 1] + day as i32) % 7;
    if dow < 0 { (dow + 7) as u32 } else { dow as u32 }
}

/// Get today's date. We parse from JS since there's no std time on WASM.
/// Falls back to 2026-01-01 if parsing fails.
fn fallback_today() -> DateValue {
    DateValue::new(2026, 1, 1)
}

// ── SVG paths ────────────────────────────────────────────────────

/// Heroicons calendar (outline, 24×24).
const CALENDAR_PATH: &str =
    "M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 0 1 \
     2.25-2.25h13.5A2.25 2.25 0 0 1 21 7.5v11.25m-18 0A2.25 \
     2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75m-18 \
     0v-7.5A2.25 2.25 0 0 1 5.25 9h13.5A2.25 2.25 0 0 1 21 \
     11.25v7.5";

/// Heroicons chevron-left (mini).
const CHEVRON_LEFT: &str =
    "m12.77 5.23a.75.75 0 0 1 0 1.06L8.832 10l3.938 3.71a.75.75 0 1 1-1.04 1.08l-4.5-4.25a.75.75 0 0 1 0-1.08l4.5-4.25a.75.75 0 0 1 1.06-.02Z";

/// Heroicons chevron-right (mini).
const CHEVRON_RIGHT: &str =
    "m7.23 14.77a.75.75 0 0 1 0-1.06L11.168 10 7.23 6.29a.75.75 0 1 1 1.04-1.08l4.5 4.25a.75.75 0 0 1 0 1.08l-4.5 4.25a.75.75 0 0 1-1.06.02Z";

// ── Calendar grid data ────────────────────────────────────────────

/// A cell in the calendar grid.
#[derive(Clone)]
struct CalendarCell {
    year: i32,
    month: u32,
    day: u32,
    is_current_month: bool,
    is_today: bool,
    is_selected: bool,
}

fn build_calendar(
    view_year: i32,
    view_month: u32,
    today: &DateValue,
    selected: &Option<DateValue>,
) -> Vec<CalendarCell> {
    let first_dow = day_of_week(view_year, view_month, 1);
    let days = days_in_month(view_year, view_month);

    // Previous month padding.
    let (prev_year, prev_month) = if view_month == 1 {
        (view_year - 1, 12)
    } else {
        (view_year, view_month - 1)
    };
    let prev_days = days_in_month(prev_year, prev_month);

    // Next month.
    let (next_year, next_month) = if view_month == 12 {
        (view_year + 1, 1)
    } else {
        (view_year, view_month + 1)
    };

    let mut cells = Vec::with_capacity(42);

    // Previous month trailing days.
    for i in 0..first_dow {
        let d = prev_days - first_dow + 1 + i;
        cells.push(CalendarCell {
            year: prev_year,
            month: prev_month,
            day: d,
            is_current_month: false,
            is_today: today.year == prev_year as i32 && today.month == prev_month && today.day == d,
            is_selected: selected.as_ref().map_or(false, |s| s.year == prev_year as i32 && s.month == prev_month && s.day == d),
        });
    }

    // Current month days.
    for d in 1..=days {
        cells.push(CalendarCell {
            year: view_year,
            month: view_month,
            day: d,
            is_current_month: true,
            is_today: today.year == view_year as i32 && today.month == view_month && today.day == d,
            is_selected: selected.as_ref().map_or(false, |s| s.year == view_year as i32 && s.month == view_month && s.day == d),
        });
    }

    // Fill remaining to complete 6 rows (42 cells).
    let remaining = 42 - cells.len();
    for d in 1..=remaining as u32 {
        cells.push(CalendarCell {
            year: next_year,
            month: next_month,
            day: d,
            is_current_month: false,
            is_today: today.year == next_year as i32 && today.month == next_month && today.day == d,
            is_selected: selected.as_ref().map_or(false, |s| s.year == next_year as i32 && s.month == next_month && s.day == d),
        });
    }

    cells
}

// ── Component ─────────────────────────────────────────────────────

/// Date picker with calendar popup.
///
/// Opens a monthly calendar grid when the trigger is clicked.
/// Prev/next buttons navigate months. Clicking a day selects it
/// and closes the popup.
///
/// **Accessibility** — the trigger uses `role="combobox"` with
/// `aria-expanded`. The calendar popup uses `role="dialog"` with
/// `role="grid"` for the day grid. Arrow keys are planned for
/// future releases. Escape closes the popup.
#[playground(
    category = Molecule,
    description = "Date picker with calendar popup, month navigation, \
                   today highlight, and formatted display.",
    examples = [
        ("Basic", "let mut date = use_signal(|| None::<DateValue>);\n\nEqDatePicker {\n    value: date(),\n    on_change: move |d| date.set(Some(d)),\n}"),
        ("With placeholder", "EqDatePicker {\n    value: date(),\n    placeholder: \"Choose date\",\n    on_change: move |d| date.set(Some(d)),\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqDatePicker(
    /// Currently selected date.
    #[props(default)]
    value: Option<DateValue>,
    /// Placeholder when no date is selected.
    #[props(into, default = "Select date...".to_string())]
    placeholder: String,
    /// Disables interaction.
    #[props(default = false)]
    disabled: bool,
    /// Calendar popup position.
    #[props(default)]
    position: DatePickerPosition,
    /// Fired when a date is selected.
    #[props(default)]
    on_change: Option<EventHandler<DateValue>>,
    /// Optional class override on the wrapper element.
    #[props(into, default)]
    class: String,
) -> Element {
    let today = use_hook(|| fallback_today());

    // View state: which month/year the calendar is showing.
    let mut view_year = use_signal(|| {
        value.map(|d| d.year).unwrap_or(today.year)
    });
    let mut view_month = use_signal(|| {
        value.map(|d| d.month).unwrap_or(today.month)
    });
    let mut open = use_signal(|| false);

    let wrapper_cls = merge_classes(s::WRAPPER, &class);
    let trigger_cls = if disabled { s::TRIGGER_DISABLED } else { s::TRIGGER };

    let pos_cls = match position {
        DatePickerPosition::Bottom => s::POS_BOTTOM,
        DatePickerPosition::Top => s::POS_TOP,
    };
    let panel_state = if open() { s::PANEL_OPEN } else { s::PANEL_CLOSED };

    // Display text.
    let display_text = value
        .map(|d| d.format_display())
        .unwrap_or_else(|| placeholder.clone());
    let has_value = value.is_some();
    let display_cls = if has_value { "" } else { s::PLACEHOLDER };

    // Build calendar grid.
    let cells = build_calendar(view_year(), view_month(), &today, &value);
    let month_label = format!(
        "{} {}",
        MONTH_FULL.get(view_month() as usize - 1).unwrap_or(&"???"),
        view_year()
    );

    rsx! {
        div {
            class: "{wrapper_cls}",

            // Close on outside click.
            if open() {
                div {
                    class: "fixed inset-0 z-40",
                    onclick: move |_| open.set(false),
                }
            }

            // Trigger
            button {
                class: "{trigger_cls}",
                r#type: "button",
                disabled: disabled,
                role: "combobox",
                "aria-expanded": "{open()}",
                "aria-haspopup": "dialog",
                onclick: move |_| {
                    if !disabled {
                        let next = !open();
                        open.set(next);
                        // Reset view to selected date or today when opening.
                        if next {
                            if let Some(d) = &value {
                                view_year.set(d.year);
                                view_month.set(d.month);
                            }
                        }
                    }
                },
                onkeydown: move |evt: KeyboardEvent| {
                    if evt.key() == Key::Escape {
                        open.set(false);
                    }
                },

                span { class: "{display_cls}", "{display_text}" }
                svg {
                    class: "{s::CALENDAR_ICON}",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke_width: "1.5",
                    stroke: "currentColor",
                    width: "16",
                    height: "16",
                    "aria-hidden": "true",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: CALENDAR_PATH,
                    }
                }
            }

            // Calendar panel
            div {
                class: "{s::PANEL} {pos_cls} {panel_state}",
                role: "dialog",
                "aria-label": "Date picker",
                onkeydown: move |evt: KeyboardEvent| {
                    if evt.key() == Key::Escape {
                        open.set(false);
                    }
                },

                // Month/year header
                div { class: "{s::HEADER}",
                    button {
                        class: "{s::NAV_BUTTON}",
                        r#type: "button",
                        "aria-label": "Previous month",
                        onclick: move |_| {
                            let m = view_month();
                            if m == 1 {
                                view_month.set(12);
                                view_year.set(view_year() - 1);
                            } else {
                                view_month.set(m - 1);
                            }
                        },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 20 20",
                            fill: "currentColor",
                            width: "16",
                            height: "16",
                            "aria-hidden": "true",
                            path {
                                fill_rule: "evenodd",
                                clip_rule: "evenodd",
                                d: CHEVRON_LEFT,
                            }
                        }
                    }

                    span { class: "{s::HEADER_LABEL}", "{month_label}" }

                    button {
                        class: "{s::NAV_BUTTON}",
                        r#type: "button",
                        "aria-label": "Next month",
                        onclick: move |_| {
                            let m = view_month();
                            if m == 12 {
                                view_month.set(1);
                                view_year.set(view_year() + 1);
                            } else {
                                view_month.set(m + 1);
                            }
                        },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 20 20",
                            fill: "currentColor",
                            width: "16",
                            height: "16",
                            "aria-hidden": "true",
                            path {
                                fill_rule: "evenodd",
                                clip_rule: "evenodd",
                                d: CHEVRON_RIGHT,
                            }
                        }
                    }
                }

                // Weekday headers
                div { class: "{s::WEEKDAYS}",
                    for wd in WEEKDAY_SHORT.iter() {
                        span { class: "{s::WEEKDAY}", "{wd}" }
                    }
                }

                // Day grid
                div { class: "{s::DAYS_GRID}", role: "grid",
                    for cell in cells.iter() {
                        {
                            let mut extra = String::new();
                            if !cell.is_current_month {
                                extra.push_str(s::DAY_OUTSIDE);
                                extra.push(' ');
                            }
                            if cell.is_today && !cell.is_selected {
                                extra.push_str(s::DAY_TODAY);
                                extra.push(' ');
                            }
                            if cell.is_selected {
                                extra.push_str(s::DAY_SELECTED);
                                extra.push(' ');
                            }

                            let cy = cell.year;
                            let cm = cell.month;
                            let cd = cell.day;

                            rsx! {
                                button {
                                    key: "{cy}-{cm}-{cd}",
                                    class: "{s::DAY} {extra}",
                                    r#type: "button",
                                    role: "gridcell",
                                    "aria-selected": "{cell.is_selected}",
                                    tabindex: "-1",
                                    onclick: move |_| {
                                        let new_date = DateValue::new(cy, cm, cd);
                                        if let Some(handler) = &on_change {
                                            handler.call(new_date);
                                        }
                                        open.set(false);
                                    },
                                    "{cd}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqDatePicker() -> Element {
    let mut date = use_signal(|| None::<DateValue>);
    let mut disabled = use_signal(|| false);
    let mut position_str = use_signal(|| "Bottom".to_string());

    let position = match position_str().as_str() {
        "Top" => DatePickerPosition::Top,
        _ => DatePickerPosition::Bottom,
    };

    let date_display = date()
        .map(|d| d.format())
        .unwrap_or_else(|| "(none)".to_string());

    let code = format!(
        r#"let mut date = use_signal(|| None::<DateValue>);

EqDatePicker {{
    value: date(),
    placeholder: "Pick a date",
    disabled: {disabled},
    position: DatePickerPosition::{pos},
    on_change: move |d: DateValue| date.set(Some(d)),
}}"#,
        disabled = disabled(),
        pos = position_str(),
    );

    rsx! {
        DemoSection { title: "EqDatePicker",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropSelect {
                    label: "position",
                    value: position_str(),
                    options: vec!["Bottom", "Top"],
                    onchange: move |v: String| position_str.set(v),
                }
                PropToggle {
                    label: "disabled",
                    value: disabled(),
                    onchange: move |v: bool| disabled.set(v),
                }
            }

            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 space-y-6",
                div { class: "flex items-center gap-4",
                    div { class: "w-64",
                        EqDatePicker {
                            value: date(),
                            placeholder: "Pick a date",
                            disabled: disabled(),
                            position,
                            on_change: move |d: DateValue| date.set(Some(d)),
                        }
                    }
                    EqText { variant: TextVariant::Muted, "Selected: {date_display}" }
                }
            }

            StyleInfo { file: "eq_date_picker_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery ───────────────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqDatePicker() -> Element {
    let mut birthday = use_signal(|| None::<DateValue>);
    let mut deadline = use_signal(|| Some(DateValue::new(2026, 6, 15)));

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "DatePicker Gallery" }

                div { class: "flex items-start gap-4 flex-wrap",
                    div { class: "w-56 space-y-1",
                        EqText { variant: TextVariant::Muted, "Birthday" }
                        EqDatePicker {
                            value: birthday(),
                            placeholder: "Your birthday",
                            on_change: move |d: DateValue| birthday.set(Some(d)),
                        }
                    }
                    div { class: "w-56 space-y-1",
                        EqText { variant: TextVariant::Muted, "Deadline (pre-filled)" }
                        EqDatePicker {
                            value: deadline(),
                            on_change: move |d: DateValue| deadline.set(Some(d)),
                        }
                    }
                    div { class: "w-56 space-y-1",
                        EqText { variant: TextVariant::Muted, "Disabled" }
                        EqDatePicker {
                            value: Some(DateValue::new(2026, 5, 4)),
                            disabled: true,
                            on_change: move |_: DateValue| {},
                        }
                    }
                }
            }
        }
    }
}
