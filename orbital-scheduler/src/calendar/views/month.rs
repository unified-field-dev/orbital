//! Month view layout shell.

use chrono::Datelike;
use leptos::prelude::*;
use orbital_base_components::OrbitalDateTime;

use orbital_base_components::DatetimeTimezone;

use super::helpers::events_on_day;
use crate::PlannedEvent;

#[component]
pub fn MonthViewShell(
    visible_date: RwSignal<OrbitalDateTime>,
    events: Signal<Vec<PlannedEvent>>,
    display_timezone: RwSignal<DatetimeTimezone>,
) -> impl IntoView {
    view! {
        <div class="orb-scheduler-view" data-testid="scheduler-calendar-view-month">
            <div class="orb-scheduler-view__month-grid">
                {move || {
                    let tz = display_timezone.get();
                    let zoned = OrbitalDateTime::from_instant(visible_date.get().instant(), tz);
                    let Some(anchor) = zoned.wall_date() else {
                        return ().into_any();
                    };
                    let month_start = anchor
                        .with_day(1)
                        .unwrap_or(anchor);
                    let grid_start = super::super::navigation::start_of_week(month_start);
                    (0..42)
                        .filter_map(|offset| {
                            grid_start.checked_add_days(chrono::Days::new(offset))
                        })
                        .map(|day| {
                            let in_month = day.month() == month_start.month();
                            let day_events = events_on_day(&events.get(), day, tz);
                            let cell_class = if in_month {
                                "orb-scheduler-view__month-cell"
                            } else {
                                "orb-scheduler-view__month-cell orb-scheduler-view__month-cell--outside"
                            };
                            view! {
                                <div class=cell_class>
                                    <div>{day.day()}</div>
                                    <div class="orb-scheduler-view__cell-events">
                                        {day_events
                                            .into_iter()
                                            .take(2)
                                            .map(|event| {
                                                view! {
                                                    <div class="orb-scheduler-event">
                                                        {event.title}
                                                    </div>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()
                        .into_any()
                }}
            </div>
        </div>
    }
}
