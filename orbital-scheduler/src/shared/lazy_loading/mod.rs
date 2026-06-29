//! Lazy loading — fetch wiring, overlays, and mock data source.

mod fetch;
mod overlays;

#[cfg(feature = "preview")]
mod mock;

pub use fetch::{
    mount_lazy_load, persist_event_changes, LazyLoadController, SchedulerLazyLoadContext,
};
pub use overlays::SchedulerLazyLoadOverlays;

#[cfg(feature = "preview")]
pub use mock::MockSlowDataSource;
