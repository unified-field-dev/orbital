/// Scroll alignment when programmatically scrolling to a row or column.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ScrollAlignment {
    #[default]
    Start,
    Center,
    End,
}

/// Compute scroll offset to bring a row index into view.
pub fn scroll_offset_for_row(
    row_index: usize,
    row_height_px: f64,
    viewport_height: f64,
    align: ScrollAlignment,
) -> f64 {
    let row_top = row_index as f64 * row_height_px;
    match align {
        ScrollAlignment::Start => row_top,
        ScrollAlignment::Center => (row_top - viewport_height / 2.0 + row_height_px / 2.0).max(0.0),
        ScrollAlignment::End => (row_top - viewport_height + row_height_px).max(0.0),
    }
}

/// Compute scroll offset to bring a column index into view.
pub fn scroll_offset_for_column(
    column_index: usize,
    column_offsets: &[f64],
    column_widths: &[f64],
    viewport_width: f64,
    align: ScrollAlignment,
) -> f64 {
    if column_offsets.is_empty() || column_index >= column_offsets.len() {
        return 0.0;
    }
    let col_left = column_offsets[column_index];
    let col_width = column_widths.get(column_index).copied().unwrap_or(100.0);
    match align {
        ScrollAlignment::Start => col_left,
        ScrollAlignment::Center => (col_left - viewport_width / 2.0 + col_width / 2.0).max(0.0),
        ScrollAlignment::End => (col_left - viewport_width + col_width).max(0.0),
    }
}

/// Build cumulative left offsets for visible columns.
pub fn column_left_offsets(widths: &[f64]) -> Vec<f64> {
    let mut offsets = Vec::with_capacity(widths.len());
    let mut acc = 0.0;
    for w in widths {
        offsets.push(acc);
        acc += w;
    }
    offsets
}

#[cfg(feature = "hydrate")]
pub fn set_scroll_top(el: &web_sys::HtmlElement, top: f64) {
    el.set_scroll_top(top as i32);
}

#[cfg(feature = "hydrate")]
pub fn set_scroll_left(el: &web_sys::HtmlElement, left: f64) {
    el.set_scroll_left(left as i32);
}

#[cfg(feature = "hydrate")]
pub fn scroll_dimensions(el: &web_sys::HtmlElement) -> (f64, f64) {
    (el.scroll_top() as f64, el.client_height() as f64)
}

#[cfg(feature = "hydrate")]
pub fn scroll_dimensions_horizontal(el: &web_sys::HtmlElement) -> (f64, f64) {
    (el.scroll_left() as f64, el.client_width() as f64)
}
