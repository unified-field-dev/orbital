//! Drag session and ghost types (SC-08, SC-20).

use chrono::NaiveDate;

use orbital_base_components::OrbitalDateTime;

use crate::PlannedEvent;

const GHOST_OFFSET_X: f32 = 8.0;
const GHOST_OFFSET_Y: f32 = 8.0;

/// What pointer interaction is active on an event chip.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragMode {
    Move,
    ResizeStart,
    ResizeEnd,
}

/// Active drag/resize session state.
#[derive(Clone, Debug, PartialEq)]
pub struct EventDragSession {
    pub event_id: String,
    pub mode: DragMode,
    pub origin_day: NaiveDate,
    pub origin_resource_id: Option<String>,
    pub origin_start: OrbitalDateTime,
    pub origin_end: OrbitalDateTime,
    /// Working copy updated during pointer move.
    pub draft: PlannedEvent,
    pub moved: bool,
}

/// Floating preview shown while dragging an event.
#[derive(Clone, Debug, PartialEq)]
pub struct EventDragGhost {
    pub title: String,
    pub width_px: f32,
    pub height_px: f32,
    pub x: f32,
    pub y: f32,
}

pub fn event_drag_ghost_at_pointer(
    title: String,
    width_px: f32,
    height_px: f32,
    x: f32,
    y: f32,
) -> EventDragGhost {
    EventDragGhost {
        title,
        width_px,
        height_px,
        x: x - GHOST_OFFSET_X,
        y: y - GHOST_OFFSET_Y,
    }
}

pub fn move_event_drag_ghost(ghost: &mut EventDragGhost, x: f32, y: f32) {
    ghost.x = x - GHOST_OFFSET_X;
    ghost.y = y - GHOST_OFFSET_Y;
}
