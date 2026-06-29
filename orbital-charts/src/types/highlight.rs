//! Axis highlight configuration types.

/// Axis highlight rendering mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AxisHighlightMode {
    /// No highlight on this axis dimension.
    #[default]
    None,
    /// Line crosshair at pointer position.
    Line,
    /// Shaded band for band-scale categories (x-axis only).
    Band,
}

/// Crosshair and band highlight configuration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AxisHighlightConfig {
    /// X-axis highlight mode.
    pub x: AxisHighlightMode,
    /// Y-axis highlight mode.
    pub y: AxisHighlightMode,
}

impl AxisHighlightConfig {
    /// Default for vertical bar charts: category band on x.
    pub fn bar_default() -> Self {
        Self {
            x: AxisHighlightMode::Band,
            y: AxisHighlightMode::None,
        }
    }
}
