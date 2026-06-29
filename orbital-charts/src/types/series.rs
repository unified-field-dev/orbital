//! Chart series definitions and related enums.

use leptos::callback::Callback;

use super::{LabelLocation, ScatterPoint};

/// Supported chart geometry families.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartType {
    #[default]
    Bar,
    Line,
    Area,
    Scatter,
    Pie,
}

/// Stacking offset strategy for grouped series.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StackOffset {
    #[default]
    None,
    Expand,
    Diverging,
}

/// Stacking order for grouped series.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StackOrder {
    #[default]
    None,
    Reverse,
    Appearance,
    Ascending,
    Descending,
}

/// Line and area curve interpolation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CurveType {
    #[default]
    Linear,
    Monotone,
    Step,
    Natural,
}

/// Configuration for series data labels.
#[derive(Clone, Debug, Default)]
pub struct SeriesLabelConfig {
    /// Whether labels are shown.
    pub show: Option<bool>,
    /// Custom label formatter.
    pub formatter: Option<Callback<(f64,), String>>,
}

/// Configuration for bar-specific labels.
#[derive(Clone, Debug, Default)]
pub struct BarLabelConfig {
    /// Whether bar labels are shown.
    pub show: Option<bool>,
    /// Custom bar label formatter.
    pub formatter: Option<Callback<(f64, usize), String>>,
}

/// Definition of one data series in a chart.
#[derive(Clone, Debug, Default)]
pub struct SeriesDef {
    /// Unique series identifier.
    pub id: String,
    /// Display label in legend and tooltips.
    pub label: Option<String>,
    /// Location-aware label formatter (overrides `label` when set).
    pub label_formatter: Option<Callback<(LabelLocation,), String>>,
    /// Required in composition mode; inferred in `*Chart` components.
    pub chart_type: Option<ChartType>,
    /// Binds to [`orbital_data::Dataset`] field key when using tabular data.
    pub field: Option<String>,
    /// Inline values when not using a dataset.
    pub data: Option<Vec<f64>>,
    /// Stack group identifier for stacked rendering.
    pub stack_group: Option<String>,
    /// Stack offset strategy; applies to the whole stack group when set on any member.
    pub stack_offset: Option<StackOffset>,
    /// Stack layer order; applies to the whole stack group when set on any member.
    pub stack_order: Option<StackOrder>,
    /// Per-series color override.
    pub color: Option<String>,
    /// Highlight and fade behavior for this series.
    pub highlight_scope: Option<crate::HighlightScope>,
    /// Value formatter for tooltips and labels.
    pub value_formatter: Option<Callback<(f64,), String>>,
    /// Series label configuration.
    pub label_config: Option<SeriesLabelConfig>,
    /// Line/area curve interpolation.
    pub curve: Option<CurveType>,
    /// Whether to render as area fill.
    pub area: Option<bool>,
    /// Whether to show point markers on lines.
    pub show_markers: Option<bool>,
    /// Whether to connect across null values.
    pub connect_nulls: Option<bool>,
    /// Bar-specific label configuration.
    pub bar_label: Option<BarLabelConfig>,
    /// Corner radius for bar marks.
    pub corner_radius: Option<f64>,
    /// Inline scatter points when not using a dataset.
    pub scatter_data: Option<Vec<ScatterPoint>>,
    /// X-axis id for scatter series (default `"x"`).
    pub x_axis_id: Option<String>,
    /// Y-axis id for scatter series (default `"y"`).
    pub y_axis_id: Option<String>,
    /// Scatter marker radius in pixels.
    pub marker_size: Option<f64>,
}
