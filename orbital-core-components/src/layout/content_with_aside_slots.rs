use leptos::prelude::*;

/// Primary column — grows to fill available width beside the aside.
#[slot]
pub struct Content {
    pub children: Children,
}

/// Secondary column — minimally fits content, sticky on the right in wide layouts.
#[slot]
pub struct Aside {
    pub children: Children,
}
