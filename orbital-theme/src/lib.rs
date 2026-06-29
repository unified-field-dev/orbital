//! Orbital theme — CSS variable injection and runtime customization.
//!
//! Provides [`OrbitalThemeProvider`] and theme hooks for light/dark mode, brand
//! color, density, typography, spacing, and elevation overrides.

mod context;
mod fonts;
mod hooks;
mod options;
mod overrides;
mod provider;
mod ramps;
mod theme;
mod tokens;

pub use context::ThemeInjection;
pub use hooks::{
    set_brand_palette, set_density, set_elevation_scale, set_spacing_scale, set_theme_mode,
    set_typography, use_set_theme, use_theme_mode, use_theme_options, use_update_theme,
};
pub use options::{Density, Direction, ThemeOptions};
pub use overrides::{
    BrandPalette, ElevationScale, ShapeOverrides, SpacingScale, ThemeOverrides, TypographyOverrides,
};
pub use provider::OrbitalThemeProvider;
pub use ramps::brand_ramp;
pub use theme::{Theme, ThemeMode};
pub use tokens::{ColorTheme, CommonTheme};
