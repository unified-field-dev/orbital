use crate::tokens::{ColorTheme, CommonTheme};
use crate::{BrandPalette, ThemeOptions, ThemeOverrides};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

impl ThemeMode {
    pub fn as_name(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

/// Bundles all injectable tokens for one theme scope.
#[derive(Clone)]
pub struct Theme {
    pub mode: ThemeMode,
    pub color: ColorTheme,
    pub common: CommonTheme,
    pub options: ThemeOptions,
}

impl Theme {
    pub fn light() -> Self {
        Self::for_mode(ThemeMode::Light)
    }

    pub fn dark() -> Self {
        Self::for_mode(ThemeMode::Dark)
    }

    pub fn for_mode(mode: ThemeMode) -> Self {
        let mut common = CommonTheme::new();
        crate::ramps::apply_orbital_shape_defaults(&mut common);
        Self {
            mode,
            color: match mode {
                ThemeMode::Light => ColorTheme::light(),
                ThemeMode::Dark => ColorTheme::dark(),
            },
            common,
            options: ThemeOptions::default(),
        }
    }

    pub fn custom(base: ThemeMode, overrides: ThemeOverrides) -> Self {
        let mut theme = Self::for_mode(base);
        theme.apply_overrides(overrides);
        theme
    }

    pub fn with_brand(base: ThemeMode, brand: BrandPalette) -> Self {
        Self::custom(
            base,
            ThemeOverrides {
                brand: Some(brand),
                ..Default::default()
            },
        )
    }

    pub fn apply_overrides(&mut self, overrides: ThemeOverrides) {
        if let Some(mode) = overrides.mode {
            self.mode = mode;
            self.rebuild_color_tokens();
        }

        if let Some(brand) = overrides.brand {
            self.options.brand = Some(brand);
            self.rebuild_color_tokens();
        }

        if let Some(density) = overrides.density {
            self.options.density = density;
            self.rebuild_common_tokens();
            crate::ramps::apply_density(&mut self.common, density);
        }

        if let Some(typography) = overrides.typography {
            crate::ramps::apply_typography(&mut self.common, &typography);
        }

        if let Some(spacing) = overrides.spacing {
            crate::ramps::apply_spacing_scale(&mut self.common, spacing);
        }

        if let Some(elevation) = overrides.elevation {
            self.options.elevation = elevation;
            self.rebuild_color_tokens();
            crate::ramps::apply_elevation_scale(&mut self.color, elevation);
        }

        if let Some(shape) = overrides.shape {
            crate::ramps::apply_shape_overrides(&mut self.common, &shape);
        }
    }

    fn rebuild_common_tokens(&mut self) {
        let mut common = CommonTheme::new();
        crate::ramps::apply_orbital_shape_defaults(&mut common);
        self.common = common;
    }

    pub(crate) fn rebuild_color_tokens(&mut self) {
        self.color = match self.mode {
            ThemeMode::Light => ColorTheme::light(),
            ThemeMode::Dark => ColorTheme::dark(),
        };
        if let Some(brand) = &self.options.brand {
            let ramp = crate::ramps::brand_ramp(&brand.primary);
            self.color = match self.mode {
                ThemeMode::Light => ColorTheme::custom_light(&ramp),
                ThemeMode::Dark => ColorTheme::custom_dark(&ramp),
            };
        }
        if (self.options.elevation.multiplier - 1.0).abs() > f32::EPSILON {
            crate::ramps::apply_elevation_scale(&mut self.color, self.options.elevation);
        }
    }

    pub fn use_theme(default: impl Fn() -> Theme) -> leptos::prelude::ReadSignal<Theme> {
        crate::context::ThemeInjection::use_theme(default)
    }

    pub fn use_rw_theme() -> leptos::prelude::RwSignal<Theme> {
        crate::context::ThemeInjection::use_rw_theme()
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

impl Theme {
    pub(crate) fn write_css_vars(&self, out: &mut String) {
        self.common.write_orb_common_css_vars(out);
        self.common.write_orb_motion_css_vars(out);
        self.color.write_orb_color_css_vars(out);
    }
}
