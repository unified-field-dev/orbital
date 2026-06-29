#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Placement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    TopStart,
    TopEnd,
    LeftStart,
    LeftEnd,
    RightStart,
    RightEnd,
    BottomStart,
    BottomEnd,
}

impl Placement {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
            Self::TopStart => "top-start",
            Self::TopEnd => "top-end",
            Self::LeftStart => "left-start",
            Self::LeftEnd => "left-end",
            Self::RightStart => "right-start",
            Self::RightEnd => "right-end",
            Self::BottomStart => "bottom-start",
            Self::BottomEnd => "bottom-end",
        }
    }

    pub fn transform_origin(&self) -> &'static str {
        match self {
            Self::Top => "bottom center",
            Self::Bottom => "top center",
            Self::Left => "center right",
            Self::Right => "center left",
            Self::TopStart => "bottom left",
            Self::TopEnd => "bottom right",
            Self::LeftStart => "top right",
            Self::LeftEnd => "bottom right",
            Self::RightStart => "top left",
            Self::RightEnd => "bottom left",
            Self::BottomStart => "top left",
            Self::BottomEnd => "top right",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_variants_have_distinct_str() {
        let variants = [
            Placement::Top,
            Placement::Bottom,
            Placement::Left,
            Placement::Right,
            Placement::TopStart,
            Placement::TopEnd,
            Placement::LeftStart,
            Placement::LeftEnd,
            Placement::RightStart,
            Placement::RightEnd,
            Placement::BottomStart,
            Placement::BottomEnd,
        ];
        let strings: Vec<_> = variants.iter().map(|p| p.as_str()).collect();
        let mut unique = strings.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(strings.len(), unique.len());
        assert_eq!(strings.len(), 12);
    }
}
