use crate::{BrandPalette, ElevationScale};

impl Default for ThemeOptions {
    fn default() -> Self {
        Self {
            density: Density::Default,
            direction: Direction::Ltr,
            brand: None,
            elevation: ElevationScale::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ThemeOptions {
    pub density: Density,
    pub direction: Direction,
    pub brand: Option<BrandPalette>,
    pub elevation: ElevationScale,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Density {
    Compact,
    #[default]
    Default,
    Spacious,
}

impl Density {
    pub fn spacing_multiplier(self) -> f32 {
        match self {
            Self::Compact => 0.875,
            Self::Default => 1.0,
            Self::Spacious => 1.125,
        }
    }

    pub fn font_size_multiplier(self) -> f32 {
        match self {
            Self::Compact => 0.9375,
            Self::Default => 1.0,
            Self::Spacious => 1.0625,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Ltr,
    Rtl,
}

impl Direction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }
}
