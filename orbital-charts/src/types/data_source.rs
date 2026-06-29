//! Chart data source types.

use orbital_data::Dataset;

/// How chart data is supplied to the container.
#[derive(Clone, Debug)]
pub enum ChartDataSource {
    /// Primary: shared [`Dataset`] from table handle, server, or static fixture.
    Dataset(Dataset),
    /// Secondary: inline arrays for docs and demos.
    Inline(InlineSeriesData),
}

/// Inline series data for standalone demos.
#[derive(Clone, Debug, Default)]
pub struct InlineSeriesData {
    /// Category axis labels.
    pub x: Vec<String>,
    /// One or more inline series.
    pub series: Vec<InlineSeries>,
}

/// One inline data series.
#[derive(Clone, Debug, Default)]
pub struct InlineSeries {
    /// Series identifier.
    pub id: String,
    /// Display label.
    pub label: String,
    /// Numeric values aligned with `InlineSeriesData::x`.
    pub data: Vec<f64>,
}
