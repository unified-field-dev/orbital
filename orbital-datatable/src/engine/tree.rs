use std::collections::{HashMap, HashSet};

use orbital_data::DataRecord;

use crate::types::{
    resolve_row_id, resolve_tree_path, tree_path_key, DataTableRowModel, GetRowId, GetTreePath,
    TreeRowMeta,
};

/// Index of branch path keys in the tree.
#[derive(Clone, Debug, Default)]
pub struct TreeIndex {
    pub branch_keys: HashSet<String>,
    pub row_meta: HashMap<String, TreeRowMeta>,
}

/// Build tree metadata for all rows.
pub fn build_tree_index(
    rows: &[DataTableRowModel],
    get_tree_path: &GetTreePath,
    get_row_id: Option<&GetRowId>,
) -> TreeIndex {
    let mut branch_keys = HashSet::new();
    let mut row_paths: Vec<(String, Vec<String>)> = Vec::new();

    for row in rows {
        if !row.is_data_row() {
            continue;
        }
        let path = resolve_tree_path(&row.record, Some(get_tree_path));
        if path.is_empty() {
            continue;
        }
        row_paths.push((row.resolved_id(get_row_id), path));
    }

    for (_, path) in &row_paths {
        for i in 0..path.len().saturating_sub(1) {
            branch_keys.insert(tree_path_key(&path[..=i]));
        }
    }

    let mut row_meta = HashMap::new();
    for (row_id, path) in row_paths {
        let depth = path.len().saturating_sub(1);
        let path_key = tree_path_key(&path);
        let is_branch = branch_keys.contains(&path_key);
        row_meta.insert(
            row_id,
            TreeRowMeta {
                depth,
                path,
                path_key,
                is_branch,
            },
        );
    }

    TreeIndex {
        branch_keys,
        row_meta,
    }
}

fn is_tree_row_visible(meta: &TreeRowMeta, expanded: &HashSet<String>) -> bool {
    if meta.depth == 0 {
        return true;
    }
    for i in 0..meta.path.len().saturating_sub(1) {
        let prefix_key = tree_path_key(&meta.path[..=i]);
        if !expanded.contains(&prefix_key) {
            return false;
        }
    }
    true
}

/// Filter sorted rows to those visible given tree expansion state.
pub fn visible_tree_rows(
    sorted_rows: Vec<DataTableRowModel>,
    get_tree_path: &GetTreePath,
    get_row_id: Option<&GetRowId>,
    expanded_tree_nodes: &HashSet<String>,
) -> Vec<DataTableRowModel> {
    let index = build_tree_index(&sorted_rows, get_tree_path, get_row_id);
    sorted_rows
        .into_iter()
        .filter(|row| {
            if !row.is_data_row() {
                return true;
            }
            let row_id = row.resolved_id(get_row_id);
            let Some(meta) = index.row_meta.get(&row_id) else {
                return true;
            };
            is_tree_row_visible(meta, expanded_tree_nodes)
        })
        .collect()
}

/// Lookup tree metadata for a row id.
pub fn tree_meta_for_row(
    record: &DataRecord,
    get_tree_path: &GetTreePath,
    all_rows: &[DataTableRowModel],
    get_row_id: Option<&GetRowId>,
) -> Option<TreeRowMeta> {
    let index = build_tree_index(all_rows, get_tree_path, get_row_id);
    let row_id = resolve_row_id(record, get_row_id);
    index.row_meta.get(&row_id).cloned()
}

/// Parent branch path key for toggling expansion (empty path segments excluded).
pub fn parent_branch_key(path: &[String]) -> Option<String> {
    if path.len() <= 1 {
        return None;
    }
    Some(tree_path_key(&path[..path.len() - 1]))
}

/// Branch path key for a row's own path when it is a branch node.
pub fn branch_key_for_path(path: &[String], branch_keys: &HashSet<String>) -> Option<String> {
    let key = tree_path_key(path);
    if branch_keys.contains(&key) {
        Some(key)
    } else {
        parent_branch_key(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::Owner;
    use std::collections::HashMap;

    fn with_owner<F: FnOnce()>(f: F) {
        Owner::new().with(f);
    }

    fn sample_rows() -> Vec<DataTableRowModel> {
        vec![
            DataTableRowModel::from_text_cells(
                "1",
                HashMap::from([("name".into(), "Branch A".into())]),
            ),
            DataTableRowModel::from_text_cells(
                "2",
                HashMap::from([("name".into(), "Child A1".into())]),
            ),
        ]
    }

    fn path_fn() -> GetTreePath {
        GetTreePath::new(|(record,)| match record.id.as_str() {
            "1" => vec!["A".into()],
            "2" => vec!["A".into(), "Child".into()],
            _ => vec![record.id.clone()],
        })
    }

    #[test]
    fn hides_nested_rows_when_parent_collapsed() {
        with_owner(|| {
            let rows = sample_rows();
            let path_fn = path_fn();
            let expanded = HashSet::new();
            let visible = visible_tree_rows(rows, &path_fn, None, &expanded);
            assert_eq!(visible.len(), 1);
            assert_eq!(visible[0].id(), "1");
        });
    }

    #[test]
    fn shows_nested_rows_when_parent_expanded() {
        with_owner(|| {
            let rows = sample_rows();
            let path_fn = path_fn();
            let mut expanded = HashSet::new();
            expanded.insert("A".into());
            let visible = visible_tree_rows(rows, &path_fn, None, &expanded);
            assert_eq!(visible.len(), 2);
        });
    }
}
