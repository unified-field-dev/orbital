//! Schedule resource types for the scheduler.
//!
//! `SchedulerResource` was renamed to [`ScheduleResource`] before API freeze (design §3.1).

/// A schedulable resource with optional nested hierarchy.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleResource {
    pub id: String,
    pub title: String,
    pub children: Vec<ScheduleResource>,
}

impl ScheduleResource {
    /// Creates a leaf resource with no children.
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            children: Vec::new(),
        }
    }
}
