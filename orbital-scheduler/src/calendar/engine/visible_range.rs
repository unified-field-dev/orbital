//! Visible date range from navigation anchor + view mode.

use chrono::{Datelike, NaiveDate};
use orbital_base_components::{DatetimeTimezone, OrbitalDateTime};

use crate::calendar::navigation::{end_of_week, start_of_week};
use crate::DateTimeRange;
use crate::SchedulerView;

/// Query window and grid days for a calendar view.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisibleRange {
    /// Inclusive start/end instants for data-source queries (Phase 5).
    pub query: DateTimeRange,
    /// Calendar days rendered in the grid.
    pub days: Vec<NaiveDate>,
}

fn rezone(anchor: OrbitalDateTime, display_tz: DatetimeTimezone) -> OrbitalDateTime {
    OrbitalDateTime::from_instant(anchor.instant(), display_tz)
}

fn day_start(date: NaiveDate, display_tz: DatetimeTimezone) -> Option<OrbitalDateTime> {
    OrbitalDateTime::from_naive_date(date, display_tz)
}

fn day_end(date: NaiveDate, display_tz: DatetimeTimezone) -> Option<OrbitalDateTime> {
    day_start(date, display_tz)?.apply_hms(23, 59, 59)
}

/// Compute the visible query range and grid days for the active view.
///
/// Wall-clock boundaries use `display_tz` (Phase 4 deepens label wiring; Phase 2 uses
/// `OrbitalDateTime::from_instant` re-zoning).
pub fn visible_range(
    anchor: OrbitalDateTime,
    view: SchedulerView,
    display_tz: DatetimeTimezone,
) -> Option<VisibleRange> {
    let zoned = rezone(anchor, display_tz);
    let current = zoned.wall_date()?;

    let (days, range_start, range_end) = match view {
        SchedulerView::Day => {
            let start = day_start(current, display_tz)?;
            let end = day_end(current, display_tz)?;
            (vec![current], start, end)
        }
        SchedulerView::Week | SchedulerView::Agenda => {
            let week_start = start_of_week(current);
            let week_end = end_of_week(current);
            let days: Vec<NaiveDate> = (0..7)
                .filter_map(|offset| week_start.checked_add_days(chrono::Days::new(offset)))
                .collect();
            let start = day_start(week_start, display_tz)?;
            let end = day_end(week_end, display_tz)?;
            (days, start, end)
        }
        SchedulerView::Month => {
            let month_start = current.with_day(1).unwrap_or(current);
            let grid_start = start_of_week(month_start);
            let days: Vec<NaiveDate> = (0..42)
                .filter_map(|offset| grid_start.checked_add_days(chrono::Days::new(offset)))
                .collect();
            let grid_end = *days.last()?;
            let start = day_start(grid_start, display_tz)?;
            let end = day_end(grid_end, display_tz)?;
            (days, start, end)
        }
    };

    Some(VisibleRange {
        query: DateTimeRange::new(range_start, range_end),
        days,
    })
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
    fn day_range_is_single_day() {
        let range =
            visible_range(anchor(), SchedulerView::Day, DatetimeTimezone::Utc).expect("range");
        assert_eq!(range.days.len(), 1);
        assert!(range.query.contains(anchor()));
    }

    #[test]
    fn week_range_has_seven_days() {
        let range =
            visible_range(anchor(), SchedulerView::Week, DatetimeTimezone::Utc).expect("range");
        assert_eq!(range.days.len(), 7);
        assert!(range.query.overlaps(&range.query));
    }

    #[test]
    fn month_range_has_forty_two_days() {
        let range =
            visible_range(anchor(), SchedulerView::Month, DatetimeTimezone::Utc).expect("range");
        assert_eq!(range.days.len(), 42);
    }

    #[test]
    fn week_query_spans_full_week() {
        let range =
            visible_range(anchor(), SchedulerView::Week, DatetimeTimezone::Utc).expect("range");
        let week_start = start_of_week(anchor().wall_date().unwrap());
        let week_end = end_of_week(anchor().wall_date().unwrap());
        let start = day_start(week_start, DatetimeTimezone::Utc).expect("start");
        let end = day_end(week_end, DatetimeTimezone::Utc).expect("end");
        assert_eq!(range.query.start, start);
        assert_eq!(range.query.end, end);
    }
}
