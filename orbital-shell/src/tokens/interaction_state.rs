//! Interaction states for hover / focus / pressed styling.

/// Resting and interactive states for token-driven class composition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InteractionState {
    Rest,
    Hover,
    Pressed,
    Selected,
    Focus,
    Disabled,
}

impl InteractionState {
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::Rest => "orbital-token-state-rest",
            Self::Hover => "orbital-token-state-hover",
            Self::Pressed => "orbital-token-state-pressed",
            Self::Selected => "orbital-token-state-selected",
            Self::Focus => "orbital-token-state-focus",
            Self::Disabled => "orbital-token-state-disabled",
        }
    }

    /// Focus ring color token (stroke focus).
    pub const fn focus_ring_token() -> &'static str {
        "var(--orb-color-border-focus)"
    }
}
