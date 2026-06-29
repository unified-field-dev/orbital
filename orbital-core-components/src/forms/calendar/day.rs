use std::sync::Arc;

use chrono::NaiveDate;
use leptos::prelude::*;

/// Weekday column headers aligned above the day grid.
#[component]
pub fn CalendarWeekdayHeader(
    /// Optional localized short weekday labels (7 entries, already rotated for first day of week).
    #[prop(optional)]
    labels: Option<[String; 7]>,
) -> impl IntoView {
    let labels = labels
        .unwrap_or_else(|| ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"].map(String::from));
    view! {
        <div class="orbital-calendar__weekdays" role="row">
            {labels.into_iter().map(|label| view! {
                <span class="orbital-calendar__weekday" role="columnheader">{label}</span>
            }).collect_view()}
        </div>
    }
}

/// Props passed to a custom [`Calendar`](super::calendar::Calendar) day cell renderer.
#[derive(Clone)]
pub struct CalendarDayProps {
    /// Calendar date for this cell.
    pub date: NaiveDate,
    /// Day-of-month number displayed in the cell.
    pub day_number: u32,
    /// Reserved for custom renderers; weekday headers render via [`CalendarWeekdayHeader`].
    pub weekday_label: Option<&'static str>,
    /// Whether this day matches the bound selection.
    pub selected: bool,
    /// Whether this day is today in the active timezone.
    pub today: bool,
    /// Whether this day belongs to a month other than the visible month.
    pub other_month: bool,
    /// Whether selection is blocked for this day.
    pub disabled: bool,
    /// Accessible name for the cell.
    pub aria_label: String,
    /// Roving tabindex value (`0` when focused, `-1` otherwise).
    pub tabindex: &'static str,
    /// Invoked when the day is activated (click or keyboard).
    pub on_select: Callback<()>,
    /// Invoked when keyboard focus moves to this cell.
    pub on_focus: Callback<()>,
}

/// Props passed to a custom month button renderer in the DatePicker panel.
#[derive(Clone)]
pub struct CalendarMonthButtonProps {
    /// Month number (`1`..=`12`).
    pub month: u32,
    /// Short month label (`Jan`, `Feb`, …).
    pub label: String,
    /// Whether this month matches the focused month.
    pub selected: bool,
    /// Invoked when the month is chosen.
    pub on_select: Callback<()>,
}

pub type CalendarDayRenderer = Arc<dyn Fn(CalendarDayProps) -> AnyView + Send + Sync>;
pub type CalendarMonthButtonRenderer =
    Arc<dyn Fn(CalendarMonthButtonProps) -> AnyView + Send + Sync>;

/// Default calendar day cell markup matching core calendar styling.
pub fn default_calendar_day(props: CalendarDayProps) -> impl IntoView {
    view! {
        <div
            class="orbital-calendar-item"
            class=("orbital-calendar-item--other-month", props.other_month)
            class=("orbital-calendar-item--today", props.today)
            class=("orbital-calendar-item--selected", props.selected)
            class=("orbital-calendar-item--disabled", props.disabled)
            role="gridcell"
            tabindex=props.tabindex
            aria-label=props.aria_label
            aria-selected=props.selected
            aria-disabled=props.disabled
            on:click=move |_| props.on_select.run(())
            on:focus=move |_| props.on_focus.run(())
        >
            <div class="orbital-calendar-item__header">
                <span class="orbital-calendar-item__header-day">{props.day_number}</span>
            </div>
            <div class="orbital-calendar-item__bar"></div>
        </div>
    }
}

/// Default month button markup for the DatePicker month panel.
pub fn default_calendar_month_button(props: CalendarMonthButtonProps) -> impl IntoView {
    view! {
        <button
            type="button"
            class="orbital-date-picker__month-button"
            class=("orbital-date-picker__month-button--selected", props.selected)
            on:click=move |_| props.on_select.run(())
        >
            {props.label}
        </button>
    }
}
