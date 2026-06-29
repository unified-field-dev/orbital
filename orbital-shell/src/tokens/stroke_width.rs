//! Stroke widths for borders, dividers, and focus rings.

/// Stroke width on the Orbital scale.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StrokeWidth {
    /// Hairline dividers, column underlines in nav.
    Thin,
    /// Focus ring and strong underlines (capability tab indicator).
    Thick,
    Thicker,
    Thickest,
}

impl StrokeWidth {
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::Thin => "orbital-token-stroke-thin",
            Self::Thick => "orbital-token-stroke-thick",
            Self::Thicker => "orbital-token-stroke-thicker",
            Self::Thickest => "orbital-token-stroke-thickest",
        }
    }

    pub const fn as_token(self) -> &'static str {
        match self {
            Self::Thin => "var(--orb-stroke-thin)",
            Self::Thick => "var(--orb-stroke-thick)",
            Self::Thicker => "var(--orb-stroke-thicker)",
            Self::Thickest => "var(--orb-stroke-thickest)",
        }
    }
}
