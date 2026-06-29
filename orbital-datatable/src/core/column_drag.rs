/// Floating preview shown while drag-reordering a column.
#[derive(Clone, Debug, PartialEq)]
pub struct ColumnDragGhost {
    pub label: String,
    pub width_px: f32,
    pub x: f32,
    pub y: f32,
}

const GHOST_OFFSET_X: f32 = 12.0;
const GHOST_OFFSET_Y: f32 = 12.0;

pub fn column_drag_ghost_at_pointer(
    label: String,
    width_px: f32,
    x: f32,
    y: f32,
) -> ColumnDragGhost {
    ColumnDragGhost {
        label,
        width_px,
        x: x - GHOST_OFFSET_X,
        y: y - GHOST_OFFSET_Y,
    }
}

pub fn move_column_drag_ghost(ghost: &mut ColumnDragGhost, x: f32, y: f32) {
    ghost.x = x - GHOST_OFFSET_X;
    ghost.y = y - GHOST_OFFSET_Y;
}
