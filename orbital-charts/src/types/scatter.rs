//! Scatter chart types.

/// One scatter point with required identity.
#[derive(Clone, Debug, PartialEq)]
pub struct ScatterPoint {
    /// X value in data space.
    pub x: f64,
    /// Y value in data space.
    pub y: f64,
    /// Unique point identifier.
    pub id: String,
    /// Optional third dimension for color/size mapping.
    pub z: Option<f64>,
}

/// One scatter series with points and axis bindings.
#[derive(Clone, Debug, PartialEq)]
pub struct ScatterSeriesData {
    /// Series identifier.
    pub series_id: String,
    /// Display label.
    pub label: String,
    /// Point data.
    pub points: Vec<ScatterPoint>,
    /// X-axis id binding.
    pub x_axis_id: String,
    /// Y-axis id binding.
    pub y_axis_id: String,
    /// Optional per-series color.
    pub color: Option<String>,
    /// Marker radius in pixels.
    pub marker_size: f64,
}

/// Projected scatter data ready for scale mapping.
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectedScatterData {
    /// One or more scatter series.
    pub series: Vec<ScatterSeriesData>,
}

/// Pixel-space scatter point for hit testing.
#[derive(Clone, Debug, PartialEq)]
pub struct PlacedScatterPoint {
    /// Pixel x in plot coordinates.
    pub px: f64,
    /// Pixel y in plot coordinates.
    pub py: f64,
    /// Series identifier.
    pub series_id: String,
    /// Index within the series.
    pub data_index: usize,
}
