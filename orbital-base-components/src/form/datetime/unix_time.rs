use super::convert::{ToUnixSeconds, TryFromUnixSeconds};
use super::orbital_datetime::OrbitalDateTime;
use super::timezone::DatetimeTimezone;

/// Deprecated unix-seconds newtype — use [`OrbitalDateTime`] instead.
#[deprecated(note = "use OrbitalDateTime")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnixTime(pub i64);

#[allow(deprecated)]
impl UnixTime {
    #[deprecated(note = "use OrbitalDateTime")]
    pub fn start_of_day(self, use_utc: bool) -> Self {
        let tz = if use_utc {
            DatetimeTimezone::Utc
        } else {
            DatetimeTimezone::Local
        };
        OrbitalDateTime::try_from_unix_seconds(self.0, tz)
            .map(|dt| Self(dt.start_of_day().to_unix_seconds()))
            .unwrap_or(self)
    }

    #[deprecated(note = "use OrbitalDateTime")]
    pub fn apply_hms(self, hour: u32, minute: u32, second: u32, use_utc: bool) -> Option<Self> {
        let tz = if use_utc {
            DatetimeTimezone::Utc
        } else {
            DatetimeTimezone::Local
        };
        OrbitalDateTime::try_from_unix_seconds(self.0, tz)
            .ok()?
            .apply_hms(hour, minute, second)
            .map(|dt| Self(dt.to_unix_seconds()))
    }

    #[deprecated(note = "use OrbitalDateTime")]
    pub fn same_calendar_day(a: Self, b: Self, use_utc: bool) -> bool {
        let tz = if use_utc {
            DatetimeTimezone::Utc
        } else {
            DatetimeTimezone::Local
        };
        let Ok(a_dt) = OrbitalDateTime::try_from_unix_seconds(a.0, tz) else {
            return false;
        };
        let Ok(b_dt) = OrbitalDateTime::try_from_unix_seconds(b.0, tz) else {
            return false;
        };
        a_dt.same_calendar_day(b_dt)
    }

    #[deprecated(note = "use OrbitalDateTime")]
    pub fn hour_minute_second(self, use_utc: bool) -> Option<(u32, u32, u32)> {
        let tz = if use_utc {
            DatetimeTimezone::Utc
        } else {
            DatetimeTimezone::Local
        };
        OrbitalDateTime::try_from_unix_seconds(self.0, tz)
            .ok()?
            .hour_minute_second()
    }
}
#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::UnixTime;
    use chrono::{TimeZone, Utc};

    #[test]
    fn start_of_day_utc_resets_time() {
        let source = Utc
            .with_ymd_and_hms(2025, 1, 2, 18, 45, 12)
            .single()
            .expect("valid dt");
        let sod = UnixTime(source.timestamp()).start_of_day(true);
        let expected = Utc
            .with_ymd_and_hms(2025, 1, 2, 0, 0, 0)
            .single()
            .expect("valid dt");
        assert_eq!(sod.0, expected.timestamp());
    }

    #[test]
    fn apply_hms_utc_replaces_time_component() {
        let source = Utc
            .with_ymd_and_hms(2025, 2, 11, 1, 2, 3)
            .single()
            .expect("valid dt");
        let updated = UnixTime(source.timestamp())
            .apply_hms(22, 10, 9, true)
            .expect("updated dt");
        let expected = Utc
            .with_ymd_and_hms(2025, 2, 11, 22, 10, 9)
            .single()
            .expect("valid dt");
        assert_eq!(updated.0, expected.timestamp());
    }

    #[test]
    fn same_calendar_day_utc_checks_date_only() {
        let a = Utc
            .with_ymd_and_hms(2025, 6, 4, 0, 0, 1)
            .single()
            .expect("valid dt");
        let b = Utc
            .with_ymd_and_hms(2025, 6, 4, 23, 59, 59)
            .single()
            .expect("valid dt");
        let c = Utc
            .with_ymd_and_hms(2025, 6, 5, 0, 0, 0)
            .single()
            .expect("valid dt");

        assert!(UnixTime::same_calendar_day(
            UnixTime(a.timestamp()),
            UnixTime(b.timestamp()),
            true
        ));
        assert!(!UnixTime::same_calendar_day(
            UnixTime(a.timestamp()),
            UnixTime(c.timestamp()),
            true
        ));
    }
}
