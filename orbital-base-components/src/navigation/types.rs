/// Vertical spacing density for navigation rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavigationDensity {
    #[default]
    Standard,
    Compact,
}

impl NavigationDensity {
    pub fn modifier_class(self) -> &'static str {
        match self {
            Self::Standard => "orbital-navigation--density-standard",
            Self::Compact => "orbital-navigation--density-compact",
        }
    }

    pub fn as_data(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Compact => "compact",
        }
    }
}

/// How the navigation rail is presented relative to page content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavigationMode {
    #[default]
    Inline,
    Overlay,
}

impl NavigationMode {
    pub fn modifier_class(self) -> &'static str {
        match self {
            Self::Inline => "orbital-navigation--mode-inline",
            Self::Overlay => "orbital-navigation--mode-overlay",
        }
    }
}
