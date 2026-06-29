use super::define_css_token_enum;

define_css_token_enum! {
    /// Border radius tokens from the active theme (`--orb-radius-*`).
    pub enum BorderRadius {
        None => "--orb-radius-none",
        Small => "--orb-radius-sm",
        Medium => "--orb-radius-md",
        Large => "--orb-radius-lg",
        XLarge => "--orb-radius-xl",
        Floating => "--orb-radius-floating",
        Circular => "--orb-radius-circular",
    }
}
