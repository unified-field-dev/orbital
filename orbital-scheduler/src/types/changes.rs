use super::PlannedEvent;

/// Batch of event mutations for persist operations.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EventChanges {
    pub added: Vec<PlannedEvent>,
    pub updated: Vec<PlannedEvent>,
    /// Event ids removed from the schedule.
    pub removed: Vec<String>,
}
