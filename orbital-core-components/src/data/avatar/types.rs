pub use orbital_base_components::{AvatarColor, AvatarShape};

/// Display configuration for [`crate::Avatar`].
#[derive(Clone, Default)]
pub struct AvatarConfig {
    /// Image URL for the avatar photo.
    pub src: Option<String>,
    /// Display name used for `aria-label` and auto-initials.
    pub name: Option<String>,
    /// Custom initials when auto-generation is not desired.
    pub initials: Option<String>,
    /// Circular (default) or square outline.
    pub shape: AvatarShape,
    /// Width and height in pixels.
    pub size: Option<u8>,
    /// Background color preset for initials and icon fallback.
    pub color: AvatarColor,
    /// Hash key for `AvatarColor::Colorful` when `name` is absent.
    pub id_for_color: Option<String>,
}

impl AvatarConfig {
    pub fn name(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            color: AvatarColor::Colorful,
            ..Default::default()
        }
    }

    pub fn image(src: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            src: Some(src.into()),
            name: Some(name.into()),
            color: AvatarColor::Colorful,
            ..Default::default()
        }
    }

    pub fn initials(initials: impl Into<String>) -> Self {
        let initials = initials.into();
        Self {
            initials: Some(initials.clone()),
            color: AvatarColor::Colorful,
            id_for_color: Some(initials),
            ..Default::default()
        }
    }

    pub fn sized(name: impl Into<String>, size: u8) -> Self {
        Self {
            name: Some(name.into()),
            size: Some(size),
            color: AvatarColor::Colorful,
            ..Default::default()
        }
    }

    pub fn shaped(name: impl Into<String>, shape: AvatarShape) -> Self {
        Self {
            name: Some(name.into()),
            shape,
            color: AvatarColor::Colorful,
            ..Default::default()
        }
    }

    pub fn colored(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            color: AvatarColor::Colorful,
            ..Default::default()
        }
    }

    pub fn with_color(name: impl Into<String>, color: AvatarColor) -> Self {
        Self {
            name: Some(name.into()),
            color,
            ..Default::default()
        }
    }
}
