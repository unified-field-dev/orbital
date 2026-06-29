use chrono::{FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

use super::convert::{ToUnixSeconds, TryFromUnixSeconds};
use super::orbital_datetime::OrbitalDateTime;
use super::timezone::DatetimeTimezone;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DatetimeFormat {
    IsoDate,
    #[default]
    UsDate,
    Time24,
    Time12,
}

pub fn format_unix(secs: i64, format: DatetimeFormat, tz: DatetimeTimezone) -> String {
    match tz {
        DatetimeTimezone::Local => Local
            .timestamp_opt(secs, 0)
            .single()
            .map(|dt| dt.format(fmt_pattern(format)).to_string())
            .unwrap_or_default(),
        DatetimeTimezone::Utc => Utc
            .timestamp_opt(secs, 0)
            .single()
            .map(|dt| dt.format(fmt_pattern(format)).to_string())
            .unwrap_or_default(),
        DatetimeTimezone::FixedOffset(offset_secs) => FixedOffset::east_opt(offset_secs)
            .and_then(|offset| offset.timestamp_opt(secs, 0).single())
            .map(|dt| dt.format(fmt_pattern(format)).to_string())
            .unwrap_or_default(),
    }
}

pub fn parse_to_unix(input: &str, format: DatetimeFormat, tz: DatetimeTimezone) -> Option<i64> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }

    let naive = match format {
        DatetimeFormat::IsoDate => {
            let date = NaiveDate::parse_from_str(input, "%Y-%m-%d").ok()?;
            date.and_hms_opt(0, 0, 0)?
        }
        DatetimeFormat::UsDate => {
            let date = NaiveDate::parse_from_str(input, "%m/%d/%Y").ok()?;
            date.and_hms_opt(0, 0, 0)?
        }
        DatetimeFormat::Time24 => {
            let time = NaiveTime::parse_from_str(input, "%H:%M").ok()?;
            let epoch = NaiveDate::from_ymd_opt(1970, 1, 1)?;
            epoch.and_time(time)
        }
        DatetimeFormat::Time12 => {
            let time = NaiveTime::parse_from_str(input, "%I:%M %p").ok()?;
            let epoch = NaiveDate::from_ymd_opt(1970, 1, 1)?;
            epoch.and_time(time)
        }
    };

    timestamp_for_timezone(naive, tz)
}

fn timestamp_for_timezone(naive: NaiveDateTime, tz: DatetimeTimezone) -> Option<i64> {
    match tz {
        DatetimeTimezone::Local => Local
            .from_local_datetime(&naive)
            .single()
            .map(|dt| dt.timestamp()),
        DatetimeTimezone::Utc => Some(Utc.from_utc_datetime(&naive).timestamp()),
        DatetimeTimezone::FixedOffset(offset_secs) => FixedOffset::east_opt(offset_secs)?
            .from_local_datetime(&naive)
            .single()
            .map(|dt| dt.timestamp()),
    }
}

/// Display a bound datetime value using its embedded timezone.
pub fn format_datetime(dt: OrbitalDateTime, format: DatetimeFormat) -> String {
    format_unix(dt.to_unix_seconds(), format, dt.timezone())
}

/// Parse display text into a bound datetime value.
pub fn parse_datetime(
    input: &str,
    format: DatetimeFormat,
    timezone: DatetimeTimezone,
) -> Option<OrbitalDateTime> {
    parse_to_unix(input, format, timezone)
        .and_then(|secs| OrbitalDateTime::try_from_unix_seconds(secs, timezone).ok())
}

fn fmt_pattern(format: DatetimeFormat) -> &'static str {
    match format {
        DatetimeFormat::IsoDate => "%Y-%m-%d",
        DatetimeFormat::UsDate => "%m/%d/%Y",
        DatetimeFormat::Time24 => "%H:%M",
        DatetimeFormat::Time12 => "%I:%M %p",
    }
}
