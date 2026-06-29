use chrono::{NaiveDate, Utc};
use orbital_base_components::{DatetimeTimezone, OrbitalDateTime};

/// In-progress or complete range selection on the calendar grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RangeSelection {
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
}

impl RangeSelection {
    pub fn is_complete(self) -> bool {
        self.start.is_some() && self.end.is_some()
    }
}

/// Visual role for a day cell in a range calendar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DayRangeRole {
    None,
    RangeStart,
    RangeEnd,
    InRange,
    PreviewRange,
}

/// Advance range selection after a day click.
pub fn apply_range_click(selection: RangeSelection, day: NaiveDate) -> RangeSelection {
    if selection.end.is_some() {
        return RangeSelection {
            start: Some(day),
            end: None,
        };
    }
    if let Some(start) = selection.start {
        let (start, end) = if day < start {
            (day, start)
        } else {
            (start, day)
        };
        RangeSelection {
            start: Some(start),
            end: Some(end),
        }
    } else {
        RangeSelection {
            start: Some(day),
            end: None,
        }
    }
}

/// Classify a day for range highlighting.
pub fn day_range_role(
    day: NaiveDate,
    selection: RangeSelection,
    hover: Option<NaiveDate>,
) -> DayRangeRole {
    if let (Some(start), Some(end)) = (selection.start, selection.end) {
        if day == start {
            return DayRangeRole::RangeStart;
        }
        if day == end {
            return DayRangeRole::RangeEnd;
        }
        if day > start && day < end {
            return DayRangeRole::InRange;
        }
        return DayRangeRole::None;
    }

    if let Some(start) = selection.start {
        if day == start {
            return DayRangeRole::RangeStart;
        }
        if let Some(hover_day) = hover {
            let (preview_start, preview_end) = if hover_day < start {
                (hover_day, start)
            } else {
                (start, hover_day)
            };
            if day == preview_start && day != start {
                return DayRangeRole::PreviewRange;
            }
            if day == preview_end && day != start {
                return DayRangeRole::PreviewRange;
            }
            if day > preview_start && day < preview_end {
                return DayRangeRole::PreviewRange;
            }
        }
    }

    DayRangeRole::None
}

pub fn is_day_disabled(
    day: NaiveDate,
    min_date: Option<OrbitalDateTime>,
    max_date: Option<OrbitalDateTime>,
) -> bool {
    if let Some(min) = min_date.and_then(|dt| dt.wall_date()) {
        if day < min {
            return true;
        }
    }
    if let Some(max) = max_date.and_then(|dt| dt.wall_date()) {
        if day > max {
            return true;
        }
    }
    false
}

pub fn orbital_from_date(date: NaiveDate, timezone: DatetimeTimezone) -> Option<OrbitalDateTime> {
    OrbitalDateTime::from_naive_date(date, timezone)
}

pub fn today_for_timezone(timezone: DatetimeTimezone) -> NaiveDate {
    use chrono::{FixedOffset, Local};
    match timezone {
        DatetimeTimezone::Local => Local::now().date_naive(),
        DatetimeTimezone::Utc => Utc::now().date_naive(),
        DatetimeTimezone::FixedOffset(offset_secs) => FixedOffset::east_opt(offset_secs)
            .map(|offset| Utc::now().with_timezone(&offset).date_naive())
            .unwrap_or_else(|| Utc::now().date_naive()),
    }
}

pub fn selection_to_range(
    selection: RangeSelection,
    timezone: DatetimeTimezone,
) -> Option<crate::DateTimeRange> {
    let (start, end) = (selection.start?, selection.end?);
    let start_dt = orbital_from_date(start, timezone)?;
    let end_dt = orbital_from_date(end, timezone)?;
    Some(crate::DateTimeRange::new(start_dt, end_dt).normalized())
}

pub fn range_to_selection(range: Option<crate::DateTimeRange>) -> RangeSelection {
    match range {
        Some(range) => RangeSelection {
            start: range.start.wall_date(),
            end: range.end.wall_date(),
        },
        None => RangeSelection::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn first_click_sets_start_only() {
        let day = NaiveDate::from_ymd_opt(2025, 6, 10).expect("date");
        let next = apply_range_click(RangeSelection::default(), day);
        assert_eq!(next.start, Some(day));
        assert_eq!(next.end, None);
    }

    #[test]
    fn second_click_completes_range() {
        let start = NaiveDate::from_ymd_opt(2025, 6, 10).expect("date");
        let end = NaiveDate::from_ymd_opt(2025, 6, 15).expect("date");
        let partial = RangeSelection {
            start: Some(start),
            end: None,
        };
        let complete = apply_range_click(partial, end);
        assert_eq!(complete.start, Some(start));
        assert_eq!(complete.end, Some(end));
    }

    #[test]
    fn click_while_complete_restarts() {
        let old_start = NaiveDate::from_ymd_opt(2025, 6, 1).expect("date");
        let old_end = NaiveDate::from_ymd_opt(2025, 6, 20).expect("date");
        let complete = RangeSelection {
            start: Some(old_start),
            end: Some(old_end),
        };
        let new_day = NaiveDate::from_ymd_opt(2025, 7, 4).expect("date");
        let restarted = apply_range_click(complete, new_day);
        assert_eq!(restarted.start, Some(new_day));
        assert_eq!(restarted.end, None);
    }
}
