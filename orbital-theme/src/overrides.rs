use crate::{Density, ThemeMode};

#[derive(Clone, Debug, Default)]
pub struct BrandPalette {
    pub primary: String,
}

#[derive(Clone, Debug, Default)]
pub struct ThemeOverrides {
    pub mode: Option<ThemeMode>,
    pub brand: Option<BrandPalette>,
    pub density: Option<Density>,
    pub typography: Option<TypographyOverrides>,
    pub spacing: Option<SpacingScale>,
    pub elevation: Option<ElevationScale>,
    pub shape: Option<ShapeOverrides>,
}

#[derive(Clone, Debug, Default)]
pub struct TypographyOverrides {
    pub font_family_base: Option<String>,
    pub font_family_monospace: Option<String>,
    pub font_family_numeric: Option<String>,
    pub font_family_display: Option<String>,
    pub font_size_scale: Option<f32>,
    pub line_height_scale: Option<f32>,
}

#[derive(Clone, Copy, Debug)]
pub struct SpacingScale {
    pub multiplier: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct ElevationScale {
    pub multiplier: f32,
}

impl Default for ElevationScale {
    fn default() -> Self {
        Self { multiplier: 1.0 }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ShapeOverrides {
    pub border_radius_scale: Option<f32>,
}
