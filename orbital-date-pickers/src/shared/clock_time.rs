use orbital_base_components::{DatetimeTimezone, OrbitalDateTime};

/// Calendar day anchor for time-only clock surfaces.
pub fn resolve_anchor(
    value: Option<OrbitalDateTime>,
    reference_date: OrbitalDateTime,
    timezone: DatetimeTimezone,
) -> OrbitalDateTime {
    let base = value.unwrap_or(reference_date);
    OrbitalDateTime::from_instant(base.instant(), timezone).start_of_day()
}

/// Snap a minute value to the nearest step increment (minimum step 1).
pub fn snap_minute(minute: u32, step: u32) -> u32 {
    let step = step.max(1);
    let snapped = ((minute + step / 2) / step) * step;
    snapped.min(59)
}

/// Apply hour/minute/second to an anchor day in the given timezone.
pub fn commit_time(
    anchor: OrbitalDateTime,
    hour: u32,
    minute: u32,
    second: u32,
    timezone: DatetimeTimezone,
) -> Option<OrbitalDateTime> {
    let anchor = OrbitalDateTime::from_instant(anchor.instant(), timezone).start_of_day();
    anchor.apply_hms(hour, minute, second)
}

/// Generate `(hour, minute)` slots for a full day at the given step.
pub fn generate_time_slots(step_minutes: u32) -> Vec<(u32, u32)> {
    let step = step_minutes.max(1);
    let mut slots = Vec::new();
    let mut total = 0u32;
    while total < 24 * 60 {
        slots.push((total / 60, total % 60));
        total += step;
    }
    slots
}

/// Format a time slot label for list display.
pub fn format_slot_label(hour: u32, minute: u32, ampm: bool) -> String {
    if ampm {
        let (display_hour, period) = to_twelve_hour(hour);
        format!("{display_hour}:{minute:02} {period}")
    } else {
        format!("{hour:02}:{minute:02}")
    }
}

/// Convert 24h hour to (12h display, AM/PM).
pub fn to_twelve_hour(hour: u32) -> (u32, &'static str) {
    let period = if hour >= 12 { "PM" } else { "AM" };
    let h = hour % 12;
    let display = if h == 0 { 12 } else { h };
    (display, period)
}

/// Convert 12h display hour + meridiem to 24h hour.
pub fn to_twenty_four_hour(display_hour: u32, is_pm: bool) -> u32 {
    let h = display_hour % 12;
    match (h, is_pm) {
        (0, false) => 0,
        (0, true) => 12,
        (hour, false) => hour,
        (hour, true) => hour + 12,
    }
}

/// Hour markers for the analog dial (12 at top for 12-hour mode).
pub fn hour_markers(ampm: bool) -> Vec<u32> {
    if ampm {
        std::iter::once(12).chain(1..=11).collect()
    } else {
        (0..24).collect()
    }
}

/// Minute markers at the given step increment.
pub fn minute_markers(step: u32) -> Vec<u32> {
    let step = step.max(1);
    (0..60).step_by(step as usize).collect()
}

/// Current wall-clock time on the anchor day as `(hour, minute, is_pm)`.
pub fn now_on_anchor(anchor: OrbitalDateTime) -> (u32, u32, bool) {
    let now = OrbitalDateTime::utc_now(anchor.timezone());
    let (hour, minute, _) = now.hour_minute_second().unwrap_or((0, 0, 0));
    (hour, minute, hour >= 12)
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_base_components::{DatetimeTimezone, TryFromUnixSeconds};

    fn sample_ref() -> OrbitalDateTime {
        OrbitalDateTime::try_from_unix_seconds(1_735_689_600, DatetimeTimezone::Local)
            .expect("valid reference")
    }

    #[test]
    fn snap_minute_rounds_to_step() {
        assert_eq!(snap_minute(7, 5), 5);
        assert_eq!(snap_minute(8, 5), 10);
        assert_eq!(snap_minute(0, 0), 0);
    }

    #[test]
    fn generate_time_slots_respects_step() {
        let slots = generate_time_slots(30);
        assert_eq!(slots.first(), Some(&(0, 0)));
        assert!(slots.contains(&(9, 30)));
        assert_eq!(slots.last(), Some(&(23, 30)));
    }

    #[test]
    fn commit_time_applies_hms() {
        let anchor = sample_ref().start_of_day();
        let committed = commit_time(anchor, 14, 30, 0, DatetimeTimezone::Local).expect("commit");
        assert_eq!(committed.hour_minute_second(), Some((14, 30, 0)));
    }

    #[test]
    fn resolve_anchor_uses_value_when_present() {
        let anchor = sample_ref().start_of_day();
        let with_time = commit_time(anchor, 9, 15, 0, DatetimeTimezone::Local).expect("time");
        let resolved = resolve_anchor(Some(with_time), sample_ref(), DatetimeTimezone::Local);
        assert_eq!(resolved.start_of_day(), anchor);
    }

    #[test]
    fn to_twelve_hour_converts_correctly() {
        assert_eq!(to_twelve_hour(0), (12, "AM"));
        assert_eq!(to_twelve_hour(12), (12, "PM"));
        assert_eq!(to_twelve_hour(15), (3, "PM"));
    }

    #[test]
    fn to_twenty_four_hour_converts_correctly() {
        assert_eq!(to_twenty_four_hour(12, false), 0);
        assert_eq!(to_twenty_four_hour(12, true), 12);
        assert_eq!(to_twenty_four_hour(3, true), 15);
    }
}
