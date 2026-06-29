use crate::engine::{ColumnPinMeta, ResolvedColumn};
use crate::types::{CellAlign, PinSide};

/// Inline width style for a resolved column.
pub fn column_width_style(resolved: &ResolvedColumn) -> String {
    let min = resolved.def.min_width;
    let max = resolved.def.max_width;
    let mut style = format!("width: {:.2}px", resolved.width_px);
    if let Some(min) = min {
        style.push_str(&format!(";min-width: {min:.2}px"));
    }
    if let Some(max) = max {
        style.push_str(&format!(";max-width: {max:.2}px"));
    }
    style
}

/// CSS classes for pin + alignment on header/body cells.
pub fn column_cell_classes(resolved: &ResolvedColumn) -> String {
    let mut parts = Vec::new();
    if let Some(pin) = resolved.pin {
        match pin.side {
            PinSide::Left => {
                parts.push("orbital-data-table__pinned-left".to_string());
                if pin.is_last_left {
                    parts.push("orbital-data-table__pinned-left-last".to_string());
                }
            }
            PinSide::Right => {
                parts.push("orbital-data-table__pinned-right".to_string());
                if pin.is_first_right {
                    parts.push("orbital-data-table__pinned-right-first".to_string());
                }
            }
        }
    }
    match resolved.def.align {
        CellAlign::Left => parts.push("orbital-data-table__align-left".to_string()),
        CellAlign::Center => parts.push("orbital-data-table__align-center".to_string()),
        CellAlign::Right => parts.push("orbital-data-table__align-right".to_string()),
    }
    parts.join(" ")
}

/// Inline sticky pin style.
pub fn column_pin_style(pin: &ColumnPinMeta) -> String {
    match pin.side {
        PinSide::Left => format!("left: {:.2}px; z-index: {};", pin.offset_px, pin.z_index),
        PinSide::Right => format!("right: {:.2}px; z-index: {};", pin.offset_px, pin.z_index),
    }
}

/// Combined width + pin inline style.
pub fn column_combined_style(resolved: &ResolvedColumn) -> String {
    let mut style = column_width_style(resolved);
    if let Some(pin) = resolved.pin {
        style.push(';');
        style.push_str(&column_pin_style(&pin));
    }
    style
}
