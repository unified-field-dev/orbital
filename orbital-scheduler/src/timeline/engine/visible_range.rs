//! Visible time range and column ticks from navigation anchor + preset.

use chrono::{Datelike, NaiveDate};
use orbital_base_components::{
    format_datetime, DatetimeFormat, DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds,
};

use crate::start_of_week;
use crate::DateTimeRange;
use crate::TimelinePreset;

/// Hours shown in the single-day preset.
pub const DAY_PRESET_HOURS: usize = 24;
/// Days shown in the single-week preset.
pub const WEEK_PRESET_DAYS: usize = 7;
/// Hours shown in the business-day preset (08:00–18:00).
pub const BUSINESS_DAY_HOURS: usize = 10;
/// Hour of day when the business-day window starts.
pub const BUSINESS_DAY_START_HOUR: i64 = 8;
/// Weekdays shown in the business-week preset.
pub const BUSINESS_WEEK_DAYS: usize = 5;
/// Width reserved for the resource label column (matches default 8rem CSS).
pub const DEFAULT_RESOURCE_COLUMN_WIDTH_PX: f64 = 128.0;

/// Fixed pixel width per column for virtual scroll math.
pub fn preset_column_width(preset: TimelinePreset) -> f64 {
    match preset {
        TimelinePreset::Day | TimelinePreset::BusinessDay => 56.0,
        TimelinePreset::Week | TimelinePreset::BusinessWeek => 72.0,
    }
}

/// Number of time columns shown for a preset zoom level.
pub fn preset_column_count(preset: TimelinePreset) -> usize {
    match preset {
        TimelinePreset::Day => DAY_PRESET_HOURS,
        TimelinePreset::BusinessDay => BUSINESS_DAY_HOURS,
        TimelinePreset::Week => WEEK_PRESET_DAYS,
        TimelinePreset::BusinessWeek => BUSINESS_WEEK_DAYS,
    }
}

/// One tick in the timeline time header.
#[derive(Clone, Debug, PartialEq)]
pub struct TimelineColumn {
    pub start: OrbitalDateTime,
    pub label: String,
    pub width_px: f64,
}

/// Query window and time columns for a timeline preset.
#[derive(Clone, Debug, PartialEq)]
pub struct TimelineVisibleRange {
    pub query: DateTimeRange,
    pub range_start: OrbitalDateTime,
    pub range_end: OrbitalDateTime,
    pub columns: Vec<TimelineColumn>,
}

fn rezone(anchor: OrbitalDateTime, display_tz: DatetimeTimezone) -> OrbitalDateTime {
    OrbitalDateTime::from_instant(anchor.instant(), display_tz)
}

fn orbital_from_date(date: NaiveDate, timezone: DatetimeTimezone) -> Option<OrbitalDateTime> {
    OrbitalDateTime::from_naive_date(date, timezone)
}

fn add_hours(dt: OrbitalDateTime, hours: i64) -> Option<OrbitalDateTime> {
    use orbital_base_components::ToUnixSeconds;
    OrbitalDateTime::try_from_unix_seconds(dt.to_unix_seconds() + hours * 3600, dt.timezone()).ok()
}

fn add_days(dt: OrbitalDateTime, days: i64) -> Option<OrbitalDateTime> {
    use orbital_base_components::ToUnixSeconds;
    OrbitalDateTime::try_from_unix_seconds(dt.to_unix_seconds() + days * 86_400, dt.timezone()).ok()
}

/// Monday of the calendar week containing `date`.
pub fn start_of_business_week(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday().num_days_from_monday();
    date.checked_sub_days(chrono::Days::new(weekday as u64))
        .unwrap_or(date)
}

/// Align the navigation anchor to the start of the visible window for a preset.
pub fn align_anchor_to_window(
    anchor: OrbitalDateTime,
    preset: TimelinePreset,
    display_tz: DatetimeTimezone,
) -> Option<OrbitalDateTime> {
    let zoned = rezone(anchor, display_tz);
    let current = zoned.wall_date()?;

    match preset {
        TimelinePreset::Day => Some(zoned.start_of_day()),
        TimelinePreset::BusinessDay => add_hours(zoned.start_of_day(), BUSINESS_DAY_START_HOUR),
        TimelinePreset::Week => orbital_from_date(start_of_week(current), display_tz),
        TimelinePreset::BusinessWeek => {
            orbital_from_date(start_of_business_week(current), display_tz)
        }
    }
}

fn column_start_at(
    window_start: OrbitalDateTime,
    preset: TimelinePreset,
    index: usize,
) -> Option<OrbitalDateTime> {
    match preset {
        TimelinePreset::Day | TimelinePreset::BusinessDay => add_hours(window_start, index as i64),
        TimelinePreset::Week | TimelinePreset::BusinessWeek => add_days(window_start, index as i64),
    }
}

fn column_label(start: OrbitalDateTime, preset: TimelinePreset, ampm: bool) -> String {
    match preset {
        TimelinePreset::Day | TimelinePreset::BusinessDay => {
            let format = if ampm {
                DatetimeFormat::Time12
            } else {
                DatetimeFormat::Time24
            };
            format_datetime(start, format)
        }
        TimelinePreset::Week | TimelinePreset::BusinessWeek => {
            format_datetime(start, DatetimeFormat::IsoDate)
        }
    }
}

fn range_end_from_columns(
    window_start: OrbitalDateTime,
    preset: TimelinePreset,
    count: usize,
) -> Option<OrbitalDateTime> {
    let next_window = column_start_at(window_start, preset, count)?;
    use orbital_base_components::ToUnixSeconds;
    OrbitalDateTime::try_from_unix_seconds(
        next_window.to_unix_seconds() - 1,
        window_start.timezone(),
    )
    .ok()
}

/// Compute the visible query range and time columns for the active preset.
pub fn timeline_visible_range(
    anchor: OrbitalDateTime,
    preset: TimelinePreset,
    display_tz: DatetimeTimezone,
    ampm: bool,
) -> Option<TimelineVisibleRange> {
    let range_start = align_anchor_to_window(anchor, preset, display_tz)?;
    let count = preset_column_count(preset);
    let width = preset_column_width(preset);
    let range_end = range_end_from_columns(range_start, preset, count)?;

    let mut columns = Vec::with_capacity(count);
    for i in 0..count {
        let start = column_start_at(range_start, preset, i)?;
        columns.push(TimelineColumn {
            label: column_label(start, preset, ampm),
            start,
            width_px: width,
        });
    }

    Some(TimelineVisibleRange {
        query: DateTimeRange::new(range_start, range_end),
        range_start,
        range_end,
        columns,
    })
}

/// Total content width in pixels for all columns.
pub fn timeline_total_width(range: &TimelineVisibleRange) -> f64 {
    range.columns.iter().map(|c| c.width_px).sum()
}

/// Stretch or shrink columns so the full preset fits the time scrollport width.
pub fn fit_timeline_columns_to_viewport(range: &mut TimelineVisibleRange, time_area_width_px: f64) {
    let count = range.columns.len();
    if count == 0 || time_area_width_px <= 0.0 {
        return;
    }

    let fitted = time_area_width_px / count as f64;
    for col in &mut range.columns {
        col.width_px = fitted;
    }
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
    fn day_preset_is_single_day_hourly() {
        let range =
            timeline_visible_range(anchor(), TimelinePreset::Day, DatetimeTimezone::Utc, true)
                .expect("range");
        assert_eq!(range.columns.len(), DAY_PRESET_HOURS);
        assert_eq!(range.range_start, anchor());
        let end_day = range.range_end.wall_date().expect("end day");
        assert_eq!(end_day, anchor().wall_date().expect("start day"));
    }

    #[test]
    fn business_day_preset_is_eight_am_to_six_pm() {
        let range = timeline_visible_range(
            anchor(),
            TimelinePreset::BusinessDay,
            DatetimeTimezone::Utc,
            true,
        )
        .expect("range");
        assert_eq!(range.columns.len(), BUSINESS_DAY_HOURS);
        assert_eq!(range.range_start.hour_minute_second(), Some((8, 0, 0)));
        assert_eq!(range.columns.first().unwrap().label, "08:00 AM");
        assert_eq!(range.columns.last().unwrap().label, "05:00 PM");
    }

    #[test]
    fn week_preset_spans_seven_days() {
        let range =
            timeline_visible_range(anchor(), TimelinePreset::Week, DatetimeTimezone::Utc, true)
                .expect("range");
        assert_eq!(range.columns.len(), WEEK_PRESET_DAYS);
    }

    #[test]
    fn business_week_preset_spans_five_weekdays() {
        let range = timeline_visible_range(
            anchor(),
            TimelinePreset::BusinessWeek,
            DatetimeTimezone::Utc,
            true,
        )
        .expect("range");
        assert_eq!(range.columns.len(), BUSINESS_WEEK_DAYS);
        let start = range.range_start.wall_date().expect("start");
        assert_eq!(start.weekday(), chrono::Weekday::Mon);
    }

    #[test]
    fn align_week_starts_on_sunday() {
        let aligned = align_anchor_to_window(anchor(), TimelinePreset::Week, DatetimeTimezone::Utc)
            .expect("aligned");
        let date = aligned.wall_date().expect("date");
        assert_eq!(date, start_of_week(anchor().wall_date().unwrap()));
    }

    #[test]
    fn align_business_week_starts_on_monday() {
        let aligned = align_anchor_to_window(
            anchor(),
            TimelinePreset::BusinessWeek,
            DatetimeTimezone::Utc,
        )
        .expect("aligned");
        let date = aligned.wall_date().expect("date");
        assert_eq!(date, start_of_business_week(anchor().wall_date().unwrap()));
    }

    #[test]
    fn day_column_labels_are_times() {
        let range =
            timeline_visible_range(anchor(), TimelinePreset::Day, DatetimeTimezone::Utc, true)
                .expect("range");
        assert!(range.columns[9].label.contains(':'));
        assert!(!range.columns[9].label.contains('-'));
    }

    #[test]
    fn day_preset_includes_full_day_through_23_00() {
        let range =
            timeline_visible_range(anchor(), TimelinePreset::Day, DatetimeTimezone::Utc, true)
                .expect("range");
        assert_eq!(range.columns.len(), DAY_PRESET_HOURS);
        assert_eq!(range.columns.first().unwrap().label, "12:00 AM");
        assert_eq!(range.columns.last().unwrap().label, "11:00 PM");
    }

    #[test]
    fn fit_columns_use_available_time_area() {
        let mut range =
            timeline_visible_range(anchor(), TimelinePreset::Day, DatetimeTimezone::Utc, true)
                .expect("range");
        fit_timeline_columns_to_viewport(&mut range, 800.0);
        assert!((timeline_total_width(&range) - 800.0).abs() < 0.01);
        assert!((range.columns[0].width_px - 800.0 / 24.0).abs() < 0.01);
    }
}
