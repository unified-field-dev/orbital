use leptos::prelude::*;
use orbital_base_components::{
    base_tree_item_layout, BaseTreeCollapseSlot, BaseTreeItem, BaseTreeItemAside,
    BaseTreeItemCheckbox, BaseTreeItemIconAfter, BaseTreeItemIconBefore, BaseTreeItemRow,
};

use super::slots::TreeItemLayout;
use super::types::TreeItemConfig;

/// One node in a [`Tree`](super::tree::Tree).
#[component]
pub fn TreeItem(
    config: TreeItemConfig,
    tree_item_layout: TreeItemLayout,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let TreeItemConfig {
        item_type,
        value,
        label,
        parent_id,
        order,
    } = config;

    let TreeItemLayout {
        class: layout_class,
        style,
        tree_item_icon_before,
        tree_item_icon_after,
        tree_item_aside,
        tree_item_checkbox,
        tree_item_label_input: _,
        tree_item_collapse,
        children: layout_children,
    } = tree_item_layout;

    let collapse_slot = tree_item_collapse.map(|collapse| BaseTreeCollapseSlot {
        motion_name: collapse.motion_name,
    });

    if let Some(collapse) = collapse_slot {
        view! {
            <BaseTreeItem
                class=class
                item_type=item_type
                value=value
                label=label.unwrap_or_default()
                parent_id=parent_id.clone().unwrap_or_default()
                order=order
                base_tree_collapse=collapse
                children=children
            >
                <BaseTreeItemRow slot>
                    <TreeItemRowLayout
                        layout_class=layout_class
                        style=style
                        tree_item_icon_before=tree_item_icon_before
                        tree_item_icon_after=tree_item_icon_after
                        tree_item_aside=tree_item_aside
                        tree_item_checkbox=tree_item_checkbox
                        layout_children=layout_children
                    />
                </BaseTreeItemRow>
            </BaseTreeItem>
        }
    } else {
        view! {
            <BaseTreeItem
                class=class
                item_type=item_type
                value=value
                label=label.unwrap_or_default()
                parent_id=parent_id.clone().unwrap_or_default()
                order=order
                children=children
            >
                <BaseTreeItemRow slot>
                    <TreeItemRowLayout
                        layout_class=layout_class
                        style=style
                        tree_item_icon_before=tree_item_icon_before
                        tree_item_icon_after=tree_item_icon_after
                        tree_item_aside=tree_item_aside
                        tree_item_checkbox=tree_item_checkbox
                        layout_children=layout_children
                    />
                </BaseTreeItemRow>
            </BaseTreeItem>
        }
    }
}

#[component]
fn TreeItemRowLayout(
    layout_class: MaybeProp<String>,
    style: MaybeProp<String>,
    tree_item_icon_before: Option<super::slots::TreeItemIconBefore>,
    tree_item_icon_after: Option<super::slots::TreeItemIconAfter>,
    tree_item_aside: Option<super::slots::TreeItemAside>,
    tree_item_checkbox: Option<super::slots::TreeItemCheckbox>,
    layout_children: leptos::children::Children,
) -> impl IntoView {
    base_tree_item_layout(
        layout_class,
        style,
        tree_item_icon_before.map(|slot| BaseTreeItemIconBefore {
            children: slot.children,
        }),
        tree_item_icon_after.map(|slot| BaseTreeItemIconAfter {
            children: slot.children,
        }),
        tree_item_aside.map(|slot| BaseTreeItemAside {
            children: slot.children,
        }),
        tree_item_checkbox.map(|slot| BaseTreeItemCheckbox {
            children: slot.children,
        }),
        layout_children,
    )
}
