use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use orbital_data::DataValue;

use super::error::DatetimeError;
use super::orbital_datetime::OrbitalDateTime;
use super::timezone::DatetimeTimezone;

/// Parse inbound unix seconds at API / legacy boundaries.
pub trait TryFromUnixSeconds {
    fn try_from_unix_seconds(
        secs: i64,
        timezone: DatetimeTimezone,
    ) -> Result<OrbitalDateTime, DatetimeError>;
}

/// Parse inbound ISO 8601 strings at REST / JSON boundaries.
pub trait TryFromIso8601 {
    fn try_from_iso8601(
        input: &str,
        timezone: DatetimeTimezone,
    ) -> Result<OrbitalDateTime, DatetimeError>;
}

/// Parse inbound dataset cell values.
pub trait TryFromDataValue {
    fn try_from_data_value(
        value: &DataValue,
        timezone: DatetimeTimezone,
    ) -> Result<OrbitalDateTime, DatetimeError>;
}

/// Serialize outbound unix seconds for APIs and legacy payloads.
pub trait ToUnixSeconds {
    fn to_unix_seconds(&self) -> i64;
}

/// Serialize outbound ISO 8601 for REST / JSON payloads.
pub trait ToIso8601 {
    fn to_iso8601(&self) -> String;
}

/// Serialize outbound dataset cell values.
pub trait ToDataValue {
    fn to_data_value(&self) -> DataValue;
}

impl TryFromUnixSeconds for OrbitalDateTime {
    fn try_from_unix_seconds(secs: i64, timezone: DatetimeTimezone) -> Result<Self, DatetimeError> {
        let instant = Utc
            .timestamp_opt(secs, 0)
            .single()
            .ok_or(DatetimeError::OutOfRange)?;
        Ok(Self::from_instant(instant, timezone))
    }
}

impl TryFromIso8601 for OrbitalDateTime {
    fn try_from_iso8601(input: &str, timezone: DatetimeTimezone) -> Result<Self, DatetimeError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(DatetimeError::InvalidInput);
        }

        if let Ok(parsed) = DateTime::parse_from_rfc3339(trimmed) {
            return Ok(Self::from_instant(parsed.with_timezone(&Utc), timezone));
        }

        if let Ok(date) = NaiveDate::parse_from_str(trimmed, "%Y-%m-%d") {
            return Self::from_naive_date(date, timezone).ok_or(DatetimeError::OutOfRange);
        }

        Err(DatetimeError::InvalidInput)
    }
}

impl TryFromDataValue for OrbitalDateTime {
    fn try_from_data_value(
        value: &DataValue,
        timezone: DatetimeTimezone,
    ) -> Result<Self, DatetimeError> {
        match value {
            DataValue::Date(date) => {
                Self::from_naive_date(*date, timezone).ok_or(DatetimeError::OutOfRange)
            }
            _ => Err(DatetimeError::InvalidInput),
        }
    }
}

impl ToUnixSeconds for OrbitalDateTime {
    fn to_unix_seconds(&self) -> i64 {
        self.instant().timestamp()
    }
}

impl ToIso8601 for OrbitalDateTime {
    fn to_iso8601(&self) -> String {
        self.instant().to_rfc3339()
    }
}

impl ToDataValue for OrbitalDateTime {
    fn to_data_value(&self) -> DataValue {
        let date = self
            .wall_date()
            .unwrap_or_else(|| self.instant().date_naive());
        DataValue::Date(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, TimeZone, Utc};

    #[test]
    fn unix_round_trip() {
        let secs = 1_735_689_600_i64;
        let dt =
            OrbitalDateTime::try_from_unix_seconds(secs, DatetimeTimezone::Utc).expect("valid");
        assert_eq!(dt.to_unix_seconds(), secs);
    }

    #[test]
    fn iso8601_round_trip() {
        let dt = OrbitalDateTime::try_from_iso8601("2025-01-01T00:00:00Z", DatetimeTimezone::Utc)
            .expect("valid");
        assert!(dt.to_iso8601().starts_with("2025-01-01"));
    }

    #[test]
    fn iso8601_naive_date_uses_timezone() {
        let dt =
            OrbitalDateTime::try_from_iso8601("2025-06-15", DatetimeTimezone::Utc).expect("valid");
        assert_eq!(dt.hour_minute_second(), Some((0, 0, 0)));
    }

    #[test]
    fn data_value_date_to_start_of_day() {
        let date = NaiveDate::from_ymd_opt(2025, 4, 10).expect("valid");
        let dt =
            OrbitalDateTime::try_from_data_value(&DataValue::Date(date), DatetimeTimezone::Utc)
                .expect("valid");
        assert_eq!(dt.hour_minute_second(), Some((0, 0, 0)));
    }

    #[test]
    fn to_data_value_returns_wall_date() {
        let instant = Utc
            .with_ymd_and_hms(2025, 7, 4, 15, 30, 0)
            .single()
            .expect("valid");
        let dt = OrbitalDateTime::from_instant(instant, DatetimeTimezone::Utc);
        match dt.to_data_value() {
            DataValue::Date(d) => assert_eq!(d, NaiveDate::from_ymd_opt(2025, 7, 4).unwrap()),
            _ => panic!("expected date"),
        }
    }

    #[test]
    fn invalid_data_value_variant() {
        let err = OrbitalDateTime::try_from_data_value(
            &DataValue::Text("nope".into()),
            DatetimeTimezone::Utc,
        );
        assert_eq!(err, Err(DatetimeError::InvalidInput));
    }
}
