//! Horizontal event positioning math for timeline lanes.

use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, ToUnixSeconds};

use crate::timeline::engine::TimelineVisibleRange;
use crate::PlannedEvent;

/// Minimum visible block width as a percentage of the visible range.
pub const MIN_EVENT_WIDTH_PCT: f64 = 0.5;

/// Percentage-based layout rect within a timeline lane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimelineEventLayoutRect {
    pub left_pct: f64,
    pub width_pct: f64,
}

fn rezone(dt: OrbitalDateTime, display_tz: DatetimeTimezone) -> OrbitalDateTime {
    OrbitalDateTime::from_instant(dt.instant(), display_tz)
}

fn span_seconds(range: &TimelineVisibleRange) -> f64 {
    let start = range.range_start.to_unix_seconds();
    let end = range.range_end.to_unix_seconds();
    (end - start).max(1) as f64
}

/// Whether the event intersects the visible timeline range in `display_tz`.
pub fn event_overlaps_range(
    event: &PlannedEvent,
    range: &TimelineVisibleRange,
    display_tz: DatetimeTimezone,
) -> bool {
    event_layout_in_range(event, range, display_tz).is_some()
}

/// Wall-clock start/end in `display_tz`, clipped to the visible range, as percentage rect.
pub fn event_layout_in_range(
    event: &PlannedEvent,
    range: &TimelineVisibleRange,
    display_tz: DatetimeTimezone,
) -> Option<TimelineEventLayoutRect> {
    let start = rezone(event.start, display_tz);
    let end = rezone(event.end, display_tz);

    if end <= range.range_start || start > range.range_end {
        return None;
    }

    let clip_start = if start < range.range_start {
        range.range_start
    } else {
        start
    };
    let clip_end = if end > range.range_end {
        range.range_end
    } else {
        end
    };

    let span = span_seconds(range);
    let offset = (clip_start.to_unix_seconds() - range.range_start.to_unix_seconds()) as f64;
    let duration = (clip_end.to_unix_seconds() - clip_start.to_unix_seconds()).max(0) as f64;

    let mut left_pct = (offset / span) * 100.0;
    let mut width_pct = (duration / span * 100.0).max(MIN_EVENT_WIDTH_PCT);

    if left_pct + width_pct > 100.0 {
        width_pct = (100.0 - left_pct).max(MIN_EVENT_WIDTH_PCT);
    }
    if left_pct + width_pct > 100.0 {
        left_pct = (100.0 - width_pct).max(0.0);
    }

    Some(TimelineEventLayoutRect {
        left_pct,
        width_pct,
    })
}

/// Events assigned to a resource lane within the visible range.
pub fn events_for_lane<'a>(
    events: &'a [PlannedEvent],
    resource_id: &str,
    range: &TimelineVisibleRange,
    display_tz: DatetimeTimezone,
) -> Vec<&'a PlannedEvent> {
    events
        .iter()
        .filter(|event| {
            event.resource_id.as_deref() == Some(resource_id)
                && event_overlaps_range(event, range, display_tz)
        })
        .collect()
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

    fn event_at(start_hms: (u32, u32, u32), end_hms: (u32, u32, u32)) -> PlannedEvent {
        let base = anchor();
        let start = base
            .apply_hms(start_hms.0, start_hms.1, start_hms.2)
            .expect("start");
        let end = base
            .apply_hms(end_hms.0, end_hms.1, end_hms.2)
            .expect("end");
        let mut event = PlannedEvent::new("evt", "Test", start, end);
        event.resource_id = Some("room-a".into());
        event
    }

    #[test]
    fn nine_to_ten_am_layout_on_day_preset() {
        let range = super::super::visible_range::timeline_visible_range(
            anchor(),
            crate::TimelinePreset::Day,
            DatetimeTimezone::Utc,
            true,
        )
        .expect("range");
        let event = event_at((9, 0, 0), (10, 0, 0));
        let rect = event_layout_in_range(&event, &range, DatetimeTimezone::Utc).expect("layout");
        assert!(rect.left_pct > 0.0);
        assert!(rect.width_pct > 0.0);
    }

    #[test]
    fn zero_duration_uses_min_width() {
        let range = super::super::visible_range::timeline_visible_range(
            anchor(),
            crate::TimelinePreset::Day,
            DatetimeTimezone::Utc,
            true,
        )
        .expect("range");
        let event = event_at((9, 0, 0), (9, 0, 0));
        let rect = event_layout_in_range(&event, &range, DatetimeTimezone::Utc).expect("layout");
        assert!((rect.width_pct - MIN_EVENT_WIDTH_PCT).abs() < 0.01);
    }
}
