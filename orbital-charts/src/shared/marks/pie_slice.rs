//! Single pie slice mark with arc sweep motion and highlight classes.

use leptos::callback::Callback;
use leptos::prelude::*;

use crate::context::use_hovered_item;
use crate::engine::{arc_label_position, PieSliceLayout};
use crate::shared::motion::ArcSweepSlice;
use crate::{
    ArcLabelMode, ChartItemId, FadeMode, HighlightMode, HighlightScope, PieArcLabelConfig,
};

/// One pie slice with optional arc label.
#[component]
pub fn PieSlice(
    /// Slice layout geometry.
    layout: PieSliceLayout,
    /// Fill color.
    #[prop(into)]
    fill: String,
    /// Series id for events.
    #[prop(into)]
    series_id: String,
    /// Center x.
    cx: f64,
    /// Center y.
    cy: f64,
    /// Inner radius.
    inner_r: f64,
    /// Outer radius.
    outer_r: f64,
    /// Arc label configuration.
    #[prop(default = None)]
    arc_label: Option<PieArcLabelConfig>,
    /// Highlight scope.
    #[prop(default = None)]
    highlight_scope: Option<HighlightScope>,
    /// Whether animations are skipped.
    #[prop(default = false)]
    skip_animation: bool,
    /// Motion duration for sweep.
    #[prop(default = orbital_motion::MotionDuration::Normal)]
    sweep_duration: orbital_motion::MotionDuration,
    /// Draw key for re-animation.
    #[prop(into)]
    draw_key: Signal<String>,
    /// Item click handler.
    #[prop(default = None)]
    on_item_click: Option<Callback<(ChartItemId,), ()>>,
) -> impl IntoView {
    let hovered = use_hovered_item();
    let item_id = ChartItemId {
        series_id: series_id.clone(),
        data_index: layout.index,
    };

    let scope = highlight_scope.unwrap_or(HighlightScope {
        highlight: HighlightMode::Item,
        fade: FadeMode::Global,
    });

    let item_id_for_hover = item_id.clone();

    let slice_class = move || {
        let mut classes = vec![
            "orb-pie-slice-group".to_string(),
            "orb-pie-slice".to_string(),
        ];
        let h = hovered.get();
        let is_hovered = h.as_ref() == Some(&item_id_for_hover);
        match scope.highlight {
            HighlightMode::Item if is_hovered => classes.push("orb-pie-slice-highlighted".into()),
            HighlightMode::Item if h.is_some() && scope.fade == FadeMode::Global => {
                classes.push("orb-pie-slice-faded".into());
            }
            _ => {}
        }
        classes.join(" ")
    };

    let label_text = arc_label_text(&layout, arc_label.as_ref());
    let show_label = label_text.is_some()
        && layout.angle_deg >= arc_label.as_ref().and_then(|c| c.min_angle).unwrap_or(0.0);
    let label_radius = arc_label
        .as_ref()
        .and_then(|c| c.radius.as_ref())
        .map(|r| r.resolve(outer_r * 2.0, outer_r * 2.0, true))
        .unwrap_or((inner_r + outer_r) / 2.0);
    let (lx, ly) = arc_label_position(cx, cy, layout.mid_rad, label_radius);

    let fill_signal: Signal<String> = Signal::from(fill);
    let on_click = on_item_click;
    let series_for_click = series_id.clone();
    let index = layout.index;

    view! {
        <g
            class=slice_class
            style="pointer-events: all; cursor: pointer;"
            on:click=move |ev| {
                if let Some(cb) = on_click.as_ref() {
                    cb.run((ChartItemId {
                        series_id: series_for_click.clone(),
                        data_index: index,
                    },));
                }
                let _ = ev;
            }
            on:mouseenter=move |_| hovered.set(Some(item_id.clone()))
            on:mouseleave=move |_| hovered.set(None)
        >
            <ArcSweepSlice
                cx=cx
                cy=cy
                inner_r=inner_r
                outer_r=outer_r
                start_rad=layout.start_rad
                end_rad=layout.end_rad
                full_path=layout.path_d.clone()
                fill=fill_signal
                skip_animation=skip_animation
                duration=sweep_duration
                draw_key=draw_key
            />
            {show_label.then(|| {
                let text = label_text.clone().unwrap_or_default();
                view! {
                    <text
                        class="orb-pie-label"
                        x=lx
                        y=ly
                        text-anchor="middle"
                        dominant-baseline="central"
                    >
                        {text}
                    </text>
                }
            })}
        </g>
    }
}

fn arc_label_text(layout: &PieSliceLayout, config: Option<&PieArcLabelConfig>) -> Option<String> {
    let mode = config
        .and_then(|c| c.mode.as_ref())
        .unwrap_or(&ArcLabelMode::FormattedValue);
    match mode {
        ArcLabelMode::Value => Some(format!("{:.0}", layout.value)),
        ArcLabelMode::FormattedValue => Some(format!("{:.1}", layout.value)),
        ArcLabelMode::Label => Some(layout.label.clone()),
    }
}
