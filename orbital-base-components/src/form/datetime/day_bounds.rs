use chrono::NaiveDate;

use super::OrbitalDateTime;

/// Whether a calendar day falls outside inclusive `min_date` / `max_date` bounds.
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

/// Whether a datetime value falls outside inclusive min/max bounds (compared at start-of-day).
pub fn is_datetime_out_of_range(
    value: OrbitalDateTime,
    min_date: Option<OrbitalDateTime>,
    max_date: Option<OrbitalDateTime>,
) -> bool {
    let day = value.start_of_day();
    if let Some(min) = min_date {
        if day < min.start_of_day() {
            return true;
        }
    }
    if let Some(max) = max_date {
        if day > max.start_of_day() {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::form::{DatetimeTimezone, TryFromUnixSeconds};

    fn date(y: i32, m: u32, d: u32) -> OrbitalDateTime {
        OrbitalDateTime::try_from_unix_seconds(
            chrono::NaiveDate::from_ymd_opt(y, m, d)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp(),
            DatetimeTimezone::Utc,
        )
        .expect("valid")
    }

    #[test]
    fn day_before_min_is_disabled() {
        let min = date(2025, 1, 10);
        let day = chrono::NaiveDate::from_ymd_opt(2025, 1, 9).unwrap();
        assert!(is_day_disabled(day, Some(min), None));
    }

    #[test]
    fn day_after_max_is_disabled() {
        let max = date(2025, 1, 10);
        let day = chrono::NaiveDate::from_ymd_opt(2025, 1, 11).unwrap();
        assert!(is_day_disabled(day, None, Some(max)));
    }

    #[test]
    fn day_in_range_is_enabled() {
        let min = date(2025, 1, 1);
        let max = date(2025, 1, 31);
        let day = chrono::NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        assert!(!is_day_disabled(day, Some(min), Some(max)));
    }
}
