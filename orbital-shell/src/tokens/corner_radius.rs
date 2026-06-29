//! Border radius scale for Orbital shapes.

/// Corner radius for marketing surfaces, cards, and controls.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CornerRadius {
    /// No rounding (rare; dividers, full-bleed strips).
    None,
    /// 4px — buttons, dense chips, small badges.
    Small,
    /// 8px — primary buttons, dense cards.
    Medium,
    /// 12px — marketing surfaces, popovers, callouts.
    Large,
    /// 16px — hero shells, large marketing cards.
    XLarge,
    /// 16px — rounded-square floating action controls.
    Floating,
    /// 50% — icon discs, avatars.
    Circle,
}

impl CornerRadius {
    /// Stable CSS class for composition (pairs with global marketing token styles).
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::None => "orbital-token-radius-none",
            Self::Small => "orbital-token-radius-small",
            Self::Medium => "orbital-token-radius-medium",
            Self::Large => "orbital-token-radius-large",
            Self::XLarge => "orbital-token-radius-xlarge",
            Self::Floating => "orbital-token-radius-floating",
            Self::Circle => "orbital-token-radius-circle",
        }
    }

    /// Theme token when available; otherwise a concrete `border-radius` value.
    pub const fn as_token(self) -> &'static str {
        match self {
            Self::None => "0",
            Self::Small => "var(--orb-radius-sm)",
            Self::Medium => "var(--orb-radius-md)",
            Self::Large => "var(--orb-radius-lg)",
            Self::XLarge => "var(--orb-radius-xl)",
            Self::Floating => "var(--orb-radius-floating)",
            Self::Circle => "50%",
        }
    }
}
