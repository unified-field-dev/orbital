#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FloatingActionsMenuTooltipSide {
    #[default]
    Left,
    Right,
}

impl FloatingActionsMenuTooltipSide {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FloatingActionsMenuDirection {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl FloatingActionsMenuDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Right => "right",
            Self::Down => "down",
            Self::Left => "left",
        }
    }

    pub fn aria_orientation(self) -> &'static str {
        match self {
            Self::Up | Self::Down => "vertical",
            Self::Right | Self::Left => "horizontal",
        }
    }
}
