use leptos::{context::Provider, prelude::*};
use orbital_motion::{OrbitalPresence, PresenceMotion};

use super::state::TreeStateInjection;
use super::types::{SubtreeInjection, TreeItemInjection};

#[slot]
pub struct BaseTreeCollapseSlot {
    #[prop(optional, into)]
    pub motion_name: MaybeProp<String>,
}

#[component]
pub fn BaseSubtree(
    #[prop(optional, into)] class: MaybeProp<String>,
    level: usize,
    #[prop(optional)] base_tree_collapse: Option<BaseTreeCollapseSlot>,
    #[prop(default = None)] subtree_children: Option<Children>,
) -> impl IntoView {
    let _ = base_tree_collapse;
    let tree_item_injection = TreeItemInjection::expect_context();
    let tree_state = TreeStateInjection::expect_context();
    let open = tree_item_injection.open;
    let subtree_ref = tree_item_injection.subtree_ref;
    let size = tree_state.size;

    let motion = Signal::from(PresenceMotion::collapse());

    let subtree_body = view! {
        <div
            class=move || {
                let mut parts = vec![
                    "orbital-tree".to_string(),
                    "orbital-subtree".to_string(),
                    format!("orbital-tree--{}", size.get().as_str()),
                ];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role="group"
            node_ref=subtree_ref
        >
            <Provider value=SubtreeInjection {
                level,
                parent_id: Some(tree_item_injection.item_id.clone()),
            }>{subtree_children.map(|children| children())}</Provider>
        </div>
    };

    view! {
        <OrbitalPresence show=open motion=motion>
            {subtree_body}
        </OrbitalPresence>
    }
}
