/// Shell body positioning relative to the viewport.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LayoutPosition {
    /// Normal document flow (default).
    #[default]
    Static,
    /// Fills the layout root (`inset: 0`) for overlay header shells.
    Absolute,
}

impl LayoutPosition {
    pub const fn as_data(self) -> &'static str {
        match self {
            Self::Static => "static",
            Self::Absolute => "absolute",
        }
    }

    pub const fn modifier_class(self) -> &'static str {
        match self {
            Self::Static => "orbital-layout--position-static",
            Self::Absolute => "orbital-layout--position-absolute",
        }
    }
}
