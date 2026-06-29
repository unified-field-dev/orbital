//! Single heatmap cell SVG mark.

use leptos::prelude::*;

use crate::context::{
    set_hovered_item, use_chart_context, use_highlighted_item, ChartInteractionContext,
};
use crate::{
    ChartItemId, FadeMode, HeatmapCellLayout, HighlightMode, HighlightScope, HEATMAP_ITEM_SERIES_ID,
};

/// One heatmap cell rectangle.
#[component]
pub fn HeatmapCellMark(
    /// Resolved cell layout.
    layout: HeatmapCellLayout,
    /// Corner radius in pixels.
    #[prop(default = 4.0)]
    corner_radius: f64,
    /// Highlight scope for fade behavior.
    #[prop(default = None)]
    highlight_scope: Option<HighlightScope>,
) -> impl IntoView {
    let highlighted = use_highlighted_item();
    let ctx = use_chart_context();
    let scope = highlight_scope
        .or(ctx.highlight_scope)
        .unwrap_or(HighlightScope {
            highlight: HighlightMode::Item,
            fade: FadeMode::Global,
        });

    let item_id = ChartItemId {
        series_id: HEATMAP_ITEM_SERIES_ID.to_string(),
        data_index: layout.index,
    };
    let item_id_for_class = item_id.clone();
    let inset_left = ctx.drawing_area.left;
    let inset_top = ctx.drawing_area.top;

    let cell_class = move || {
        let mut classes = vec!["orb-heatmap-cell".to_string()];
        let active = highlighted.get();
        let is_active = active.as_ref() == Some(&item_id_for_class);
        if scope.highlight != HighlightMode::None && active.is_some() {
            if is_active {
                classes.push("orb-heatmap-cell-highlighted".into());
            } else if scope.fade == FadeMode::Global {
                classes.push("orb-heatmap-cell-faded".into());
            }
        }
        classes.join(" ")
    };

    view! {
        <rect
            class=cell_class
            x=layout.x
            y=layout.y
            width=layout.width
            height=layout.height
            rx=corner_radius
            ry=corner_radius
            fill=layout.fill.clone()
            data-value=layout.value
            style="pointer-events: all; cursor: crosshair;"
            on:mouseenter=move |_| {
                set_hovered_item(Some(item_id.clone()));
                expect_context::<ChartInteractionContext>()
                    .pointer_plot
                    .set(Some((
                        inset_left + layout.x + layout.width / 2.0,
                        inset_top + layout.y + layout.height / 2.0,
                    )));
            }
            on:mouseleave=move |_| {
                set_hovered_item(None);
                expect_context::<ChartInteractionContext>()
                    .pointer_plot
                    .set(None);
            }
        />
    }
}
