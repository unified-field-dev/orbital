//! Week grid helpers for calendar views.

use chrono::NaiveDate;
use orbital_base_components::{DatetimeTimezone, OrbitalDateTime};

use crate::calendar::engine::event_overlaps_day;
use crate::calendar::navigation::start_of_week_for;
use crate::PlannedEvent;
use crate::WeekLayoutPrefs;

fn rezone(dt: OrbitalDateTime, display_tz: DatetimeTimezone) -> OrbitalDateTime {
    OrbitalDateTime::from_instant(dt.instant(), display_tz)
}

/// Events that intersect the given calendar day in `display_tz`.
pub fn events_on_day(
    events: &[PlannedEvent],
    day: NaiveDate,
    display_tz: DatetimeTimezone,
) -> Vec<PlannedEvent> {
    events
        .iter()
        .filter(|event| event_overlaps_day(event, day, display_tz))
        .cloned()
        .collect()
}

/// Events that intersect any day in the week containing `anchor` (wall clock in `display_tz`).
pub fn events_in_week(
    events: &[PlannedEvent],
    anchor: OrbitalDateTime,
    display_tz: DatetimeTimezone,
    layout: WeekLayoutPrefs,
) -> Vec<PlannedEvent> {
    let zoned = rezone(anchor, display_tz);
    let Some(current) = zoned.wall_date() else {
        return Vec::new();
    };
    let start = start_of_week_for(current, layout.week_starts_on);
    let days: Vec<NaiveDate> = (0..7)
        .filter_map(|offset| start.checked_add_days(chrono::Days::new(offset)))
        .collect();
    events
        .iter()
        .filter(|event| {
            days.iter()
                .any(|day| event_overlaps_day(event, *day, display_tz))
        })
        .cloned()
        .collect()
}

/// Days to show in the week containing `anchor` in `display_tz`.
pub fn week_days(
    anchor: OrbitalDateTime,
    display_tz: DatetimeTimezone,
    layout: WeekLayoutPrefs,
) -> Vec<NaiveDate> {
    let zoned = rezone(anchor, display_tz);
    let Some(day) = zoned.wall_date() else {
        return Vec::new();
    };
    let start = start_of_week_for(day, layout.week_starts_on);
    let count = if layout.show_weekends { 7 } else { 5 };
    (0..count)
        .filter_map(|offset| start.checked_add_days(chrono::Days::new(offset)))
        .collect()
}

/// Hour labels for time-grid shells (wall clock in display timezone).
pub fn hour_label(hour: u32, ampm: bool) -> String {
    if ampm {
        match hour {
            0 => "12 AM".to_string(),
            1..=11 => format!("{hour} AM"),
            12 => "12 PM".to_string(),
            h => format!("{} PM", h - 12),
        }
    } else {
        format!("{hour:02}:00")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_base_components::TryFromIso8601;

    fn utc_late_event() -> PlannedEvent {
        let start =
            OrbitalDateTime::try_from_iso8601("2025-01-01T23:00:00Z", DatetimeTimezone::Utc)
                .expect("valid");
        let end = start.apply_hms(23, 30, 0).expect("valid end");
        PlannedEvent::new("evt-late", "Test".to_string(), start, end)
    }

    #[test]
    fn event_on_utc_day_appears_on_next_day_in_tokyo() {
        let event = utc_late_event();
        let utc_day = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let tokyo = DatetimeTimezone::FixedOffset(9 * 3600);
        let tokyo_day = NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();

        assert!(
            events_on_day(std::slice::from_ref(&event), utc_day, DatetimeTimezone::Utc)
                .contains(&event)
        );
        assert!(events_on_day(&[event], tokyo_day, tokyo)
            .iter()
            .any(|e| e.id == "evt-late"));
    }
}
