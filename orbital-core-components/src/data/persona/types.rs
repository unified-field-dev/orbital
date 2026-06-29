use leptos::prelude::*;

use orbital_base_components::{AvatarColor, FlexGap};

#[slot]
pub struct PersonaPrimaryText {
    pub children: Children,
}

#[slot]
pub struct PersonaSecondaryText {
    pub children: Children,
}

#[slot]
pub struct PersonaTertiaryText {
    pub children: Children,
}

#[slot]
pub struct PersonaQuaternaryText {
    pub children: Children,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum PersonaTextAlignment {
    #[default]
    Start,
    Center,
}

impl PersonaTextAlignment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum PersonaTextPosition {
    Before,
    #[default]
    After,
    Below,
}

impl PersonaTextPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Before => "before",
            Self::After => "after",
            Self::Below => "below",
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum PersonaSize {
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
    Huge,
}

impl PersonaSize {
    pub(crate) fn as_avatar_size(&self) -> u8 {
        match self {
            Self::ExtraSmall => 20,
            Self::Small => 28,
            Self::Medium => 32,
            Self::Large => 36,
            Self::ExtraLarge => 40,
            Self::Huge => 56,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExtraSmall => "extra-small",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::ExtraLarge => "extra-large",
            Self::Huge => "huge",
        }
    }

    pub(crate) fn avatar_gap(self) -> FlexGap {
        match self {
            Self::ExtraSmall => FlexGap::Size(6),
            Self::Small | Self::Medium => FlexGap::Size(8),
            Self::Large | Self::ExtraLarge => FlexGap::Size(10),
            Self::Huge => FlexGap::Size(12),
        }
    }
}

/// Identity and layout configuration for [`crate::Persona`].
#[derive(Clone, Default)]
pub struct PersonaConfig {
    /// Display name; used for avatar initials and default primary text.
    pub name: Option<String>,
    /// Size preset controlling avatar and typography scale.
    pub size: PersonaSize,
    /// Vertical alignment of text relative to the avatar.
    pub text_alignment: PersonaTextAlignment,
    /// Text position: before, after, or below the avatar.
    pub text_position: PersonaTextPosition,
    /// Optional avatar image URL.
    pub avatar_src: Option<String>,
    /// Avatar background color preset forwarded to the internal avatar.
    pub color: AvatarColor,
}

impl PersonaConfig {
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            color: AvatarColor::Colorful,
            ..Default::default()
        }
    }
}
