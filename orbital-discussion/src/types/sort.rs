use serde::{Deserialize, Serialize};

/// Sort order applied when projecting visible replies.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscussionSort {
    /// Preserve server / input order (default for tree mode).
    #[default]
    OldestFirst,
    /// Reverse sibling order at each tree level.
    NewestFirst,
}
