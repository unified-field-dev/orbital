use leptos::prelude::*;

use super::{TagAppearance, TagSize};
use crate::Handler;

#[derive(Clone)]
pub struct TagGroupInjection {
    pub size: Signal<TagSize>,
    pub appearance: Signal<TagAppearance>,
    pub on_dismiss: Option<Handler<String>>,
    pub dismissible: Signal<bool>,
}

impl TagGroupInjection {
    pub fn use_context() -> Option<Self> {
        use_context()
    }
}
