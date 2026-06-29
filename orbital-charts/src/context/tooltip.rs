//! Tooltip data hooks built from chart and interaction context.

use leptos::prelude::*;

use crate::context::{
    use_axis_data_index, use_chart_context, use_hovered_item, HeatmapPlotContext,
};
use crate::engine::{resolve_mark_color, resolve_series_label};
use crate::{LabelLocation, SeriesDef, TooltipTrigger, HEATMAP_ITEM_SERIES_ID};

/// Data for an item-triggered tooltip row.
#[derive(Clone, Debug, PartialEq)]
pub struct ItemTooltipData {
    /// Series identifier.
    pub series_id: String,
    /// Index within series data.
    pub data_index: usize,
    /// Series display label.
    pub label: String,
    /// Raw numeric value.
    pub value: f64,
    /// Formatted value string.
    pub formatted_value: String,
    /// Resolved mark color.
    pub color: String,
}

/// One row in an axis-triggered tooltip.
#[derive(Clone, Debug, PartialEq)]
pub struct AxisTooltipRow {
    /// Series identifier.
    pub series_id: String,
    /// Series display label.
    pub label: String,
    /// Raw value at the axis index.
    pub value: f64,
    /// Formatted value string.
    pub formatted_value: String,
    /// Resolved mark color.
    pub color: String,
}

/// Data for an axis-triggered tooltip.
#[derive(Clone, Debug, PartialEq)]
pub struct AxisTooltipData {
    /// Formatted axis category or value label.
    pub axis_label: String,
    /// One row per visible series.
    pub rows: Vec<AxisTooltipRow>,
}

/// Active tooltip trigger configuration stored in context.
#[derive(Clone, Copy, Default)]
pub struct ChartTooltipContext {
    /// Current tooltip trigger mode.
    pub trigger: RwSignal<TooltipTrigger>,
    /// Hide x-axis header in axis tooltips.
    pub hide_x_header: RwSignal<bool>,
}

impl ChartTooltipContext {
    /// Create tooltip context with defaults.
    pub fn new(trigger: TooltipTrigger, hide_x_header: bool) -> Self {
        Self {
            trigger: RwSignal::new(trigger),
            hide_x_header: RwSignal::new(hide_x_header),
        }
    }
}

/// Provide tooltip configuration to descendants.
pub fn provide_tooltip_context(trigger: TooltipTrigger, hide_x_header: bool) {
    provide_context(ChartTooltipContext::new(trigger, hide_x_header));
}

fn format_value(series: &SeriesDef, value: f64) -> String {
    if let Some(formatter) = series.value_formatter.as_ref() {
        return formatter.run((value,));
    }
    if (value - value.round()).abs() < f64::EPSILON {
        format!("{:.0}", value)
    } else {
        format!("{:.2}", value)
    }
}

/// Item tooltip data derived from hover state.
pub fn use_item_tooltip() -> Signal<Option<ItemTooltipData>> {
    let ctx = use_chart_context();
    let hovered = use_hovered_item();

    Signal::derive(move || {
        let item = hovered.get()?;

        if item.series_id == HEATMAP_ITEM_SERIES_ID {
            let heatmap = use_context::<HeatmapPlotContext>()?;
            let cell = heatmap.cells.get(item.data_index)?;
            let x_label = ctx
                .x_axes
                .first()
                .and_then(|a| a.data.as_ref())
                .and_then(|cats| cats.get(cell.x).cloned())
                .unwrap_or_else(|| cell.x.to_string());
            let y_label = ctx
                .y_axes
                .first()
                .and_then(|a| a.data.as_ref())
                .and_then(|cats| cats.get(cell.y).cloned())
                .unwrap_or_else(|| cell.y.to_string());
            let color = crate::engine::resolve_color(
                cell.value,
                &heatmap.color_scale,
                crate::engine::heatmap_value_domain(
                    &heatmap.cells,
                    heatmap.value_min,
                    heatmap.value_max,
                ),
            );
            return Some(ItemTooltipData {
                series_id: item.series_id.clone(),
                data_index: item.data_index,
                label: format!("{y_label} × {x_label}"),
                value: cell.value,
                formatted_value: format!("{:.1}", cell.value),
                color,
            });
        }

        let projected = ctx.projected.as_ref()?;
        let series_idx = ctx.series.iter().position(|s| s.id == item.series_id)?;
        let series_def = ctx.series.get(series_idx)?;
        let projected_series = projected.series.iter().find(|s| s.id == item.series_id)?;
        let value = projected_series.data.get(item.data_index).copied()?;
        let category = projected
            .categories
            .get(item.data_index)
            .map(String::as_str);
        let x_axis = ctx.x_axes.first();
        let y_axis = ctx.y_axes.first();
        let color = resolve_mark_color(
            series_idx,
            series_def,
            &ctx.palette,
            category,
            Some(value),
            x_axis,
            y_axis,
        );

        Some(ItemTooltipData {
            series_id: item.series_id.clone(),
            data_index: item.data_index,
            label: resolve_series_label(series_def, LabelLocation::Tooltip),
            value,
            formatted_value: format_value(series_def, value),
            color,
        })
    })
}

/// Axis tooltip data derived from pointer band index.
pub fn use_axis_tooltip() -> Signal<Option<AxisTooltipData>> {
    let ctx = use_chart_context();
    let axis_index = use_axis_data_index();

    Signal::derive(move || {
        let idx = axis_index.get()?;
        let projected = ctx.projected.as_ref()?;
        let category = projected.categories.get(idx)?.clone();

        let rows: Vec<AxisTooltipRow> = ctx
            .series
            .iter()
            .enumerate()
            .filter_map(|(series_idx, series_def)| {
                let projected_series = projected.series.iter().find(|s| s.id == series_def.id)?;
                let value = projected_series.data.get(idx).copied()?;
                let color = resolve_mark_color(
                    series_idx,
                    series_def,
                    &ctx.palette,
                    Some(category.as_str()),
                    Some(value),
                    ctx.x_axes.first(),
                    ctx.y_axes.first(),
                );
                Some(AxisTooltipRow {
                    series_id: series_def.id.clone(),
                    label: resolve_series_label(series_def, LabelLocation::Tooltip),
                    value,
                    formatted_value: format_value(series_def, value),
                    color,
                })
            })
            .collect();

        if rows.is_empty() {
            return None;
        }

        Some(AxisTooltipData {
            axis_label: category,
            rows,
        })
    })
}
