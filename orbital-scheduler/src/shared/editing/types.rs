//! Event dialog request types (SC-09, SC-21).

use chrono::NaiveDate;

use orbital_base_components::OrbitalDateTime;

/// How the event dialog was opened.
#[derive(Clone, Debug, PartialEq)]
pub enum EventDialogMode {
    /// Create a new event with prefilled start/end from a slot click or toolbar.
    Create {
        day: NaiveDate,
        resource_id: Option<String>,
        default_start: OrbitalDateTime,
        default_end: OrbitalDateTime,
    },
    /// Edit an existing event by id.
    Edit { event_id: String },
}

/// Payload passed to open the shared event dialog.
#[derive(Clone, Debug, PartialEq)]
pub struct EventDialogRequest {
    pub mode: EventDialogMode,
}
