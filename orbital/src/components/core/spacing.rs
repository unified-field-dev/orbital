//! Orbital design language global spacing ramp.
//!
//! The spacing ramp constrains gap, padding, and margin values to the set
//! defined by the Orbital layout chapter on the Introduction page (`/#layout`). The base
//! unit is 4 px; values 2, 6, and 10 exist to align icons to the 4 px grid.
//!
//! Use `SpacingSize` anywhere a pixel gap or spacing value is needed (e.g.
//! `AutoGrid`'s `gap` prop) so that the UI stays within the design language.
//!
//!
use crate::primitives::*;

/// A spacing value from the Orbital design language global spacing ramp.
///
/// Each variant maps to a fixed pixel value. The token names follow the Orbital convention (`Size{N}`) where the number is an abstract scale value, **not** the pixel count.
///
/// | Variant    | Pixels |
/// |------------|--------|
/// | `None`     | 0      |
/// | `Size20`   | 2      |
/// | `Size40`   | 4      |
/// | `Size60`   | 6      |
/// | `Size80`   | 8      |
/// | `Size100`  | 10     |
/// | `Size120`  | 12     |
/// | `Size160`  | 16     |
/// | `Size200`  | 20     |
/// | `Size240`  | 24     |
/// | `Size280`  | 28     |
/// | `Size320`  | 32     |
/// | `Size360`  | 36     |
/// | `Size400`  | 40     |
/// | `Size480`  | 48     |
/// | `Size520`  | 52     |
/// | `Size560`  | 56     |
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpacingSize {
    /// 0 px
    None,
    /// 2 px
    Size20,
    /// 4 px
    Size40,
    /// 6 px
    Size60,
    /// 8 px
    Size80,
    /// 10 px
    Size100,
    /// 12 px
    Size120,
    /// 16 px (default)
    #[default]
    Size160,
    /// 20 px
    Size200,
    /// 24 px
    Size240,
    /// 28 px
    Size280,
    /// 32 px
    Size320,
    /// 36 px
    Size360,
    /// 40 px
    Size400,
    /// 48 px
    Size480,
    /// 52 px
    Size520,
    /// 56 px
    Size560,
}

impl SpacingSize {
    /// Returns the pixel value for this spacing size.
    pub const fn px(self) -> u16 {
        match self {
            Self::None => 0,
            Self::Size20 => 2,
            Self::Size40 => 4,
            Self::Size60 => 6,
            Self::Size80 => 8,
            Self::Size100 => 10,
            Self::Size120 => 12,
            Self::Size160 => 16,
            Self::Size200 => 20,
            Self::Size240 => 24,
            Self::Size280 => 28,
            Self::Size320 => 32,
            Self::Size360 => 36,
            Self::Size400 => 40,
            Self::Size480 => 48,
            Self::Size520 => 52,
            Self::Size560 => 56,
        }
    }
}

impl std::fmt::Display for SpacingSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}px", self.px())
    }
}

// ---------------------------------------------------------------------------
// Conversions to FlexGap
// ---------------------------------------------------------------------------

impl SpacingSize {
    /// Convert to a [`FlexGap`] for use with the `Flex` component's `gap` prop.
    ///
    /// ```rust,ignore
    /// use orbital::components::SpacingSize;
    /// use Flex;
    ///
    /// view! {
    ///     <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
    ///         // children get 24 px gaps
    ///     </Flex>
    /// }
    /// ```
    pub fn flex_gap(self) -> FlexGap {
        FlexGap::Size(self.px())
    }

    /// Fixed-pixel padding/margin inset (all sides) for [`Flex`] `padding` / `margin` props.
    pub fn inset(self) -> orbital_base_components::SpacingInset {
        orbital_base_components::SpacingInset::uniform_px(self.px())
    }
}

impl From<SpacingSize> for FlexGap {
    fn from(s: SpacingSize) -> Self {
        s.flex_gap()
    }
}
