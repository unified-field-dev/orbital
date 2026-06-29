#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ComboboxSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ComboboxSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}
