//! Chart tooltip overlay.

use leptos::prelude::*;
#[cfg(all(not(feature = "ssr"), feature = "hydrate"))]
use orbital_base_components::ThemedPortal;
use orbital_core_components::overlay::FloatingPanel;
use orbital_core_components::MaterialElevation;
use orbital_motion::use_reduced_motion;

use crate::context::{
    use_axis_tooltip, use_chart_context, use_item_tooltip, use_overlay_context, use_pointer_plot,
    ChartOverlayContext, ChartTooltipContext,
};
use crate::{OverlayMount, TooltipConfig, TooltipTrigger};

use super::content::{ChartTooltipAxisContent, ChartTooltipItemContent};

/// Floating tooltip panel for item or axis hover modes.
#[component]
pub fn ChartTooltip(
    /// Tooltip configuration.
    #[prop(default = TooltipConfig::item())]
    config: TooltipConfig,
) -> impl IntoView {
    if matches!(config.trigger, TooltipTrigger::None) {
        return ().into_view().into_any();
    }

    provide_tooltip_runtime(&config);

    let item_data = use_item_tooltip();
    let axis_data = use_axis_tooltip();
    let pointer = use_pointer_plot();
    let ctx = use_chart_context();
    let overlay_ctx = use_overlay_context();
    let prefers_reduced = use_reduced_motion();
    let trigger = config.trigger;
    let hide_header = config.hide_x_header;
    let item_slot = config.slots.item_content.clone();
    let axis_slot = config.slots.axis_content.clone();
    let chart_w = ctx.width;
    let chart_h = ctx.height;
    let use_host_portal = matches!(overlay_ctx.overlay_mount, OverlayMount::HostElement { .. });

    let has_data = Signal::derive(move || match trigger {
        TooltipTrigger::Item => item_data.get().is_some(),
        TooltipTrigger::Axis => axis_data.get().is_some(),
        TooltipTrigger::None => false,
    });
    let delay_ms = Signal::derive(move || {
        if prefers_reduced.get() {
            0
        } else {
            ctx.charts_theme.tooltip_delay_ms
        }
    });
    let tooltip_visible = use_delayed_tooltip_visible(has_data, delay_ms);

    view! {
        {move || {
            if !tooltip_visible.get() {
                return ().into_any();
            }
            let style = tooltip_viewport_style(
                pointer.get(),
                overlay_ctx.root_ref,
                chart_w,
                chart_h,
            );
            let panel = match trigger {
                TooltipTrigger::Item => {
                    let Some(data) = item_data.get() else {
                        return ().into_any();
                    };
                    if let Some(slot) = item_slot.clone() {
                        tooltip_panel((slot)(), style)
                    } else {
                        tooltip_panel(
                            view! { <ChartTooltipItemContent data=data /> }.into_any(),
                            style,
                        )
                    }
                }
                TooltipTrigger::Axis => {
                    let Some(data) = axis_data.get() else {
                        return ().into_any();
                    };
                    if let Some(slot) = axis_slot.clone() {
                        tooltip_panel((slot)(), style)
                    } else {
                        tooltip_panel(
                            view! {
                                <ChartTooltipAxisContent data=data hide_header=hide_header />
                            }
                            .into_any(),
                            style,
                        )
                    }
                }
                TooltipTrigger::None => return ().into_any(),
            };

            if use_host_portal {
                host_portal_tooltip(panel, &overlay_ctx)
            } else {
                panel
            }
        }}
    }
    .into_any()
}

fn use_delayed_tooltip_visible(has_data: Signal<bool>, delay_ms: Signal<u64>) -> ReadSignal<bool> {
    let visible = RwSignal::new(false);
    Effect::new(move |_| {
        if !has_data.get() {
            visible.set(false);
            return;
        }

        let delay = delay_ms.get();
        if delay == 0 {
            visible.set(true);
            return;
        }

        visible.set(false);

        #[cfg(all(not(feature = "ssr"), feature = "hydrate"))]
        {
            use wasm_bindgen::JsCast;

            if let Some(win) = web_sys::window() {
                let show = visible;
                let closure = wasm_bindgen::closure::Closure::once(move || {
                    show.set(true);
                });
                let result = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    delay as i32,
                );
                let handle = result.ok();
                closure.forget();
                on_cleanup(move || {
                    if let (Some(win), Some(handle)) = (web_sys::window(), handle) {
                        win.clear_timeout_with_handle(handle);
                    }
                });
                return;
            }
        }

        visible.set(true);
    });
    visible.read_only()
}

fn tooltip_panel(content: AnyView, style: String) -> AnyView {
    view! {
        <div class="orb-chart-tooltip" style=style>
            <FloatingPanel elevation=MaterialElevation::Raised role="tooltip">
                {content}
            </FloatingPanel>
        </div>
    }
    .into_any()
}

fn host_portal_tooltip(content: AnyView, overlay_ctx: &ChartOverlayContext) -> AnyView {
    #[cfg(all(not(feature = "ssr"), feature = "hydrate"))]
    {
        let mount = overlay_ctx.resolve_portal_mount();
        return match mount {
            Some(el) => view! {
                <ThemedPortal immediate=true mount=el>
                    {content}
                </ThemedPortal>
            }
            .into_any(),
            None => content,
        };
    }
    #[cfg(not(all(not(feature = "ssr"), feature = "hydrate")))]
    {
        let _ = overlay_ctx;
        content
    }
}

fn provide_tooltip_runtime(config: &TooltipConfig) {
    if leptos::prelude::use_context::<ChartTooltipContext>().is_none() {
        crate::context::provide_tooltip_context(config.trigger, config.hide_x_header);
    }
}

fn tooltip_viewport_style(
    pointer: Option<(f64, f64)>,
    root_ref: NodeRef<leptos::html::Div>,
    chart_w: f64,
    chart_h: f64,
) -> String {
    #[cfg(not(all(not(feature = "ssr"), feature = "hydrate")))]
    let _ = root_ref;
    let (left, top) = pointer.unwrap_or((chart_w / 2.0, chart_h / 4.0));

    #[cfg(all(not(feature = "ssr"), feature = "hydrate"))]
    {
        if let Some(root) = root_ref.get() {
            let rect = root.get_bounding_client_rect();
            let fixed_left = rect.left() + left;
            let fixed_top = rect.top() + top;
            let viewport_w = web_sys::window()
                .and_then(|w| w.inner_width().ok())
                .and_then(|v| v.as_f64())
                .unwrap_or(chart_w);
            let viewport_h = web_sys::window()
                .and_then(|w| w.inner_height().ok())
                .and_then(|v| v.as_f64())
                .unwrap_or(chart_h);
            let left = fixed_left.clamp(8.0, viewport_w - 120.0);
            let top = fixed_top.clamp(8.0, viewport_h - 80.0);
            return format!("left: {left}px; top: {top}px;");
        }
    }

    let left = left.clamp(8.0, chart_w - 120.0);
    let top = top.clamp(8.0, chart_h - 80.0);
    format!("left: {left}px; top: {top}px;")
}
