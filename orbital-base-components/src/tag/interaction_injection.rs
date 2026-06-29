use leptos::prelude::*;

use super::{TagAppearance, TagSize};

#[derive(Clone, Copy)]
pub struct InteractionTagInjection {
    pub primary_id: StoredValue<String>,
    pub secondary_id: StoredValue<String>,
    pub appearance: Signal<TagAppearance>,
    pub size: Signal<TagSize>,
}

impl InteractionTagInjection {
    pub fn use_context() -> Option<Self> {
        use_context()
    }
}
