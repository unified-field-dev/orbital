use leptos::prelude::*;

use crate::{
    BrandPalette, Density, ElevationScale, SpacingScale, Theme, ThemeMode, ThemeOptions,
    ThemeOverrides, TypographyOverrides,
};

pub fn use_set_theme() -> WriteSignal<Theme> {
    Theme::use_rw_theme().write_only()
}

pub fn use_theme_mode() -> (Signal<ThemeMode>, impl Fn(ThemeMode) + Clone) {
    let theme = Theme::use_rw_theme();
    let read = Signal::derive(move || theme.with(|t| t.mode));
    let set = move |mode: ThemeMode| set_theme_mode(theme, mode);
    (read, set)
}

pub fn use_theme_options() -> Signal<ThemeOptions> {
    let theme = Theme::use_rw_theme();
    Signal::derive(move || theme.with(|t| t.options.clone()))
}

pub fn use_update_theme() -> impl Fn(ThemeOverrides) + Clone {
    let theme = Theme::use_rw_theme();
    move |overrides: ThemeOverrides| {
        theme.update(|t| t.apply_overrides(overrides));
    }
}

pub fn set_theme_mode(theme: RwSignal<Theme>, mode: ThemeMode) {
    theme.update(|t| {
        t.mode = mode;
        t.rebuild_color_tokens();
    });
}

pub fn set_brand_palette(theme: RwSignal<Theme>, brand: BrandPalette) {
    theme.update(|t| {
        t.apply_overrides(ThemeOverrides {
            brand: Some(brand),
            ..Default::default()
        });
    });
}

pub fn set_density(theme: RwSignal<Theme>, density: Density) {
    theme.update(|t| {
        t.apply_overrides(ThemeOverrides {
            density: Some(density),
            ..Default::default()
        });
    });
}

pub fn set_typography(theme: RwSignal<Theme>, typography: TypographyOverrides) {
    theme.update(|t| {
        t.apply_overrides(ThemeOverrides {
            typography: Some(typography),
            ..Default::default()
        });
    });
}

pub fn set_spacing_scale(theme: RwSignal<Theme>, scale: SpacingScale) {
    theme.update(|t| {
        t.apply_overrides(ThemeOverrides {
            spacing: Some(scale),
            ..Default::default()
        });
    });
}

pub fn set_elevation_scale(theme: RwSignal<Theme>, scale: ElevationScale) {
    theme.update(|t| {
        t.apply_overrides(ThemeOverrides {
            elevation: Some(scale),
            ..Default::default()
        });
    });
}
