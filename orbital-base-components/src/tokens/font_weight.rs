use super::define_css_token_enum;

define_css_token_enum! {
    /// Font weight tokens from the active theme (`--orb-type-weight-*`).
    pub enum FontWeight {
        Regular => "--orb-type-weight-regular",
        Semibold => "--orb-type-weight-semibold",
        Bold => "--orb-type-weight-bold",
    }
}
