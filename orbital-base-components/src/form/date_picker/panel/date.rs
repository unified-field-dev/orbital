use chrono::{FixedOffset, Local, NaiveDate, TimeZone, Utc};

use crate::form::datetime::DatetimeTimezone;

use super::super::month::month_name;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatePickerPanelMode {
    Date,
    Month,
    Year,
}

pub fn month_heading(year: i32, month: u32) -> String {
    format!("{} {}", month_name(month), year)
}

pub fn date_from_unix(secs: i64, timezone: DatetimeTimezone) -> Option<NaiveDate> {
    match timezone {
        DatetimeTimezone::Local => Local
            .timestamp_opt(secs, 0)
            .single()
            .map(|dt| dt.date_naive()),
        DatetimeTimezone::Utc => Utc
            .timestamp_opt(secs, 0)
            .single()
            .map(|dt| dt.date_naive()),
        DatetimeTimezone::FixedOffset(offset_secs) => FixedOffset::east_opt(offset_secs)?
            .timestamp_opt(secs, 0)
            .single()
            .map(|dt| dt.date_naive()),
    }
}

pub fn today_for_timezone(timezone: DatetimeTimezone) -> NaiveDate {
    match timezone {
        DatetimeTimezone::Local => Local::now().date_naive(),
        DatetimeTimezone::Utc => Utc::now().date_naive(),
        DatetimeTimezone::FixedOffset(offset_secs) => FixedOffset::east_opt(offset_secs)
            .map(|offset| Utc::now().with_timezone(&offset).date_naive())
            .unwrap_or_else(|| Utc::now().date_naive()),
    }
}

pub fn start_of_day_unix(date: NaiveDate, timezone: DatetimeTimezone) -> Option<i64> {
    let naive = date.and_hms_opt(0, 0, 0)?;
    match timezone {
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
