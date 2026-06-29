use chrono::{DateTime, Utc};

use crate::DiscussionLocale;

/// Format a UTC timestamp as a relative time string using English defaults.
///
/// Prefer [`DiscussionLocale::format_relative_time`] when locale context is available.
pub fn format_relative_time(at: DateTime<Utc>, now: DateTime<Utc>) -> String {
    DiscussionLocale::english().format_relative_time(at, now)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn ts(y: i32, m: u32, d: u32, h: u32, min: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, m, d, h, min, 0).unwrap()
    }

    #[test]
    fn just_now_for_recent() {
        let now = ts(2026, 6, 19, 12, 0);
        assert_eq!(format_relative_time(now, now), "just now");
    }

    #[test]
    fn hours_ago() {
        let now = ts(2026, 6, 19, 14, 0);
        let at = ts(2026, 6, 19, 12, 0);
        assert_eq!(format_relative_time(at, now), "2h ago");
    }

    #[test]
    fn days_ago() {
        let now = ts(2026, 6, 19, 12, 0);
        let at = ts(2026, 6, 17, 12, 0);
        assert_eq!(format_relative_time(at, now), "2d ago");
    }
}
