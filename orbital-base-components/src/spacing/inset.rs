use super::{SpacingHorizontal, SpacingVertical};

/// Theme-aware padding or margin on all sides using Orbital spacing tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpacingInset {
    /// Same horizontal and vertical theme token (e.g. card body padding).
    Symmetric {
        horizontal: SpacingHorizontal,
        vertical: SpacingVertical,
    },
    /// Fixed pixel inset on all sides (escape hatch; prefer theme tokens).
    UniformPx(u16),
}

impl SpacingInset {
    pub fn symmetric(horizontal: SpacingHorizontal, vertical: SpacingVertical) -> Self {
        Self::Symmetric {
            horizontal,
            vertical,
        }
    }

    pub fn all_m() -> Self {
        Self::Symmetric {
            horizontal: SpacingHorizontal::M,
            vertical: SpacingVertical::M,
        }
    }

    pub fn all_l() -> Self {
        Self::Symmetric {
            horizontal: SpacingHorizontal::L,
            vertical: SpacingVertical::L,
        }
    }

    pub fn uniform_px(px: u16) -> Self {
        Self::UniformPx(px)
    }

    pub fn padding_css(self) -> String {
        self.box_css("padding")
    }

    pub fn margin_css(self) -> String {
        self.box_css("margin")
    }

    fn box_css(self, property: &str) -> String {
        match self {
            Self::Symmetric {
                horizontal,
                vertical,
            } => format!(
                "{property}: {} {};",
                vertical.css_var(),
                horizontal.css_var()
            ),
            Self::UniformPx(px) => format!("{property}: {px}px;"),
        }
    }
}
