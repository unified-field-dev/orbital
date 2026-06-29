/// Visual styling for reply rows — orthogonal to [`super::DiscussionViewMode`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DiscussionAppearance {
    /// Nested cards with tree connector (default).
    #[default]
    Surface,
    /// Outlined cards, minimal elevation, no connector.
    Plain,
}

/// Resolved card tint for a single reply row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiscussionReplySurface {
    /// OP or root post — brand-tinted card.
    Op,
    /// Authored by the current viewer — accent card.
    Viewer,
    /// Default neutral surface card.
    Neutral,
}

impl DiscussionReplySurface {
    pub fn class_suffix(self) -> &'static str {
        match self {
            Self::Op => "op",
            Self::Viewer => "viewer",
            Self::Neutral => "neutral",
        }
    }
}
