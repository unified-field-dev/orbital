use serde::{Deserialize, Serialize};

/// Layout projection for the same flat reply list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscussionViewMode {
    #[default]
    Tree,
    Flat,
    Compact,
}
