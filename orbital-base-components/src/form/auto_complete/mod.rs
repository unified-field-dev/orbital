use crate::form::InputSize;

/// Visual size variants for auto-complete controls.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum AutoCompleteSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl AutoCompleteSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

impl From<AutoCompleteSize> for InputSize {
    fn from(value: AutoCompleteSize) -> Self {
        match value {
            AutoCompleteSize::Small => Self::Small,
            AutoCompleteSize::Medium => Self::Medium,
            AutoCompleteSize::Large => Self::Large,
        }
    }
}
