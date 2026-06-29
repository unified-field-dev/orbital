use leptos::prelude::*;
use orbital_base_components::{OptionBind, OrbitalDateTime};

use crate::DateTimeRange;

/// Child endpoint signals coordinated with one master [`OptionBind<DateTimeRange>`].
pub struct RangeCoordinatorBinds {
    pub start: RwSignal<Option<OrbitalDateTime>>,
    pub end: RwSignal<Option<OrbitalDateTime>>,
}

/// Wire start/end field or picker proxies to one master range bind.
pub fn use_range_coordinator(master: OptionBind<DateTimeRange>) -> RangeCoordinatorBinds {
    let master = StoredValue::new(master);
    let start_proxy = RwSignal::new(None::<OrbitalDateTime>);
    let end_proxy = RwSignal::new(None::<OrbitalDateTime>);
    let last_master = RwSignal::new(None::<DateTimeRange>);
    let last_start = RwSignal::new(None::<OrbitalDateTime>);
    let last_end = RwSignal::new(None::<OrbitalDateTime>);
    let master_initialized = RwSignal::new(false);

    Effect::new(move |_| {
        let current = master.with_value(|v| v.get());
        if master_initialized.get_untracked() && last_master.get_untracked() == current {
            return;
        }
        master_initialized.set(true);
        last_master.set(current.clone());
        let (start, end) = match current {
            Some(range) => (Some(range.start), Some(range.end)),
            None => (None, None),
        };
        if last_start.get_untracked() != start {
            last_start.set(start);
            start_proxy.set(start);
        }
        if last_end.get_untracked() != end {
            last_end.set(end);
            end_proxy.set(end);
        }
    });

    Effect::new(move |_| {
        let start = start_proxy.get();
        let end = end_proxy.get();
        if last_start.get_untracked() == start && last_end.get_untracked() == end {
            return;
        }
        last_start.set(start);
        last_end.set(end);
        let merged = match (start, end) {
            (Some(start), Some(end)) => Some(DateTimeRange::new(start, end).normalized()),
            _ => None,
        };
        if last_master.get_untracked() == merged {
            return;
        }
        last_master.set(merged.clone());
        master.with_value(|v| v.set(merged));
    });

    RangeCoordinatorBinds {
        start: start_proxy,
        end: end_proxy,
    }
}
