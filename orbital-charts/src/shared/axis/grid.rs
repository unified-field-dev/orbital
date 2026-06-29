//! [`ChartGrid`] — horizontal and vertical grid lines.

use leptos::prelude::*;

use crate::context::{use_chart_context, use_x_scale, use_y_scale, use_y_ticks};

use crate::axis_categories;
use crate::{ChartScale, GridConfig, ScaleType};

/// Renders background grid lines inside the plot area (relative coordinates).
#[component]
pub fn ChartGrid(
    /// Override horizontal grid lines (defaults to context `grid` config).
    #[prop(optional)]
    horizontal: Option<bool>,
    /// Override vertical grid lines (defaults to context `grid` config).
    #[prop(optional)]
    vertical: Option<bool>,
) -> impl IntoView {
    let ctx = use_chart_context();
    let config = ctx.grid.unwrap_or(GridConfig {
        horizontal: true,
        vertical: false,
    });
    let show_h = horizontal.unwrap_or(config.horizontal);
    let show_v = vertical.unwrap_or(config.vertical);
    let area = ctx.drawing_area;

    let x_axis = ctx.x_axes.first().cloned().unwrap_or_default();
    let y_axis = ctx.y_axes.first().cloned().unwrap_or_default();
    let x_scale = use_x_scale(x_axis.id.clone());
    let y_scale = use_y_scale(y_axis.id.clone());
    let y_tick_values = use_y_ticks(y_axis.id.clone());
    let categories = axis_categories(&x_axis, ctx.projected.as_ref());

    view! {
        <g class="orb-grid">
            {show_h.then(|| view! {
                {y_tick_values.into_iter().map(|value| {
                    let y = match &y_scale {
                        ChartScale::Linear(s) => s.scale(value),
                        _ => 0.0,
                    };
                    view! {
                        <line
                            class="orb-grid-line orb-grid-line--horizontal"
                            x1=0.0
                            y1=y
                            x2=area.plot_width
                            y2=y
                        />
                    }
                }).collect_view()}
            })}
            {show_v.then(|| {
                let positions: Vec<f64> = match (&x_axis.scale_type, &x_scale) {
                    (ScaleType::Band | ScaleType::Point, ChartScale::Band(band)) => categories
                        .iter()
                        .filter_map(|cat| band.scale(cat))
                        .collect(),
                    _ => Vec::new(),
                };
                view! {
                    {positions.into_iter().map(|x| view! {
                        <line
                            class="orb-grid-line orb-grid-line--vertical"
                            x1=x
                            y1=0.0
                            x2=x
                            y2=area.plot_height
                        />
                    }).collect_view()}
                }
            })}
        </g>
    }
}
