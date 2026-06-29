//! Authoritative z-order contract for chart composition children.
//!
//! See [`ChartEmbedMode`](crate::ChartEmbedMode) and the crate README for embed and portal rules.

/// Documented child order for chart composition — no runtime enforcement.
///
/// # Plot group (`orb-plot-content`)
///
/// 1. [`ChartGrid`](crate::ChartGrid)
/// 2. [`PlotClip`](crate::PlotClip) wrapping plot marks
/// 3. Plot children inside clip (`BarPlot`, `LinePlot`, custom layers) — first child = bottom
/// 4. [`ChartZoomLayer`](crate::ChartZoomLayer) (unclipped)
/// 5. [`ChartPointerLayer`](crate::ChartPointerLayer)
/// 6. [`AxisHighlight`](crate::AxisHighlight)
///
/// # SVG axes (outside plot `<g>`)
///
/// Y axes (left, then right), then X axis.
///
/// # Overlay layer ([`ChartOverlayLayer`](crate::ChartOverlayLayer))
///
/// Loading / empty / error overlays → [`Legend`](crate::Legend) → [`ChartTooltip`](crate::ChartTooltip).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartCompositionOrder {
    #[default]
    Default,
}
