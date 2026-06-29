//! Brand tone and per-platform-family palette mapping.

/// Cross-page accent vs family-scoped accent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BrandTone {
    /// Global marketing CTA / hero cross-family accent.
    Brand,
    Neutral,
    Subtle,
    Accent,
    /// Family-scoped hero glow, tab indicator, card top border.
    Family(PlatformFamilyBrand),
}

/// First-class platform families — maps to shared palette ramps.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlatformFamilyBrand {
    Valence,
    Gluon,
    Nucleus,
    Chronon,
    Boson,
    Photon,
    Orbital,
    Spectra,
    Neutrino,
    Higgs,
    Phonon,
    Polaron,
    Magnon,
}

impl PlatformFamilyBrand {
    /// Stable class for family-scoped surfaces (combined with token CSS).
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::Valence => "orbital-token-family-valence",
            Self::Gluon => "orbital-token-family-gluon",
            Self::Nucleus => "orbital-token-family-nucleus",
            Self::Chronon => "orbital-token-family-chronon",
            Self::Boson => "orbital-token-family-boson",
            Self::Photon => "orbital-token-family-photon",
            Self::Orbital => "orbital-token-family-orbital",
            Self::Spectra => "orbital-token-family-spectra",
            Self::Neutrino => "orbital-token-family-neutrino",
            Self::Higgs => "orbital-token-family-higgs",
            Self::Phonon => "orbital-token-family-phonon",
            Self::Polaron => "orbital-token-family-polaron",
            Self::Magnon => "orbital-token-family-magnon",
        }
    }

    /// Primary family background (palette group).
    pub const fn bg1(self) -> &'static str {
        match self {
            Self::Valence => "var(--orb-color-family-valence-bg-subtle)",
            Self::Gluon => "var(--orb-color-family-gluon-bg-subtle)",
            Self::Nucleus => "var(--orb-color-family-nucleus-bg-subtle)",
            Self::Chronon => "var(--orb-color-family-chronon-bg-subtle)",
            Self::Boson => "var(--orb-color-family-boson-bg-subtle)",
            Self::Photon => "var(--orb-color-family-photon-bg-subtle)",
            Self::Orbital => "var(--orb-color-family-orbital-bg-subtle)",
            Self::Spectra => "var(--orb-color-family-spectra-bg-subtle)",
            Self::Neutrino => "var(--orb-color-family-neutrino-bg-subtle)",
            Self::Higgs => "var(--orb-color-family-higgs-bg-subtle)",
            Self::Phonon => "var(--orb-color-family-phonon-bg-subtle)",
            Self::Polaron => "var(--orb-color-family-polaron-bg-subtle)",
            Self::Magnon => "var(--orb-color-family-magnon-bg-subtle)",
        }
    }

    pub const fn bg2(self) -> &'static str {
        match self {
            Self::Valence => "var(--orb-color-family-valence-bg-muted)",
            Self::Gluon => "var(--orb-color-family-gluon-bg-muted)",
            Self::Nucleus => "var(--orb-color-family-nucleus-bg-muted)",
            Self::Chronon => "var(--orb-color-family-chronon-bg-muted)",
            Self::Boson => "var(--orb-color-family-boson-bg-muted)",
            Self::Photon => "var(--orb-color-family-photon-bg-muted)",
            Self::Orbital => "var(--orb-color-family-orbital-bg-muted)",
            Self::Spectra => "var(--orb-color-family-spectra-bg-muted)",
            Self::Neutrino => "var(--orb-color-family-neutrino-bg-muted)",
            Self::Higgs => "var(--orb-color-family-higgs-bg-muted)",
            Self::Phonon => "var(--orb-color-family-phonon-bg-muted)",
            Self::Polaron => "var(--orb-color-family-polaron-bg-muted)",
            Self::Magnon => "var(--orb-color-family-magnon-bg-muted)",
        }
    }

    pub const fn fg1(self) -> &'static str {
        match self {
            Self::Valence => "var(--orb-color-family-valence-fg)",
            Self::Gluon => "var(--orb-color-family-gluon-fg)",
            Self::Nucleus => "var(--orb-color-family-nucleus-fg)",
            Self::Chronon => "var(--orb-color-family-chronon-fg)",
            Self::Boson => "var(--orb-color-family-boson-fg)",
            Self::Photon => "var(--orb-color-family-photon-fg)",
            Self::Orbital => "var(--orb-color-family-orbital-fg)",
            Self::Spectra => "var(--orb-color-family-spectra-fg)",
            Self::Neutrino => "var(--orb-color-family-neutrino-fg)",
            Self::Higgs => "var(--orb-color-family-higgs-fg)",
            Self::Phonon => "var(--orb-color-family-phonon-fg)",
            Self::Polaron => "var(--orb-color-family-polaron-fg)",
            Self::Magnon => "var(--orb-color-family-magnon-fg)",
        }
    }

    pub const fn fg2(self) -> &'static str {
        match self {
            Self::Valence => "var(--orb-color-family-valence-fg-muted)",
            Self::Gluon => "var(--orb-color-family-gluon-fg-muted)",
            Self::Nucleus => "var(--orb-color-family-nucleus-fg-muted)",
            Self::Chronon => "var(--orb-color-family-chronon-fg-muted)",
            Self::Boson => "var(--orb-color-family-boson-fg-muted)",
            Self::Photon => "var(--orb-color-family-photon-fg-muted)",
            Self::Orbital => "var(--orb-color-family-orbital-fg-muted)",
            Self::Spectra => "var(--orb-color-family-spectra-fg-muted)",
            Self::Neutrino => "var(--orb-color-family-neutrino-fg-muted)",
            Self::Higgs => "var(--orb-color-family-higgs-fg-muted)",
            Self::Phonon => "var(--orb-color-family-phonon-fg-muted)",
            Self::Polaron => "var(--orb-color-family-polaron-fg-muted)",
            Self::Magnon => "var(--orb-color-family-magnon-fg-muted)",
        }
    }

    pub const fn stroke1(self) -> &'static str {
        match self {
            Self::Valence => "var(--orb-color-family-valence-border)",
            Self::Gluon => "var(--orb-color-family-gluon-border)",
            Self::Nucleus => "var(--orb-color-family-nucleus-border)",
            Self::Chronon => "var(--orb-color-family-chronon-border)",
            Self::Boson => "var(--orb-color-family-boson-border)",
            Self::Photon => "var(--orb-color-family-photon-border)",
            Self::Orbital => "var(--orb-color-family-orbital-border)",
            Self::Spectra => "var(--orb-color-family-spectra-border)",
            Self::Neutrino => "var(--orb-color-family-neutrino-border)",
            Self::Higgs => "var(--orb-color-family-higgs-border)",
            Self::Phonon => "var(--orb-color-family-phonon-border)",
            Self::Polaron => "var(--orb-color-family-polaron-border)",
            Self::Magnon => "var(--orb-color-family-magnon-border)",
        }
    }

    /// Active tab / strong underline (`*BorderActive` where the theme defines it).
    ///
    /// Note: some dark palettes omit `*BorderActive` for several hues; those resolve effectively transparent in the browser. Prefer [`stroke1`](Self::stroke1) for always-visible family strokes (e.g. small marketing accent bars).
    pub const fn border_active(self) -> &'static str {
        match self {
            Self::Valence => "var(--orb-color-family-valence-border-active)",
            Self::Gluon => "var(--orb-color-family-gluon-border-active)",
            Self::Nucleus => "var(--orb-color-family-nucleus-border-active)",
            Self::Chronon => "var(--orb-color-family-chronon-border-active)",
            Self::Boson => "var(--orb-color-family-boson-border-active)",
            Self::Photon => "var(--orb-color-family-photon-border-active)",
            Self::Orbital => "var(--orb-color-family-orbital-border-active)",
            Self::Spectra => "var(--orb-color-family-spectra-border-active)",
            Self::Neutrino => "var(--orb-color-family-neutrino-border-active)",
            Self::Higgs => "var(--orb-color-family-higgs-border-active)",
            Self::Phonon => "var(--orb-color-family-phonon-border-active)",
            Self::Polaron => "var(--orb-color-family-polaron-border-active)",
            Self::Magnon => "var(--orb-color-family-magnon-border-active)",
        }
    }

    /// Parse URL slug → brand (for `/platform/:slug` until explicit routes land).
    pub fn from_slug(slug: &str) -> Option<Self> {
        Some(match slug {
            "valence" => Self::Valence,
            "gluon" => Self::Gluon,
            "nucleus" => Self::Nucleus,
            "chronon" => Self::Chronon,
            "boson" => Self::Boson,
            "photon" => Self::Photon,
            "orbital" => Self::Orbital,
            "spectra" => Self::Spectra,
            "neutrino" => Self::Neutrino,
            "higgs" => Self::Higgs,
            "phonon" => Self::Phonon,
            "polaron" => Self::Polaron,
            "magnon" => Self::Magnon,
            _ => return None,
        })
    }
}

impl BrandTone {
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::Brand => "orbital-token-tone-brand",
            Self::Neutral => "orbital-token-tone-neutral",
            Self::Subtle => "orbital-token-tone-subtle",
            Self::Accent => "orbital-token-tone-accent",
            Self::Family(f) => f.as_class(),
        }
    }

    /// Primary CSS token for accent (background or glow source).
    pub fn accent_token(self) -> &'static str {
        match self {
            Self::Brand => "var(--orb-color-brand-bg)",
            Self::Neutral => "var(--orb-color-surface-subtle)",
            Self::Subtle => "var(--orb-color-surface-overlay)",
            Self::Accent => "var(--orb-color-brand-bg-subtle)",
            Self::Family(f) => f.bg2(),
        }
    }
}
