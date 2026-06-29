use leptos::prelude::*;
use orbital_base_components::{Handler, OverlayAppearance, OverlayPanelSize, Placement};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PopoverSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl PopoverSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

impl From<PopoverSize> for OverlayPanelSize {
    fn from(value: PopoverSize) -> Self {
        match value {
            PopoverSize::Small => OverlayPanelSize::Small,
            PopoverSize::Medium => OverlayPanelSize::Medium,
            PopoverSize::Large => OverlayPanelSize::Large,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PopoverAppearance {
    Brand,
    Inverted,
}

impl PopoverAppearance {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Brand => "brand",
            Self::Inverted => "inverted",
        }
    }
}

impl From<PopoverAppearance> for OverlayAppearance {
    fn from(value: PopoverAppearance) -> Self {
        match value {
            PopoverAppearance::Brand => OverlayAppearance::Brand,
            PopoverAppearance::Inverted => OverlayAppearance::Inverted,
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum PopoverTriggerType {
    #[default]
    Hover,
    Click,
}

impl From<PopoverTriggerType> for orbital_base_components::OverlayTriggerType {
    fn from(value: PopoverTriggerType) -> Self {
        match value {
            PopoverTriggerType::Hover => Self::Hover,
            PopoverTriggerType::Click => Self::Click,
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum PopoverPosition {
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

impl From<PopoverPosition> for Placement {
    fn from(value: PopoverPosition) -> Self {
        match value {
            PopoverPosition::Top => Placement::Top,
            PopoverPosition::Bottom => Placement::Bottom,
            PopoverPosition::Left => Placement::Left,
            PopoverPosition::Right => Placement::Right,
            PopoverPosition::TopStart => Placement::TopStart,
            PopoverPosition::TopEnd => Placement::TopEnd,
            PopoverPosition::LeftStart => Placement::LeftStart,
            PopoverPosition::LeftEnd => Placement::LeftEnd,
            PopoverPosition::RightStart => Placement::RightStart,
            PopoverPosition::RightEnd => Placement::RightEnd,
            PopoverPosition::BottomStart => Placement::BottomStart,
            PopoverPosition::BottomEnd => Placement::BottomEnd,
        }
    }
}

/// Trigger, placement, appearance, and size for [`Popover`](super::popover::Popover).
#[derive(Clone, Copy)]
pub struct PopoverConfig {
    /// How the popover opens: `Hover` (default) or `Click`.
    pub trigger_type: PopoverTriggerType,
    /// Placement relative to the trigger.
    pub position: PopoverPosition,
    /// Visual variant: `Brand` or `Inverted`; omit for default surface.
    pub appearance: MaybeProp<PopoverAppearance>,
    /// Panel width preset.
    pub size: Signal<PopoverSize>,
}

impl Default for PopoverConfig {
    fn default() -> Self {
        Self {
            trigger_type: PopoverTriggerType::Hover,
            position: PopoverPosition::Top,
            appearance: MaybeProp::default(),
            size: Signal::from(PopoverSize::Medium),
        }
    }
}

/// Open/close lifecycle hooks for [`Popover`](super::popover::Popover).
#[derive(Default)]
pub struct PopoverLifecycle {
    /// Called when the popover becomes visible.
    pub on_open: Option<Handler>,
    /// Called when the popover is dismissed.
    pub on_close: Option<Handler>,
}
