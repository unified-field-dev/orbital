use leptos::prelude::*;
use orbital_base_components::{OverlayAppearance, Placement};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum TooltipAppearance {
    #[default]
    Normal,
    Inverted,
}

impl TooltipAppearance {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Inverted => "inverted",
        }
    }
}

impl From<TooltipAppearance> for OverlayAppearance {
    fn from(value: TooltipAppearance) -> Self {
        match value {
            TooltipAppearance::Normal => OverlayAppearance::Normal,
            TooltipAppearance::Inverted => OverlayAppearance::Inverted,
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    TopStart,
    TopEnd,
    LeftStart,
    LeftEnd,
    RightStart,
    RightEnd,
    BottomStart,
    BottomEnd,
}

impl From<TooltipPosition> for Placement {
    fn from(value: TooltipPosition) -> Self {
        match value {
            TooltipPosition::Top => Placement::Top,
            TooltipPosition::Bottom => Placement::Bottom,
            TooltipPosition::Left => Placement::Left,
            TooltipPosition::Right => Placement::Right,
            TooltipPosition::TopStart => Placement::TopStart,
            TooltipPosition::TopEnd => Placement::TopEnd,
            TooltipPosition::LeftStart => Placement::LeftStart,
            TooltipPosition::LeftEnd => Placement::LeftEnd,
            TooltipPosition::RightStart => Placement::RightStart,
            TooltipPosition::RightEnd => Placement::RightEnd,
            TooltipPosition::BottomStart => Placement::BottomStart,
            TooltipPosition::BottomEnd => Placement::BottomEnd,
        }
    }
}

/// Placement and appearance for [`Tooltip`](super::tooltip::Tooltip).
#[derive(Clone, Copy)]
pub struct TooltipConfig {
    /// Placement relative to the trigger (`Top` default).
    pub position: TooltipPosition,
    /// Visual variant: `Normal` (default) or `Inverted`.
    pub appearance: Signal<TooltipAppearance>,
}

impl Default for TooltipConfig {
    fn default() -> Self {
        Self {
            position: TooltipPosition::Top,
            appearance: Signal::from(TooltipAppearance::Normal),
        }
    }
}
