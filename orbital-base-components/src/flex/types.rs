#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlexWrap {
    #[default]
    NoWrap,
    Wrap,
    WrapReverse,
}

impl FlexWrap {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoWrap => "nowrap",
            Self::Wrap => "wrap",
            Self::WrapReverse => "wrap-reverse",
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum FlexGap {
    Small,
    #[default]
    Medium,
    Large,
    Size(u16),
    WH(u16, u16),
}

impl FlexGap {
    /// CSS `gap` value for inline flex/space layouts.
    pub fn css_value(self) -> String {
        match self {
            Self::Small => "4px 8px".into(),
            Self::Medium => "8px 12px".into(),
            Self::Large => "12px 16px".into(),
            Self::Size(size) => format!("{size}px {size}px"),
            Self::WH(width, height) => format!("{width}px {height}px"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum FlexAlign {
    FlexStart,
    FlexEnd,
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl FlexAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FlexStart => "flex-start",
            Self::FlexEnd => "flex-end",
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
            Self::Baseline => "baseline",
            Self::Stretch => "stretch",
        }
    }
}

#[derive(Clone, Copy)]
pub enum FlexJustify {
    FlexStart,
    FlexEnd,
    Start,
    End,
    Center,
    SpaceAround,
    SpaceBetween,
    SpaceEvenly,
}

impl FlexJustify {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FlexStart => "flex-start",
            Self::FlexEnd => "flex-end",
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
            Self::SpaceAround => "space-around",
            Self::SpaceBetween => "space-between",
            Self::SpaceEvenly => "space-evenly",
        }
    }
}
