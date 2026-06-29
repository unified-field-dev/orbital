/// Document-flow placement of the application header.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AppBarPosition {
    /// Normal document flow (default).
    #[default]
    Static,
    /// Pins to the top of the scroll container while content moves beneath.
    Sticky,
    /// Fixed to the viewport top; host must offset scrollable content.
    Fixed,
}

/// Vertical size tier for shell chrome on the Orbital spacing ramp.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AppBarDensity {
    /// 56px — default application shell (`Size560`).
    #[default]
    Standard,
    /// 48px — compact secondary tool rows (`Size480`).
    Compact,
    /// 96px — expanded branding or hero title band.
    Expanded,
}

impl AppBarPosition {
    pub const fn as_data(self) -> &'static str {
        match self {
            Self::Static => "static",
            Self::Sticky => "sticky",
            Self::Fixed => "fixed",
        }
    }

    pub const fn modifier_class(self) -> &'static str {
        match self {
            Self::Static => "orbital-app-bar--position-static",
            Self::Sticky => "orbital-app-bar--position-sticky",
            Self::Fixed => "orbital-app-bar--position-fixed",
        }
    }
}

impl AppBarDensity {
    pub const fn as_data(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Compact => "compact",
            Self::Expanded => "expanded",
        }
    }

    pub const fn modifier_class(self) -> &'static str {
        match self {
            Self::Standard => "orbital-app-bar--density-standard",
            Self::Compact => "orbital-app-bar--density-compact",
            Self::Expanded => "orbital-app-bar--density-expanded",
        }
    }

    /// Pixel height on the Orbital spacing ramp.
    pub const fn height_px(self) -> u16 {
        match self {
            Self::Standard => 56,
            Self::Compact => 48,
            Self::Expanded => 96,
        }
    }
}
