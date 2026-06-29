//! Lazy-load data source trait and fetch coordination (SC-11, SC-23).

use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;

use leptos::prelude::{RwSignal, Update, WithUntracked};
use orbital_date_pickers::DateTimeRange;

use crate::{EventChanges, PlannedEvent, SchedulerError};

/// Cancellation token for lazy-load fetches.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AbortSignal {
    aborted: bool,
}

impl AbortSignal {
    /// Returns whether this fetch has been cancelled.
    pub fn is_aborted(self) -> bool {
        self.aborted
    }

    /// Marks the signal as cancelled.
    pub fn abort(&mut self) {
        self.aborted = true;
    }
}

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Async event provider for remote lazy loading (SC-11, SC-23).
pub trait SchedulerDataSource: Send + Sync {
    fn get_events(
        &self,
        range: DateTimeRange,
        signal: AbortSignal,
    ) -> BoxFuture<'_, Result<Vec<PlannedEvent>, SchedulerError>>;

    fn persist_events(&self, changes: EventChanges) -> BoxFuture<'_, Result<(), SchedulerError>>;
}

/// Static client events or trait-backed remote loading.
pub enum SchedulerDataSourceMode {
    /// Static `events` prop on product component.
    Client(Vec<PlannedEvent>),
    /// Trait-backed async load ([`SchedulerFeatures::LAZY_LOADING`](crate::SchedulerFeatures::LAZY_LOADING)).
    Remote(Box<dyn SchedulerDataSource>),
}

/// Tracks lazy-fetch generations so stale responses are ignored.
#[derive(Clone, Default)]
pub struct LazyFetchCoordinator {
    generation: u32,
}

impl LazyFetchCoordinator {
    /// Start a new fetch; returns the generation id for this request.
    pub fn begin(&mut self) -> u32 {
        self.generation = self.generation.wrapping_add(1);
        self.generation
    }

    /// True when `gen` is still the latest generation (response should be applied).
    pub fn is_current(&self, gen: u32) -> bool {
        self.generation == gen
    }
}

/// Run [`LazyFetchCoordinator::begin`] and return its result.
pub fn coordinator_begin(coordinator: RwSignal<LazyFetchCoordinator>) -> u32 {
    let mut generation = 0;
    coordinator.update(|c| {
        generation = c.begin();
    });
    generation
}

/// Whether `generation` is still current on `coordinator`.
pub fn coordinator_is_current(
    coordinator: RwSignal<LazyFetchCoordinator>,
    generation: u32,
) -> bool {
    coordinator.with_untracked(|c| c.is_current(generation))
}

/// Diff two event collections into added, updated, and removed ids.
pub fn diff_event_changes(before: &[PlannedEvent], after: &[PlannedEvent]) -> EventChanges {
    let before_map: HashMap<&str, &PlannedEvent> =
        before.iter().map(|e| (e.id.as_str(), e)).collect();
    let after_map: HashMap<&str, &PlannedEvent> =
        after.iter().map(|e| (e.id.as_str(), e)).collect();

    let before_ids: HashSet<&str> = before_map.keys().copied().collect();
    let after_ids: HashSet<&str> = after_map.keys().copied().collect();

    let added = after
        .iter()
        .filter(|e| !before_ids.contains(e.id.as_str()))
        .cloned()
        .collect();

    let updated = after
        .iter()
        .filter(|e| before_map.get(e.id.as_str()).is_some_and(|prev| prev != e))
        .cloned()
        .collect();

    let removed = before_ids
        .difference(&after_ids)
        .map(|id| (*id).to_string())
        .collect();

    EventChanges {
        added,
        updated,
        removed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};

    fn event(id: &str, title: &str) -> PlannedEvent {
        let start = OrbitalDateTime::try_from_unix_seconds(1_735_689_600, DatetimeTimezone::Utc)
            .expect("valid");
        let end = start.apply_hms(10, 0, 0).expect("valid end");
        PlannedEvent::new(id, title, start, end)
    }

    #[test]
    fn coordinator_increments_generation() {
        let mut coord = LazyFetchCoordinator::default();
        let g1 = coord.begin();
        let g2 = coord.begin();
        assert_ne!(g1, g2);
        assert!(coord.is_current(g2));
        assert!(!coord.is_current(g1));
    }

    #[test]
    fn diff_detects_added_updated_removed() {
        let mut evt1 = event("evt-1", "One");
        let evt2 = event("evt-2", "Two");
        let before = vec![evt1.clone(), evt2.clone()];

        evt1.title = "One updated".into();
        let evt3 = event("evt-3", "Three");
        let after = vec![evt1.clone(), evt3.clone()];
        let evt3_added = evt3;

        let changes = diff_event_changes(&before, &after);
        assert_eq!(changes.added, vec![evt3_added]);
        assert_eq!(changes.updated, vec![evt1]);
        assert_eq!(changes.removed, vec!["evt-2".to_string()]);
    }
}
