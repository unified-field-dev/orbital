use orbital_base_components::{FlexAlign, FlexGap, FlexJustify};

/// Distribution and alignment configuration for [`crate::Space`].
#[derive(Clone, Copy)]
pub struct SpaceConfig {
    /// Gap preset or custom size between items when justify is not space-between.
    pub gap: FlexGap,
    /// When true, stacks items vertically.
    pub vertical: bool,
    /// Cross-axis alignment.
    pub align: Option<FlexAlign>,
    /// Main-axis distribution. Defaults to [`FlexJustify::SpaceBetween`].
    pub justify: Option<FlexJustify>,
}

impl Default for SpaceConfig {
    fn default() -> Self {
        Self {
            gap: FlexGap::Medium,
            vertical: false,
            align: None,
            justify: Some(FlexJustify::SpaceBetween),
        }
    }
}

impl SpaceConfig {
    pub fn horizontal() -> Self {
        Self::default()
    }

    pub fn vertical(gap: FlexGap) -> Self {
        Self {
            gap,
            vertical: true,
            align: None,
            justify: Some(FlexJustify::SpaceBetween),
        }
    }

    /// Even-gap row — prefer [`crate::Stack`] for the same layout without overrides.
    pub fn even_gap(gap: FlexGap) -> Self {
        Self {
            gap,
            vertical: false,
            align: None,
            justify: None,
        }
    }
}

/// Gap spacing preset alias for [`FlexGap`].
pub type SpaceGap = FlexGap;
