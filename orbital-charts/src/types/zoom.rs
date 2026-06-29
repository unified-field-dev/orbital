//! Controlled zoom window state.

/// Percent window (0–100) over an axis's full domain.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoomWindow {
    /// Axis id matching [`crate::AxisDef::id`].
    pub axis_id: String,
    /// Start of visible range as percent of full domain (0 = min).
    pub start: f64,
    /// End of visible range as percent of full domain (100 = max).
    pub end: f64,
}

impl ZoomWindow {
    /// Full visible range for an axis.
    pub fn full(axis_id: impl Into<String>) -> Self {
        Self {
            axis_id: axis_id.into(),
            start: 0.0,
            end: 100.0,
        }
    }
}
