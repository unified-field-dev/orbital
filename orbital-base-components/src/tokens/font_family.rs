use super::define_css_token_enum;

define_css_token_enum! {
    /// Font family tokens from the active theme (`--orb-type-family-*`).
    pub enum FontFamily {
        Base => "--orb-type-family-sans",
        Monospace => "--orb-type-family-mono",
        Numeric => "--orb-type-family-numeric",
        Display => "--orb-type-family-display",
    }
}
