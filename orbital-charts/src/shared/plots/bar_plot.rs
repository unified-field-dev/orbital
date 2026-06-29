//! Grouped and stacked bar plot layer.

use leptos::prelude::*;
use orbital_motion::{OrbitalPresenceGroup, OrbitalPresenceGroupItem};

use crate::context::{
    is_series_visible, use_chart_context, use_x_scale, use_y_scale, ChartInteractionContext,
};
use crate::engine::{
    compute_grouped_bars, compute_stacked_bars, projected_for_plot_type, resolve_bar_color,
    resolve_stack_config, stack_groups_from_series, stack_series, BandScale, LinearScale,
};
use crate::shared::marks::BarMark;
use crate::shared::motion::bar_entrance_motion;
use crate::{BarLabelConfig, ChartOrientation, ChartScale, ChartType, ScaleType};

/// Renders grouped or stacked bar marks inside the plot area.
#[component]
pub fn BarPlot(
    #[prop(default = None)] orientation: Option<ChartOrientation>,
    #[prop(default = None)] bar_label: Option<BarLabelConfig>,
    #[prop(default = None)] corner_radius: Option<f64>,
) -> impl IntoView {
    let ctx = use_chart_context();
    let interaction = expect_context::<ChartInteractionContext>();
    let orient = orientation.unwrap_or(ctx.orientation);
    let skip = ctx.skip_animation;
    let motion = ctx.motion.clone();
    let on_item = ctx.on_item_click;
    let highlight_scope = ctx.highlight_scope;
    let bar_label = bar_label.or_else(|| ctx.series.first().and_then(|s| s.bar_label.clone()));
    let radius = corner_radius.or_else(|| ctx.series.first().and_then(|s| s.corner_radius));
    let y_domain = ctx.y_domain;
    let projected = ctx.projected.clone();
    let series_defs = ctx.series.clone();
    let palette = ctx.palette.clone();
    let x_axes = ctx.x_axes.clone();
    let y_axes = ctx.y_axes.clone();

    let (category_axis_id, value_axis_id) = match orient {
        ChartOrientation::Vertical => ("x".to_string(), "y".to_string()),
        ChartOrientation::Horizontal => ("y".to_string(), "x".to_string()),
    };

    let category_scale = match orient {
        ChartOrientation::Vertical => use_x_scale(category_axis_id.clone()),
        ChartOrientation::Horizontal => use_y_scale(category_axis_id.clone()),
    };
    let value_scale = match orient {
        ChartOrientation::Vertical => use_y_scale(value_axis_id.clone()),
        ChartOrientation::Horizontal => use_x_scale(value_axis_id.clone()),
    };

    let bar_gap = ctx
        .x_axes
        .iter()
        .chain(ctx.y_axes.iter())
        .find(|a| a.scale_type == ScaleType::Band)
        .and_then(|a| a.bar_gap_ratio)
        .unwrap_or(0.1);

    let cat_scale = category_scale.clone();
    let val_scale = value_scale.clone();
    let projected_for_bars = projected.clone();
    let series_defs_for_bars = series_defs.clone();
    let bars = Memo::new(move |_| {
        let Some(data) = projected_for_bars.as_ref() else {
            return Vec::new();
        };
        let plot_data = projected_for_plot_type(data, &series_defs_for_bars, ChartType::Bar);
        if plot_data.series.is_empty() {
            return Vec::new();
        }

        let (band, linear) = extract_scales(&cat_scale, &val_scale);
        let baseline = 0.0f64.min(y_domain.0);
        let stack_groups = stack_groups_from_series(&series_defs_for_bars);
        let has_stack = plot_data
            .series
            .iter()
            .any(|s| stack_groups.contains_key(&s.id));

        if has_stack {
            let group = stack_groups
                .values()
                .next()
                .cloned()
                .unwrap_or_else(|| "stack".into());
            let (offset, order) =
                resolve_stack_config(&group, &series_defs_for_bars, ChartType::Bar);
            let stacked = stack_series(&plot_data.series, &stack_groups, offset, order);
            let mut bars = compute_stacked_bars(
                orient,
                &plot_data.categories,
                &plot_data.series,
                &stacked,
                &stack_groups,
                &series_defs_for_bars,
                &band,
                &linear,
                offset,
                order,
            );

            let unstacked: Vec<_> = plot_data
                .series
                .iter()
                .filter(|s| !stack_groups.contains_key(&s.id))
                .cloned()
                .collect();
            if !unstacked.is_empty() {
                bars.extend(compute_grouped_bars(
                    orient,
                    &plot_data.categories,
                    &unstacked,
                    &band,
                    &linear,
                    bar_gap,
                    baseline,
                ));
            }
            bars
        } else {
            compute_grouped_bars(
                orient,
                &plot_data.categories,
                &plot_data.series,
                &band,
                &linear,
                bar_gap,
                baseline,
            )
        }
    });

    Effect::new(move |_| {
        interaction.plot_bars.set(bars.get());
    });

    let motion_preset = bar_entrance_motion(&motion);
    let entrance = Signal::from(motion_preset);
    let stagger = Signal::from(motion.stagger);

    view! {
        {move || {
            let bar_list = bars.get();
            if bar_list.is_empty() {
                return ().into_any();
            }

            let categories = projected
                .as_ref()
                .map(|p| p.categories.clone())
                .unwrap_or_default();
            let category_axis = x_axes.first();
            let value_axis = y_axes.first();

            let bars_view = bar_list
                .iter()
                .enumerate()
                .filter(|(_, bar)| is_series_visible(&bar.series_id))
                .map(|(idx, bar)| {
                let bar = bar.clone();
                let series_idx = series_defs
                    .iter()
                    .position(|s| s.id == bar.series_id)
                    .unwrap_or(0);
                let def = series_defs.get(series_idx).cloned().unwrap_or_default();
                let scope = def.highlight_scope.or(highlight_scope);
                let color = resolve_bar_color(
                    &bar,
                    series_idx,
                    &def,
                    &palette,
                    &categories,
                    category_axis,
                    value_axis,
                );
                let bar_view = view! {
                    <BarMark
                        x=bar.x
                        y=bar.y
                        width=bar.width
                        height=bar.height
                        fill=color
                        corner_radius=radius.unwrap_or(0.0)
                        orientation=orient
                        series_id=bar.series_id.clone()
                        data_index=bar.data_index
                        value=bar.value
                        bar_label=bar_label.clone()
                        highlight_scope=scope
                        on_item_click=on_item
                    />
                };
                (idx, bar_view)
            }).collect::<Vec<_>>();

            if skip {
                bars_view.into_iter().map(|(_, v)| v).collect_view().into_any()
            } else {
                view! {
                    <OrbitalPresenceGroup motion=entrance stagger=stagger>
                        {bars_view.into_iter().map(|(idx, bar_view)| {
                            view! {
                                <OrbitalPresenceGroupItem show=Signal::from(true) index=Signal::from(idx)>
                                    {bar_view}
                                </OrbitalPresenceGroupItem>
                            }
                        }).collect_view()}
                    </OrbitalPresenceGroup>
                }.into_any()
            }
        }}
    }
}

fn extract_scales(category: &ChartScale, value: &ChartScale) -> (BandScale, LinearScale) {
    let band = match category {
        ChartScale::Band(b) => b.clone(),
        _ => BandScale::new(vec![], (0.0, 100.0), 0.1),
    };
    let linear = match value {
        ChartScale::Linear(l) => l.clone(),
        _ => LinearScale::new((0.0, 1.0), (100.0, 0.0)),
    };
    (band, linear)
}
