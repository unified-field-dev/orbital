//! Shared chart configuration types.

/// Chart layout orientation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartOrientation {
    #[default]
    Vertical,
    Horizontal,
}

/// Space between the SVG border and the plot area. User docs: "plot inset".
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PlotInset {
    /// Top inset in pixels.
    pub top: f64,
    /// Bottom inset in pixels.
    pub bottom: f64,
    /// Left inset in pixels.
    pub left: f64,
    /// Right inset in pixels.
    pub right: f64,
}

impl PlotInset {
    /// Uniform inset on all sides.
    pub fn uniform(value: f64) -> Self {
        Self {
            top: value,
            bottom: value,
            left: value,
            right: value,
        }
    }

    /// Default asymmetric inset that reserves space for axis tick and title labels.
    pub fn with_axes() -> Self {
        Self {
            top: 36.0,
            right: 40.0,
            bottom: 72.0,
            left: 80.0,
        }
    }
}

/// Background grid line configuration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GridConfig {
    /// Draw horizontal grid lines.
    pub horizontal: bool,
    /// Draw vertical grid lines.
    pub vertical: bool,
}

/// Highlight mode for hovered or selected items.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HighlightMode {
    #[default]
    Item,
    Series,
    None,
}

/// Fade mode for non-highlighted items.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FadeMode {
    #[default]
    Global,
    None,
}

/// Scope controlling highlight and fade behavior.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HighlightScope {
    /// What to highlight on interaction.
    pub highlight: HighlightMode,
    /// How non-highlighted items fade.
    pub fade: FadeMode,
}

/// Kind of color scale applied to an axis or series.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColorScaleKind {
    #[default]
    Ordinal,
    Continuous,
    Piecewise,
}

/// Color scale mapping values to colors.
#[derive(Clone, Debug, Default)]
pub struct ColorScale {
    /// Scale kind.
    pub kind: ColorScaleKind,
    /// Color stops or palette entries.
    pub colors: Vec<String>,
    /// Thresholds for piecewise scales.
    pub thresholds: Option<Vec<f64>>,
}

/// Domain limit strategy for value axes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DomainLimit {
    #[default]
    Nice,
    Strict,
}

/// Tooltip activation trigger mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipTrigger {
    #[default]
    Item,
    Axis,
    None,
}
