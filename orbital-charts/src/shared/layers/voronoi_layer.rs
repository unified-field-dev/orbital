//! Voronoi-style nearest-neighbor hit testing layer for scatter charts.

use leptos::prelude::*;

use crate::context::{use_chart_context, use_hovered_item, ChartScales};
use crate::engine::find_nearest_point;
use crate::{ChartItemId, ChartScale, PlacedScatterPoint};

/// Transparent overlay for nearest-point hover and click hit testing.
#[component]
pub fn VoronoiLayer(
    /// Maximum pointer distance for hit selection (pixels); `None` = unlimited.
    #[prop(default = None)]
    voronoi_max_radius: Option<f64>,
    /// When true, this layer is not rendered (use direct point hit targets).
    #[prop(default = false)]
    disable_voronoi: bool,
) -> impl IntoView {
    if disable_voronoi {
        return {
            let _: () = view! { <></> };
            ().into_any()
        };
    }

    let ctx = use_chart_context();
    let scatter = ctx.scatter.clone();
    let plot_w = ctx.drawing_area.plot_width;
    let plot_h = ctx.drawing_area.plot_height;
    let on_item = ctx.on_item_click;
    let scales = expect_context::<ChartScales>().clone();
    let hovered = use_hovered_item();

    let placed_points = Memo::new(move |_| {
        let Some(data) = scatter.as_ref() else {
            return Vec::new();
        };
        let mut points = Vec::new();
        for series in &data.series {
            let x_scale = scales
                .x
                .get(&series.x_axis_id)
                .or_else(|| scales.x.values().next())
                .cloned()
                .unwrap_or_else(|| {
                    ChartScale::Linear(crate::LinearScale::new((0.0, 1.0), (0.0, 100.0)))
                });
            let y_scale = scales
                .y
                .get(&series.y_axis_id)
                .or_else(|| scales.y.values().next())
                .cloned()
                .unwrap_or_else(|| {
                    ChartScale::Linear(crate::LinearScale::new((0.0, 1.0), (100.0, 0.0)))
                });
            for (i, point) in series.points.iter().enumerate() {
                points.push(PlacedScatterPoint {
                    px: x_scale.scale_f64(point.x),
                    py: y_scale.scale_f64(point.y),
                    series_id: series.series_id.clone(),
                    data_index: i,
                });
            }
        }
        points
    });

    let points_store = StoredValue::new(Vec::<PlacedScatterPoint>::new());
    Effect::new(move |_| {
        points_store.set_value(placed_points.get());
    });

    let handle_move = move |ev: leptos::ev::MouseEvent| {
        let px = ev.offset_x() as f64;
        let py = ev.offset_y() as f64;
        let hit = find_nearest_point(px, py, &points_store.get_value(), voronoi_max_radius);
        hovered.set(hit.map(|h| ChartItemId {
            series_id: h.series_id,
            data_index: h.data_index,
        }));
    };

    let on_click = on_item;
    let handle_click = move |ev: leptos::ev::MouseEvent| {
        let px = ev.offset_x() as f64;
        let py = ev.offset_y() as f64;
        if let Some(hit) = find_nearest_point(px, py, &points_store.get_value(), voronoi_max_radius)
        {
            if let Some(cb) = on_click.as_ref() {
                cb.run((ChartItemId {
                    series_id: hit.series_id,
                    data_index: hit.data_index,
                },));
            }
        }
    };

    view! {
        <rect
            class="orb-voronoi-layer"
            x=0
            y=0
            width=plot_w
            height=plot_h
            fill="rgba(0,0,0,0)"
            on:mousemove=handle_move
            on:mouseleave=move |_| hovered.set(None)
            on:click=handle_click
        />
    }
    .into_any()
}
