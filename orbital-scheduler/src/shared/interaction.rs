//! Shared interaction context for drag, resize, and event editing.

use leptos::prelude::*;

use orbital_base_components::DatetimeTimezone;

use crate::master_event_id;
use crate::shared::drag::EventDragGhost;
use crate::shared::lazy_loading::{persist_event_changes, SchedulerLazyLoadContext};
use crate::{
    EventDialogRequest, EventDragSession, PlannedEvent, ScheduleResource, SchedulerEvents,
    SchedulerRenderers, SchedulerSlots,
};

/// Context provided by scheduler products for timed-grid interactions.
#[derive(Clone)]
pub struct SchedulerInteractionContext {
    pub are_events_draggable: Signal<bool>,
    pub are_events_resizable: Signal<bool>,
    pub event_creation: Signal<bool>,
    pub events: RwSignal<Vec<PlannedEvent>>,
    pub resources: RwSignal<Vec<ScheduleResource>>,
    pub display_timezone: RwSignal<DatetimeTimezone>,
    pub scheduler_events: StoredValue<SchedulerEvents>,
    pub renderers: StoredValue<SchedulerRenderers>,
    pub slots: StoredValue<SchedulerSlots>,
    pub open_dialog: Callback<EventDialogRequest, ()>,
    /// Ephemeral drag state kept in `StoredValue` so window listeners do not notify the reactive graph.
    pub drag_session: StoredValue<Option<EventDragSession>>,
    pub drag_ghost: StoredValue<Option<EventDragGhost>>,
    pub drag_listeners: StoredValue<Vec<leptos::leptos_dom::helpers::WindowListenerHandle>>,
    /// When false, window drag listeners must not touch drag state (component teardown).
    pub drag_active: StoredValue<bool>,
    /// Bumps when drag ghost/session changes so the ghost view can repaint without `RwSignal` drag state.
    pub drag_repaint: RwSignal<u32>,
    pub lazy_context: Option<SchedulerLazyLoadContext>,
}

/// Read interaction context from the nearest scheduler product.
pub fn use_scheduler_interaction() -> SchedulerInteractionContext {
    expect_context::<SchedulerInteractionContext>()
}

/// Whether this event can be dragged given global and per-event flags.
pub fn event_is_draggable(event: &PlannedEvent, global: bool) -> bool {
    if !global {
        return false;
    }
    event.is_draggable.unwrap_or(true)
}

/// Whether this event can be resized given global and per-event flags.
pub fn event_is_resizable(event: &PlannedEvent, global: bool) -> bool {
    if !global {
        return false;
    }
    event.is_resizable.unwrap_or(true)
}

/// Whether this event can be edited (dialog) given per-event flag.
pub fn event_is_editable(event: &PlannedEvent) -> bool {
    event.is_editable.unwrap_or(true)
}

/// True while the scheduler product owns an active drag session (window listeners may fire).
pub fn scheduler_drag_active(ctx: &SchedulerInteractionContext) -> bool {
    ctx.drag_active.get_value()
}

/// Whether window drag listeners may read or write drag session state.
pub fn scheduler_drag_handlers_enabled(ctx: &SchedulerInteractionContext) -> bool {
    scheduler_drag_active(ctx)
}

fn scheduler_bump_drag_repaint(ctx: &SchedulerInteractionContext) {
    if scheduler_drag_handlers_enabled(ctx) {
        ctx.drag_repaint.update(|frame| *frame += 1);
    }
}

/// Read the in-flight drag session without subscribing.
pub fn scheduler_drag_session(ctx: &SchedulerInteractionContext) -> Option<EventDragSession> {
    ctx.drag_session.get_value()
}

/// Read the drag ghost without subscribing.
pub fn scheduler_drag_ghost(ctx: &SchedulerInteractionContext) -> Option<EventDragGhost> {
    ctx.drag_ghost.get_value()
}

/// Begin a drag session and show the ghost.
pub fn scheduler_start_drag(
    ctx: &SchedulerInteractionContext,
    session: EventDragSession,
    ghost: EventDragGhost,
) {
    ctx.drag_session.set_value(Some(session));
    ctx.drag_ghost.set_value(Some(ghost));
    ctx.drag_active.set_value(true);
    scheduler_bump_drag_repaint(ctx);
}

/// Replace the in-flight drag session draft.
pub fn scheduler_update_drag_session(ctx: &SchedulerInteractionContext, session: EventDragSession) {
    if !scheduler_drag_handlers_enabled(ctx) {
        return;
    }
    ctx.drag_session.set_value(Some(session));
    scheduler_bump_drag_repaint(ctx);
}

/// Move the drag ghost follower.
pub fn scheduler_update_drag_ghost(ctx: &SchedulerInteractionContext, ghost: EventDragGhost) {
    if !scheduler_drag_handlers_enabled(ctx) {
        return;
    }
    ctx.drag_ghost.set_value(Some(ghost));
    scheduler_bump_drag_repaint(ctx);
}

/// Clear drag session state and mark listeners inactive.
pub fn scheduler_end_drag_session(ctx: &SchedulerInteractionContext) {
    if !scheduler_drag_handlers_enabled(ctx) {
        return;
    }
    ctx.drag_session.set_value(None);
    ctx.drag_ghost.set_value(None);
    ctx.drag_active.set_value(false);
    // Repaint after drag_active is cleared; bump helper skips when inactive.
    ctx.drag_repaint.update(|frame| *frame += 1);
}

/// Write events to the signal and notify listeners.
pub fn commit_events(
    events: RwSignal<Vec<PlannedEvent>>,
    scheduler_events: StoredValue<SchedulerEvents>,
    next: Vec<PlannedEvent>,
) {
    commit_events_with_lazy(events, scheduler_events, None, next);
}

/// Write events to the signal, notify callbacks, and persist when lazy context is set.
pub fn commit_events_with_lazy(
    events: RwSignal<Vec<PlannedEvent>>,
    scheduler_events: StoredValue<SchedulerEvents>,
    lazy_context: Option<&SchedulerLazyLoadContext>,
    next: Vec<PlannedEvent>,
) {
    let before = events.get_untracked();
    events.set(next.clone());
    scheduler_events.with_value(|events_cb| events_cb.notify_events_change(next.clone()));
    if let Some(lazy_context) = lazy_context {
        persist_event_changes(lazy_context, &before, &next);
    }
}

/// Generate the next `evt-N` id from existing events.
pub fn next_event_id(events: &[PlannedEvent]) -> String {
    let max = events
        .iter()
        .filter_map(|e| {
            e.id.strip_prefix("evt-")
                .and_then(|s| s.parse::<u32>().ok())
        })
        .max()
        .unwrap_or(0);
    format!("evt-{}", max + 1)
}

/// Patch a single event in the collection by id.
pub fn patch_event(
    events: RwSignal<Vec<PlannedEvent>>,
    scheduler_events: StoredValue<SchedulerEvents>,
    lazy_context: Option<&SchedulerLazyLoadContext>,
    event_id: &str,
    patch: impl FnOnce(&mut PlannedEvent),
) {
    let master_id = master_event_id(event_id);
    let mut list = events.get_untracked();
    if let Some(event) = list.iter_mut().find(|e| e.id == master_id) {
        patch(event);
        commit_events_with_lazy(events, scheduler_events, lazy_context, list);
    }
}
