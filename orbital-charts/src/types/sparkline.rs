//! Sparkline chart types.

/// Line or bar rendering for a sparkline.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SparklinePlotType {
    /// Connected line stroke (default).
    #[default]
    Line,
    /// Vertical bar marks.
    Bar,
}
