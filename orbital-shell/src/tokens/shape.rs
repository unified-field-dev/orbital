//! High-level shape vocabulary (rectangle, pill, circle, beak).

/// Shape for buttons, chips, popover anchors, and icon discs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Shape {
    Rectangle,
    Circle,
    Pill,
    Beak,
}

impl Shape {
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::Rectangle => "orbital-token-shape-rect",
            Self::Circle => "orbital-token-shape-circle",
            Self::Pill => "orbital-token-shape-pill",
            Self::Beak => "orbital-token-shape-beak",
        }
    }

    /// Hint for border-radius; beak uses clip-path in composed components.
    pub const fn as_token(self) -> &'static str {
        match self {
            Self::Rectangle => "0",
            Self::Circle => "50%",
            Self::Pill => "9999px",
            Self::Beak => "12px",
        }
    }
}
