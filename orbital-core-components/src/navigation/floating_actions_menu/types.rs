pub use orbital_base_components::{FloatingActionsMenuDirection, FloatingActionsMenuTooltipSide};

use leptos::prelude::*;

/// Configuration for [`FloatingActionsMenu`].
#[derive(Clone)]
pub struct FloatingActionsMenuConfig {
    /// Direction actions expand from the main trigger.
    pub direction: Signal<FloatingActionsMenuDirection>,
    /// Required accessible name for the menu root.
    pub aria_label: String,
    /// When true, item tooltips stay visible while the menu is open.
    pub persistent_tooltips: Signal<bool>,
    /// Side of each action button where persistent tooltips render.
    pub persistent_tooltip_side: Signal<FloatingActionsMenuTooltipSide>,
    /// Optional offset from the right edge in pixels.
    pub right: Signal<Option<f64>>,
    /// Optional offset from the bottom edge in pixels.
    pub bottom: Signal<Option<f64>>,
    /// When true, `right`/`bottom` pin to the viewport; otherwise anchor inside a positioned parent.
    pub viewport_fixed: Signal<bool>,
}

impl Default for FloatingActionsMenuConfig {
    fn default() -> Self {
        Self {
            direction: Signal::from(FloatingActionsMenuDirection::Up),
            aria_label: String::new(),
            persistent_tooltips: Signal::from(false),
            persistent_tooltip_side: Signal::from(FloatingActionsMenuTooltipSide::Left),
            right: Signal::from(Some(24.0)),
            bottom: Signal::from(Some(24.0)),
            viewport_fixed: Signal::from(true),
        }
    }
}

impl FloatingActionsMenuConfig {
    /// Pin to the viewport corner for app-level placement.
    pub fn fixed(right: f64, bottom: f64) -> Self {
        Self {
            right: Signal::from(Some(right)),
            bottom: Signal::from(Some(bottom)),
            viewport_fixed: Signal::from(true),
            ..Default::default()
        }
    }

    /// Pin to the bottom-right of a positioned parent — typical preview/demo frames.
    pub fn anchored(right: f64, bottom: f64) -> Self {
        Self {
            right: Signal::from(Some(right)),
            bottom: Signal::from(Some(bottom)),
            viewport_fixed: Signal::from(false),
            ..Default::default()
        }
    }
}
