//! Chart theme extension types.

use super::{HighlightScope, TooltipTrigger};

/// Chart-specific theme defaults applied via CSS variables on `[data-orbital-chart]`.
#[derive(Clone, Debug, PartialEq)]
pub struct OrbitalChartsTheme {
    /// Opacity for faded non-highlighted marks (0.0–1.0).
    pub fade_opacity: f64,
    /// Default gap between legend items (maps to `--orbital-chart-legend-gap`).
    pub legend_gap: f64,
    /// Tooltip show delay in milliseconds.
    pub tooltip_delay_ms: u64,
    /// Default highlight scope when not overridden on the chart.
    pub default_highlight_scope: HighlightScope,
    /// Default tooltip trigger when not overridden.
    pub default_tooltip_trigger: TooltipTrigger,
}

impl Default for OrbitalChartsTheme {
    fn default() -> Self {
        Self {
            fade_opacity: 0.35,
            legend_gap: 12.0,
            tooltip_delay_ms: 0,
            default_highlight_scope: HighlightScope {
                highlight: super::HighlightMode::Item,
                fade: super::FadeMode::Global,
            },
            default_tooltip_trigger: TooltipTrigger::Item,
        }
    }
}

impl OrbitalChartsTheme {
    /// CSS custom properties for injection on the chart root.
    pub fn css_vars(&self) -> String {
        format!(
            "--orbital-chart-fade-opacity: {}; --orbital-chart-legend-gap: {}px; --orbital-chart-tooltip-delay: {}ms;",
            self.fade_opacity, self.legend_gap, self.tooltip_delay_ms
        )
    }
}
