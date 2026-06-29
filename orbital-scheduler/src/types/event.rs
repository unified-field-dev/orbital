//! Planned event types for the scheduler.
//!
//! `SchedulerEvent` was renamed to [`PlannedEvent`] before API freeze (design §3.1).

use orbital_base_components::OrbitalDateTime;

/// A scheduled event with typed start/end datetimes.
#[derive(Clone, Debug, PartialEq)]
pub struct PlannedEvent {
    pub id: String,
    pub title: String,
    pub start: OrbitalDateTime,
    pub end: OrbitalDateTime,
    pub resource_id: Option<String>,
    /// RFC 5545 recurrence rule when [`SchedulerFeatures::RECURRING_EVENTS`](crate::SchedulerFeatures::RECURRING_EVENTS) is enabled.
    pub recurrence_rule: Option<String>,
    pub color: Option<String>,
    pub is_draggable: Option<bool>,
    pub is_resizable: Option<bool>,
    pub is_editable: Option<bool>,
}

impl PlannedEvent {
    /// Creates an event with the required fields; optional fields default to `None`.
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        start: OrbitalDateTime,
        end: OrbitalDateTime,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            start,
            end,
            resource_id: None,
            recurrence_rule: None,
            color: None,
            is_draggable: None,
            is_resizable: None,
            is_editable: None,
        }
    }
}
