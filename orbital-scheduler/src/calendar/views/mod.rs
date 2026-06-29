//! Calendar view layout shells and dispatcher (SC-10).

mod agenda;
mod day;
mod helpers;
mod month;
mod timed_grid;
mod week;

use leptos::prelude::*;
use orbital_base_components::DatetimeTimezone;

use crate::SchedulerView;
use crate::{PlannedEvent, ScheduleResource};

pub use agenda::AgendaViewShell;
pub use day::DayViewShell;
pub use month::MonthViewShell;
pub use week::WeekViewShell;

/// Dispatches to the active calendar view shell.
#[component]
pub fn SchedulerViewBody(
    view: RwSignal<SchedulerView>,
    visible_date: RwSignal<orbital_base_components::OrbitalDateTime>,
    events: Signal<Vec<PlannedEvent>>,
    resources: RwSignal<Vec<ScheduleResource>>,
    display_timezone: RwSignal<DatetimeTimezone>,
) -> impl IntoView {
    view! {
        {move || match view.get() {
            SchedulerView::Day => view! {
                <DayViewShell
                    visible_date=visible_date
                    events=events
                    resources=resources
                    display_timezone=display_timezone
                />
            }.into_any(),
            SchedulerView::Week => view! {
                <WeekViewShell
                    visible_date=visible_date
                    events=events
                    resources=resources
                    display_timezone=display_timezone
                />
            }.into_any(),
            SchedulerView::Month => view! {
                <MonthViewShell
                    visible_date=visible_date
                    events=events
                    display_timezone=display_timezone
                />
            }.into_any(),
            SchedulerView::Agenda => view! {
                <AgendaViewShell
                    visible_date=visible_date
                    events=events
                    display_timezone=display_timezone
                />
            }.into_any(),
        }}
    }
}
