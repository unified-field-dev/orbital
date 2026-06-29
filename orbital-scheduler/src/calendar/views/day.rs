//! Day view layout shell with timed event grid.

use leptos::prelude::*;
use orbital_base_components::{DatetimeTimezone, OrbitalDateTime};

use super::timed_grid::{day_header_label, grid_columns, TimedGridBody};
use crate::calendar::resources::SchedulerResourceHeaderCell;
use crate::{PlannedEvent, ScheduleResource};

#[component]
pub fn DayViewShell(
    visible_date: RwSignal<OrbitalDateTime>,
    events: Signal<Vec<PlannedEvent>>,
    resources: RwSignal<Vec<ScheduleResource>>,
    display_timezone: RwSignal<DatetimeTimezone>,
) -> impl IntoView {
    view! {
        <div class="orb-scheduler-view" data-testid="scheduler-calendar-view-day">
            {move || {
                let tz = display_timezone.get();
                let zoned = OrbitalDateTime::from_instant(visible_date.get().instant(), tz);
                let day = zoned.wall_date();
                let days = day.into_iter().collect::<Vec<_>>();
                let has_resources = !resources.get().is_empty();
                let columns = grid_columns(has_resources, days.len().max(1));
                let header_label = day
                    .map(|d| day_header_label(d, tz))
                    .unwrap_or_default();
                view! {
                    <>
                        <div
                            class="orb-scheduler-view__header-row"
                            style=format!("grid-template-columns: {};", columns)
                        >
                            <div class="orb-scheduler-view__time-gutter" />
                            {if has_resources { view! { <SchedulerResourceHeaderCell /> }.into_any() } else { ().into_any() }}
                            <div class="orb-scheduler-view__day-header">{header_label}</div>
                        </div>
                        <TimedGridBody
                            days=days
                            events=events
                            resources=resources
                            display_tz=display_timezone
                        />
                    </>
                }
            }}
        </div>
    }
}
