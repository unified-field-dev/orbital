use std::collections::HashSet;

use leptos::prelude::*;
use orbital_core_components::TableHeaderCell;

use crate::types::{DataTableColumnGroupChild, DataTableColumnGroupDef, DataTableTableState};

/// A group header cell to render in the top header row.
#[derive(Clone)]
pub struct GroupHeaderCell {
    pub label: String,
    pub colspan: u32,
    pub header_view: Option<std::sync::Arc<dyn Fn() -> leptos::prelude::AnyView + Send + Sync>>,
}

/// Count visible leaf columns under a group child tree.
fn count_visible_leaves(
    child: &DataTableColumnGroupChild,
    visible_fields: &HashSet<String>,
) -> u32 {
    match child {
        DataTableColumnGroupChild::Column { field } => {
            if visible_fields.contains(field) {
                1
            } else {
                0
            }
        }
        DataTableColumnGroupChild::Group(group) => group
            .children
            .iter()
            .map(|c| count_visible_leaves(c, visible_fields))
            .sum(),
    }
}

fn collect_group_cells(
    groups: &[DataTableColumnGroupDef],
    visible_fields: &HashSet<String>,
    out: &mut Vec<GroupHeaderCell>,
) {
    for group in groups {
        let colspan = group
            .children
            .iter()
            .map(|c| count_visible_leaves(c, visible_fields))
            .sum();
        if colspan > 0 {
            out.push(GroupHeaderCell {
                label: group.header_name.clone(),
                colspan,
                header_view: group.header_view.clone(),
            });
        }
    }
}

/// Build group header cells for the top header row.
pub fn build_group_header_row(state: DataTableTableState) -> Option<Vec<GroupHeaderCell>> {
    let groups = state.column_groups.get_value()?;
    if groups.is_empty() {
        return None;
    }
    let visible_fields: HashSet<String> = state
        .column_layout
        .get()
        .columns
        .iter()
        .map(|c| c.def.field.clone())
        .collect();
    let mut cells = Vec::new();
    collect_group_cells(&groups, &visible_fields, &mut cells);
    if cells.is_empty() {
        None
    } else {
        Some(cells)
    }
}

/// Render a group header row.
#[component]
pub fn DataTableGroupHeaderRow(cells: Vec<GroupHeaderCell>, leading_count: usize) -> impl IntoView {
    view! {
        <tr class="orbital-data-table__group-header-row">
            {(leading_count > 0).then(|| view! {
                <TableHeaderCell rowspan=Some(2u32) colspan=Some(leading_count as u32)>" "</TableHeaderCell>
            })}
            {cells
                .into_iter()
                .map(|cell| {
                    let content: AnyView = if let Some(view_fn) = cell.header_view {
                        view_fn()
                    } else {
                        view! { {cell.label.clone()} }.into_any()
                    };
                    view! {
                        <TableHeaderCell colspan=Some(cell.colspan)>
                            {content}
                        </TableHeaderCell>
                    }
                })
                .collect_view()}
        </tr>
    }
}
