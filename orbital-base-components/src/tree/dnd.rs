use leptos::{leptos_dom::helpers::WindowListenerHandle, prelude::*};
use wasm_bindgen::JsCast;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeDropPosition {
    Before,
    After,
}

#[derive(Clone)]
pub struct TreeDragState {
    pub dragging_id: RwSignal<Option<String>>,
    pub drop_target_id: RwSignal<Option<String>>,
    pub drop_position: RwSignal<Option<TreeDropPosition>>,
}

impl TreeDragState {
    pub fn new() -> Self {
        Self {
            dragging_id: RwSignal::new(None),
            drop_target_id: RwSignal::new(None),
            drop_position: RwSignal::new(None),
        }
    }

    pub fn clear(&self) {
        self.dragging_id.set(None);
        self.drop_target_id.set(None);
        self.drop_position.set(None);
    }
}

impl Default for TreeDragState {
    fn default() -> Self {
        Self::new()
    }
}

/// Drag handle state for reorderable tree items.
pub fn use_tree_item_drag(
    item_id: Signal<String>,
    reorderable: Signal<bool>,
    drag_state: TreeDragState,
) -> (Signal<bool>, Callback<leptos::ev::PointerEvent>) {
    let dragging = Signal::derive({
        let drag_state = drag_state.clone();
        move || {
            drag_state
                .dragging_id
                .get()
                .is_some_and(|id| id == item_id.get())
        }
    });

    let on_pointerdown = Callback::new({
        let drag_state = drag_state.clone();
        move |event: leptos::ev::PointerEvent| {
            if !reorderable.get_untracked() {
                return;
            }
            let from_handle = event
                .target()
                .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
                .is_some_and(|element| {
                    element
                        .closest(".orbital-tree-item-layout__drag-handle")
                        .ok()
                        .flatten()
                        .is_some()
                });
            if !from_handle {
                return;
            }
            event.stop_propagation();
            drag_state.dragging_id.set(Some(item_id.get_untracked()));
        }
    });

    (dragging, on_pointerdown)
}

/// Move a registered tree item element before or after its drop target sibling.
pub fn apply_sibling_dom_reorder(
    dom_registry: &crate::collection::state::CollectionItemDomRegistry,
    source_id: &str,
    target_id: &str,
    order: usize,
) {
    let Some(source_el) = dom_registry.get_dom_element(source_id) else {
        return;
    };
    let Some(target_el) = dom_registry.get_dom_element(target_id) else {
        return;
    };
    let Some(parent) = source_el.parent_node() else {
        return;
    };

    if order == 0 {
        let _ = parent.insert_before(&source_el, Some(&target_el));
        return;
    }

    if let Some(next) = target_el.next_sibling() {
        let _ = parent.insert_before(&source_el, Some(&next));
    } else {
        let _ = parent.append_child(&source_el);
    }
}

/// Document-level drag listeners installed by [`install_tree_drag_listeners`].
pub struct TreeDragListenerHandle {
    handles: Vec<WindowListenerHandle>,
}

impl TreeDragListenerHandle {
    pub fn remove(self) {
        for handle in self.handles {
            handle.remove();
        }
    }
}

/// Install document-level pointer listeners for drag reorder. Call once at tree root and remove via [`TreeDragListenerHandle::remove`] when the tree unmounts.
pub fn install_tree_drag_listeners(
    drag_state: TreeDragState,
    reorderable: Signal<bool>,
    on_drop: Callback<(String, String, TreeDropPosition)>,
) -> TreeDragListenerHandle {
    let drag_state_for_move = drag_state.clone();
    let drag_state_for_up = drag_state.clone();

    let move_listener = leptos::leptos_dom::helpers::window_event_listener(
        leptos::ev::pointermove,
        move |event: leptos::ev::PointerEvent| {
            if !reorderable.get_untracked() {
                return;
            }
            if drag_state_for_move.dragging_id.get_untracked().is_none() {
                return;
            }
            let target = event
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok());
            if let Some(element) = target {
                if let Ok(Some(row)) = element.closest(".orbital-tree-item") {
                    if let Some(id) = row.get_attribute("data-item-id") {
                        drag_state_for_move.drop_target_id.set(Some(id));
                        let rect = row.get_bounding_client_rect();
                        let mid = rect.top() + rect.height() / 2.0;
                        let pos = if (event.client_y() as f64) < mid {
                            TreeDropPosition::Before
                        } else {
                            TreeDropPosition::After
                        };
                        drag_state_for_move.drop_position.set(Some(pos));
                    }
                }
            }
        },
    );

    let up_listener = leptos::leptos_dom::helpers::window_event_listener(
        leptos::ev::pointerup,
        move |_event: leptos::ev::PointerEvent| {
            if !reorderable.get_untracked() {
                return;
            }
            let Some(source_id) = drag_state_for_up.dragging_id.get_untracked() else {
                return;
            };
            if let (Some(target_id), Some(pos)) = (
                drag_state_for_up.drop_target_id.get_untracked(),
                drag_state_for_up.drop_position.get_untracked(),
            ) {
                if source_id != target_id {
                    on_drop.run((source_id, target_id, pos));
                }
            }
            drag_state_for_up.clear();
        },
    );

    TreeDragListenerHandle {
        handles: vec![move_listener, up_listener],
    }
}
