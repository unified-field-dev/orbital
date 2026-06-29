#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FeedbackIntent {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl FeedbackIntent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}
