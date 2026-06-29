use super::define_css_token_enum;

define_css_token_enum! {
    /// Elevation shadow tokens from the active theme (`--orb-elev-*`).
    pub enum Shadow {
        Shadow2 => "--orb-elev-raised-xs",
        Shadow4 => "--orb-elev-raised-sm",
        Shadow8 => "--orb-elev-raised-md",
        Shadow16 => "--orb-elev-floating",
        Shadow28 => "--orb-elev-overlay",
        Shadow64 => "--orb-elev-modal",
    }
}
