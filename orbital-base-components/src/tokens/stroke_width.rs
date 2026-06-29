use super::define_css_token_enum;

define_css_token_enum! {
    /// Stroke width tokens from the active theme (`--orb-stroke-*`).
    pub enum StrokeWidth {
        Thin => "--orb-stroke-thin",
        Thick => "--orb-stroke-thick",
        Thicker => "--orb-stroke-thicker",
        Thickest => "--orb-stroke-thickest",
    }
}
