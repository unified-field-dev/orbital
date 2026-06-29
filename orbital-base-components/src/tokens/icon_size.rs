/// Common icon dimensions for theme-aware sizing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconSize {
    /// 16 px
    Size16,
    /// 20 px
    Size20,
    /// 24 px
    Size24,
    /// 32 px
    Size32,
    /// 1 em (inherits parent font size)
    Em,
}

impl IconSize {
    pub const fn px(self) -> Option<u16> {
        match self {
            Self::Size16 => Some(16),
            Self::Size20 => Some(20),
            Self::Size24 => Some(24),
            Self::Size32 => Some(32),
            Self::Em => None,
        }
    }

    pub fn css_value(self) -> String {
        match self.px() {
            Some(px) => format!("{px}px"),
            None => "1em".into(),
        }
    }
}

impl std::fmt::Display for IconSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.css_value())
    }
}
