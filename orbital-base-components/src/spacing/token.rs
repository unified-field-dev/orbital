/// Horizontal spacing tokens from the active theme (`--orb-space-inline-*`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpacingHorizontal {
    None,
    XXS,
    XS,
    SNudge,
    S,
    MNudge,
    #[default]
    M,
    L,
    XL,
    XXL,
    XXXL,
}

impl SpacingHorizontal {
    pub fn css_var(self) -> &'static str {
        match self {
            Self::None => "var(--orb-space-inline-none)",
            Self::XXS => "var(--orb-space-inline-2xs)",
            Self::XS => "var(--orb-space-inline-xs)",
            Self::SNudge => "var(--orb-space-inline-snudge)",
            Self::S => "var(--orb-space-inline-sm)",
            Self::MNudge => "var(--orb-space-inline-mnudge)",
            Self::M => "var(--orb-space-inline-md)",
            Self::L => "var(--orb-space-inline-lg)",
            Self::XL => "var(--orb-space-inline-xl)",
            Self::XXL => "var(--orb-space-inline-2xl)",
            Self::XXXL => "var(--orb-space-inline-3xl)",
        }
    }
}

/// Vertical spacing tokens from the active theme (`--orb-space-block-*`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpacingVertical {
    None,
    XXS,
    XS,
    SNudge,
    S,
    MNudge,
    #[default]
    M,
    L,
    XL,
    XXL,
    XXXL,
}

impl SpacingVertical {
    pub fn css_var(self) -> &'static str {
        match self {
            Self::None => "var(--orb-space-block-none)",
            Self::XXS => "var(--orb-space-block-2xs)",
            Self::XS => "var(--orb-space-block-xs)",
            Self::SNudge => "var(--orb-space-block-snudge)",
            Self::S => "var(--orb-space-block-sm)",
            Self::MNudge => "var(--orb-space-block-mnudge)",
            Self::M => "var(--orb-space-block-md)",
            Self::L => "var(--orb-space-block-lg)",
            Self::XL => "var(--orb-space-block-xl)",
            Self::XXL => "var(--orb-space-block-2xl)",
            Self::XXXL => "var(--orb-space-block-3xl)",
        }
    }
}
