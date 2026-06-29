//! Single bar mark with optional label and click target.

use leptos::callback::Callback;
use leptos::prelude::*;

use crate::context::{
    set_hovered_item, use_chart_context, use_highlighted_item, ChartInteractionContext,
};
use crate::{
    BarLabelConfig, ChartItemId, ChartOrientation, FadeMode, HighlightMode, HighlightScope,
};

/// One grouped bar rectangle in plot space.
#[component]
pub fn BarMark(
    /// Left x in plot coordinates.
    x: f64,
    /// Top y in plot coordinates.
    y: f64,
    /// Bar width.
    width: f64,
    /// Bar height.
    height: f64,
    /// Fill color.
    #[prop(into)]
    fill: String,
    /// Corner radius.
    #[prop(default = 0.0)]
    corner_radius: f64,
    /// Chart orientation (affects label placement and transform origin).
    #[prop(default = ChartOrientation::Vertical)]
    orientation: ChartOrientation,
    /// Series id for click events.
    #[prop(into)]
    series_id: String,
    /// Data index for click events.
    data_index: usize,
    /// Numeric value for labels.
    value: f64,
    /// Bar label configuration.
    #[prop(default = None)]
    bar_label: Option<BarLabelConfig>,
    /// Highlight scope for fade behavior.
    #[prop(default = None)]
    highlight_scope: Option<HighlightScope>,
    /// Item click handler.
    #[prop(default = None)]
    on_item_click: Option<Callback<(ChartItemId,), ()>>,
) -> impl IntoView {
    let label_text = bar_label_text(value, data_index, bar_label.as_ref());
    let show_label = label_text.is_some();
    let label = label_text.unwrap_or_default();
    let origin_class = match orientation {
        ChartOrientation::Vertical => "orb-bar-mark--vertical",
        ChartOrientation::Horizontal => "orb-bar-mark--horizontal",
    };
    let label_x = x + width / 2.0;
    let label_y = if orientation == ChartOrientation::Vertical {
        y - 4.0
    } else {
        y + height / 2.0
    };

    let highlighted = use_highlighted_item();
    let item_id = ChartItemId {
        series_id: series_id.clone(),
        data_index,
    };
    let scope = highlight_scope.unwrap_or(HighlightScope {
        highlight: HighlightMode::Item,
        fade: FadeMode::Global,
    });
    let item_id_for_class = item_id.clone();

    let mark_class = move || {
        let mut classes = vec!["orb-bar-mark".to_string(), origin_class.to_string()];
        let active = highlighted.get();
        let is_active = active.as_ref() == Some(&item_id_for_class);
        if scope.highlight != HighlightMode::None && active.is_some() {
            if is_active {
                classes.push("orb-bar-mark-highlighted".into());
            } else if scope.fade == FadeMode::Global {
                classes.push("orb-bar-mark-faded".into());
            }
        }
        classes.join(" ")
    };

    let item_for_hover = item_id.clone();
    let series_for_click = series_id.clone();
    let plot_left = use_chart_context().drawing_area.left;
    let plot_top = use_chart_context().drawing_area.top;
    let pointer_x = x + width / 2.0;
    let pointer_y = y + height / 2.0;
    let interaction = expect_context::<ChartInteractionContext>();

    view! {
        <g class="orb-bar-mark-group" style="pointer-events: all;">
            <rect
                class=mark_class
                x=x
                y=y
                width=width
                height=height
                rx=corner_radius
                ry=corner_radius
                fill=fill
                on:mouseenter=move |_| {
                    interaction
                        .pointer_plot
                        .set(Some((plot_left + pointer_x, plot_top + pointer_y)));
                    set_hovered_item(Some(item_for_hover.clone()));
                }
                on:mouseleave=move |_| {
                    interaction.pointer_plot.set(None);
                    set_hovered_item(None);
                }
                on:click=move |ev| {
                    ev.stop_propagation();
                    if let Some(cb) = &on_item_click {
                        cb.run((ChartItemId {
                            series_id: series_for_click.clone(),
                            data_index,
                        },));
                    }
                }
            />
            {show_label.then(|| view! {
                <text
                    class="orb-bar-label"
                    x=label_x
                    y=label_y
                    text-anchor="middle"
                    dominant-baseline="auto"
                >
                    {label}
                </text>
            })}
        </g>
    }
}

fn bar_label_text(
    value: f64,
    data_index: usize,
    config: Option<&BarLabelConfig>,
) -> Option<String> {
    let config = config?;
    if config.show == Some(false) {
        return None;
    }
    if let Some(fmt) = &config.formatter {
        return Some(fmt.run((value, data_index)));
    }
    Some(format!("{value:.0}"))
}
