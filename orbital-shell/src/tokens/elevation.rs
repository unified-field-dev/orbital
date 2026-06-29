//! Elevation (shadow depth) for surfaces.

/// Orbital shadow tiers. Use sparingly — each surface maps to one tier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Elevation {
    /// Page background, flat sections.
    Flat,
    /// Hover lift on inline cards.
    Shadow2,
    /// Family cards at rest, capability-tab panels, balance-sheet rows.
    Shadow4,
    /// Hero primary CTA, raised CTA panel, scrolled top-bar.
    Shadow8,
    /// Callouts, device-profile popovers.
    Shadow16,
    /// Mobile nav dropdown panels.
    Shadow28,
    /// Sponsor coachmark, dialog-style surfaces.
    Shadow64,
}

impl Elevation {
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::Flat => "orbital-token-elev-flat",
            Self::Shadow2 => "orbital-token-elev-shadow2",
            Self::Shadow4 => "orbital-token-elev-shadow4",
            Self::Shadow8 => "orbital-token-elev-shadow8",
            Self::Shadow16 => "orbital-token-elev-shadow16",
            Self::Shadow28 => "orbital-token-elev-shadow28",
            Self::Shadow64 => "orbital-token-elev-shadow64",
        }
    }

    /// Maps to theme shadow CSS variables where defined.
    pub const fn as_token(self) -> &'static str {
        match self {
            Self::Flat => "none",
            Self::Shadow2 => "var(--orb-elev-raised-xs)",
            Self::Shadow4 => "var(--orb-elev-raised-sm)",
            Self::Shadow8 => "var(--orb-elev-raised-md)",
            Self::Shadow16 => "var(--orb-elev-floating)",
            Self::Shadow28 => "var(--orb-elev-overlay)",
            Self::Shadow64 => "var(--orb-elev-modal)",
        }
    }
}
