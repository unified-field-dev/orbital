//! Chart palette types.

use orbital_theme::BrandPalette;

/// Theme-driven chart color palette.
#[derive(Clone, Debug, Default)]
pub struct OrbitalChartPalette {
    /// Series color tokens resolved from theme accent ramp.
    pub colors: Vec<String>,
}

impl OrbitalChartPalette {
    /// Create a palette from explicit color tokens.
    pub fn new(colors: Vec<String>) -> Self {
        Self { colors }
    }

    /// Build a palette from Orbital theme accent CSS variables.
    pub fn from_theme(brand: Option<&BrandPalette>) -> Self {
        let primary = brand
            .map(|b| b.primary.clone())
            .unwrap_or_else(|| "var(--orb-color-accent-primary, #2563eb)".into());
        Self {
            colors: vec![
                primary.clone(),
                "var(--orb-color-accent-secondary, #7c3aed)".into(),
                "var(--orb-color-accent-tertiary, #059669)".into(),
                "var(--orb-color-accent-quaternary, #d97706)".into(),
                primary,
                "var(--orb-color-accent-secondary, #7c3aed)".into(),
            ],
        }
    }
}
