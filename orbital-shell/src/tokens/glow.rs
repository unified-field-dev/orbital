//! Ambient glow intensity behind marketing heroes and accents.

/// Brand-tinted hero glow behind `MarketingSurface` and family heroes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GlowIntensity {
    None,
    /// 12–18% opacity mix — default hero ambience.
    Subtle,
    /// Stronger glow for sponsor / key CTA moments.
    Strong,
}

impl GlowIntensity {
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::None => "orbital-token-glow-none",
            Self::Subtle => "orbital-token-glow-subtle",
            Self::Strong => "orbital-token-glow-strong",
        }
    }

    /// Opacity hint for `::after` glow (caller applies `color-mix` with brand token).
    pub const fn opacity_percent(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Subtle => 18,
            Self::Strong => 28,
        }
    }
}
