#[derive(Debug, Clone, PartialEq)]
pub enum FieldValidationState {
    Error(String),
    Success(String),
    Warning(String),
}

impl FieldValidationState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Error(_) => "error",
            Self::Success(_) => "success",
            Self::Warning(_) => "warning",
        }
    }

    pub fn message(&self) -> &str {
        match self {
            Self::Error(m) | Self::Success(m) | Self::Warning(m) => m,
        }
    }
}
