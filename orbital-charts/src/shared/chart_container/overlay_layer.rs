//! Chart-local overlay portal mount for legend, tooltip, and loading chrome.

use std::sync::Arc;

use leptos::prelude::*;
use orbital_core_components::OverlayLayerRoot;

use crate::shared::legend::Legend;
use crate::shared::tooltip::ChartTooltip;
use crate::{LegendConfig, TooltipConfig, TooltipTrigger};
use leptos::callback::Callback;

use super::overlays::ChartOverlays;

/// Scoped portal mount inside `[data-orbital-chart]` for interactive overlay chrome.
///
/// Wraps [`OverlayLayerRoot`] with chart-specific stacking CSS. Descendants such as
/// [`ChartTooltip`](crate::ChartTooltip) portal into this layer via
/// [`ThemedPortal`](orbital_core_components::ThemedPortal).
#[component]
pub fn ChartOverlayLayer(
    /// Mount target for descendant portals. When omitted, an internal ref is allocated.
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    children: Children,
) -> impl IntoView {
    let layer_ref = node_ref.unwrap_or_default();

    view! {
        <OverlayLayerRoot node_ref=layer_ref class="orb-chart-overlay-layer">
            {children()}
        </OverlayLayerRoot>
    }
}

/// Loading and legend layers mounted above the SVG viewport.
#[component]
pub fn ChartRootOverlayChrome(
    layer_ref: NodeRef<leptos::html::Div>,
    loading: bool,
    is_empty: bool,
    projection_error: Option<String>,
    loading_view: Option<Arc<dyn Fn() -> AnyView + Send + Sync>>,
    empty_view: Option<Arc<dyn Fn() -> AnyView + Send + Sync>>,
    legend_config: Option<LegendConfig>,
    tooltip_cfg: Option<TooltipConfig>,
    on_legend_click: Option<Callback<(String,), ()>>,
) -> impl IntoView {
    view! {
        <ChartOverlayLayer node_ref=layer_ref>
            <ChartOverlays
                loading=loading
                is_empty=Signal::derive(move || is_empty)
                projection_error=Signal::derive(move || projection_error.clone())
                loading_view=loading_view
                empty_view=empty_view
            />
            {legend_config
                .filter(|c| !c.hidden)
                .map(|config| view! {
                    <Legend config=config on_legend_click=on_legend_click />
                })}
            {tooltip_cfg
                .filter(|c| !matches!(c.trigger, TooltipTrigger::None))
                .map(|config| view! { <ChartTooltip config=config /> })}
        </ChartOverlayLayer>
    }
}
