//! Preview registration for gap stubs.

use icondata_core::Icon;
use leptos::prelude::*;

#[cfg(all(feature = "preview", not(target_arch = "wasm32")))]
inventory::collect!(PreviewRegistration);

/// Static metadata for a generated primitive preview page.
pub struct PreviewRegistration {
    pub slug: &'static str,
    pub label: &'static str,
    pub section: &'static str,
    pub section_priority: u16,
    pub category: &'static str,
    pub category_priority: u16,
    pub category_default_collapsed: bool,
    pub group: &'static str,
    pub group_priority: u16,
    pub nav_item: bool,
    pub icon: Icon,
    pub render: fn() -> AnyView,
}

pub mod static_registrations;

#[cfg(feature = "preview")]
pub mod fixtures;

pub use orbital_core_components::preview::{ComponentPreviewCard, OrbitalComponentView};

#[cfg(feature = "preview")]
pub use fixtures::{sample_planned_events, sample_schedule_resources};
