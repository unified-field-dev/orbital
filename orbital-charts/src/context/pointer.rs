//! Pointer tracking over the plot area.

use leptos::ev::MouseEvent;
use leptos::prelude::*;

use crate::context::{
    set_hovered_item, use_chart_context, use_x_scale, use_y_scale, ChartInteractionContext,
    ChartScale,
};
use crate::engine::{category_index_at_x, find_bar_at_pointer, find_nearest_marker};
use crate::{ChartItemId, ChartOrientation, TooltipTrigger};

/// Transparent overlay capturing pointer events for axis tooltips and crosshair.
#[component]
pub fn ChartPointerLayer(
    #[prop(default = TooltipTrigger::Item)] trigger: TooltipTrigger,
) -> impl IntoView {
    let ctx = use_chart_context();
    let plot_w = ctx.drawing_area.plot_width;
    let plot_h = ctx.drawing_area.plot_height;
    let inset_left = ctx.drawing_area.left;
    let inset_top = ctx.drawing_area.top;
    let orientation = ctx.orientation;

    let category_scale = match orientation {
        ChartOrientation::Vertical => use_x_scale("x".to_string()),
        ChartOrientation::Horizontal => use_y_scale("y".to_string()),
    };

    let interaction = expect_context::<ChartInteractionContext>();
    let pointer_events = if matches!(trigger, TooltipTrigger::None) {
        "none"
    } else {
        "all"
    };

    view! {
        <rect
            class="orb-pointer-layer"
            x=0
            y=0
            width=plot_w
            height=plot_h
            fill="transparent"
            style=format!("pointer-events: {pointer_events}; cursor: crosshair;")
            on:mousemove=move |ev: MouseEvent| {
                let px = ev.offset_x() as f64;
                let py = ev.offset_y() as f64;
                interaction
                    .pointer_plot
                    .set(Some((inset_left + px, inset_top + py)));

                let bars = interaction.plot_bars.get_untracked();
                let markers = interaction.plot_line_markers.get_untracked();

                if matches!(trigger, TooltipTrigger::Axis) {
                    if let ChartScale::Band(band) = category_scale.clone() {
                        interaction
                            .axis_data_index
                            .set(category_index_at_x(&band, px));
                    }
                    if let Some(hit) = find_bar_at_pointer(&bars, px, py) {
                        set_hovered_item(Some(ChartItemId {
                            series_id: hit.series_id,
                            data_index: hit.data_index,
                        }));
                    } else if let Some(hit) = find_nearest_marker(px, py, &markers, 12.0) {
                        set_hovered_item(Some(ChartItemId {
                            series_id: hit.series_id,
                            data_index: hit.data_index,
                        }));
                    } else {
                        set_hovered_item(None);
                    }
                    return;
                }

                if let Some(hit) = find_bar_at_pointer(&bars, px, py) {
                    set_hovered_item(Some(ChartItemId {
                        series_id: hit.series_id,
                        data_index: hit.data_index,
                    }));
                } else if let Some(hit) = find_nearest_marker(px, py, &markers, 12.0) {
                    set_hovered_item(Some(ChartItemId {
                        series_id: hit.series_id,
                        data_index: hit.data_index,
                    }));
                } else {
                    set_hovered_item(None);
                }
                interaction.axis_data_index.set(None);
            }
            on:mouseleave=move |_| {
                interaction.pointer_plot.set(None);
                interaction.axis_data_index.set(None);
                set_hovered_item(None);
            }
        />
    }
}

/// Read-only pointer position in chart shell coordinates.
pub fn use_pointer_tracker() -> ReadSignal<Option<(f64, f64)>> {
    expect_context::<ChartInteractionContext>()
        .pointer_plot
        .read_only()
}
