//! Canonical typed enums for Orbital theme CSS custom properties.

mod border_radius;
mod font_family;
mod font_size;
mod font_weight;
mod icon_size;
mod line_height;
mod motion;
mod shadow;
mod stroke_width;
mod theme_color;

pub use border_radius::BorderRadius;
pub use font_family::FontFamily;
pub use font_size::FontSize;
pub use font_weight::FontWeight;
pub use icon_size::IconSize;
pub use line_height::LineHeight;
pub use motion::{MotionCurve, MotionDuration};
pub use shadow::Shadow;
pub use stroke_width::StrokeWidth;
pub use theme_color::ThemeColor;

/// Shared helpers for token enums that map to `var(--token)` CSS values.
macro_rules! define_css_token_enum {
    (
        $(#[$enum_meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident => $token:literal),* $(,)?
        }
    ) => {
        $(#[$enum_meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis enum $name {
            $($variant,)*
        }

        impl $name {
            pub const fn css_var(self) -> &'static str {
                match self {
                    $(Self::$variant => concat!("var(", $token, ")"),)*
                }
            }

            pub const fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $token,)*
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.css_var())
            }
        }
    };
}

pub(crate) use define_css_token_enum;
