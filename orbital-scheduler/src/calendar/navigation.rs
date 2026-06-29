//! Calendar navigation engine, imperative handle, and toolbar (SC-07).

use chrono::{Datelike, Local, Months, NaiveDate, Utc};
use leptos::prelude::*;
use orbital_base_components::{format_datetime, DatetimeFormat, DatetimeTimezone, OrbitalDateTime};
use orbital_core_components::{Button, ButtonAppearance, Select, SelectAppearance, Toolbar};
use orbital_style::inject_style;
use orbital_theme::use_theme_options;

use crate::calendar::styles::{scheduler_calendar_styles, scheduler_density_class};
use crate::use_scheduler_chrome;
use crate::SchedulerView;

/// Navigation direction for visible date stepping.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavDirection {
    Next,
    Previous,
}

/// Imperative navigation handle for [`SchedulerCalendar`].
#[derive(Clone)]
pub struct SchedulerCalendarHandle {
    /// Jump to a specific anchor date.
    pub go_to_date: Callback<(OrbitalDateTime,), ()>,
    /// Advance by one step for the current view.
    pub go_to_next: Callback<(), ()>,
    /// Retreat by one step for the current view.
    pub go_to_previous: Callback<(), ()>,
    /// Jump to today in the calendar timezone context.
    pub go_to_today: Callback<(), ()>,
}

/// Start of week (Sunday) for the given calendar day.
pub fn start_of_week(date: NaiveDate) -> NaiveDate {
    start_of_week_for(date, 0)
}

/// Start of week for `week_starts_on`: `0` = Sunday, `1` = Monday.
pub fn start_of_week_for(date: NaiveDate, week_starts_on: u8) -> NaiveDate {
    match week_starts_on {
        1 => {
            let weekday = date.weekday().num_days_from_monday();
            date.checked_sub_days(chrono::Days::new(weekday as u64))
                .unwrap_or(date)
        }
        _ => {
            let weekday = date.weekday().num_days_from_sunday();
            date.checked_sub_days(chrono::Days::new(weekday as u64))
                .unwrap_or(date)
        }
    }
}

/// End of week (Saturday) for the given calendar day.
pub fn end_of_week(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday().num_days_from_sunday();
    date.checked_add_days(chrono::Days::new(6 - weekday as u64))
        .unwrap_or(date)
}

fn today_for_timezone(timezone: DatetimeTimezone) -> NaiveDate {
    match timezone {
        DatetimeTimezone::Local => Local::now().date_naive(),
        DatetimeTimezone::Utc => Utc::now().date_naive(),
        DatetimeTimezone::FixedOffset(offset_secs) => chrono::FixedOffset::east_opt(offset_secs)
            .map(|offset| Utc::now().with_timezone(&offset).date_naive())
            .unwrap_or_else(|| Utc::now().date_naive()),
    }
}

fn date_from_orbital(value: OrbitalDateTime) -> Option<NaiveDate> {
    value.wall_date()
}

fn orbital_from_date(date: NaiveDate, timezone: DatetimeTimezone) -> Option<OrbitalDateTime> {
    OrbitalDateTime::from_naive_date(date, timezone)
}

/// Advance or retreat the navigation anchor for a view.
pub fn advance_visible_date(
    date: OrbitalDateTime,
    view: SchedulerView,
    direction: NavDirection,
) -> Option<OrbitalDateTime> {
    let current = date_from_orbital(date)?;
    let tz = date.timezone();
    let signed_days = match direction {
        NavDirection::Next => 1_i64,
        NavDirection::Previous => -1,
    };

    let next_date = match view {
        SchedulerView::Day => {
            if signed_days > 0 {
                current.checked_add_days(chrono::Days::new(1))
            } else {
                current.checked_sub_days(chrono::Days::new(1))
            }
        }
        SchedulerView::Week | SchedulerView::Agenda => {
            let anchor = start_of_week(current);
            if signed_days > 0 {
                anchor.checked_add_days(chrono::Days::new(7))
            } else {
                anchor.checked_sub_days(chrono::Days::new(7))
            }
        }
        SchedulerView::Month => {
            if signed_days > 0 {
                current.checked_add_months(Months::new(1))
            } else {
                current.checked_sub_months(Months::new(1))
            }
        }
    }?;

    orbital_from_date(next_date, tz)
}

/// Format the toolbar title for the current view and anchor date.
pub fn format_visible_range_label(date: OrbitalDateTime, view: SchedulerView) -> String {
    let Some(current) = date.wall_date() else {
        return String::new();
    };

    match view {
        SchedulerView::Day => format_datetime(date.start_of_day(), DatetimeFormat::IsoDate),
        SchedulerView::Week | SchedulerView::Agenda => {
            let start = start_of_week(current);
            let end = end_of_week(current);
            let start_dt = orbital_from_date(start, date.timezone()).unwrap_or(date);
            let end_dt = orbital_from_date(end, date.timezone()).unwrap_or(date);
            format!(
                "{} – {}",
                format_datetime(start_dt, DatetimeFormat::IsoDate),
                format_datetime(end_dt, DatetimeFormat::IsoDate)
            )
        }
        SchedulerView::Month => {
            format!("{} {}", current.format("%B"), current.year())
        }
    }
}

/// Calendar toolbar with date navigation and view switcher.
#[component]
pub fn SchedulerCalendarToolbar(
    visible_date: RwSignal<OrbitalDateTime>,
    view: RwSignal<SchedulerView>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orb-scheduler-calendar", scheduler_calendar_styles());

    let theme_options = use_theme_options();
    let density_class = move || scheduler_density_class(theme_options.get().density);
    let chrome = use_scheduler_chrome();

    let view_select = RwSignal::new(view.get_untracked().wire_value().to_string());
    Effect::new(move |_| {
        let wire = view.get().wire_value().to_string();
        untrack(|| {
            if view_select.get_untracked() != wire {
                view_select.set(wire);
            }
        });
    });
    Effect::new(move |_| {
        let wire = view_select.get();
        untrack(|| {
            if let Some(next) = SchedulerView::from_wire_value(&wire) {
                if view.get_untracked() != next {
                    view.set(next);
                }
            }
        });
    });

    let go_previous = move |_| {
        if let Some(next) =
            advance_visible_date(visible_date.get(), view.get(), NavDirection::Previous)
        {
            visible_date.set(next);
        }
    };
    let go_next = move |_| {
        if let Some(next) = advance_visible_date(visible_date.get(), view.get(), NavDirection::Next)
        {
            visible_date.set(next);
        }
    };
    let go_today = move |_| {
        let tz = visible_date.get_untracked().timezone();
        if let Some(today) = orbital_from_date(today_for_timezone(tz), tz) {
            visible_date.set(today.start_of_day());
        }
    };

    let title = move || format_visible_range_label(visible_date.get(), view.get());

    let label_today = move || {
        chrome
            .and_then(|c| c.locale_text.try_get().map(|locale| locale.today.clone()))
            .unwrap_or_else(|| "Today".to_string())
    };
    let label_previous = move || {
        chrome
            .and_then(|c| {
                c.locale_text
                    .try_get()
                    .map(|locale| locale.previous.clone())
            })
            .unwrap_or_else(|| "Previous".to_string())
    };
    let label_next = move || {
        chrome
            .and_then(|c| c.locale_text.try_get().map(|locale| locale.next.clone()))
            .unwrap_or_else(|| "Next".to_string())
    };
    let view_labels = move || {
        chrome.and_then(|c| {
            c.locale_text.try_get().map(|locale| {
                SchedulerView::all()
                    .iter()
                    .map(|item| (item.wire_value(), locale.view_label(*item).to_string()))
                    .collect::<Vec<_>>()
            })
        })
    };

    let toolbar_class = move || {
        let mut parts = vec![
            "orb-scheduler-toolbar".to_string(),
            density_class().to_string(),
        ];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    };

    view! {
        <div class=toolbar_class>
            <Toolbar>
                <div class="orb-scheduler-toolbar__nav">
                    <span data-testid="scheduler-calendar-nav-previous">
                        <Button appearance=ButtonAppearance::Subtle on:click=go_previous>
                            {label_previous}
                        </Button>
                    </span>
                    <span data-testid="scheduler-calendar-nav-today">
                        <Button appearance=ButtonAppearance::Subtle on:click=go_today>
                            {label_today}
                        </Button>
                    </span>
                    <span data-testid="scheduler-calendar-nav-next">
                        <Button appearance=ButtonAppearance::Subtle on:click=go_next>
                            {label_next}
                        </Button>
                    </span>
                </div>
                <span class="orb-scheduler-toolbar__title" data-testid="scheduler-calendar-header-title">
                    {title}
                </span>
                <div class="orb-scheduler-toolbar__view" data-testid="scheduler-calendar-view-select">
                    <Select
                        bind=view_select
                        appearance=SelectAppearance {
                            default_value: Some(view.get_untracked().wire_value().to_string()),
                            ..Default::default()
                        }
                    >
                        {move || {
                            let labels = view_labels();
                            SchedulerView::all()
                                .iter()
                                .map(|item| {
                                    let value = item.wire_value();
                                    let label = labels
                                        .as_ref()
                                        .and_then(|pairs| {
                                            pairs.iter().find(|(v, _)| *v == value).map(|(_, l)| l.clone())
                                        })
                                        .unwrap_or_else(|| item.label().to_string());
                                    view! { <option value=value>{label}</option> }
                                })
                                .collect_view()
                        }}
                    </Select>
                </div>
            </Toolbar>
        </div>
    }
}

#[cfg(feature = "preview")]
pub fn preview_anchor_date() -> OrbitalDateTime {
    use orbital_base_components::TryFromUnixSeconds;
    OrbitalDateTime::try_from_unix_seconds(1_735_689_600_i64, DatetimeTimezone::Local)
        .expect("valid anchor")
        .start_of_day()
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_base_components::TryFromUnixSeconds;

    fn anchor() -> OrbitalDateTime {
        OrbitalDateTime::try_from_unix_seconds(1_735_689_600_i64, DatetimeTimezone::Utc)
            .expect("valid")
            .start_of_day()
    }

    #[test]
    fn advance_day_moves_one_day() {
        let next =
            advance_visible_date(anchor(), SchedulerView::Day, NavDirection::Next).expect("next");
        let prev =
            advance_visible_date(next, SchedulerView::Day, NavDirection::Previous).expect("prev");
        assert_eq!(prev, anchor());
    }

    #[test]
    fn advance_week_moves_seven_days_from_week_start() {
        let week_start = orbital_from_date(
            start_of_week(anchor().wall_date().unwrap()),
            DatetimeTimezone::Utc,
        )
        .expect("week start");
        let next = advance_visible_date(week_start, SchedulerView::Week, NavDirection::Next)
            .expect("next");
        let label = format_visible_range_label(next, SchedulerView::Week);
        assert!(!label.is_empty());
        let prev =
            advance_visible_date(next, SchedulerView::Week, NavDirection::Previous).expect("prev");
        assert_eq!(prev.start_of_day(), week_start.start_of_day());
    }

    #[test]
    fn advance_month_changes_month() {
        let next =
            advance_visible_date(anchor(), SchedulerView::Month, NavDirection::Next).expect("next");
        assert_ne!(
            next.wall_date().map(|d| d.month()),
            anchor().wall_date().map(|d| d.month())
        );
    }

    #[test]
    fn labels_differ_by_view() {
        let date = anchor();
        let day = format_visible_range_label(date, SchedulerView::Day);
        let week = format_visible_range_label(date, SchedulerView::Week);
        let month = format_visible_range_label(date, SchedulerView::Month);
        assert!(!day.is_empty());
        assert!(!week.is_empty());
        assert!(!month.is_empty());
        assert_ne!(day, week);
        assert_ne!(week, month);
    }

    #[test]
    fn start_of_week_is_sunday() {
        // 2025-01-01 is Wednesday
        let date = NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid");
        let start = start_of_week(date);
        assert_eq!(start, NaiveDate::from_ymd_opt(2024, 12, 29).expect("valid"));
    }
}
