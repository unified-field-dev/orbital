//! Axis click routing for category band axes.

use leptos::prelude::*;

use crate::context::{is_series_visible, use_chart_context, use_x_scale, use_y_scale};
use crate::engine::projected_for_plot_type;
use crate::{AxisClickData, ChartOrientation, ChartScale, ChartType};

/// Transparent band rects for category axis click handling.
#[component]
pub fn AxisClickLayer(
    /// Category axis id (defaults to `"x"` vertical / `"y"` horizontal).
    #[prop(default = None)]
    axis_id: Option<String>,
    /// Bar orientation override.
    #[prop(default = None)]
    orientation: Option<ChartOrientation>,
) -> impl IntoView {
    let ctx = use_chart_context();
    let on_axis = ctx.on_axis_click;
    let orient = orientation.unwrap_or(ctx.orientation);
    let projected = ctx.projected.clone();
    let series_defs = ctx.series.clone();
    let plot_width = ctx.drawing_area.plot_width;
    let plot_height = ctx.drawing_area.plot_height;

    let category_axis_id = axis_id.unwrap_or_else(|| match orient {
        ChartOrientation::Vertical => "x".to_string(),
        ChartOrientation::Horizontal => "y".to_string(),
    });

    let category_scale = match orient {
        ChartOrientation::Vertical => use_x_scale(category_axis_id.clone()),
        ChartOrientation::Horizontal => use_y_scale(category_axis_id.clone()),
    };

    view! {
        {move || {
            if on_axis.is_none() {
                return ().into_any();
            }
            let Some(data) = projected.as_ref() else {
                return ().into_any();
            };
            let plot_data = projected_for_plot_type(data, &series_defs, ChartType::Bar);
            if plot_data.categories.is_empty() {
                return ().into_any();
            }

            let bands = axis_click_bands(
                orient,
                &plot_data.categories,
                &category_scale,
                plot_width,
                plot_height,
            );

            bands
                .into_iter()
                .map(|band| {
                    let axis_id = category_axis_id.clone();
                    let on_axis = on_axis;
                    let projected = projected.clone();
                    let series_defs = series_defs.clone();
                    view! {
                        <rect
                            class="orb-axis-click-band"
                            x=band.x
                            y=band.y
                            width=band.w
                            height=band.h
                            on:click=move |ev| {
                                ev.stop_propagation();
                                if let Some(cb) = &on_axis {
                                    let series_values = projected
                                        .as_ref()
                                        .map(|p| {
                                            let plot_data = projected_for_plot_type(
                                                p,
                                                &series_defs,
                                                ChartType::Bar,
                                            );
                                            plot_data
                                                .series
                                                .iter()
                                                .filter(|s| is_series_visible(&s.id))
                                                .map(|s| {
                                                    (
                                                        s.id.clone(),
                                                        s.data.get(band.index).copied().unwrap_or(0.0),
                                                    )
                                                })
                                                .collect()
                                        })
                                        .unwrap_or_default();
                                    cb.run((AxisClickData {
                                        axis_id: axis_id.clone(),
                                        value: band.index as f64,
                                        series_values,
                                    },));
                                }
                            }
                        />
                    }
                })
                .collect_view()
                .into_any()
        }}
    }
}

struct ClickBand {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    index: usize,
}

fn axis_click_bands(
    orient: ChartOrientation,
    categories: &[String],
    category_scale: &ChartScale,
    plot_width: f64,
    plot_height: f64,
) -> Vec<ClickBand> {
    let ChartScale::Band(band) = category_scale else {
        return Vec::new();
    };
    let bw = band.bandwidth();
    categories
        .iter()
        .enumerate()
        .filter_map(|(i, cat)| {
            band.scale(cat).map(|center| match orient {
                ChartOrientation::Vertical => ClickBand {
                    x: center - bw / 2.0,
                    y: 0.0,
                    w: bw,
                    h: plot_height,
                    index: i,
                },
                ChartOrientation::Horizontal => ClickBand {
                    x: 0.0,
                    y: center - bw / 2.0,
                    w: plot_width,
                    h: bw,
                    index: i,
                },
            })
        })
        .collect()
}
