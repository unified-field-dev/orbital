use leptos::prelude::*;

pub use orbital_base_components::FloatingButtonVariant;

/// Color emphasis for [`crate::FloatingButton`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FloatingButtonColor {
    #[default]
    Primary,
    Secondary,
}

impl FloatingButtonColor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
        }
    }
}

/// Size preset for [`crate::FloatingButton`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FloatingButtonSize {
    Small,
    Medium,
    #[default]
    Large,
}

impl FloatingButtonSize {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

/// Size preset for [`crate::FloatingButton`].
#[derive(Clone, Copy, Debug)]
pub struct FloatingButtonConfig {
    /// Primary or secondary color treatment.
    pub color: Signal<FloatingButtonColor>,
    /// Small, medium, or large control size.
    pub size: Signal<FloatingButtonSize>,
    /// Circular icon-only, rounded-square icon-only, or extended with label text.
    pub variant: Signal<FloatingButtonVariant>,
    /// When set, pins the button to the viewport with this right offset in pixels.
    pub right: Option<Signal<i32>>,
    /// When set, pins the button to the viewport with this bottom offset in pixels.
    pub bottom: Option<Signal<i32>>,
}

impl Default for FloatingButtonConfig {
    fn default() -> Self {
        Self {
            color: FloatingButtonColor::Primary.into(),
            size: FloatingButtonSize::Large.into(),
            variant: FloatingButtonVariant::Rounded.into(),
            right: None,
            bottom: None,
        }
    }
}

impl FloatingButtonConfig {
    pub fn fixed(right: i32, bottom: i32) -> Self {
        Self {
            right: Some(right.into()),
            bottom: Some(bottom.into()),
            ..Default::default()
        }
    }
}
