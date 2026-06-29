//! Axis crosshair and band highlight overlay.

use leptos::prelude::*;

use crate::context::{use_axis_data_index, use_chart_context, use_pointer_plot, use_x_scale};
use crate::{AxisHighlightConfig, AxisHighlightMode, ChartScale};

/// Renders axis-aligned crosshair or category band highlight.
#[component]
pub fn AxisHighlight(
    /// Highlight configuration.
    #[prop(default = AxisHighlightConfig::bar_default())]
    config: AxisHighlightConfig,
) -> impl IntoView {
    let ctx = use_chart_context();
    let plot_h = ctx.drawing_area.plot_height;
    let plot_w = ctx.drawing_area.plot_width;
    let pointer = use_pointer_plot();
    let axis_index = use_axis_data_index();
    let x_scale = use_x_scale("x");

    view! {
        <g class="orb-axis-highlight" style="pointer-events: none;">
            {move || {
                let mut views = Vec::new();

                if config.x == AxisHighlightMode::Line {
                    if let Some((px, _)) = pointer.get() {
                        let plot_x = px - ctx.drawing_area.left;
                        views.push(view! {
                            <line
                                class="orb-axis-highlight-line"
                                x1=plot_x
                                y1=0
                                x2=plot_x
                                y2=plot_h
                            />
                        }.into_any());
                    }
                }

                if config.x == AxisHighlightMode::Band {
                    if let Some(idx) = axis_index.get() {
                        if let ChartScale::Band(band) = &x_scale {
                            if let Some((x, width)) = band.band_rect(idx) {
                                views.push(view! {
                                    <rect
                                        class="orb-axis-highlight-band"
                                        x=x
                                        y=0
                                        width=width
                                        height=plot_h
                                    />
                                }.into_any());
                            }
                        }
                    }
                }

                if config.y == AxisHighlightMode::Line {
                    if let Some((_, py)) = pointer.get() {
                        let plot_y = py - ctx.drawing_area.top;
                        views.push(view! {
                            <line
                                class="orb-axis-highlight-line"
                                x1=0
                                y1=plot_y
                                x2=plot_w
                                y2=plot_y
                            />
                        }.into_any());
                    }
                }

                views
            }}
        </g>
    }
}
