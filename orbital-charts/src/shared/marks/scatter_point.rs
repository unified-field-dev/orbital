//! Single scatter point mark.

use leptos::prelude::*;

use crate::context::use_hovered_item;
use crate::{ChartItemId, FadeMode, HighlightMode, HighlightScope};

/// One scatter point circle.
#[component]
pub fn ScatterPoint(
    /// Pixel x in plot coordinates.
    cx: f64,
    /// Pixel y in plot coordinates.
    cy: f64,
    /// Marker radius.
    r: f64,
    /// Fill color.
    #[prop(into)]
    fill: String,
    /// Series id.
    #[prop(into)]
    series_id: String,
    /// Data index.
    data_index: usize,
    /// Highlight scope.
    #[prop(default = None)]
    highlight_scope: Option<HighlightScope>,
    /// Whether pointer events hit this element directly.
    #[prop(default = false)]
    pointer_events: bool,
) -> impl IntoView {
    let hovered = use_hovered_item();
    let item_id = ChartItemId {
        series_id: series_id.clone(),
        data_index,
    };

    let scope = highlight_scope.unwrap_or(HighlightScope {
        highlight: HighlightMode::Item,
        fade: FadeMode::Global,
    });

    let item_id_for_hover = item_id.clone();

    let class = move || {
        let mut classes = vec!["orb-scatter-point".to_string()];
        let h = hovered.get();
        let is_hovered = h.as_ref() == Some(&item_id);
        match scope.highlight {
            HighlightMode::Item if is_hovered => {
                classes.push("orb-scatter-point-highlighted".into())
            }
            HighlightMode::Item if h.is_some() && scope.fade == FadeMode::Global => {
                classes.push("orb-scatter-point-faded".into());
            }
            _ => {}
        }
        classes.join(" ")
    };

    let pe = if pointer_events { "all" } else { "all" };

    view! {
        <circle
            class=class
            cx=cx
            cy=cy
            r=r
            fill=fill
            style=format!("pointer-events: {pe}; cursor: pointer;")
            on:mouseenter=move |_| hovered.set(Some(item_id_for_hover.clone()))
            on:mouseleave=move |_| hovered.set(None)
        />
    }
}
