use super::define_css_token_enum;

define_css_token_enum! {
    /// Font size tokens from the active theme (`--orb-type-size-*`).
    pub enum FontSize {
        Base100 => "--orb-type-size-2xs",
        Base200 => "--orb-type-size-xs",
        Base300 => "--orb-type-size-sm",
        Base400 => "--orb-type-size-md",
        Base500 => "--orb-type-size-lg",
        Base600 => "--orb-type-size-xl",
        Base700 => "--orb-type-size-2xl",
        Base800 => "--orb-type-size-3xl",
        Base900 => "--orb-type-size-4xl",
        Base1000 => "--orb-type-size-5xl",
    }
}
