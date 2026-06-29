#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OverlayAppearance {
    #[default]
    Default,
    Brand,
    Inverted,
    Normal,
}

impl OverlayAppearance {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Brand => "brand",
            Self::Inverted => "inverted",
            Self::Normal => "normal",
        }
    }

    pub fn modifier_class(&self) -> Option<&'static str> {
        match self {
            Self::Default => None,
            other => Some(other.as_str()),
        }
    }
}
