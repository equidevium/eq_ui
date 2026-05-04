//! EqCalendar — standalone calendar molecule.
//!
//! An always-visible monthly calendar grid with month navigation,
//! single-date selection, today highlight, and optional min/max
//! date constraints. Pure Rust date math, no external dependencies.
//!
//! ```rust,ignore
//! use eq_ui::molecules::eq_date_picker::DateValue;
//!
//! let mut selected = use_signal(|| None::<DateValue>);
//!
//! EqCalendar {
//!     selected: selected(),
//!     on_select: move |d| selected.set(Some(d)),
//! }
//! ```

use super::eq_calendar_styles as s;
use crate::molecules::eq_date_picker::DateValue;
use crate::theme::merge_classes;
use crate::playground;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropToggle, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// ── Event types ───────────────────────────────────────────────────

/// Color of an event indicator dot.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum EventColor {
    /// Theme accent color (default).
    #[default]
    Default,
    /// Green.
    Success,
    /// Amber.
    Warning,
    /// Red.
    Danger,
    /// Blue.
    Info,
}

/// Calendar display mode.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum CalendarMode {
    /// Monthly grid (default).
    #[default]
    Month,
    /// Weekly time grid with hourly rows.
    Week,
}

/// An event marker on a specific date, with optional time range.
#[derive(Clone, PartialEq)]
pub struct CalendarEvent {
    /// The date this event falls on.
    pub date: DateValue,
    /// Optional label (used for aria-label / tooltip).
    pub label: String,
    /// Dot color.
    pub color: EventColor,
    /// Start hour (0–23). None = all-day event.
    pub start_hour: Option<u32>,
    /// Start minute (0–59).
    pub start_min: u32,
    /// End hour (0–23).
    pub end_hour: Option<u32>,
    /// End minute (0–59).
    pub end_min: u32,
}

impl CalendarEvent {
    /// Create an all-day event with the default accent color.
    pub fn new(date: DateValue, label: impl Into<String>) -> Self {
        Self {
            date,
            label: label.into(),
            color: EventColor::Default,
            start_hour: None,
            start_min: 0,
            end_hour: None,
            end_min: 0,
        }
    }

    /// Create a timed event.
    pub fn timed(
        date: DateValue,
        label: impl Into<String>,
        start_hour: u32,
        start_min: u32,
        end_hour: u32,
        end_min: u32,
    ) -> Self {
        Self {
            date,
            label: label.into(),
            color: EventColor::Default,
            start_hour: Some(start_hour),
            start_min,
            end_hour: Some(end_hour),
            end_min,
        }
    }

    /// Builder: set the dot color.
    pub fn color(mut self, color: EventColor) -> Self {
        self.color = color;
        self
    }

    /// Is this a timed event (not all-day)?
    pub fn is_timed(&self) -> bool {
        self.start_hour.is_some()
    }

    /// Format the time range as "HH:MM – HH:MM".
    pub fn time_display(&self) -> String {
        match (self.start_hour, self.end_hour) {
            (Some(sh), Some(eh)) => {
                format!("{:02}:{:02} – {:02}:{:02}", sh, self.start_min, eh, self.end_min)
            }
            _ => "All day".to_string(),
        }
    }
}

// ── Date math (shared with EqDatePicker) ──────────────────────────

const MONTH_SHORT: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun",
    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

const MONTH_FULL: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
];

const WEEKDAY_SHORT: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

fn is_leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 => 31, 2 => if is_leap(year) { 29 } else { 28 },
        3 => 31, 4 => 30, 5 => 31, 6 => 30,
        7 => 31, 8 => 31, 9 => 30, 10 => 31,
        11 => 30, 12 => 31, _ => 30,
    }
}

fn day_of_week(year: i32, month: u32, day: u32) -> u32 {
    let t = [0i32, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = year;
    if month < 3 { y -= 1; }
    let dow = (y + y / 4 - y / 100 + y / 400 + t[month as usize - 1] + day as i32) % 7;
    if dow < 0 { (dow + 7) as u32 } else { dow as u32 }
}

/// Heroicons chevron-left (mini).
const CHEVRON_LEFT: &str =
    "m12.77 5.23a.75.75 0 0 1 0 1.06L8.832 10l3.938 3.71a.75.75 0 1 1-1.04 1.08l-4.5-4.25a.75.75 0 0 1 0-1.08l4.5-4.25a.75.75 0 0 1 1.06-.02Z";

/// Heroicons chevron-right (mini).
const CHEVRON_RIGHT: &str =
    "m7.23 14.77a.75.75 0 0 1 0-1.06L11.168 10 7.23 6.29a.75.75 0 1 1 1.04-1.08l4.5 4.25a.75.75 0 0 1 0 1.08l-4.5 4.25a.75.75 0 0 1-1.06.02Z";

const WEEKDAY_FULL: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

/// Advance a date by `n` days.
fn add_days(date: DateValue, n: i32) -> DateValue {
    let mut y = date.year;
    let mut m = date.month;
    let mut d = date.day as i32 + n;

    while d < 1 {
        if m == 1 { m = 12; y -= 1; } else { m -= 1; }
        d += days_in_month(y, m) as i32;
    }
    loop {
        let dim = days_in_month(y, m) as i32;
        if d <= dim { break; }
        d -= dim;
        if m == 12 { m = 1; y += 1; } else { m += 1; }
    }
    DateValue::new(y, m, d as u32)
}

/// Get the Sunday that starts the week containing `date`.
fn week_start(date: DateValue) -> DateValue {
    let dow = day_of_week(date.year, date.month, date.day);
    add_days(date, -(dow as i32))
}

/// Get the 7 days of the week containing `date` (Sun..Sat).
fn week_days(date: DateValue) -> Vec<DateValue> {
    let start = week_start(date);
    (0..7).map(|i| add_days(start, i)).collect()
}

/// Get the event color class for week view blocks.
fn week_event_color_cls(color: EventColor) -> &'static str {
    match color {
        EventColor::Default => s::WEEK_EVENT_DEFAULT,
        EventColor::Success => s::WEEK_EVENT_SUCCESS,
        EventColor::Warning => s::WEEK_EVENT_WARNING,
        EventColor::Danger => s::WEEK_EVENT_DANGER,
        EventColor::Info => s::WEEK_EVENT_INFO,
    }
}

// ── Calendar cell ────────────────────────────────────────────────

#[derive(Clone)]
struct CalendarCell {
    year: i32,
    month: u32,
    day: u32,
    is_current_month: bool,
    is_today: bool,
    is_selected: bool,
    is_disabled: bool,
    /// Event dot colors for this day (may have multiple).
    event_colors: Vec<EventColor>,
}

fn events_for_date(events: &[CalendarEvent], y: i32, m: u32, d: u32) -> Vec<EventColor> {
    events.iter()
        .filter(|e| e.date.year == y && e.date.month == m && e.date.day == d)
        .map(|e| e.color)
        .collect()
}

fn build_calendar(
    view_year: i32,
    view_month: u32,
    today: &DateValue,
    selected: &Option<DateValue>,
    min_date: &Option<DateValue>,
    max_date: &Option<DateValue>,
    events: &[CalendarEvent],
) -> Vec<CalendarCell> {
    let first_dow = day_of_week(view_year, view_month, 1);
    let days = days_in_month(view_year, view_month);

    let (prev_year, prev_month) = if view_month == 1 {
        (view_year - 1, 12)
    } else {
        (view_year, view_month - 1)
    };
    let prev_days = days_in_month(prev_year, prev_month);

    let (next_year, next_month) = if view_month == 12 {
        (view_year + 1, 1)
    } else {
        (view_year, view_month + 1)
    };

    let is_disabled = |y: i32, m: u32, d: u32| -> bool {
        if let Some(min) = min_date {
            if y < min.year || (y == min.year && m < min.month)
                || (y == min.year && m == min.month && d < min.day)
            {
                return true;
            }
        }
        if let Some(max) = max_date {
            if y > max.year || (y == max.year && m > max.month)
                || (y == max.year && m == max.month && d > max.day)
            {
                return true;
            }
        }
        false
    };

    let mut cells = Vec::with_capacity(42);

    // Previous month trailing days.
    for i in 0..first_dow {
        let d = prev_days - first_dow + 1 + i;
        cells.push(CalendarCell {
            year: prev_year, month: prev_month, day: d,
            is_current_month: false,
            is_today: today.year == prev_year && today.month == prev_month && today.day == d,
            is_selected: selected.as_ref().map_or(false, |s| s.year == prev_year && s.month == prev_month && s.day == d),
            is_disabled: is_disabled(prev_year, prev_month, d),
            event_colors: events_for_date(events, prev_year, prev_month, d),
        });
    }

    // Current month.
    for d in 1..=days {
        cells.push(CalendarCell {
            year: view_year, month: view_month, day: d,
            is_current_month: true,
            is_today: today.year == view_year && today.month == view_month && today.day == d,
            is_selected: selected.as_ref().map_or(false, |s| s.year == view_year && s.month == view_month && s.day == d),
            is_disabled: is_disabled(view_year, view_month, d),
            event_colors: events_for_date(events, view_year, view_month, d),
        });
    }

    // Next month fill to 42.
    let remaining = 42 - cells.len();
    for d in 1..=remaining as u32 {
        cells.push(CalendarCell {
            year: next_year, month: next_month, day: d,
            is_current_month: false,
            is_today: today.year == next_year && today.month == next_month && today.day == d,
            is_selected: selected.as_ref().map_or(false, |s| s.year == next_year && s.month == next_month && s.day == d),
            is_disabled: is_disabled(next_year, next_month, d),
            event_colors: events_for_date(events, next_year, next_month, d),
        });
    }

    cells
}

// ── Component ─────────────────────────────────────────────────────

/// Always-visible calendar with month and week views.
///
/// **Month mode** — full month grid with navigation arrows.
/// **Week mode** — 7-day columns × 24-hour rows with positioned event blocks.
///
/// Clicking a day fires `on_select`. Optional `min_date` / `max_date`
/// props disable out-of-range days.
///
/// **Accessibility** — uses `role="grid"` with `role="gridcell"` on
/// each day, `aria-selected` on the chosen day, `aria-disabled` on
/// constrained days, and `aria-label` on nav buttons.
#[playground(
    category = Molecule,
    description = "Standalone calendar with month & week views, navigation, \
                   today highlight, timed events, single selection, min/max \
                   constraints, and WAI-ARIA grid pattern.",
    examples = [
        ("Basic", "let mut sel = use_signal(|| None::<DateValue>);\n\nEqCalendar {\n    selected: sel(),\n    on_select: move |d| sel.set(Some(d)),\n}"),
        ("Week mode", "EqCalendar {\n    selected: sel(),\n    mode: CalendarMode::Week,\n    events: my_events,\n    on_select: move |d| sel.set(Some(d)),\n}"),
    ],
    custom_demo,
    custom_gallery,
)]
#[component]
pub fn EqCalendar(
    /// Currently selected date.
    #[props(default)]
    selected: Option<DateValue>,
    /// Earliest selectable date.
    #[props(default)]
    min_date: Option<DateValue>,
    /// Latest selectable date.
    #[props(default)]
    max_date: Option<DateValue>,
    /// Show days from adjacent months in the grid.
    #[props(default = true)]
    show_outside_days: bool,
    /// Event markers to display as dots on specific dates.
    #[props(default)]
    events: Vec<CalendarEvent>,
    /// Display mode — Month grid or Week time-grid.
    #[props(default)]
    mode: CalendarMode,
    /// Fired when a day is clicked.
    #[props(default)]
    on_select: Option<EventHandler<DateValue>>,
    /// Optional class override on the wrapper.
    #[props(into, default)]
    class: String,
) -> Element {
    let today = use_hook(|| DateValue::new(2026, 5, 4));

    let mut view_year = use_signal(|| {
        selected.map(|d| d.year).unwrap_or(today.year)
    });
    let mut view_month = use_signal(|| {
        selected.map(|d| d.month).unwrap_or(today.month)
    });
    // 0 = days/week, 1 = month picker, 2 = year picker
    let mut view_mode = use_signal(|| 0u8);

    // Anchor date for week view (the selected date or today).
    let mut week_anchor = use_signal(|| {
        selected.unwrap_or(today)
    });

    let is_week = mode == CalendarMode::Week;
    let width_cls = if is_week { s::WRAPPER_WEEK } else { s::WRAPPER_MONTH };
    let wrapper_cls = merge_classes(&format!("{} {}", s::WRAPPER, width_cls), &class);

    let cells = build_calendar(
        view_year(), view_month(), &today, &selected, &min_date, &max_date, &events,
    );
    let month_name = MONTH_FULL.get(view_month() as usize - 1).unwrap_or(&"???");
    let year_val = view_year();

    // Year picker: show a 4×3 grid centered on current year.
    let year_start = (year_val / 12) * 12;

    // Week view data.
    let w_days = week_days(week_anchor());
    rsx! {
        div {
            class: "{wrapper_cls}",

            // Header — changes based on view mode
            div { class: "{s::HEADER}",
                // Left nav
                button {
                    class: "{s::NAV_BUTTON}",
                    r#type: "button",
                    "aria-label": if is_week && view_mode() == 0 { "Previous week" } else if view_mode() == 0 { "Previous month" } else if view_mode() == 1 { "Previous year" } else { "Previous decade" },
                    onclick: move |_| {
                        if is_week && view_mode() == 0 {
                            let new_anchor = add_days(week_anchor(), -7);
                            week_anchor.set(new_anchor);
                            view_year.set(new_anchor.year);
                            view_month.set(new_anchor.month);
                        } else {
                            match view_mode() {
                                0 => {
                                    let m = view_month();
                                    if m == 1 {
                                        view_month.set(12);
                                        view_year.set(view_year() - 1);
                                    } else {
                                        view_month.set(m - 1);
                                    }
                                }
                                1 => { view_year.set(view_year() - 1); }
                                _ => { view_year.set(view_year() - 12); }
                            }
                        }
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 20 20",
                        fill: "currentColor",
                        width: "16", height: "16",
                        "aria-hidden": "true",
                        path { fill_rule: "evenodd", clip_rule: "evenodd", d: CHEVRON_LEFT }
                    }
                }

                // Center label(s)
                match view_mode() {
                    0 => rsx! {
                        span { class: "flex gap-1.5",
                            button {
                                class: "{s::HEADER_MONTH}",
                                r#type: "button",
                                onclick: move |_| view_mode.set(1),
                                "{month_name}"
                            }
                            button {
                                class: "{s::HEADER_YEAR}",
                                r#type: "button",
                                onclick: move |_| view_mode.set(2),
                                "{year_val}"
                            }
                        }
                    },
                    1 => rsx! {
                        button {
                            class: "{s::HEADER_LABEL}",
                            r#type: "button",
                            onclick: move |_| view_mode.set(2),
                            "{year_val}"
                        }
                    },
                    _ => rsx! {
                        span { class: "{s::HEADER_LABEL}",
                            "{year_start} – {year_start + 11}"
                        }
                    },
                }

                // Right nav
                button {
                    class: "{s::NAV_BUTTON}",
                    r#type: "button",
                    "aria-label": if is_week && view_mode() == 0 { "Next week" } else if view_mode() == 0 { "Next month" } else if view_mode() == 1 { "Next year" } else { "Next decade" },
                    onclick: move |_| {
                        if is_week && view_mode() == 0 {
                            let new_anchor = add_days(week_anchor(), 7);
                            week_anchor.set(new_anchor);
                            view_year.set(new_anchor.year);
                            view_month.set(new_anchor.month);
                        } else {
                            match view_mode() {
                                0 => {
                                    let m = view_month();
                                    if m == 12 {
                                        view_month.set(1);
                                        view_year.set(view_year() + 1);
                                    } else {
                                        view_month.set(m + 1);
                                    }
                                }
                                1 => { view_year.set(view_year() + 1); }
                                _ => { view_year.set(view_year() + 12); }
                            }
                        }
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 20 20",
                        fill: "currentColor",
                        width: "16", height: "16",
                        "aria-hidden": "true",
                        path { fill_rule: "evenodd", clip_rule: "evenodd", d: CHEVRON_RIGHT }
                    }
                }
            }

            // ── View: Month picker (3×4 grid) ──────────────────
            if view_mode() == 1 {
                div { class: "{s::PICKER_GRID}",
                    for mi in 1u32..=12 {
                        {
                            let is_active = mi == view_month();
                            let active_cls = if is_active { s::PICKER_CELL_ACTIVE } else { "" };
                            let label = MONTH_SHORT[mi as usize - 1];

                            rsx! {
                                button {
                                    key: "month-{mi}",
                                    class: "{s::PICKER_CELL} {active_cls}",
                                    r#type: "button",
                                    onclick: move |_| {
                                        view_month.set(mi);
                                        view_mode.set(0);
                                    },
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }

            // ── View: Year picker (4×3 grid) ───────────────────
            if view_mode() == 2 {
                div { class: "{s::PICKER_GRID}",
                    for yi in 0..12 {
                        {
                            let y = year_start + yi;
                            let is_active = y == view_year();
                            let active_cls = if is_active { s::PICKER_CELL_ACTIVE } else { "" };

                            rsx! {
                                button {
                                    key: "year-{y}",
                                    class: "{s::PICKER_CELL} {active_cls}",
                                    r#type: "button",
                                    onclick: move |_| {
                                        view_year.set(y);
                                        view_mode.set(1);
                                    },
                                    "{y}"
                                }
                            }
                        }
                    }
                }
            }

            // ── View: Week time-grid ──────────────────────────
            if view_mode() == 0 && is_week {
                // Day column headers
                div { class: "{s::WEEK_HEADER}",
                    // Empty gutter header
                    div { class: "{s::WEEK_TIME_GUTTER} py-2", "" }

                    for wd in w_days.iter() {
                        {
                            let is_today = wd.year == today.year && wd.month == today.month && wd.day == today.day;
                            let day_name = WEEKDAY_FULL[day_of_week(wd.year, wd.month, wd.day) as usize];
                            let day_num = wd.day;
                            let num_cls = if is_today { s::WEEK_DAY_NUM_TODAY } else { s::WEEK_DAY_NUM };

                            rsx! {
                                div {
                                    key: "wh-{wd.year}-{wd.month}-{wd.day}",
                                    class: "{s::WEEK_DAY_HEADER}",
                                    span { class: "{s::WEEK_DAY_NAME}", "{day_name}" }
                                    span { class: "{num_cls}", "{day_num}" }
                                }
                            }
                        }
                    }
                }

                // All-day events bar
                {
                    let allday_events: Vec<(usize, &CalendarEvent)> = events.iter()
                        .filter(|e| !e.is_timed())
                        .filter_map(|e| {
                            w_days.iter().position(|wd| wd.year == e.date.year && wd.month == e.date.month && wd.day == e.date.day)
                                .map(|idx| (idx, e))
                        })
                        .collect();

                    let has_allday = !allday_events.is_empty();

                    rsx! {
                        if has_allday {
                            div { class: "{s::WEEK_ALLDAY_ROW}",
                                // Gutter label
                                div { class: "{s::WEEK_TIME_GUTTER} py-1 text-[10px]", "all-day" }

                                for col_idx in 0usize..7 {
                                    {
                                        let col_events: Vec<&&CalendarEvent> = allday_events.iter()
                                            .filter(|(idx, _)| *idx == col_idx)
                                            .map(|(_, e)| e)
                                            .collect();

                                        rsx! {
                                            div {
                                                key: "allday-col-{col_idx}",
                                                class: "border-r border-[var(--color-card-border)]/30",
                                                for (ei , evt) in col_events.iter().enumerate() {
                                                    {
                                                        let chip_color = week_event_color_cls(evt.color);
                                                        let evt_label = evt.label.clone();
                                                        rsx! {
                                                            div {
                                                                key: "allday-{col_idx}-{ei}",
                                                                class: "{s::WEEK_ALLDAY_CHIP} {chip_color}",
                                                                title: "{evt_label}",
                                                                "{evt_label}"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Scrollable hour rows
                div { class: "{s::WEEK_BODY}",
                    for hour in 0u32..24 {
                        {
                            let hour_label = format!("{:02}:00", hour);

                            rsx! {
                                div {
                                    key: "hour-{hour}",
                                    class: "{s::WEEK_HOUR_ROW}",

                                    // Time gutter
                                    div { class: "{s::WEEK_TIME_GUTTER} pt-0.5", "{hour_label}" }

                                    // 7 day cells
                                    for (col_i , wd) in w_days.iter().enumerate() {
                                        {
                                            // Find timed events that overlap this hour in this day.
                                            let cell_events: Vec<&CalendarEvent> = events.iter()
                                                .filter(|e| {
                                                    e.is_timed()
                                                    && e.date.year == wd.year
                                                    && e.date.month == wd.month
                                                    && e.date.day == wd.day
                                                    && e.start_hour.unwrap_or(0) == hour
                                                })
                                                .collect();

                                            let wd_y = wd.year;
                                            let wd_m = wd.month;
                                            let wd_d = wd.day;

                                            rsx! {
                                                div {
                                                    key: "cell-{hour}-{col_i}",
                                                    class: "{s::WEEK_DAY_CELL}",
                                                    onclick: move |_| {
                                                        if let Some(handler) = &on_select {
                                                            handler.call(DateValue::new(wd_y, wd_m, wd_d));
                                                        }
                                                    },
                                                    for (ei , evt) in cell_events.iter().enumerate() {
                                                        {
                                                            let evt_color_cls = week_event_color_cls(evt.color);
                                                            let duration_hours = match (evt.start_hour, evt.end_hour) {
                                                                (Some(sh), Some(eh)) => {
                                                                    let start_mins = sh * 60 + evt.start_min;
                                                                    let end_mins = eh * 60 + evt.end_min;
                                                                    if end_mins > start_mins {
                                                                        (end_mins - start_mins) as f64 / 60.0
                                                                    } else {
                                                                        1.0
                                                                    }
                                                                }
                                                                _ => 1.0,
                                                            };
                                                            // Height in pixels: each hour row is 48px.
                                                            let height_px = (duration_hours * 48.0).round() as u32;
                                                            // Offset from top based on start_min.
                                                            let top_px = (evt.start_min as f64 / 60.0 * 48.0).round() as u32;
                                                            let height_style = format!("height: {}px; top: {}px;", height_px, top_px);
                                                            let evt_label = evt.label.clone();
                                                            let time_str = evt.time_display();

                                                            rsx! {
                                                                div {
                                                                    key: "evt-{hour}-{col_i}-{ei}",
                                                                    class: "{s::WEEK_EVENT} {evt_color_cls}",
                                                                    style: "{height_style}",
                                                                    title: "{evt_label} ({time_str})",
                                                                    "{evt_label}"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ── View: Month day grid (default) ────────────────
            if view_mode() == 0 && !is_week {
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
                            let skip = !cell.is_current_month && !show_outside_days;

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
                            if cell.is_disabled {
                                extra.push_str(s::DAY_DISABLED);
                                extra.push(' ');
                            }

                            let cy = cell.year;
                            let cm = cell.month;
                            let cd = cell.day;
                            let disabled = cell.is_disabled;
                            let dots = cell.event_colors.clone();
                            let has_dots = !dots.is_empty();

                            rsx! {
                                if skip {
                                    span { key: "empty-{cy}-{cm}-{cd}", class: "size-9" }
                                } else {
                                    button {
                                        key: "{cy}-{cm}-{cd}",
                                        class: "{s::DAY} {extra}",
                                        r#type: "button",
                                        role: "gridcell",
                                        "aria-selected": "{cell.is_selected}",
                                        "aria-disabled": if disabled { "true" } else { "false" },
                                        tabindex: "-1",
                                        disabled: disabled,
                                        onclick: move |_| {
                                            if let Some(handler) = &on_select {
                                                handler.call(DateValue::new(cy, cm, cd));
                                            }
                                        },

                                        if has_dots {
                                            span { class: "{s::DAY_INNER}",
                                                span { "{cd}" }
                                                span { class: "flex gap-0.5",
                                                    for (di , dot_color) in dots.iter().enumerate() {
                                                        {
                                                            let color_cls = match dot_color {
                                                                EventColor::Default => s::EVENT_DOT_DEFAULT,
                                                                EventColor::Success => s::EVENT_DOT_SUCCESS,
                                                                EventColor::Warning => s::EVENT_DOT_WARNING,
                                                                EventColor::Danger => s::EVENT_DOT_DANGER,
                                                                EventColor::Info => s::EVENT_DOT_INFO,
                                                            };
                                                            rsx! {
                                                                span {
                                                                    key: "dot-{di}",
                                                                    class: "{s::EVENT_DOT} {color_cls}",
                                                                    "aria-hidden": "true",
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            span { "{cd}" }
                                        }
                                    }
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
fn DemoEqCalendar() -> Element {
    let mut selected = use_signal(|| None::<DateValue>);
    let mut show_outside = use_signal(|| true);
    let mut constrained = use_signal(|| false);
    let mut week_mode = use_signal(|| false);

    let min_date = if constrained() { Some(DateValue::new(2026, 5, 1)) } else { None };
    let max_date = if constrained() { Some(DateValue::new(2026, 5, 31)) } else { None };

    // Sample events — mix of all-day and timed events.
    let events = vec![
        // All-day events
        CalendarEvent::new(DateValue::new(2026, 5, 4), "Team standup"),
        CalendarEvent::new(DateValue::new(2026, 5, 4), "Design review")
            .color(EventColor::Info),
        CalendarEvent::new(DateValue::new(2026, 5, 10), "Sprint planning")
            .color(EventColor::Success),
        CalendarEvent::new(DateValue::new(2026, 5, 15), "Release deadline")
            .color(EventColor::Danger),
        CalendarEvent::new(DateValue::new(2026, 5, 20), "Retrospective")
            .color(EventColor::Warning),
        CalendarEvent::new(DateValue::new(2026, 5, 22), "1:1 meeting"),
        CalendarEvent::new(DateValue::new(2026, 5, 28), "Demo day")
            .color(EventColor::Success),
        // Timed events (visible in week mode)
        CalendarEvent::timed(DateValue::new(2026, 5, 4), "Standup", 9, 0, 9, 30),
        CalendarEvent::timed(DateValue::new(2026, 5, 4), "Design sync", 14, 0, 15, 30)
            .color(EventColor::Info),
        CalendarEvent::timed(DateValue::new(2026, 5, 5), "Sprint review", 10, 0, 11, 0)
            .color(EventColor::Success),
        CalendarEvent::timed(DateValue::new(2026, 5, 6), "Lunch & Learn", 12, 0, 13, 0)
            .color(EventColor::Warning),
        CalendarEvent::timed(DateValue::new(2026, 5, 7), "Deploy window", 16, 0, 18, 0)
            .color(EventColor::Danger),
        CalendarEvent::timed(DateValue::new(2026, 5, 8), "Team retro", 15, 0, 16, 0)
            .color(EventColor::Success),
    ];

    // Find events for the selected date.
    let selected_events: Vec<String> = selected()
        .map(|sel| {
            events.iter()
                .filter(|e| e.date == sel)
                .map(|e| {
                    if e.is_timed() {
                        format!("{} ({})", e.label, e.time_display())
                    } else {
                        e.label.clone()
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let sel_display = selected()
        .map(|d| d.format_display())
        .unwrap_or_else(|| "(none)".to_string());

    let code = r#"let events = vec![
    CalendarEvent::new(DateValue::new(2026, 5, 4), "Team standup"),
    CalendarEvent::timed(DateValue::new(2026, 5, 4), "Standup", 9, 0, 9, 30),
    CalendarEvent::timed(DateValue::new(2026, 5, 5), "Sprint review", 10, 0, 11, 0)
        .color(EventColor::Success),
];

let mut selected = use_signal(|| None::<DateValue>);

// Month mode (default):
EqCalendar {
    selected: selected(),
    events: events.clone(),
    on_select: move |d: DateValue| selected.set(Some(d)),
}

// Week mode:
EqCalendar {
    selected: selected(),
    mode: CalendarMode::Week,
    events,
    on_select: move |d: DateValue| selected.set(Some(d)),
}"#.to_string();

    rsx! {
        DemoSection { title: "EqCalendar",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropToggle {
                    label: "show_outside_days",
                    value: show_outside(),
                    onchange: move |v: bool| show_outside.set(v),
                }
                PropToggle {
                    label: "constrained (May 2026 only)",
                    value: constrained(),
                    onchange: move |v: bool| constrained.set(v),
                }
                PropToggle {
                    label: "week mode",
                    value: week_mode(),
                    onchange: move |v: bool| week_mode.set(v),
                }
            }

            div { class: "rounded-lg border border-dashed border-[var(--color-card-border)] p-6 space-y-4",
                div { class: "flex items-start gap-6 flex-wrap",
                    EqCalendar {
                        selected: selected(),
                        show_outside_days: show_outside(),
                        min_date,
                        max_date,
                        mode: if week_mode() { CalendarMode::Week } else { CalendarMode::Month },
                        events: events.clone(),
                        on_select: move |d: DateValue| selected.set(Some(d)),
                    }

                    // Detail panel showing selected date + its events.
                    div { class: "space-y-3 min-w-[200px]",
                        EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Selected" }
                        EqText { variant: TextVariant::Body, "{sel_display}" }

                        if !selected_events.is_empty() {
                            div { class: "space-y-2 mt-2",
                                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Events" }
                                for evt_label in selected_events.iter() {
                                    div {
                                        class: "flex items-center gap-2 px-3 py-2 rounded-md \
                                               border border-[var(--color-card-border)] \
                                               bg-[var(--color-primary-dark)]/30",
                                        span {
                                            class: "{s::EVENT_DOT} {s::EVENT_DOT_DEFAULT}",
                                        }
                                        EqText { variant: TextVariant::Body, "{evt_label}" }
                                    }
                                }
                            }
                        }

                        if selected().is_some() && selected_events.is_empty() {
                            EqText { variant: TextVariant::Muted, "No events on this date." }
                        }
                    }
                }
            }

            StyleInfo { file: "eq_calendar_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery ───────────────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqCalendar() -> Element {
    let mut date1 = use_signal(|| Some(DateValue::new(2026, 5, 4)));
    let mut date2 = use_signal(|| None::<DateValue>);
    let mut date3 = use_signal(|| Some(DateValue::new(2026, 5, 4)));

    let sample_events = vec![
        CalendarEvent::new(DateValue::new(2026, 5, 4), "Standup"),
        CalendarEvent::new(DateValue::new(2026, 5, 4), "Review")
            .color(EventColor::Info),
        CalendarEvent::new(DateValue::new(2026, 5, 12), "Deploy")
            .color(EventColor::Success),
        CalendarEvent::new(DateValue::new(2026, 5, 20), "Deadline")
            .color(EventColor::Danger),
    ];

    let week_events = vec![
        CalendarEvent::new(DateValue::new(2026, 5, 5), "Team offsite")
            .color(EventColor::Info),
        CalendarEvent::timed(DateValue::new(2026, 5, 4), "Standup", 9, 0, 9, 30),
        CalendarEvent::timed(DateValue::new(2026, 5, 4), "Design sync", 14, 0, 15, 30)
            .color(EventColor::Info),
        CalendarEvent::timed(DateValue::new(2026, 5, 5), "Sprint review", 10, 0, 11, 0)
            .color(EventColor::Success),
        CalendarEvent::timed(DateValue::new(2026, 5, 6), "Lunch talk", 12, 0, 13, 0)
            .color(EventColor::Warning),
        CalendarEvent::timed(DateValue::new(2026, 5, 7), "Deploy", 16, 0, 18, 0)
            .color(EventColor::Danger),
    ];

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Calendar Gallery" }

                div { class: "flex items-start gap-4 flex-wrap",
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Month — with events" }
                        EqCalendar {
                            selected: date1(),
                            events: sample_events,
                            on_select: move |d: DateValue| date1.set(Some(d)),
                        }
                    }
                    div { class: "space-y-1",
                        EqText { variant: TextVariant::Muted, "Month — no outside days" }
                        EqCalendar {
                            selected: date2(),
                            show_outside_days: false,
                            on_select: move |d: DateValue| date2.set(Some(d)),
                        }
                    }
                }

                div { class: "space-y-1 mt-4",
                    EqText { variant: TextVariant::Muted, "Week mode — timed events" }
                    EqCalendar {
                        selected: date3(),
                        mode: CalendarMode::Week,
                        events: week_events,
                        on_select: move |d: DateValue| date3.set(Some(d)),
                    }
                }
            }
        }
    }
}
