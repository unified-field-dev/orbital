#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FloatingButtonVariant {
    /// Icon-only floating control with a rounded square silhouette (default).
    #[default]
    Rounded,
    /// Icon-only floating control with a full circle silhouette.
    Circular,
    /// Icon plus visible label with the same rounded-square corners as [`Self::Rounded`].
    Extended,
}

impl FloatingButtonVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rounded => "rounded",
            Self::Circular => "circular",
            Self::Extended => "extended",
        }
    }
}
