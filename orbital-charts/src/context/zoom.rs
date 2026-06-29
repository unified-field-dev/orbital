//! Zoom state context for controlled zoom windows.

use std::collections::HashMap;

use leptos::callback::Callback;
use leptos::prelude::*;

use crate::engine::{clamp_zoom_window, default_zoom_window, enabled_zoom_axes};
use crate::{AxisDef, ChartFeatures, ZoomConfig, ZoomWindow};

/// Active drag state for pan gesture.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoomDragState {
    /// Axis being panned.
    pub axis_id: String,
    /// Pointer x at drag start (plot coordinates).
    pub start_x: f64,
    /// Zoom window at drag start.
    pub start_window: ZoomWindow,
}

/// Zoom interaction context provided to chart children.
#[derive(Clone)]
pub struct ChartZoomContext {
    /// Current zoom windows per axis.
    pub windows: RwSignal<Vec<ZoomWindow>>,
    /// Active pan drag, if any.
    pub dragging: RwSignal<Option<ZoomDragState>>,
    /// Zoom config per enabled axis id.
    pub enabled_axes: HashMap<String, ZoomConfig>,
    /// Feature flags gating zoom.
    pub features: ChartFeatures,
    /// Plot width for pan delta conversion.
    pub plot_width: f64,
    /// Fired when zoom windows change.
    pub on_zoom_change: Option<Callback<(Vec<ZoomWindow>,), ()>>,
}

impl ChartZoomContext {
    /// Whether any axis has zoom enabled.
    pub fn is_active(&self) -> bool {
        !self.enabled_axes.is_empty()
    }

    /// Config for an axis id.
    pub fn config_for(&self, axis_id: &str) -> Option<&ZoomConfig> {
        self.enabled_axes.get(axis_id)
    }

    /// Update windows, clamp per axis config, and notify listeners.
    pub fn set_windows(&self, mut windows: Vec<ZoomWindow>) {
        for window in &mut windows {
            if let Some(config) = self.enabled_axes.get(&window.axis_id) {
                *window = clamp_zoom_window(window.clone(), config);
            }
        }
        self.windows.set(windows.clone());
        if let Some(cb) = self.on_zoom_change.as_ref() {
            cb.run((windows,));
        }
    }

    /// Update a single axis window.
    pub fn update_axis_window(&self, axis_id: &str, window: ZoomWindow) {
        let mut windows = self.windows.get_untracked();
        if let Some(existing) = windows.iter_mut().find(|w| w.axis_id == axis_id) {
            *existing = window;
        } else {
            windows.push(window);
        }
        self.set_windows(windows);
    }
}

/// Build initial zoom windows from props and axis definitions.
pub fn initial_zoom_windows(
    controlled: Option<&[ZoomWindow]>,
    features: ChartFeatures,
    x_axes: &[AxisDef],
    y_axes: &[AxisDef],
) -> Vec<ZoomWindow> {
    let enabled = enabled_zoom_axes(features, x_axes, y_axes);
    if enabled.is_empty() {
        return Vec::new();
    }

    if let Some(controlled) = controlled {
        return controlled.to_vec();
    }

    enabled
        .into_iter()
        .map(|(axis, _)| default_zoom_window(axis.id.clone()))
        .collect()
}

/// Provide zoom context to descendants.
#[component]
pub fn ChartZoomProvider(
    windows: RwSignal<Vec<ZoomWindow>>,
    dragging: RwSignal<Option<ZoomDragState>>,
    enabled_axes: HashMap<String, ZoomConfig>,
    features: ChartFeatures,
    plot_width: f64,
    on_zoom_change: Option<Callback<(Vec<ZoomWindow>,), ()>>,
    children: Children,
) -> impl IntoView {
    provide_context(ChartZoomContext {
        windows,
        dragging,
        enabled_axes,
        features,
        plot_width,
        on_zoom_change,
    });
    children()
}

/// Access zoom context when present.
pub fn use_chart_zoom() -> Option<ChartZoomContext> {
    use_context::<ChartZoomContext>()
}
