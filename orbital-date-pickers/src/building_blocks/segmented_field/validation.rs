use orbital_base_components::DatetimeFormat;

use super::sections::{SegmentKind, SegmentSpec};

/// Filter, length-limit, and range-clamp a segment while the user types.
pub fn normalize_segment_input(raw: &str, spec: SegmentSpec, format: DatetimeFormat) -> String {
    match spec.kind {
        SegmentKind::Meridiem => normalize_meridiem(raw, spec.max_len),
        _ => normalize_numeric_segment(raw, spec.kind, spec.max_len, format),
    }
}

/// Zero-pad numeric segments on blur (`6` → `06`); meridiem is uppercased as-is.
pub fn pad_segment_on_blur(text: &str, spec: SegmentSpec) -> String {
    match spec.kind {
        SegmentKind::Meridiem => text.trim().to_ascii_uppercase(),
        SegmentKind::Year => text.to_string(),
        _ if text.is_empty() => String::new(),
        _ => format!("{:0>width$}", text, width = spec.max_len),
    }
}

/// Whether the segment has enough content to advance to the next field.
pub fn segment_is_complete(text: &str, spec: SegmentSpec) -> bool {
    match spec.kind {
        SegmentKind::Meridiem => text == "AM" || text == "PM",
        SegmentKind::Year => text.len() == spec.max_len,
        _ => text.len() >= spec.max_len,
    }
}

fn normalize_numeric_segment(
    raw: &str,
    kind: SegmentKind,
    max_len: usize,
    format: DatetimeFormat,
) -> String {
    let digits: String = raw
        .chars()
        .filter(|ch| ch.is_ascii_digit())
        .take(max_len)
        .collect();
    if digits.is_empty() {
        return String::new();
    }

    if digits.len() < max_len {
        return digits;
    }

    let (min, max) = numeric_bounds(kind, format);
    let parsed = digits.parse::<u32>().unwrap_or(min);
    format_fixed_width(parsed.clamp(min, max), max_len)
}

fn normalize_meridiem(raw: &str, max_len: usize) -> String {
    let letters: String = raw
        .chars()
        .filter(|ch| ch.is_ascii_alphabetic())
        .take(max_len)
        .collect::<String>()
        .to_ascii_uppercase();

    if letters.is_empty() {
        return String::new();
    }

    if letters.starts_with('A') {
        if letters.len() >= 2 && letters.as_bytes().get(1) == Some(&b'M') {
            return "AM".to_string();
        }
        return "A".to_string();
    }

    if letters.starts_with('P') {
        if letters.len() >= 2 && letters.as_bytes().get(1) == Some(&b'M') {
            return "PM".to_string();
        }
        return "P".to_string();
    }

    String::new()
}

/// Min/max numeric bounds for a segment kind in the given format.
pub fn numeric_bounds(kind: SegmentKind, format: DatetimeFormat) -> (u32, u32) {
    match kind {
        SegmentKind::Month => (1, 12),
        SegmentKind::Day => (1, 31),
        SegmentKind::Hour => match format {
            DatetimeFormat::Time12 => (1, 12),
            _ => (0, 23),
        },
        SegmentKind::Minute => (0, 59),
        SegmentKind::Year => (1, 9999),
        SegmentKind::Meridiem => (0, 1),
    }
}

fn format_fixed_width(value: u32, width: usize) -> String {
    format!("{value:0width$}")
}

pub fn segment_field_format(
    kind: SegmentKind,
    combined: bool,
    is_time: bool,
    date_format: DatetimeFormat,
    time_format: DatetimeFormat,
) -> DatetimeFormat {
    match kind {
        SegmentKind::Hour | SegmentKind::Minute | SegmentKind::Meridiem => {
            if combined || is_time {
                time_format
            } else {
                date_format
            }
        }
        _ => date_format,
    }
}

#[cfg(test)]
mod tests {
    use super::super::sections::segment_specs;
    use super::*;

    #[test]
    fn truncates_hour_to_max_len_and_clamps_12h() {
        let spec = segment_specs(DatetimeFormat::Time12)[0];
        assert_eq!(
            normalize_segment_input("13122", spec, DatetimeFormat::Time12),
            "12"
        );
    }

    #[test]
    fn clamps_month_to_twelve() {
        let spec = segment_specs(DatetimeFormat::UsDate)[0];
        assert_eq!(
            normalize_segment_input("15", spec, DatetimeFormat::UsDate),
            "12"
        );
    }

    #[test]
    fn clamps_minute_to_fifty_nine() {
        let spec = segment_specs(DatetimeFormat::Time24)[1];
        assert_eq!(
            normalize_segment_input("99", spec, DatetimeFormat::Time24),
            "59"
        );
    }

    #[test]
    fn meridiem_accepts_am_prefix() {
        let spec = segment_specs(DatetimeFormat::Time12)[2];
        assert_eq!(
            normalize_segment_input("am", spec, DatetimeFormat::Time12),
            "AM"
        );
        assert!(segment_is_complete("AM", spec));
    }

    #[test]
    fn pads_single_digit_month_on_blur() {
        let spec = segment_specs(DatetimeFormat::UsDate)[0];
        assert_eq!(pad_segment_on_blur("6", spec), "06");
    }
}
