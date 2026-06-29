/// Default number of extra rows rendered above/below the visible window.
pub const DEFAULT_ROW_OVERSCAN: usize = 5;
/// Default number of extra columns rendered left/right of the visible window.
pub const DEFAULT_COL_OVERSCAN: usize = 2;

/// Virtual row window with spacer padding heights.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RowViewport {
    pub start: usize,
    pub end: usize,
    pub padding_top_px: f64,
    pub padding_bottom_px: f64,
}

/// Virtual column window with spacer padding widths.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColumnViewport {
    pub start: usize,
    pub end: usize,
    pub padding_left_px: f64,
    pub padding_right_px: f64,
}

/// Compute which row indices to render for a scroll position.
pub fn compute_row_viewport(
    scroll_top: f64,
    viewport_height: f64,
    row_count: usize,
    row_height: f64,
    overscan: usize,
) -> RowViewport {
    if row_count == 0 || row_height <= 0.0 {
        return RowViewport {
            start: 0,
            end: 0,
            padding_top_px: 0.0,
            padding_bottom_px: 0.0,
        };
    }

    let first_visible = (scroll_top / row_height).floor() as usize;
    let visible_count = ((viewport_height / row_height).ceil() as usize).max(1);
    let start = first_visible.saturating_sub(overscan);
    let end = (first_visible + visible_count + overscan).min(row_count);

    RowViewport {
        start,
        end,
        padding_top_px: start as f64 * row_height,
        padding_bottom_px: (row_count.saturating_sub(end)) as f64 * row_height,
    }
}

/// Compute which column indices to render for a horizontal scroll position.
pub fn compute_column_viewport(
    scroll_left: f64,
    viewport_width: f64,
    column_widths: &[f64],
    overscan: usize,
) -> ColumnViewport {
    let col_count = column_widths.len();
    if col_count == 0 {
        return ColumnViewport {
            start: 0,
            end: 0,
            padding_left_px: 0.0,
            padding_right_px: 0.0,
        };
    }

    let offsets = super::scroll::column_left_offsets(column_widths);
    let total_width: f64 = column_widths.iter().sum();

    let mut start = 0usize;
    for (i, &off) in offsets.iter().enumerate() {
        let w = column_widths[i];
        if off + w > scroll_left {
            start = i;
            break;
        }
        if i == col_count - 1 {
            start = col_count;
        }
    }

    let right_edge = scroll_left + viewport_width;
    let mut end = col_count;
    for (i, &off) in offsets.iter().enumerate() {
        if off >= right_edge {
            end = i;
            break;
        }
    }

    let start = start.saturating_sub(overscan);
    let end = (end + overscan).min(col_count);

    let padding_left = offsets.get(start).copied().unwrap_or(0.0);
    let padding_right = total_width - offsets.get(end).unwrap_or(&total_width);

    ColumnViewport {
        start,
        end,
        padding_left_px: padding_left,
        padding_right_px: padding_right.max(0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_viewport_covers_visible_range() {
        let vp = compute_row_viewport(200.0, 400.0, 100, 40.0, 2);
        assert!(vp.start <= 5);
        assert!(vp.end >= 15);
        assert_eq!(vp.padding_top_px, vp.start as f64 * 40.0);
    }

    #[test]
    fn column_viewport_empty() {
        let vp = compute_column_viewport(0.0, 500.0, &[], 2);
        assert_eq!(vp.start, 0);
        assert_eq!(vp.end, 0);
    }
}
