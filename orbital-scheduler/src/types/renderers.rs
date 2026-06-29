use std::sync::Arc;

use leptos::prelude::*;
use orbital_core_components::Text;

use super::PlannedEvent;

/// Surface where event inner content is rendered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventRenderSurface {
    TimedGrid,
    TimelineBar,
    AgendaRow,
    MonthCell,
}

/// Context passed to [`EventContentView`] callbacks.
#[derive(Clone, Debug)]
pub struct EventRenderContext {
    pub event: PlannedEvent,
    pub resource_id: Option<String>,
    pub surface: EventRenderSurface,
}

impl EventRenderContext {
    pub fn new(
        event: PlannedEvent,
        resource_id: Option<String>,
        surface: EventRenderSurface,
    ) -> Self {
        Self {
            event,
            resource_id,
            surface,
        }
    }
}

/// Flattened resource row for label rendering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceRenderContext {
    pub id: String,
    pub title: String,
    pub depth: usize,
}

impl ResourceRenderContext {
    pub fn from_row(row: crate::ResourceRow) -> Self {
        Self {
            id: row.id,
            title: row.title,
            depth: row.depth,
        }
    }
}

/// Custom inner body for an event chip or timeline bar. Return `None` for built-in title text.
pub type EventContentView = Arc<dyn Fn(EventRenderContext) -> Option<AnyView> + Send + Sync>;

/// Custom resource column header cell.
pub type ResourceHeaderView = Arc<dyn Fn(()) -> AnyView + Send + Sync>;

/// Custom resource row label in the resource column.
pub type ResourceLabelView = Arc<dyn Fn(ResourceRenderContext) -> AnyView + Send + Sync>;

/// Custom agenda list row. Return `None` for built-in title + time range.
pub type AgendaEventRowView = Arc<dyn Fn(EventRenderContext) -> Option<AnyView> + Send + Sync>;

/// Internal render bundle consumed by scheduler leaf components.
#[derive(Clone, Default)]
pub struct SchedulerRenderers {
    pub resource_header: Option<ResourceHeaderView>,
    pub resource_label: Option<ResourceLabelView>,
    pub event_content: Option<EventContentView>,
    pub agenda_event_row: Option<AgendaEventRowView>,
}

impl SchedulerRenderers {
    /// Build render callbacks from collected slot props.
    pub fn from_slots(slots: &super::SchedulerSlots) -> Self {
        Self {
            resource_header: slots
                .resource_header
                .as_ref()
                .map(|slot| slot.render.clone()),
            resource_label: slots
                .resource_label
                .as_ref()
                .map(|slot| slot.render.clone()),
            event_content: slots.event_content.as_ref().map(|slot| slot.render.clone()),
            agenda_event_row: slots
                .agenda_event_row
                .as_ref()
                .map(|slot| slot.render.clone()),
        }
    }
}

/// Render event chip inner content using slot renderers or default title text.
pub fn render_event_content(
    renderers: &SchedulerRenderers,
    event: &PlannedEvent,
    resource_id: Option<&str>,
    surface: EventRenderSurface,
) -> AnyView {
    let ctx = EventRenderContext::new(event.clone(), resource_id.map(str::to_string), surface);
    if let Some(view) = renderers
        .event_content
        .as_ref()
        .and_then(|render| render(ctx))
    {
        return view;
    }
    let title = event.title.clone();
    view! { <Text>{title}</Text> }.into_any()
}

/// Render an agenda list row using slot renderers or the provided default view.
pub fn render_agenda_event_row(
    renderers: &SchedulerRenderers,
    event: &PlannedEvent,
    default: impl FnOnce() -> AnyView,
) -> AnyView {
    let ctx = EventRenderContext::new(event.clone(), None, EventRenderSurface::AgendaRow);
    if let Some(view) = renderers
        .agenda_event_row
        .as_ref()
        .and_then(|render| render(ctx))
    {
        return view;
    }
    default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SchedulerSlots;

    #[test]
    fn from_slots_maps_render_fields() {
        let event_view: EventContentView = Arc::new(|_| None);
        let label_view: ResourceLabelView =
            Arc::new(|ctx| view! { <span>{ctx.title.clone()}</span> }.into_any());
        let slots = SchedulerSlots {
            event_content: Some(crate::SchedulerEventContent {
                render: event_view.clone(),
            }),
            resource_label: Some(crate::SchedulerResourceLabel {
                render: label_view.clone(),
            }),
            ..Default::default()
        };
        let renderers = SchedulerRenderers::from_slots(&slots);
        assert!(renderers.event_content.is_some());
        assert!(renderers.resource_label.is_some());
        assert!(renderers.resource_header.is_none());
    }
}
