use serde::{Deserialize, Serialize};

/// Focus state for tree navigation (drill-in UI deferred to Phase 2).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscussionFocus {
    #[default]
    Root,
    Branch {
        anchor_id: String,
        breadcrumb: Vec<String>,
    },
}
