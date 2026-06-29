//! Legend positioning helpers.

use crate::{LegendConfig, LegendHorizontalAlign, LegendVerticalAlign};

/// Approximate width reserved when the legend sits outside the plot on the side.
const OUTSIDE_LEGEND_RESERVE_PX: i32 = 148;

/// CSS class for legend placement within the chart shell.
pub fn legend_position_class(config: &LegendConfig) -> &'static str {
    match (config.position.vertical, config.position.horizontal) {
        (LegendVerticalAlign::Top, LegendHorizontalAlign::Left) => "orb-legend--top-left",
        (LegendVerticalAlign::Top, LegendHorizontalAlign::Middle) => "orb-legend--top-middle",
        (LegendVerticalAlign::Top, LegendHorizontalAlign::Right) => "orb-legend--top-right",
        (LegendVerticalAlign::Middle, LegendHorizontalAlign::Left) => "orb-legend--middle-left",
        (LegendVerticalAlign::Middle, LegendHorizontalAlign::Right) => "orb-legend--middle-right",
        (LegendVerticalAlign::Bottom, LegendHorizontalAlign::Left) => "orb-legend--bottom-left",
        (LegendVerticalAlign::Bottom, LegendHorizontalAlign::Middle) => "orb-legend--bottom-middle",
        (LegendVerticalAlign::Bottom, LegendHorizontalAlign::Right) => "orb-legend--bottom-right",
        _ => "orb-legend--middle-right",
    }
}

/// Inline offset style applying [`LegendConfig::padding`] on the outer edge.
pub fn legend_padding_style(config: &LegendConfig) -> String {
    let pad = config.padding;
    match config.position.horizontal {
        LegendHorizontalAlign::Right | LegendHorizontalAlign::Left => String::new(),
        LegendHorizontalAlign::Middle => match config.position.vertical {
            LegendVerticalAlign::Top => format!("margin-top: {pad}px;"),
            LegendVerticalAlign::Bottom => format!("margin-bottom: {pad}px;"),
            _ => String::new(),
        },
    }
}

/// Inline padding on the chart shell so an outside legend is not clipped.
pub fn legend_outside_reserve_style(config: &LegendConfig) -> Option<String> {
    if config.hidden {
        return None;
    }
    let pad = config.padding as i32 + 8;
    match config.position.horizontal {
        LegendHorizontalAlign::Right => Some(format!(
            "padding-right: {}px;",
            OUTSIDE_LEGEND_RESERVE_PX + pad
        )),
        LegendHorizontalAlign::Left => Some(format!(
            "padding-left: {}px;",
            OUTSIDE_LEGEND_RESERVE_PX + pad
        )),
        _ => None,
    }
}
