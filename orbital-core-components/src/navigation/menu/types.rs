use leptos::prelude::*;
use orbital_base_components::{OverlayAppearance, Placement};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MenuAppearance {
    Brand,
    Inverted,
}

impl MenuAppearance {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Brand => "brand",
            Self::Inverted => "inverted",
        }
    }
}

impl From<MenuAppearance> for OverlayAppearance {
    fn from(value: MenuAppearance) -> Self {
        match value {
            MenuAppearance::Brand => OverlayAppearance::Brand,
            MenuAppearance::Inverted => OverlayAppearance::Inverted,
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum MenuTriggerType {
    #[default]
    Click,
    Hover,
}

impl From<MenuTriggerType> for orbital_base_components::OverlayTriggerType {
    fn from(value: MenuTriggerType) -> Self {
        match value {
            MenuTriggerType::Click => Self::Click,
            MenuTriggerType::Hover => Self::Hover,
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum MenuPosition {
    Top,
    #[default]
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

impl From<MenuPosition> for Placement {
    fn from(value: MenuPosition) -> Self {
        match value {
            MenuPosition::Top => Placement::Top,
            MenuPosition::Bottom => Placement::Bottom,
            MenuPosition::Left => Placement::Left,
            MenuPosition::Right => Placement::Right,
            MenuPosition::TopStart => Placement::TopStart,
            MenuPosition::TopEnd => Placement::TopEnd,
            MenuPosition::LeftStart => Placement::LeftStart,
            MenuPosition::LeftEnd => Placement::LeftEnd,
            MenuPosition::RightStart => Placement::RightStart,
            MenuPosition::RightEnd => Placement::RightEnd,
            MenuPosition::BottomStart => Placement::BottomStart,
            MenuPosition::BottomEnd => Placement::BottomEnd,
        }
    }
}

/// Trigger, placement, and appearance for [`Menu`](super::menu::Menu).
#[derive(Clone, Copy)]
pub struct MenuConfig {
    /// How the menu opens: `Click` (default) or `Hover`.
    pub trigger_type: MenuTriggerType,
    /// Placement relative to the trigger.
    pub position: MenuPosition,
    /// Visual variant: `Brand` or `Inverted`; omit for default surface.
    pub appearance: MaybeProp<MenuAppearance>,
}

impl Default for MenuConfig {
    fn default() -> Self {
        Self {
            trigger_type: MenuTriggerType::Click,
            position: MenuPosition::Bottom,
            appearance: MaybeProp::default(),
        }
    }
}
