#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OverlayPanelSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl OverlayPanelSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}
