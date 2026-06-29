/// Floating preview shown while drag-reordering a row.
#[derive(Clone, Debug, PartialEq)]
pub struct RowDragGhost {
    pub cells: Vec<String>,
    pub widths_px: Vec<f32>,
    pub x: f32,
    pub y: f32,
}

const GHOST_OFFSET_X: f32 = 12.0;
const GHOST_OFFSET_Y: f32 = 12.0;

pub fn row_drag_ghost_at_pointer(
    cells: Vec<String>,
    widths_px: Vec<f32>,
    x: f32,
    y: f32,
) -> RowDragGhost {
    RowDragGhost {
        cells,
        widths_px,
        x: x - GHOST_OFFSET_X,
        y: y - GHOST_OFFSET_Y,
    }
}

pub fn move_row_drag_ghost(ghost: &mut RowDragGhost, x: f32, y: f32) {
    ghost.x = x - GHOST_OFFSET_X;
    ghost.y = y - GHOST_OFFSET_Y;
}
