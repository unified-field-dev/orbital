use leptos::{html, prelude::*};
use web_sys::DomRect;

#[derive(Clone, Copy)]
pub struct AnchorInjection {
    anchor_ref: NodeRef<html::Div>,
    bar_ref: NodeRef<html::Div>,
    element_ids: RwSignal<Vec<String>>,
    pub active_id: RwSignal<Option<String>>,
}

impl AnchorInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn new(
        anchor_ref: NodeRef<html::Div>,
        bar_ref: NodeRef<html::Div>,
        element_ids: RwSignal<Vec<String>>,
        active_id: RwSignal<Option<String>>,
    ) -> Self {
        Self {
            anchor_ref,
            bar_ref,
            element_ids,
            active_id,
        }
    }

    pub fn scroll_into_view(&self, id: &str) {
        let Some(link_el) = document().get_element_by_id(id) else {
            return;
        };
        link_el.scroll_into_view();
    }

    pub fn append_id(&self, id: String) {
        self.element_ids.update(|ids| ids.push(id));
    }

    pub fn remove_id(&self, id: &str) {
        self.element_ids.update(|ids| {
            if let Some(index) = ids.iter().position(|item_id| item_id == id) {
                ids.remove(index);
            }
        });
    }

    pub fn update_background_position(&self, title_rect: DomRect) {
        use wasm_bindgen::JsCast;

        let Some(anchor_el) = self.anchor_ref.get_untracked() else {
            return;
        };
        let Some(bar_el) = self.bar_ref.get_untracked() else {
            return;
        };
        let Some(bar_el) = bar_el.dyn_ref::<web_sys::HtmlElement>() else {
            return;
        };
        let anchor_rect = anchor_el.get_bounding_client_rect();
        let offset_top = title_rect.top() - anchor_rect.top();
        let _ = bar_el
            .style()
            .set_property("top", &format!("{offset_top}px"));
        let _ = bar_el
            .style()
            .set_property("height", &format!("{}px", title_rect.height()));
    }
}
