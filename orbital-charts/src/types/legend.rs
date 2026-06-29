//! Legend configuration types.

/// Legend item flow direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LegendDirection {
    #[default]
    Column,
    Row,
}

/// Vertical alignment of the legend within the chart shell.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LegendVerticalAlign {
    Top,
    #[default]
    Middle,
    Bottom,
}

/// Horizontal alignment of the legend within the chart shell.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LegendHorizontalAlign {
    Left,
    #[default]
    Middle,
    Right,
}

/// Legend placement within the chart shell.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LegendPosition {
    /// Vertical alignment.
    pub vertical: LegendVerticalAlign,
    /// Horizontal alignment.
    pub horizontal: LegendHorizontalAlign,
}

/// Configuration for the embedded or composed legend.
#[derive(Clone, Debug, PartialEq)]
pub struct LegendConfig {
    /// When true, the legend is not rendered.
    pub hidden: bool,
    /// Item layout direction.
    pub direction: LegendDirection,
    /// Placement within the chart shell.
    pub position: LegendPosition,
    /// Inset from the chart shell edge in pixels.
    pub padding: f64,
    /// Gap between legend items in pixels.
    pub item_gap: f64,
    /// Gap between color mark and label in pixels.
    pub mark_gap: f64,
    /// Color swatch width and height in pixels.
    pub item_mark_size: f64,
    /// When true, series visibility checkboxes are not shown.
    pub disable_series_toggle: bool,
}

impl Default for LegendConfig {
    fn default() -> Self {
        Self {
            hidden: false,
            direction: LegendDirection::Column,
            position: LegendPosition {
                vertical: LegendVerticalAlign::Middle,
                horizontal: LegendHorizontalAlign::Right,
            },
            padding: 8.0,
            item_gap: 8.0,
            mark_gap: 6.0,
            item_mark_size: 12.0,
            disable_series_toggle: false,
        }
    }
}
