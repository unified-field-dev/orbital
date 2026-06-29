use leptos::prelude::*;
use orbital_base_components::{
    DatetimeTimezone, OptionBind, OrbitalDateTime, ToUnixSeconds, TryFromUnixSeconds,
};

const LEGACY_BIND_TZ: DatetimeTimezone = DatetimeTimezone::Local;

pub fn orbital_from_i64(secs: i64) -> Option<OrbitalDateTime> {
    OrbitalDateTime::try_from_unix_seconds(secs, LEGACY_BIND_TZ).ok()
}

pub fn orbital_from_i64_tz(secs: i64, timezone: DatetimeTimezone) -> Option<OrbitalDateTime> {
    OrbitalDateTime::try_from_unix_seconds(secs, timezone).ok()
}

/// Two-way bridge between an orbital bind and headless unix-second APIs.
pub fn use_unix_bridge(
    value: OptionBind<OrbitalDateTime>,
    timezone: Signal<DatetimeTimezone>,
) -> OptionBind<i64> {
    let value = StoredValue::new(value);
    let bridge = RwSignal::new(
        value
            .get_value()
            .get_untracked()
            .map(|dt| dt.to_unix_seconds()),
    );

    Effect::new(move |_| {
        let next = value.get_value().get().map(|dt| dt.to_unix_seconds());
        if bridge.get_untracked() != next {
            bridge.set(next);
        }
    });

    Effect::new(move |_| {
        let secs = bridge.get();
        let tz = timezone.get();
        let next = secs.and_then(|s| orbital_from_i64_tz(s, tz));
        if value.get_value().get_untracked() != next {
            value.get_value().set(next);
        }
    });

    bridge.into()
}

pub fn bridge_reference_date_to_unix(
    reference_date: Signal<Option<OrbitalDateTime>>,
) -> Signal<Option<i64>> {
    Signal::derive(move || reference_date.get().map(|dt| dt.to_unix_seconds()))
}
