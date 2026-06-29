//! Event positioning math for timed grid columns.

use chrono::NaiveDate;
use orbital_base_components::{DatetimeTimezone, OrbitalDateTime};

use crate::PlannedEvent;

/// Minutes in a calendar day for percentage layout.
pub const MINUTES_PER_DAY: f64 = 1440.0;
/// Minimum visible block height (~36 minutes).
pub const MIN_EVENT_HEIGHT_PCT: f64 = 2.5;

/// Percentage-based layout rect within a day column.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EventLayoutRect {
    pub top_pct: f64,
    pub height_pct: f64,
}

fn rezone(dt: OrbitalDateTime, display_tz: DatetimeTimezone) -> OrbitalDateTime {
    OrbitalDateTime::from_instant(dt.instant(), display_tz)
}

fn minutes_from_midnight(dt: OrbitalDateTime) -> Option<f64> {
    let (h, m, s) = dt.hour_minute_second()?;
    Some(h as f64 * 60.0 + m as f64 + s as f64 / 60.0)
}

fn day_bounds(
    day: NaiveDate,
    display_tz: DatetimeTimezone,
) -> Option<(OrbitalDateTime, OrbitalDateTime)> {
    let start = OrbitalDateTime::from_naive_date(day, display_tz)?;
    let end = start.apply_hms(23, 59, 59)?;
    Some((start, end))
}

/// Whether the event intersects the given calendar day in `display_tz`.
pub fn event_overlaps_day(
    event: &PlannedEvent,
    day: NaiveDate,
    display_tz: DatetimeTimezone,
) -> bool {
    event_layout_on_day(event, day, display_tz).is_some()
}

/// Wall-clock start/end in `display_tz`, clipped to `day`, as percentage rect.
pub fn event_layout_on_day(
    event: &PlannedEvent,
    day: NaiveDate,
    display_tz: DatetimeTimezone,
) -> Option<EventLayoutRect> {
    let start = rezone(event.start, display_tz);
    let end = rezone(event.end, display_tz);
    let (day_start, day_end) = day_bounds(day, display_tz)?;

    if end <= day_start || start > day_end {
        return None;
    }

    let clip_start = if start < day_start { day_start } else { start };
    let clip_end = if end > day_end { day_end } else { end };

    let start_minutes = minutes_from_midnight(clip_start)?;
    let end_minutes = minutes_from_midnight(clip_end)?;
    let duration = (end_minutes - start_minutes).max(0.0);

    let mut top_pct = (start_minutes / MINUTES_PER_DAY) * 100.0;
    let mut height_pct = (duration / MINUTES_PER_DAY * 100.0).max(MIN_EVENT_HEIGHT_PCT);

    if top_pct + height_pct > 100.0 {
        height_pct = (100.0 - top_pct).max(MIN_EVENT_HEIGHT_PCT);
    }
    if top_pct + height_pct > 100.0 {
        top_pct = (100.0 - height_pct).max(0.0);
    }

    Some(EventLayoutRect {
        top_pct,
        height_pct,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_base_components::TryFromUnixSeconds;

    fn day_anchor() -> OrbitalDateTime {
        OrbitalDateTime::try_from_unix_seconds(1_735_689_600_i64, DatetimeTimezone::Utc)
            .expect("valid")
            .start_of_day()
    }

    fn event_at(start_hms: (u32, u32, u32), end_hms: (u32, u32, u32)) -> PlannedEvent {
        let base = day_anchor();
        let start = base
            .apply_hms(start_hms.0, start_hms.1, start_hms.2)
            .expect("start");
        let end = base
            .apply_hms(end_hms.0, end_hms.1, end_hms.2)
            .expect("end");
        PlannedEvent::new("evt", "Test", start, end)
    }

    #[test]
    fn nine_to_ten_am_layout() {
        let event = event_at((9, 0, 0), (10, 0, 0));
        let day = day_anchor().wall_date().unwrap();
        let rect = event_layout_on_day(&event, day, DatetimeTimezone::Utc).expect("layout");
        assert!((rect.top_pct - 37.5).abs() < 0.01);
        assert!((rect.height_pct - (100.0 / 24.0)).abs() < 0.01);
    }

    #[test]
    fn zero_duration_uses_min_height() {
        let event = event_at((9, 0, 0), (9, 0, 0));
        let day = day_anchor().wall_date().unwrap();
        let rect = event_layout_on_day(&event, day, DatetimeTimezone::Utc).expect("layout");
        assert!((rect.height_pct - MIN_EVENT_HEIGHT_PCT).abs() < 0.01);
    }

    #[test]
    fn event_on_other_day_returns_none() {
        let event = event_at((9, 0, 0), (10, 0, 0));
        let day = day_anchor()
            .wall_date()
            .unwrap()
            .checked_add_days(chrono::Days::new(3))
            .unwrap();
        assert!(event_layout_on_day(&event, day, DatetimeTimezone::Utc).is_none());
    }
}
