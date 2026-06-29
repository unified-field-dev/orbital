use orbital_base_components::{FlexAlign, FlexGap, FlexJustify};

/// Layout configuration for [`Stack`](crate::Stack).
#[derive(Clone, Copy, Default)]
pub struct StackConfig {
    /// Gap between stacked items.
    pub gap: FlexGap,
    /// Row direction when true; default is vertical stack.
    pub horizontal: bool,
    /// Cross-axis alignment forwarded to the inner flex container.
    pub align: Option<FlexAlign>,
    /// Main-axis distribution forwarded to the inner flex container.
    pub justify: Option<FlexJustify>,
}

impl StackConfig {
    pub fn vertical(gap: FlexGap) -> Self {
        Self {
            gap,
            horizontal: false,
            align: None,
            justify: None,
        }
    }

    pub fn horizontal(gap: FlexGap) -> Self {
        Self {
            gap,
            horizontal: true,
            align: None,
            justify: None,
        }
    }
}
