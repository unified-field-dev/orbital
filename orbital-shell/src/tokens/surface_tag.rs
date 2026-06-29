//! HTML tag choice for marketing surface roots.

/// Which element wraps a `MarketingSurface` for semantics / a11y.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SurfaceTag {
    #[default]
    Section,
    Article,
    Div,
}

impl SurfaceTag {
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::Section => "orbital-token-surface-section",
            Self::Article => "orbital-token-surface-article",
            Self::Div => "orbital-token-surface-div",
        }
    }
}
