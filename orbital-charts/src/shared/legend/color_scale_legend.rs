//! Continuous color scale legend for heatmaps.

use leptos::prelude::*;
use orbital_core_components::{Caption1, Caption2, Material, MaterialElevation};

use crate::context::use_heatmap_plot_context;
use crate::engine::heatmap_value_domain;
use crate::{LegendConfig, LegendDirection, LegendHorizontalAlign};

use super::position::{legend_padding_style, legend_position_class};

/// Gradient bar legend for a continuous or piecewise color scale.
#[component]
pub fn ContinuousColorLegend(
    /// Legend layout configuration.
    #[prop(default = LegendConfig::default())]
    config: LegendConfig,
) -> impl IntoView {
    if config.hidden {
        return ().into_view().into_any();
    }

    let heatmap = use_heatmap_plot_context();
    let domain = heatmap_value_domain(&heatmap.cells, heatmap.value_min, heatmap.value_max);
    let colors = heatmap.color_scale.colors.clone();
    let gradient = if colors.len() >= 2 {
        format!("linear-gradient(to right, {})", colors.join(", "))
    } else {
        format!(
            "linear-gradient(to right, {}, {})",
            colors.first().cloned().unwrap_or_else(|| "#ccc".into()),
            colors.last().cloned().unwrap_or_else(|| "#333".into())
        )
    };

    let direction_class = match config.direction {
        LegendDirection::Row => "orb-legend--row",
        LegendDirection::Column => "orb-legend--column",
    };
    let position_class = legend_position_class(&config);
    let edge_class = match config.position.horizontal {
        LegendHorizontalAlign::Right => "orb-legend--outside-right",
        LegendHorizontalAlign::Left => "orb-legend--outside-left",
        _ => "",
    };
    let inset_style = legend_padding_style(&config);

    view! {
        <div
            class=format!("orb-color-scale-legend {direction_class} {position_class} {edge_class}")
            style=inset_style
        >
            <Material elevation=MaterialElevation::Raised class="orb-color-scale-surface">
                <div class="orb-color-scale-body">
                    <div class="orb-color-scale-bar" style=format!("background: {gradient};") />
                    <div class="orb-color-scale-labels">
                        <Caption2>{format!("{:.0}", domain.0)}</Caption2>
                        <Caption2>{format!("{:.0}", domain.1)}</Caption2>
                    </div>
                    <Caption1 class="orb-color-scale-caption">"Intensity"</Caption1>
                </div>
            </Material>
        </div>
    }
    .into_any()
}
