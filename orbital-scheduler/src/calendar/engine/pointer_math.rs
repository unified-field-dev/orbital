//! Inverse layout math — pointer position to event start/end times.

use chrono::{Duration, NaiveDate};

use orbital_base_components::{DatetimeTimezone, OrbitalDateTime};

use super::event_layout::MINUTES_PER_DAY;
use crate::PlannedEvent;

/// Grid snap interval in minutes.
pub const SNAP_STEP_MINUTES: f64 = 15.0;
/// Minimum event duration when resizing.
pub const MIN_EVENT_DURATION_MINUTES: f64 = 15.0;
/// Default duration for click-to-create events.
pub const DEFAULT_CREATION_DURATION_MINUTES: f64 = 60.0;

/// Map a clamped vertical ratio within a day column to minutes from midnight.
pub fn minutes_from_column_y(ratio: f64) -> f64 {
    ratio.clamp(0.0, 1.0) * MINUTES_PER_DAY
}

/// Snap minutes to the nearest grid step.
pub fn snap_minutes(minutes: f64, step: f64) -> f64 {
    (minutes / step).round() * step
}

/// Build an [`OrbitalDateTime`] at `minutes` from midnight on `day`.
pub fn datetime_at_minutes(
    day: NaiveDate,
    minutes: f64,
    tz: DatetimeTimezone,
) -> Option<OrbitalDateTime> {
    let minutes = minutes.clamp(0.0, MINUTES_PER_DAY - 1.0);
    let h = (minutes / 60.0).floor() as u32;
    let m = (minutes % 60.0).floor() as u32;
    let s = ((minutes.fract()) * 60.0).round() as u32;
    OrbitalDateTime::from_naive_date(day, tz)?.apply_hms(h, m, s)
}

/// Event duration in minutes (wall-clock instant delta).
pub fn event_duration_minutes(event: &PlannedEvent) -> f64 {
    let secs = (event.end.instant() - event.start.instant()).num_seconds();
    (secs as f64 / 60.0).max(MIN_EVENT_DURATION_MINUTES)
}

/// Move an event to a new day and start time while preserving duration.
pub fn move_event_to(
    event: &PlannedEvent,
    day: NaiveDate,
    start_minutes: f64,
    display_tz: DatetimeTimezone,
) -> Option<(OrbitalDateTime, OrbitalDateTime)> {
    let start_minutes = snap_minutes(start_minutes, SNAP_STEP_MINUTES);
    let start = datetime_at_minutes(day, start_minutes, display_tz)?;
    let duration = event_duration_minutes(event);
    let end_instant = start.instant() + Duration::minutes(duration as i64);
    let end = OrbitalDateTime::from_instant(end_instant, start.timezone());
    Some((start, end))
}

/// Resize the event end to `end_minutes` on `day`, enforcing minimum duration.
pub fn resize_event_end(
    event: &PlannedEvent,
    day: NaiveDate,
    end_minutes: f64,
    display_tz: DatetimeTimezone,
) -> Option<OrbitalDateTime> {
    let end_minutes = snap_minutes(end_minutes, SNAP_STEP_MINUTES);
    let end = datetime_at_minutes(day, end_minutes, display_tz)?;
    let min_end = event.start.instant() + Duration::minutes(MIN_EVENT_DURATION_MINUTES as i64);
    if end.instant() < min_end {
        return Some(OrbitalDateTime::from_instant(
            min_end,
            event.start.timezone(),
        ));
    }
    Some(end)
}

/// Resize the event start to `start_minutes` on `day`, enforcing minimum duration.
pub fn resize_event_start(
    event: &PlannedEvent,
    day: NaiveDate,
    start_minutes: f64,
    display_tz: DatetimeTimezone,
) -> Option<OrbitalDateTime> {
    let start_minutes = snap_minutes(start_minutes, SNAP_STEP_MINUTES);
    let start = datetime_at_minutes(day, start_minutes, display_tz)?;
    let max_start = event.end.instant() - Duration::minutes(MIN_EVENT_DURATION_MINUTES as i64);
    if start.instant() > max_start {
        return Some(OrbitalDateTime::from_instant(
            max_start,
            event.start.timezone(),
        ));
    }
    Some(start)
}

/// Default one-hour block from a snapped click position.
pub fn default_creation_range(
    click_minutes: f64,
    day: NaiveDate,
    tz: DatetimeTimezone,
) -> Option<(OrbitalDateTime, OrbitalDateTime)> {
    let start_minutes = snap_minutes(click_minutes, SNAP_STEP_MINUTES);
    let start = datetime_at_minutes(day, start_minutes, tz)?;
    let end_instant = start.instant() + Duration::minutes(DEFAULT_CREATION_DURATION_MINUTES as i64);
    let end = OrbitalDateTime::from_instant(end_instant, start.timezone());
    Some((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_base_components::TryFromUnixSeconds;

    fn day_anchor() -> (NaiveDate, DatetimeTimezone) {
        let dt = OrbitalDateTime::try_from_unix_seconds(1_735_689_600_i64, DatetimeTimezone::Utc)
            .expect("valid")
            .start_of_day();
        (dt.wall_date().unwrap(), DatetimeTimezone::Utc)
    }

    fn event_at(start_h: u32, end_h: u32) -> PlannedEvent {
        let (day, tz) = day_anchor();
        let base = OrbitalDateTime::from_naive_date(day, tz).unwrap();
        let start = base.apply_hms(start_h, 0, 0).unwrap();
        let end = base.apply_hms(end_h, 0, 0).unwrap();
        PlannedEvent::new("e1", "Test", start, end)
    }

    #[test]
    fn minutes_from_column_y_maps_ratio() {
        assert!((minutes_from_column_y(0.5) - 720.0).abs() < 0.01);
    }

    #[test]
    fn snap_minutes_rounds_to_step() {
        assert_eq!(snap_minutes(47.0, 15.0), 45.0);
        assert_eq!(snap_minutes(53.0, 15.0), 60.0);
    }

    #[test]
    fn move_preserves_duration() {
        let event = event_at(9, 10);
        let (day, tz) = day_anchor();
        let (start, end) = move_event_to(&event, day, 11.0 * 60.0, tz).unwrap();
        assert_eq!(start.hour_minute_second(), Some((11, 0, 0)));
        assert_eq!(end.hour_minute_second(), Some((12, 0, 0)));
    }

    #[test]
    fn default_creation_is_one_hour() {
        let (day, tz) = day_anchor();
        let (start, end) = default_creation_range(9.0 * 60.0, day, tz).unwrap();
        let dur = (end.instant() - start.instant()).num_minutes();
        assert_eq!(dur, 60);
    }
}
