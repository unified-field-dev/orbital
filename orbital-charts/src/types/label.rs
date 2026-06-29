//! Label location surfaces for series and slice names.

/// Surface where a chart label is rendered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelLocation {
    /// Legend item text.
    Legend,
    /// Tooltip series or slice name.
    Tooltip,
    /// Pie arc label text.
    Arc,
}
