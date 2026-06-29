use chrono::Days;
use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, PickerShortcut};

/// Shortcut preset for today's calendar day at start-of-day in `timezone`.
pub fn today_shortcut(timezone: DatetimeTimezone) -> PickerShortcut {
    PickerShortcut::new("Today", OrbitalDateTime::utc_now(timezone).start_of_day())
}

/// Shortcut preset for yesterday at start-of-day in `timezone`.
pub fn yesterday_shortcut(timezone: DatetimeTimezone) -> PickerShortcut {
    let today = OrbitalDateTime::utc_now(timezone).start_of_day();
    let yesterday = today
        .instant()
        .checked_sub_days(Days::new(1))
        .map(|instant| OrbitalDateTime::from_instant(instant, timezone).start_of_day())
        .unwrap_or(today);
    PickerShortcut::new("Yesterday", yesterday)
}

/// Common Today + Yesterday presets for shortcut bars.
pub fn today_and_yesterday_shortcuts(timezone: DatetimeTimezone) -> Vec<PickerShortcut> {
    vec![today_shortcut(timezone), yesterday_shortcut(timezone)]
}
