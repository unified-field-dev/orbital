use orbital_base_components::{
    format_datetime, parse_datetime, DatetimeFormat, DatetimeTimezone, OrbitalDateTime,
};

use super::sections::{segment_specs, SegmentKind, SegmentSpec};

/// Join segment text into a single parseable string.
pub fn join_segments(values: &[String], specs: &[SegmentSpec]) -> String {
    let mut out = String::new();
    for (index, spec) in specs.iter().enumerate() {
        let value = values.get(index).map(String::as_str).unwrap_or("");
        out.push_str(value);
        if let Some(separator) = spec.separator {
            out.push_str(separator);
        }
    }
    out
}

/// Split a formatted datetime string into segment values.
pub fn split_formatted_value(formatted: &str, specs: &[SegmentSpec]) -> Vec<String> {
    if formatted.is_empty() {
        return vec![String::new(); specs.len()];
    }

    let mut values = Vec::with_capacity(specs.len());
    let mut rest = formatted.trim();

    for spec in specs.iter() {
        if rest.is_empty() {
            values.push(String::new());
            continue;
        }

        if spec.kind == SegmentKind::Meridiem {
            values.push(rest.trim().to_ascii_uppercase());
            break;
        }

        if let Some(separator) = spec.separator {
            if let Some(pos) = rest.find(separator) {
                values.push(rest[..pos].trim().to_string());
                rest = rest[(pos + separator.len())..].trim_start();
            } else {
                values.push(rest.trim().to_string());
                rest = "";
            }
        } else {
            values.push(rest.trim().to_string());
            rest = "";
        }
    }

    while values.len() < specs.len() {
        values.push(String::new());
    }
    values
}

/// Whether every segment has the minimum characters required to attempt parsing.
pub fn segments_complete(values: &[String], specs: &[SegmentSpec]) -> bool {
    values.iter().zip(specs.iter()).all(|(value, spec)| {
        value.len() >= spec.max_len || spec.kind == SegmentKind::Meridiem && value.len() >= 2
    })
}

/// Parse segment values into an [`OrbitalDateTime`] for date formats.
pub fn parse_date_segments(
    values: &[String],
    format: DatetimeFormat,
    timezone: DatetimeTimezone,
) -> Option<OrbitalDateTime> {
    let specs = segment_specs(format);
    if !segments_complete(values, specs) {
        return None;
    }
    let joined = join_segments(values, specs);
    parse_datetime(&joined, format, timezone).map(|dt| dt.start_of_day())
}

/// Parse segment values into an [`OrbitalDateTime`] anchored to `reference_date`.
pub fn parse_time_segments(
    values: &[String],
    format: DatetimeFormat,
    timezone: DatetimeTimezone,
    reference_date: OrbitalDateTime,
) -> Option<OrbitalDateTime> {
    let specs = segment_specs(format);
    if !segments_complete(values, specs) {
        return None;
    }
    let joined = join_segments(values, specs);
    let parsed = parse_datetime(&joined, format, timezone)?;
    let (hour, minute, _) = parsed.hour_minute_second()?;
    reference_date.start_of_day().apply_hms(hour, minute, 0)
}

/// Parse segment values into an [`OrbitalDateTime`] for combined date + time formats.
pub fn parse_datetime_segments(
    values: &[String],
    date_format: DatetimeFormat,
    time_format: DatetimeFormat,
    timezone: DatetimeTimezone,
) -> Option<OrbitalDateTime> {
    let date_specs = segment_specs(date_format);
    let time_specs = segment_specs(time_format);
    let expected_len = date_specs.len() + time_specs.len();
    if values.len() < expected_len {
        return None;
    }

    let date_values = &values[..date_specs.len()];
    let time_values = &values[date_specs.len()..expected_len];

    if !segments_complete(date_values, date_specs) || !segments_complete(time_values, time_specs) {
        return None;
    }

    let date_dt = parse_date_segments(date_values, date_format, timezone)?;
    parse_time_segments(time_values, time_format, timezone, date_dt)
}

/// Format a bound value into combined date + time segment strings.
pub fn datetime_to_combined_segments(
    value: Option<OrbitalDateTime>,
    date_format: DatetimeFormat,
    time_format: DatetimeFormat,
) -> Vec<String> {
    let date_specs = segment_specs(date_format);
    let time_specs = segment_specs(time_format);
    match value {
        Some(dt) => {
            let mut segments = split_formatted_value(&format_datetime(dt, date_format), date_specs);
            segments.extend(split_formatted_value(
                &format_datetime(dt, time_format),
                time_specs,
            ));
            segments
        }
        None => vec![String::new(); date_specs.len() + time_specs.len()],
    }
}

/// Format a bound value into segment strings.
pub fn datetime_to_segments(value: Option<OrbitalDateTime>, format: DatetimeFormat) -> Vec<String> {
    let specs = segment_specs(format);
    match value {
        Some(dt) => split_formatted_value(&format_datetime(dt, format), specs),
        None => vec![String::new(); specs.len()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_base_components::{ToUnixSeconds, TryFromUnixSeconds};

    #[test]
    fn split_us_date_segments() {
        let specs = segment_specs(DatetimeFormat::UsDate);
        let values = split_formatted_value("01/15/2025", specs);
        assert_eq!(values, vec!["01", "15", "2025"]);
    }

    #[test]
    fn split_iso_date_segments() {
        let specs = segment_specs(DatetimeFormat::IsoDate);
        let values = split_formatted_value("2025-01-15", specs);
        assert_eq!(values, vec!["2025", "01", "15"]);
    }

    #[test]
    fn parse_us_date_round_trip() {
        let specs = segment_specs(DatetimeFormat::UsDate);
        let values = vec!["01".into(), "15".into(), "2025".into()];
        let parsed = parse_date_segments(&values, DatetimeFormat::UsDate, DatetimeTimezone::Utc)
            .expect("valid date");
        let formatted = format_datetime(parsed, DatetimeFormat::UsDate);
        assert_eq!(split_formatted_value(&formatted, specs), values);
    }

    #[test]
    fn parse_combined_us_date_time12_round_trip() {
        let reference =
            OrbitalDateTime::try_from_unix_seconds(1_735_741_800, DatetimeTimezone::Utc)
                .expect("valid");
        let date_specs = segment_specs(DatetimeFormat::UsDate);
        let time_specs = segment_specs(DatetimeFormat::Time12);
        let mut values = split_formatted_value(
            &format_datetime(reference, DatetimeFormat::UsDate),
            date_specs,
        );
        values.extend(split_formatted_value(
            &format_datetime(reference, DatetimeFormat::Time12),
            time_specs,
        ));
        let parsed = parse_datetime_segments(
            &values,
            DatetimeFormat::UsDate,
            DatetimeFormat::Time12,
            DatetimeTimezone::Utc,
        )
        .expect("valid datetime");
        let (hour, minute, _) = parsed.hour_minute_second().expect("hms");
        assert_eq!((hour, minute), (14, 30));
    }

    #[test]
    fn parse_combined_iso_time24_round_trip() {
        let reference =
            OrbitalDateTime::try_from_unix_seconds(1_735_741_800, DatetimeTimezone::Utc)
                .expect("valid");
        let values = datetime_to_combined_segments(
            Some(reference),
            DatetimeFormat::IsoDate,
            DatetimeFormat::Time24,
        );
        let parsed = parse_datetime_segments(
            &values,
            DatetimeFormat::IsoDate,
            DatetimeFormat::Time24,
            DatetimeTimezone::Utc,
        )
        .expect("valid datetime");
        assert_eq!(parsed.to_unix_seconds(), reference.to_unix_seconds());
    }
}
