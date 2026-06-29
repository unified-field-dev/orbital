//! Chart event types.

use leptos::callback::Callback;

/// Identifier for a clicked or hovered chart item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChartItemId {
    /// Series that owns the item.
    pub series_id: String,
    /// Index within the series data array.
    pub data_index: usize,
}

/// Data emitted when an axis is clicked.
#[derive(Clone, Debug, PartialEq)]
pub struct AxisClickData {
    /// Axis that was clicked.
    pub axis_id: String,
    /// Axis value at the click position.
    pub value: f64,
    /// All series values at the click position.
    pub series_values: Vec<(String, f64)>,
}

/// Chart interaction event callbacks.
#[derive(Clone, Debug, Default)]
pub struct ChartEvents {
    /// Fired when a chart mark is clicked.
    pub on_item_click: Option<Callback<(ChartItemId,), ()>>,
    /// Fired when an axis is clicked.
    pub on_axis_click: Option<Callback<(AxisClickData,), ()>>,
    /// Fired when a legend entry is clicked.
    pub on_legend_click: Option<Callback<(String,), ()>>,
}
