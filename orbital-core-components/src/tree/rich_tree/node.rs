use leptos::prelude::*;
use std::collections::HashSet;

#[derive(Clone)]
pub(crate) struct RichTreeNode {
    pub id: String,
    pub label: String,
    pub children: Vec<RichTreeNode>,
    pub disabled: bool,
    pub editable: bool,
    pub lazy: bool,
}

#[derive(Clone, Copy)]
pub(crate) struct RichTreeRuntimeCtx {
    pub virtualize: bool,
    pub loading_ids: RwSignal<HashSet<String>>,
}

impl RichTreeRuntimeCtx {
    pub fn expect_context() -> Self {
        expect_context()
    }
}
