//! Pie plot layer.

use leptos::prelude::*;
use orbital_motion::{OrbitalPresenceGroup, OrbitalPresenceGroupItem};

use crate::context::use_chart_context;
use crate::engine::{compute_pie_slice_layouts, resolve_pie_geometry, resolve_series_color};
use crate::shared::marks::PieSlice;
use crate::shared::motion::arc_sweep_motion;
use crate::{PieArcLabelConfig, PieGeometry, PieRadius};

/// Renders pie slices inside the plot area.
#[component]
pub fn PiePlot(
    /// Inner radius (px or percent string).
    #[prop(optional, into, default = String::new())]
    inner_radius: String,
    /// Outer radius (px or percent string).
    #[prop(optional, into, default = String::new())]
    outer_radius: String,
    /// Padding angle between slices in degrees.
    #[prop(default = 0.0)]
    padding_angle: f64,
    /// Arc label configuration.
    #[prop(default = None)]
    arc_label: Option<PieArcLabelConfig>,
    /// Start angle in degrees.
    #[prop(default = 0.0)]
    start_angle: f64,
    /// End angle in degrees.
    #[prop(default = 360.0)]
    end_angle: f64,
) -> impl IntoView {
    let ctx = use_chart_context();
    let pie = ctx.pie.clone();
    let skip = ctx.skip_animation;
    let motion = ctx.motion.clone();
    let palette = ctx.palette.clone();
    let on_item = ctx.on_item_click;
    let highlight_scope = ctx.highlight_scope;
    let series_defs = ctx.series.clone();
    let plot_w = ctx.drawing_area.plot_width;
    let plot_h = ctx.drawing_area.plot_height;

    let inner = if inner_radius.is_empty() {
        PieRadius::Px(0.0)
    } else {
        PieRadius::parse(&inner_radius)
    };
    let outer = if outer_radius.is_empty() {
        PieRadius::Percent(90.0)
    } else {
        PieRadius::parse(&outer_radius)
    };

    let geometry = PieGeometry {
        inner_radius: inner,
        outer_radius: outer,
        padding_angle,
        start_angle,
        end_angle,
        ..PieGeometry::default()
    };
    let geometry_for_layout = geometry.clone();

    let pie_data = pie.clone();
    let slices = Memo::new(move |_| {
        let Some(data) = pie_data.as_ref() else {
            return Vec::new();
        };
        let layout = resolve_pie_geometry(plot_w, plot_h, &geometry);
        compute_pie_slice_layouts(&data.slices, &layout)
    });

    let pie_layout = Memo::new(move |_| resolve_pie_geometry(plot_w, plot_h, &geometry_for_layout));

    let draw_key = Signal::derive(move || {
        slices
            .get()
            .iter()
            .map(|s| format!("{}:{}", s.id, s.value))
            .collect::<Vec<_>>()
            .join("|")
    });

    let (sweep_duration, _) = arc_sweep_motion(&motion);
    let stagger = Signal::from(motion.stagger);
    let arc_label_stored = StoredValue::new(arc_label);

    view! {
        {move || {
            let defs = series_defs.clone();
            let pal = palette.clone();
            let slice_list = slices.get();
            let layout = pie_layout.get();
            if slice_list.is_empty() {
                return ().into_any();
            }

            let series_id = pie
                .as_ref()
                .map(|p| p.series_id.clone())
                .unwrap_or_else(|| "pie".into());

            if skip {
                return view! {
                    <g class="orb-pie-plot">
                        {slice_list.into_iter().enumerate().map(|(i, slice)| {
                            let def = defs.get(i).cloned().unwrap_or_default();
                            let fill = resolve_series_color(i, &def, &pal);
                            view! {
                                <PieSlice
                                    layout=slice
                                    fill=fill
                                    series_id=series_id.clone()
                                    cx=layout.cx
                                    cy=layout.cy
                                    inner_r=layout.inner_radius
                                    outer_r=layout.outer_radius
                                    arc_label=arc_label_stored.get_value()
                                    highlight_scope=highlight_scope
                                    skip_animation=true
                                    draw_key=draw_key
                                    on_item_click=on_item
                                />
                            }
                        }).collect_view()}
                    </g>
                }.into_any();
            }

            view! {
                <OrbitalPresenceGroup
                    motion=Signal::from(orbital_motion::PresenceMotion::fade_scale())
                    stagger=stagger
                >
                    <g class="orb-pie-plot">
                        {slice_list.into_iter().enumerate().map(|(i, slice)| {
                            let def = defs.get(i).cloned().unwrap_or_default();
                            let fill = resolve_series_color(i, &def, &pal);
                            let sid = series_id.clone();
                            view! {
                                <OrbitalPresenceGroupItem
                                    show=Signal::from(true)
                                    index=Signal::from(i)
                                >
                                    <PieSlice
                                        layout=slice
                                        fill=fill
                                        series_id=sid
                                        cx=layout.cx
                                        cy=layout.cy
                                        inner_r=layout.inner_radius
                                        outer_r=layout.outer_radius
                                        arc_label=arc_label_stored.get_value()
                                        highlight_scope=highlight_scope
                                        skip_animation=skip
                                        sweep_duration=sweep_duration
                                        draw_key=draw_key
                                        on_item_click=on_item
                                    />
                                </OrbitalPresenceGroupItem>
                            }
                        }).collect_view()}
                    </g>
                </OrbitalPresenceGroup>
            }.into_any()
        }}
    }
}
