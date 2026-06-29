use leptos::prelude::*;

/// Leading slot for icons, currency symbols, or units inside [`Input`](super::input::Input).
#[slot]
pub struct InputPrefix {
    #[prop(default = true)]
    pub if_: bool,
    pub children: Children,
}

/// Trailing slot for units, actions, or affix text inside [`Input`](super::input::Input).
#[slot]
pub struct InputSuffix {
    #[prop(default = true)]
    pub if_: bool,
    pub children: Children,
}
