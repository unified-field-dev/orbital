//! Default chart tooltip content panels.

use leptos::prelude::*;
use orbital_core_components::{Body1, Caption1};

use crate::context::tooltip::{AxisTooltipData, ItemTooltipData};

/// Default item tooltip body.
#[component]
pub fn ChartTooltipItemContent(data: ItemTooltipData) -> impl IntoView {
    view! {
        <div class="orb-chart-tooltip-item">
            <span
                class="orb-chart-tooltip-mark"
                style=format!("background: {};", data.color)
            />
            <div class="orb-chart-tooltip-text">
                <Caption1>{data.label.clone()}</Caption1>
                <Body1>{data.formatted_value.clone()}</Body1>
            </div>
        </div>
    }
}

/// Default axis tooltip body.
#[component]
pub fn ChartTooltipAxisContent(
    data: AxisTooltipData,
    #[prop(default = false)] hide_header: bool,
) -> impl IntoView {
    view! {
        <div class="orb-chart-tooltip-axis">
            {(!hide_header).then(|| view! {
                <Caption1 class="orb-chart-tooltip-axis-header">{data.axis_label.clone()}</Caption1>
            })}
            <div class="orb-chart-tooltip-rows">
                {data.rows.iter().map(|row| view! {
                    <div class="orb-chart-tooltip-row">
                        <span
                            class="orb-chart-tooltip-mark"
                            style=format!("background: {};", row.color)
                        />
                        <span class="orb-chart-tooltip-row-label">{row.label.clone()}</span>
                        <span class="orb-chart-tooltip-row-value">{row.formatted_value.clone()}</span>
                    </div>
                }).collect_view()}
            </div>
        </div>
    }
}
