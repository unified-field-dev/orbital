use leptos::prelude::*;

use super::renderers::{
    AgendaEventRowView, EventContentView, ResourceHeaderView, ResourceLabelView,
};

/// Collected Leptos slot regions for scheduler products.
#[derive(Default)]
pub struct SchedulerSlots {
    pub toolbar: Option<SchedulerToolbar>,
    pub editing_tools: Option<SchedulerEditingTools>,
    pub loading_view: Option<SchedulerLoadingView>,
    pub error_view: Option<SchedulerErrorView>,
    pub resource_header: Option<SchedulerResourceHeader>,
    pub resource_label: Option<SchedulerResourceLabel>,
    pub event_content: Option<SchedulerEventContent>,
    pub agenda_event_row: Option<SchedulerAgendaEventRow>,
}

impl SchedulerSlots {
    #[allow(clippy::too_many_arguments)]
    pub fn from_slot_props(
        toolbar: Option<SchedulerToolbar>,
        editing_tools: Option<SchedulerEditingTools>,
        loading_view: Option<SchedulerLoadingView>,
        error_view: Option<SchedulerErrorView>,
        resource_header: Option<SchedulerResourceHeader>,
        resource_label: Option<SchedulerResourceLabel>,
        event_content: Option<SchedulerEventContent>,
        agenda_event_row: Option<SchedulerAgendaEventRow>,
    ) -> Self {
        Self {
            toolbar,
            editing_tools,
            loading_view,
            error_view,
            resource_header,
            resource_label,
            event_content,
            agenda_event_row,
        }
    }
}

/// Custom toolbar region above the calendar or timeline view.
#[slot]
pub struct SchedulerToolbar {
    pub(crate) children: ChildrenFn,
}

/// Host-owned editing affordances (e.g. New event button).
#[slot]
pub struct SchedulerEditingTools {
    pub(crate) children: ChildrenFn,
}

/// Custom lazy-load loading overlay.
#[slot]
pub struct SchedulerLoadingView {
    pub(crate) children: ChildrenFn,
}

/// Custom lazy-load error overlay.
#[slot]
pub struct SchedulerErrorView {
    pub(crate) children: ChildrenFn,
}

/// Custom resource column header cell.
#[slot]
pub struct SchedulerResourceHeader {
    #[prop(into)]
    pub render: ResourceHeaderView,
}

/// Custom resource row label in the resource column.
#[slot]
pub struct SchedulerResourceLabel {
    #[prop(into)]
    pub render: ResourceLabelView,
}

/// Custom inner content for event chips and timeline bars.
#[slot]
pub struct SchedulerEventContent {
    #[prop(into)]
    pub render: EventContentView,
}

/// Custom agenda list row content.
#[slot]
pub struct SchedulerAgendaEventRow {
    #[prop(into)]
    pub render: AgendaEventRowView,
}
