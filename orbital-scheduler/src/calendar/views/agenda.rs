//! Agenda view layout shell.

use leptos::prelude::*;
use orbital_base_components::{format_datetime, DatetimeFormat, DatetimeTimezone, OrbitalDateTime};
use orbital_core_components::Text;

use super::helpers::events_in_week;
use crate::PlannedEvent;
use crate::{render_agenda_event_row, use_scheduler_chrome, use_scheduler_interaction};

fn format_in_display_tz(
    dt: OrbitalDateTime,
    display_tz: DatetimeTimezone,
    format: DatetimeFormat,
) -> String {
    let zoned = OrbitalDateTime::from_instant(dt.instant(), display_tz);
    format_datetime(zoned, format)
}

#[component]
pub fn AgendaViewShell(
    visible_date: RwSignal<OrbitalDateTime>,
    events: Signal<Vec<PlannedEvent>>,
    display_timezone: RwSignal<DatetimeTimezone>,
) -> impl IntoView {
    let chrome = use_scheduler_chrome();
    let ctx = use_scheduler_interaction();
    view! {
        <div class="orb-scheduler-view" data-testid="scheduler-calendar-view-agenda">
            <div class="orb-scheduler-view__agenda-list">
                {move || {
                    let tz = display_timezone.get();
                    let layout = chrome
                        .and_then(|c| c.preferences.try_week_layout())
                        .unwrap_or_default();
                    let week_events =
                        events_in_week(&events.get(), visible_date.get(), tz, layout);
                    if week_events.is_empty() {
                        return view! { <Text>"No planned events in this range."</Text> }.into_any();
                    }
                    week_events
                        .into_iter()
                        .map(|event| {
                            let start = format_in_display_tz(event.start, tz, DatetimeFormat::IsoDate);
                            let end = format_in_display_tz(event.end, tz, DatetimeFormat::Time24);
                            let title = event.title.clone();
                            view! {
                                <div class="orb-scheduler-view__agenda-item">
                                    {ctx.renderers.with_value(|renderers| {
                                        render_agenda_event_row(renderers, &event, || {
                                            view! {
                                                <>
                                                    <Text>{title.clone()}</Text>
                                                    <Text>{start.clone()} " → " {end.clone()}</Text>
                                                </>
                                            }
                                            .into_any()
                                        })
                                    })}
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
