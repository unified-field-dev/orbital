//! Tooltip configuration types.

use std::sync::Arc;

use leptos::prelude::*;

use super::TooltipTrigger;

/// Custom tooltip content slots.
#[derive(Clone, Default)]
pub struct TooltipContentSlots {
    /// Replaces default item tooltip body.
    pub item_content: Option<Arc<dyn Fn() -> AnyView + Send + Sync>>,
    /// Replaces default axis tooltip body.
    pub axis_content: Option<Arc<dyn Fn() -> AnyView + Send + Sync>>,
}

/// Configuration for chart tooltips.
#[derive(Clone, Default)]
pub struct TooltipConfig {
    /// How hover activates the tooltip.
    pub trigger: TooltipTrigger,
    /// When true, axis tooltips omit the x-axis value header row.
    pub hide_x_header: bool,
    /// Optional custom content renderers.
    pub slots: TooltipContentSlots,
}

impl TooltipConfig {
    /// Item-triggered tooltip with defaults.
    pub fn item() -> Self {
        Self {
            trigger: TooltipTrigger::Item,
            ..Default::default()
        }
    }

    /// Axis-triggered tooltip with defaults.
    pub fn axis() -> Self {
        Self {
            trigger: TooltipTrigger::Axis,
            ..Default::default()
        }
    }
}
