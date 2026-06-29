//! Heatmap chart types.

/// Synthetic series id used for heatmap cell hover and tooltips.
pub const HEATMAP_ITEM_SERIES_ID: &str = "__heatmap__";

/// One heatmap cell: x index, y index, and numeric value (z).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeatmapCell {
    /// Index into x category list.
    pub x: usize,
    /// Index into y category list.
    pub y: usize,
    /// Cell value mapped through the color scale.
    pub value: f64,
}

/// Layout for a single rendered heatmap cell.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatmapCellLayout {
    /// Cell index in the input slice.
    pub index: usize,
    /// X category index.
    pub x_index: usize,
    /// Y category index.
    pub y_index: usize,
    /// Left x in plot coordinates.
    pub x: f64,
    /// Top y in plot coordinates.
    pub y: f64,
    /// Cell width.
    pub width: f64,
    /// Cell height.
    pub height: f64,
    /// Resolved fill color.
    pub fill: String,
    /// Original z value.
    pub value: f64,
}

/// Projected heatmap data ready for layout.
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectedHeatmapData {
    /// X category labels.
    pub x_categories: Vec<String>,
    /// Y category labels.
    pub y_categories: Vec<String>,
    /// Cell tuples.
    pub cells: Vec<HeatmapCell>,
}
