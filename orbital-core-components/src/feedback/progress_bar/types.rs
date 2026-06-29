#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum ProgressBarColor {
    #[default]
    Brand,
    Error,
    Warning,
    Success,
}

impl ProgressBarColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Brand => "brand",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Success => "success",
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum ProgressCircleColor {
    #[default]
    Brand,
    Error,
    Warning,
    Success,
}

impl ProgressCircleColor {
    pub fn stroke_color(&self) -> &'static str {
        match self {
            Self::Brand => "var(--orb-color-brand-compound-bg)",
            Self::Error => "var(--orb-color-palette-red-bg)",
            Self::Warning => "var(--orb-color-palette-orange-bg)",
            Self::Success => "var(--orb-color-palette-green-bg)",
        }
    }
}
