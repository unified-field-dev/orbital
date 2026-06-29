//! Scatter plot layer.

use leptos::prelude::*;
use orbital_motion::{OrbitalPresenceGroup, OrbitalPresenceGroupItem};

use crate::context::{use_chart_context, ChartScales};
use crate::engine::resolve_series_color;
use crate::shared::marks::ScatterPoint;
use crate::shared::motion::scatter_entrance_motion;
use crate::ChartScale;

/// Renders scatter points inside the plot area.
#[component]
pub fn ScatterPlot(
    /// Default marker radius when not set on series.
    #[prop(default = 4.0)]
    marker_size: f64,
    /// When true, points receive direct pointer events (disable Voronoi).
    #[prop(default = false)]
    disable_voronoi: bool,
) -> impl IntoView {
    let ctx = use_chart_context();
    let scatter = ctx.scatter.clone();
    let skip = ctx.skip_animation;
    let motion = ctx.motion.clone();
    let palette = ctx.palette.clone();
    let highlight_scope = ctx.highlight_scope;
    let series_defs = ctx.series.clone();
    let scales = expect_context::<ChartScales>().clone();

    let entrance = Signal::from(scatter_entrance_motion(&motion));
    let stagger = Signal::from(motion.stagger);

    view! {
        {move || {
            let Some(data) = scatter.as_ref() else {
                return ().into_any();
            };
            if data.series.is_empty() {
                return ().into_any();
            }

            let points_view = data.series.iter().enumerate().map(|(si, series)| {
                let def = series_defs.get(si).cloned().unwrap_or_default();
                let r = series.marker_size.max(marker_size);
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

                let marks_view: Vec<_> = series
                    .points
                    .iter()
                    .enumerate()
                    .map(|(pi, point)| {
                        let fill = resolve_series_color(si, &def, &palette);
                        let cx = x_scale.scale_f64(point.x);
                        let cy = y_scale.scale_f64(point.y);
                        let point_view = view! {
                            <ScatterPoint
                                cx=cx
                                cy=cy
                                r=r
                                fill=fill
                                series_id=series.series_id.clone()
                                data_index=pi
                                highlight_scope=highlight_scope
                                pointer_events=disable_voronoi
                            />
                        };
                        (pi, point_view)
                    })
                    .collect();

                if skip {
                    marks_view.into_iter().map(|(_, v)| v).collect_view().into_any()
                } else {
                    view! {
                        <OrbitalPresenceGroup motion=entrance stagger=stagger>
                            {marks_view.into_iter().map(|(pi, point_view)| {
                                view! {
                                    <OrbitalPresenceGroupItem
                                        show=Signal::from(true)
                                        index=Signal::from(pi)
                                    >
                                        {point_view}
                                    </OrbitalPresenceGroupItem>
                                }
                            }).collect_view()}
                        </OrbitalPresenceGroup>
                    }.into_any()
                }
            }).collect_view();

            view! {
                <g class="orb-scatter-plot">
                    {points_view}
                </g>
            }.into_any()
        }}
    }
}
