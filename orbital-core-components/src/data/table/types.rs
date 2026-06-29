use leptos::prelude::*;

/// Resizable column configuration for [`crate::TableHeaderCell`].
#[derive(Clone, Copy, Default)]
pub struct TableHeaderCellConfig {
    /// When true, enables column resize handles.
    pub resizable: bool,
    /// Minimum column width in pixels when resizable.
    pub min_width: Option<f64>,
    /// Maximum column width in pixels when resizable.
    pub max_width: Option<f64>,
    /// Fired on mouseup after a resize drag with the final width in pixels.
    pub on_resize_end: Option<Callback<f64, ()>>,
    /// Fired on double-click of the resize handle (autosize).
    pub on_autosize: Option<Callback<(), ()>>,
}

impl TableHeaderCellConfig {
    pub fn resizable(min_width: f64, max_width: f64) -> Self {
        Self {
            resizable: true,
            min_width: Some(min_width),
            max_width: Some(max_width),
            on_resize_end: None,
            on_autosize: None,
        }
    }
}

/// Truncation configuration for [`crate::TableCellLayout`].
#[derive(Clone, Copy, Default)]
pub struct TableCellLayoutConfig {
    /// When true, truncates overflowing text with an ellipsis.
    pub truncate: bool,
}

impl From<bool> for TableCellLayoutConfig {
    fn from(truncate: bool) -> Self {
        Self { truncate }
    }
}
