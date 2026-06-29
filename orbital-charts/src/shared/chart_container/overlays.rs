//! Loading and empty overlay layer.

use std::sync::Arc;

use leptos::prelude::*;
use orbital_core_components::{
    MessageBar, MessageBarBody, MessageBarIntent, Skeleton, SkeletonItem, SkeletonItemSize,
    Spinner, SpinnerSize,
};
use orbital_motion::{OrbitalPresence, PresenceMotion};

/// Overlay layer for loading, empty, and error states.
#[component]
pub fn ChartOverlays(
    /// Whether the chart is in a loading state.
    loading: bool,
    /// Whether the chart has no data to display.
    is_empty: Signal<bool>,
    /// Projection error message.
    projection_error: Signal<Option<String>>,
    /// Custom loading slot.
    #[prop(default = None)]
    loading_view: Option<Arc<dyn Fn() -> leptos::prelude::AnyView + Send + Sync>>,
    /// Custom empty slot.
    #[prop(default = None)]
    empty_view: Option<Arc<dyn Fn() -> leptos::prelude::AnyView + Send + Sync>>,
) -> impl IntoView {
    let show_loading = Signal::derive(move || loading);
    let show_empty = Signal::derive(move || !loading && is_empty.get());
    let show_error =
        Signal::derive(move || !loading && projection_error.get().is_some() && !is_empty.get());

    view! {
        <OrbitalPresence show=show_loading motion=Signal::from(PresenceMotion::fade()) appear=true>
            <div class="orb-chart-overlay" role="status" aria-live="polite" data-testid="chart-loading-overlay">
                {loading_view.as_ref().map(|slot| slot()).unwrap_or_else(|| {
                    view! {
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 12px; width: 80%;">
                            <Skeleton>
                                <SkeletonItem size=Signal::from(SkeletonItemSize::S120) />
                            </Skeleton>
                            <Spinner size=Signal::from(SpinnerSize::Small) label="Loading chart data…" />
                        </div>
                    }.into_any()
                })}
            </div>
        </OrbitalPresence>
        <OrbitalPresence show=show_empty motion=Signal::from(PresenceMotion::fade()) appear=true>
            <div class="orb-chart-overlay" role="status" aria-live="polite" data-testid="chart-empty-overlay">
                <div class="orb-chart-overlay__message">
                    {empty_view.as_ref().map(|slot| slot()).unwrap_or_else(|| {
                        view! {
                            <MessageBar intent=MessageBarIntent::Info>
                                <MessageBarBody>"No chart data to display."</MessageBarBody>
                            </MessageBar>
                        }.into_any()
                    })}
                </div>
            </div>
        </OrbitalPresence>
        <OrbitalPresence show=show_error motion=Signal::from(PresenceMotion::fade()) appear=true>
            <div class="orb-chart-overlay" role="alert" data-testid="chart-error-overlay">
                <div class="orb-chart-overlay__message">
                    <MessageBar intent=MessageBarIntent::Error>
                        <MessageBarBody>
                            {move || projection_error.get().unwrap_or_default()}
                        </MessageBarBody>
                    </MessageBar>
                </div>
            </div>
        </OrbitalPresence>
    }
}
