use leptos::prelude::*;

use super::{SkeletonItemShape, SkeletonItemSize};

#[derive(Clone)]
pub struct SkeletonInjection {
    pub size: Option<Signal<SkeletonItemSize>>,
    pub shape: Option<Signal<SkeletonItemShape>>,
}

impl SkeletonInjection {
    pub fn use_context() -> Option<Self> {
        use_context()
    }
}
