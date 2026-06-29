use chrono::{Datelike, FixedOffset, Local, Months, NaiveDate, TimeZone, Utc};
use leptos::prelude::*;

use crate::form::{build_month_grid, DatetimeTimezone, OptionBind};

const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

/// Headless calendar control with month navigation and day selection.
#[component(transparent)]
pub fn BaseCalendar(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] value: OptionBind<i64>,
    #[prop(into)] timezone: Signal<DatetimeTimezone>,
) -> impl IntoView {
    let value = StoredValue::new(value);
    let initial_date = value
        .get_value()
        .get_untracked()
        .and_then(|secs| date_from_unix(secs, timezone.get_untracked()))
        .unwrap_or_else(|| today_for_timezone(timezone.get_untracked()));
    let show_date = RwSignal::new(initial_date);

    Effect::new(move |_| {
        if let Some(selected) = value.get_value().get() {
            if let Some(selected_date) = date_from_unix(selected, timezone.get()) {
                show_date.update(|current| {
                    if current.year() != selected_date.year()
                        || current.month() != selected_date.month()
                    {
                        *current = selected_date;
                    }
                });
            }
        }
    });

    let month_grid = Memo::new(move |_| {
        let current = show_date.get();
        build_month_grid(current.year(), current.month())
    });

    let previous_month = move |_| {
        show_date.update(|date| {
            if let Some(prev) = date.checked_sub_months(Months::new(1)) {
                *date = prev;
            }
        });
    };

    let today = move |_| {
        let now = today_for_timezone(timezone.get_untracked());
        show_date.set(now);
        if let Some(unix_secs) = start_of_day_unix(now, timezone.get_untracked()) {
            value.with_value(|v| v.set(Some(unix_secs)));
        }
    };

    let next_month = move |_| {
        show_date.update(|date| {
            if let Some(next) = date.checked_add_months(Months::new(1)) {
                *date = next;
            }
        });
    };

    view! {
        <div class=move || {
            let mut parts = vec!["orbital-calendar".to_string()];
            if let Some(extra) = class.get() {
                if !extra.is_empty() {
                    parts.push(extra);
                }
            }
            parts.join(" ")
        }>
            <div class="orbital-calendar__header">
                <span class="orbital-calendar__header-title">
                    {move || {
                        show_date.with(|date| date.format("%B %Y").to_string())
                    }}
                </span>
                <div class="orbital-calendar__nav">
                    <button
                        class="orbital-calendar__nav-button"
                        type="button"
                        on:click=previous_month
                        aria-label="Previous month"
                    >
                        "Prev"
                    </button>
                    <button
                        class="orbital-calendar__nav-button"
                        type="button"
                        on:click=today
                        aria-label="Go to today"
                    >
                        "Today"
                    </button>
                    <button
                        class="orbital-calendar__nav-button"
                        type="button"
                        on:click=next_month
                        aria-label="Next month"
                    >
                        "Next"
                    </button>
                </div>
            </div>

            <div class="orbital-calendar__dates">
                {move || {
                    let selected_unix = value.get_value().get();
                    let tz = timezone.get();
                    let today = today_for_timezone(tz);
                    month_grid
                        .get()
                        .into_iter()
                        .enumerate()
                        .map(|(index, day)| {
                            let day_num = day.date.day();
                            let is_other_month = day.kind != crate::form::GridDayKind::Current;
                            let is_today = day.date == today;
                            let is_selected = selected_unix
                                .and_then(|secs| date_from_unix(secs, tz))
                                .map(|selected_date| selected_date == day.date)
                                .unwrap_or(false);
                            let on_click = {
                                let date = day.date;
                                move |_| {
                                    show_date.set(date);
                                    if let Some(unix_secs) = start_of_day_unix(date, tz) {
                                        value.with_value(|v| v.set(Some(unix_secs)));
                                    }
                                }
                            };

                            view! {
                                <div
                                    class="orbital-calendar-item"
                                    class=("orbital-calendar-item--other-month", is_other_month)
                                    class=("orbital-calendar-item--today", is_today)
                                    class=("orbital-calendar-item--selected", is_selected)
                                    role="button"
                                    tabindex="0"
                                    on:click=on_click
                                >
                                    <div class="orbital-calendar-item__header">
                                        <span class="orbital-calendar-item__header-day">{day_num}</span>
                                        {if index < 7 {
                                            Some(view! {
                                                <span class="orbital-calendar-item__header-title">
                                                    {WEEKDAYS[index]}
                                                </span>
                                            })
                                        } else {
                                            None
                                        }}
                                    </div>
                                    <div class="orbital-calendar-item__bar"></div>
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </div>
    }
}

fn date_from_unix(secs: i64, timezone: DatetimeTimezone) -> Option<NaiveDate> {
    match timezone {
        DatetimeTimezone::Local => Local
            .timestamp_opt(secs, 0)
            .single()
            .map(|dt| dt.date_naive()),
        DatetimeTimezone::Utc => Utc
            .timestamp_opt(secs, 0)
            .single()
            .map(|dt| dt.date_naive()),
        DatetimeTimezone::FixedOffset(offset_secs) => FixedOffset::east_opt(offset_secs)?
            .timestamp_opt(secs, 0)
            .single()
            .map(|dt| dt.date_naive()),
    }
}

fn today_for_timezone(timezone: DatetimeTimezone) -> NaiveDate {
    match timezone {
        DatetimeTimezone::Local => Local::now().date_naive(),
        DatetimeTimezone::Utc => Utc::now().date_naive(),
        DatetimeTimezone::FixedOffset(offset_secs) => FixedOffset::east_opt(offset_secs)
            .map(|offset| Utc::now().with_timezone(&offset).date_naive())
            .unwrap_or_else(|| Utc::now().date_naive()),
    }
}

fn start_of_day_unix(date: NaiveDate, timezone: DatetimeTimezone) -> Option<i64> {
    let naive = date.and_hms_opt(0, 0, 0)?;
    match timezone {
        DatetimeTimezone::Local => Local
            .from_local_datetime(&naive)
            .single()
            .map(|dt| dt.timestamp()),
        DatetimeTimezone::Utc => Some(Utc.from_utc_datetime(&naive).timestamp()),
        DatetimeTimezone::FixedOffset(offset_secs) => FixedOffset::east_opt(offset_secs)?
            .from_local_datetime(&naive)
            .single()
            .map(|dt| dt.timestamp()),
    }
}
