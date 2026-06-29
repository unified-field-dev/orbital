use leptos::prelude::*;
use orbital_base_components::{OptionBind, OrbitalDateTime};

use super::field_types::{DateFieldBind, TimeFieldBind};
use crate::shared::wire_datetime_master;

/// Coordinated child binds for a combined date + time segmented field.
pub struct DateTimeFieldCoordinatorBinds {
    pub date_bind: DateFieldBind,
    pub time_bind: TimeFieldBind,
    pub reference_date: Signal<OrbitalDateTime>,
}

/// Wire date and time segmented fields to one master [`OptionBind<OrbitalDateTime>`].
pub fn use_datetime_field_coordinator(
    master: OptionBind<OrbitalDateTime>,
    id: MaybeProp<String>,
    name: MaybeProp<String>,
    fallback_reference: Signal<OrbitalDateTime>,
) -> DateTimeFieldCoordinatorBinds {
    let master = StoredValue::new(master);
    let state = wire_datetime_master(master, fallback_reference);

    let date_bind = DateFieldBind {
        value: state.date_proxy.into(),
        id,
        name,
        ..Default::default()
    };

    let time_bind = TimeFieldBind {
        value: master.with_value(|v| v.clone()),
        ..Default::default()
    };

    DateTimeFieldCoordinatorBinds {
        date_bind,
        time_bind,
        reference_date: state.reference_date,
    }
}
