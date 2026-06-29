/// Orbital material treatment for a surface root.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialVariant {
    /// Opaque surface using neutral background tokens.
    #[default]
    Solid,
    /// Semi-transparent frosted glass for transient overlays and popovers.
    Frost,
    /// Opaque shell base layer with subtle wallpaper tint (app chrome).
    Shell,
    /// Translucent dimming scrim behind modals.
    Scrim,
    /// Flat bordered surface — stroke outline, typically paired with [`MaterialElevation::Flat`].
    Outlined,
}

/// Corner treatment for the surface root.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialCorners {
    /// Default rounded corners (`--orb-radius-md`).
    #[default]
    Rounded,
    /// Square corners (`border-radius: 0`).
    Square,
}

/// Orbital elevation tier mapped to shadow CSS variables.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialElevation {
    /// No shadow — flush with the parent plane.
    Flat,
    /// Default lift (`--orb-elev-raised-sm`) for cards and panels at rest.
    #[default]
    Resting,
    /// Raised lift (`--orb-elev-raised-md`) for emphasized surfaces.
    Raised,
    /// Floating lift (`--orb-elev-floating`) for popovers and callouts.
    Floating,
    /// Modal lift (`--orb-elev-modal`) for dialogs and blocking overlays.
    Modal,
}

impl MaterialVariant {
    pub const fn as_data(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Frost => "frost",
            Self::Shell => "shell",
            Self::Scrim => "scrim",
            Self::Outlined => "outlined",
        }
    }

    pub const fn modifier_class(self) -> &'static str {
        match self {
            Self::Solid => "orbital-material--solid",
            Self::Frost => "orbital-material--frost",
            Self::Shell => "orbital-material--shell",
            Self::Scrim => "orbital-material--scrim",
            Self::Outlined => "orbital-material--outlined",
        }
    }
}

impl MaterialCorners {
    pub const fn as_data(self) -> &'static str {
        match self {
            Self::Rounded => "rounded",
            Self::Square => "square",
        }
    }

    pub const fn modifier_class(self) -> &'static str {
        match self {
            Self::Rounded => "orbital-material--rounded",
            Self::Square => "orbital-material--square",
        }
    }
}

impl MaterialElevation {
    pub const fn shadow_token(self) -> &'static str {
        match self {
            Self::Flat => "none",
            Self::Resting => "var(--orb-elev-raised-sm)",
            Self::Raised => "var(--orb-elev-raised-md)",
            Self::Floating => "var(--orb-elev-floating)",
            Self::Modal => "var(--orb-elev-modal)",
        }
    }

    pub const fn as_data(self) -> &'static str {
        match self {
            Self::Flat => "flat",
            Self::Resting => "resting",
            Self::Raised => "raised",
            Self::Floating => "floating",
            Self::Modal => "modal",
        }
    }

    pub const fn modifier_class(self) -> &'static str {
        match self {
            Self::Flat => "orbital-material--elev-flat",
            Self::Resting => "orbital-material--elev-resting",
            Self::Raised => "orbital-material--elev-raised",
            Self::Floating => "orbital-material--elev-floating",
            Self::Modal => "orbital-material--elev-modal",
        }
    }
}
