//! Embed mode and overlay mount targets for charts in scroll, dialog, and table hosts.

/// How a chart is embedded in its host layout.
///
/// Controls portal mount resolution and overflow CSS hooks on the chart root.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartEmbedMode {
    /// Default — chart root is the overlay mount target.
    #[default]
    Inline,
    /// Chart inside a [`ScrollArea`](orbital_core_components::ScrollArea) or other scroll parent.
    ScrollHost,
    /// Chart inside a dialog overlay.
    DialogHost,
    /// Chart inside a table cell or compact dashboard slot.
    TableCell,
}

impl ChartEmbedMode {
    /// `data-orbital-chart-embed` attribute value for the chart root.
    pub fn data_attr(&self) -> &'static str {
        match self {
            Self::Inline => "inline",
            Self::ScrollHost => "scroll-host",
            Self::DialogHost => "dialog-host",
            Self::TableCell => "table-cell",
        }
    }
}

/// Portal mount override for chart overlay chrome.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum OverlayMount {
    /// Mount into the chart-local [`ChartOverlayLayer`](crate::ChartOverlayLayer).
    #[default]
    ChartLocal,
    /// Mount into a host element identified by `id` or `data-orbital-chart-host`.
    HostElement {
        /// Host element id or `data-orbital-chart-host` value.
        id: String,
    },
}
