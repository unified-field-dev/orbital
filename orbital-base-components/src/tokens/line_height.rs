use super::define_css_token_enum;

define_css_token_enum! {
    /// Line height tokens from the active theme (`--orb-type-line-*`).
    pub enum LineHeight {
        Base200 => "--orb-type-line-sm",
        Base300 => "--orb-type-line-md",
        Base400 => "--orb-type-line-lg",
        Base500 => "--orb-type-line-xl",
    }
}
