//! Heatmap plot context for cell data separate from cartesian projection.

use leptos::prelude::*;

use crate::{ColorScale, HeatmapCell};

/// Heatmap cell data and color scale provided by [`Heatmap`].
#[derive(Clone, Debug)]
pub struct HeatmapPlotContext {
    /// Cell tuples `[x_index, y_index, value]`.
    pub cells: Vec<HeatmapCell>,
    /// Z-axis color scale.
    pub color_scale: ColorScale,
    /// Optional domain minimum override.
    pub value_min: Option<f64>,
    /// Optional domain maximum override.
    pub value_max: Option<f64>,
}

/// Read heatmap cell data from context inside the plot layer.
pub fn use_heatmap_plot_context() -> HeatmapPlotContext {
    expect_context::<HeatmapPlotContext>()
}

/// Provide heatmap plot data to child layers.
#[component]
pub fn HeatmapPlotProvider(context: HeatmapPlotContext, children: Children) -> impl IntoView {
    provide_context(context);
    children()
}
