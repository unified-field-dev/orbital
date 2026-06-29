use leptos::prelude::*;

#[slot]
pub struct TreeItemLayout {
    #[prop(optional, into)]
    pub class: MaybeProp<String>,
    #[prop(optional, into)]
    pub style: MaybeProp<String>,
    #[prop(optional)]
    pub tree_item_icon_before: Option<TreeItemIconBefore>,
    #[prop(optional)]
    pub tree_item_icon_after: Option<TreeItemIconAfter>,
    #[prop(optional)]
    pub tree_item_aside: Option<TreeItemAside>,
    #[prop(optional)]
    pub tree_item_checkbox: Option<TreeItemCheckbox>,
    #[prop(optional)]
    pub tree_item_label_input: Option<TreeItemLabelInput>,
    #[prop(optional)]
    pub tree_item_collapse: Option<TreeItemCollapse>,
    pub children: Children,
}

#[slot]
pub struct TreeItemIconBefore {
    pub children: Children,
}

#[slot]
pub struct TreeItemIconAfter {
    pub children: Children,
}

#[slot]
pub struct TreeItemAside {
    pub children: Children,
}

#[slot]
pub struct TreeItemCheckbox {
    pub children: Children,
}

#[slot]
pub struct TreeItemLabelInput {
    pub children: Children,
}

#[slot]
pub struct TreeItemCollapse {
    #[prop(optional, into)]
    pub motion_name: MaybeProp<String>,
}
