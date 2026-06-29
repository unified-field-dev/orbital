use orbital_motion::{PresenceMotion, SlideFrom};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum DrawerPosition {
    Top,
    Bottom,
    #[default]
    Left,
    Right,
}

impl DrawerPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum DrawerSize {
    #[default]
    Small,
    Medium,
    Large,
    Full,
}

impl DrawerSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Full => "full",
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum DrawerModalType {
    #[default]
    Modal,
    NonModal,
}

/// Slide preset for drawer panels from [`DrawerPosition`].
pub const fn drawer_presence_motion(position: DrawerPosition) -> PresenceMotion {
    PresenceMotion::slide(match position {
        DrawerPosition::Top => SlideFrom::Top,
        DrawerPosition::Bottom => SlideFrom::Bottom,
        DrawerPosition::Left => SlideFrom::Left,
        DrawerPosition::Right => SlideFrom::Right,
    })
}

/// CSS dimension for drawer panel along the opening axis.
pub fn drawer_size_css(size: DrawerSize, position: DrawerPosition) -> &'static str {
    match size {
        DrawerSize::Small => "320px",
        DrawerSize::Medium => "592px",
        DrawerSize::Large => "940px",
        DrawerSize::Full => match position {
            DrawerPosition::Top | DrawerPosition::Bottom => "100vh",
            DrawerPosition::Left | DrawerPosition::Right => "100vw",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drawer_size_css_values() {
        assert_eq!(
            drawer_size_css(DrawerSize::Small, DrawerPosition::Left),
            "320px"
        );
        assert_eq!(
            drawer_size_css(DrawerSize::Full, DrawerPosition::Top),
            "100vh"
        );
        assert_eq!(
            drawer_size_css(DrawerSize::Full, DrawerPosition::Right),
            "100vw"
        );
    }
}
