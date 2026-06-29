use leptos::prelude::*;
use orbital_data::DataRecord;

/// Resolve hierarchical path segments for a row (root → leaf).
pub type GetTreePath = Callback<(DataRecord,), Vec<String>>;

/// Resolve tree path using optional callback.
pub fn resolve_tree_path(record: &DataRecord, resolver: Option<&GetTreePath>) -> Vec<String> {
    resolver
        .map(|r| r.run((record.clone(),)))
        .unwrap_or_default()
}

/// Derived metadata for a tree row used by the grouping column.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeRowMeta {
    pub depth: usize,
    pub path: Vec<String>,
    pub path_key: String,
    pub is_branch: bool,
}

/// Build a stable path key from segments.
pub fn tree_path_key(path: &[String]) -> String {
    path.join("/")
}
