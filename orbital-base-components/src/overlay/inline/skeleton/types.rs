/// Preset pixel heights for [`super::BaseSkeletonItem`], matching the Orbital type scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SkeletonItemSize {
    S8,
    S12,
    S14,
    #[default]
    S16,
    S20,
    S22,
    S24,
    S28,
    S32,
    S36,
    S40,
    S48,
    S52,
    S56,
    S64,
    S72,
    S92,
    S96,
    S120,
    S128,
}

impl SkeletonItemSize {
    pub fn px(self) -> u16 {
        match self {
            Self::S8 => 8,
            Self::S12 => 12,
            Self::S14 => 14,
            Self::S16 => 16,
            Self::S20 => 20,
            Self::S22 => 22,
            Self::S24 => 24,
            Self::S28 => 28,
            Self::S32 => 32,
            Self::S36 => 36,
            Self::S40 => 40,
            Self::S48 => 48,
            Self::S52 => 52,
            Self::S56 => 56,
            Self::S64 => 64,
            Self::S72 => 72,
            Self::S92 => 92,
            Self::S96 => 96,
            Self::S120 => 120,
            Self::S128 => 128,
        }
    }
}

/// Placeholder geometry for [`super::BaseSkeletonItem`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SkeletonItemShape {
    #[default]
    Rectangle,
    Square,
    Circle,
}

impl SkeletonItemShape {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rectangle => "rectangle",
            Self::Square => "square",
            Self::Circle => "circle",
        }
    }
}
