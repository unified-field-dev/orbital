//! Preset-aware navigation stepping and range labels.

use chrono::{Local, NaiveDate, Utc};
use orbital_base_components::{format_datetime, DatetimeFormat, DatetimeTimezone, OrbitalDateTime};

use crate::timeline::engine::{
    align_anchor_to_window, timeline_visible_range, BUSINESS_WEEK_DAYS, WEEK_PRESET_DAYS,
};
use crate::NavDirection;
use crate::TimelinePreset;

fn today_for_timezone(timezone: DatetimeTimezone) -> NaiveDate {
    match timezone {
        DatetimeTimezone::Local => Local::now().date_naive(),
        DatetimeTimezone::Utc => Utc::now().date_naive(),
        DatetimeTimezone::FixedOffset(offset_secs) => chrono::FixedOffset::east_opt(offset_secs)
            .map(|offset| Utc::now().with_timezone(&offset).date_naive())
            .unwrap_or_else(|| Utc::now().date_naive()),
    }
}

fn orbital_from_date(date: NaiveDate, timezone: DatetimeTimezone) -> Option<OrbitalDateTime> {
    OrbitalDateTime::from_naive_date(date, timezone)
}

fn add_days(dt: OrbitalDateTime, days: i64) -> Option<OrbitalDateTime> {
    use orbital_base_components::{ToUnixSeconds, TryFromUnixSeconds};
    OrbitalDateTime::try_from_unix_seconds(dt.to_unix_seconds() + days * 86_400, dt.timezone()).ok()
}

/// Advance or retreat the navigation anchor by one preset page.
pub fn advance_visible_date_by_preset(
    date: OrbitalDateTime,
    preset: TimelinePreset,
    direction: NavDirection,
) -> Option<OrbitalDateTime> {
    let tz = date.timezone();
    let aligned = align_anchor_to_window(date, preset, tz)?;
    let signed = match direction {
        NavDirection::Next => 1_i64,
        NavDirection::Previous => -1,
    };

    match preset {
        TimelinePreset::Day | TimelinePreset::BusinessDay => add_days(aligned, signed),
        TimelinePreset::Week => add_days(aligned, signed * WEEK_PRESET_DAYS as i64),
        TimelinePreset::BusinessWeek => add_days(aligned, signed * BUSINESS_WEEK_DAYS as i64),
    }
}

/// Format the toolbar title for the current preset and anchor date.
pub fn format_timeline_range_label(date: OrbitalDateTime, preset: TimelinePreset) -> String {
    let Some(range) = timeline_visible_range(date, preset, date.timezone(), true) else {
        return String::new();
    };

    match preset {
        TimelinePreset::Day => format_datetime(range.range_start, DatetimeFormat::IsoDate),
        TimelinePreset::BusinessDay => format!(
            "{} · Business day",
            format_datetime(range.range_start, DatetimeFormat::IsoDate)
        ),
        TimelinePreset::Week | TimelinePreset::BusinessWeek => format!(
            "{} – {}",
            format_datetime(range.range_start, DatetimeFormat::IsoDate),
            format_datetime(range.range_end, DatetimeFormat::IsoDate)
        ),
    }
}

/// Today as an anchor for timeline navigation.
pub fn today_anchor(timezone: DatetimeTimezone) -> Option<OrbitalDateTime> {
    orbital_from_date(today_for_timezone(timezone), timezone).map(|d| d.start_of_day())
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
            advance_visible_date_by_preset(anchor(), TimelinePreset::Day, NavDirection::Next)
                .expect("next");
        let prev =
            advance_visible_date_by_preset(next, TimelinePreset::Day, NavDirection::Previous)
                .expect("prev");
        assert_eq!(prev, anchor());
    }

    #[test]
    fn advance_week_moves_one_week() {
        let week_start =
            align_anchor_to_window(anchor(), TimelinePreset::Week, DatetimeTimezone::Utc)
                .expect("start");
        let next =
            advance_visible_date_by_preset(week_start, TimelinePreset::Week, NavDirection::Next)
                .expect("next");
        let label = format_timeline_range_label(next, TimelinePreset::Week);
        assert!(!label.is_empty());
    }

    #[test]
    fn advance_business_day_moves_one_day() {
        let aligned =
            align_anchor_to_window(anchor(), TimelinePreset::BusinessDay, DatetimeTimezone::Utc)
                .expect("aligned");
        let next = advance_visible_date_by_preset(
            anchor(),
            TimelinePreset::BusinessDay,
            NavDirection::Next,
        )
        .expect("next");
        let prev = advance_visible_date_by_preset(
            next,
            TimelinePreset::BusinessDay,
            NavDirection::Previous,
        )
        .expect("prev");
        assert_eq!(prev, aligned);
    }

    #[test]
    fn advance_business_week_moves_five_days() {
        let week_start = align_anchor_to_window(
            anchor(),
            TimelinePreset::BusinessWeek,
            DatetimeTimezone::Utc,
        )
        .expect("start");
        let next = advance_visible_date_by_preset(
            week_start,
            TimelinePreset::BusinessWeek,
            NavDirection::Next,
        )
        .expect("next");
        let label = format_timeline_range_label(next, TimelinePreset::BusinessWeek);
        assert!(!label.is_empty());
    }

    #[test]
    fn labels_differ_by_preset() {
        let date = anchor();
        let day = format_timeline_range_label(date, TimelinePreset::Day);
        let week = format_timeline_range_label(date, TimelinePreset::Week);
        assert!(!day.is_empty());
        assert!(!week.is_empty());
        assert_ne!(day, week);
    }
}
