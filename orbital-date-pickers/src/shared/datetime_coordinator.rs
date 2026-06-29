use leptos::prelude::*;
use orbital_base_components::{OptionBind, OrbitalDateTime};
use orbital_core_components::{DatePickerBind, TimePickerBind};

/// Merge a new calendar day with the time-of-day from a previous value.
pub fn merge_date_preserve_time(
    new_date: OrbitalDateTime,
    previous: Option<OrbitalDateTime>,
) -> OrbitalDateTime {
    match previous.and_then(|p| p.hour_minute_second()) {
        Some((hour, minute, second)) => {
            new_date.apply_hms(hour, minute, second).unwrap_or(new_date)
        }
        None => new_date,
    }
}

pub(crate) struct DateTimeCoordinatorState {
    pub(crate) date_proxy: RwSignal<Option<OrbitalDateTime>>,
    pub(crate) reference_date: Signal<OrbitalDateTime>,
}

pub(crate) fn wire_datetime_master(
    master: StoredValue<OptionBind<OrbitalDateTime>>,
    fallback_reference: Signal<OrbitalDateTime>,
) -> DateTimeCoordinatorState {
    let date_proxy = RwSignal::new(None::<OrbitalDateTime>);
    let last_master = RwSignal::new(None::<OrbitalDateTime>);
    let last_date_proxy = RwSignal::new(None::<OrbitalDateTime>);

    Effect::new(move |_| {
        let current = master.with_value(|v| v.get());
        if last_master.get_untracked() == current {
            return;
        }
        last_master.set(current);
        let date_only = current.map(|v| v.start_of_day());
        if last_date_proxy.get_untracked() != date_only {
            last_date_proxy.set(date_only);
            date_proxy.set(date_only);
        }
    });

    Effect::new(move |_| {
        let current_date = date_proxy.get();
        if last_date_proxy.get_untracked() == current_date {
            return;
        }
        last_date_proxy.set(current_date);
        match current_date {
            Some(new_date) => {
                let previous = master.with_value(|v| v.get_untracked());
                let merged = merge_date_preserve_time(new_date, previous);
                if previous != Some(merged) {
                    last_master.set(Some(merged));
                    master.with_value(|v| v.set(Some(merged)));
                }
            }
            None => {
                last_master.set(None);
                master.with_value(|v| v.set(None));
            }
        }
    });

    let reference_date = Signal::derive(move || {
        master
            .with_value(|v| v.get())
            .map(|v| v.start_of_day())
            .unwrap_or_else(|| fallback_reference.get())
    });

    DateTimeCoordinatorState {
        date_proxy,
        reference_date,
    }
}

/// Coordinated child binds for a combined date + time picker surface.
pub struct DateTimeCoordinatorBinds {
    pub date_bind: DatePickerBind,
    pub time_bind: TimePickerBind,
    pub reference_date: Signal<Option<OrbitalDateTime>>,
}

/// Wire date and time pickers to one master [`OptionBind<OrbitalDateTime>`], preserving
/// time-of-day when the calendar day changes.
pub fn use_datetime_coordinator(
    master: OptionBind<OrbitalDateTime>,
    id: MaybeProp<String>,
    name: MaybeProp<String>,
    fallback_reference: Signal<OrbitalDateTime>,
) -> DateTimeCoordinatorBinds {
    let master = StoredValue::new(master);
    let state = wire_datetime_master(master, fallback_reference);

    let reference_date = Signal::derive(move || {
        master
            .with_value(|v| v.get())
            .map(|v| v.start_of_day())
            .or_else(|| Some(fallback_reference.get()))
    });

    let date_bind = DatePickerBind {
        value: state.date_proxy.into(),
        id,
        name,
        rules: Vec::new(),
    };

    let time_bind = TimePickerBind::new(master.with_value(|v| v.clone()));

    DateTimeCoordinatorBinds {
        date_bind,
        time_bind,
        reference_date,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_base_components::{DatetimeTimezone, TryFromUnixSeconds};

    #[test]
    fn merge_preserves_time_on_new_day() {
        let previous = OrbitalDateTime::try_from_unix_seconds(1_735_741_800, DatetimeTimezone::Utc)
            .expect("valid");
        let new_date = previous.start_of_day();
        let (hour, minute, second) = previous.hour_minute_second().expect("hms");
        let merged = merge_date_preserve_time(new_date, Some(previous));
        assert_eq!(merged.hour_minute_second(), Some((hour, minute, second)));
    }

    #[test]
    fn merge_without_previous_returns_start_of_day() {
        let new_date = OrbitalDateTime::try_from_unix_seconds(1_735_689_600, DatetimeTimezone::Utc)
            .expect("valid")
            .start_of_day();
        let merged = merge_date_preserve_time(new_date, None);
        assert_eq!(merged.hour_minute_second(), Some((0, 0, 0)));
    }
}
