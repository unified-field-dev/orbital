use super::option_walker::{use_option_walker, OptionWalker};
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use std::{cell::RefCell, sync::Arc};
use web_sys::{HtmlElement, Node};

/// Styling attribute applied to the active option element.
const ACTIVEDESCENDANT_ATTRIBUTE: &str = "data-activedescendant";
/// Focus-visible styling attribute for keyboard navigation.
const ACTIVEDESCENDANT_FOCUSVISIBLE_ATTRIBUTE: &str = "data-activedescendant-focusvisible";

pub fn use_active_descendant<MF>(
    match_option: MF,
) -> (Arc<dyn Fn(Node) + Send + Sync>, ActiveDescendantController)
where
    MF: Fn(HtmlElement) -> bool + Send + Sync + 'static,
{
    let (set_listbox, option_walker) = use_option_walker(match_option);
    let set_listbox = Arc::new(move |node| {
        set_listbox(&node);
    });
    let controller = ActiveDescendantController {
        option_walker,
        active: Arc::new(SendWrapper::new(Default::default())),
        active_id: RwSignal::new(None),
    };

    (set_listbox, controller)
}

#[derive(Clone)]
pub struct ActiveDescendantController {
    option_walker: OptionWalker,
    active: Arc<SendWrapper<RefCell<Option<HtmlElement>>>>,
    active_id: RwSignal<Option<String>>,
}

impl ActiveDescendantController {
    fn blur_active_descendant(&self) {
        let mut active = self.active.borrow_mut();
        let Some(active_el) = active.as_mut() else {
            self.active_id.set(None);
            return;
        };
        let _ = active_el.remove_attribute(ACTIVEDESCENDANT_ATTRIBUTE);
        let _ = active_el.remove_attribute(ACTIVEDESCENDANT_FOCUSVISIBLE_ATTRIBUTE);

        *active = None;
        self.active_id.set(None);
    }

    fn focus_active_descendant(&self, next_active: HtmlElement) {
        self.blur_active_descendant();
        next_active.scroll_into_view();
        let _ = next_active.set_attribute(ACTIVEDESCENDANT_ATTRIBUTE, "");
        let _ = next_active.set_attribute(ACTIVEDESCENDANT_FOCUSVISIBLE_ATTRIBUTE, "");
        let id = next_active.id();
        *self.active.borrow_mut() = Some(next_active);
        self.active_id.set(Some(id));
    }
}

impl ActiveDescendantController {
    pub fn first(&self) {
        if let Some(first) = self.option_walker.first() {
            self.focus_active_descendant(first);
        }
    }

    pub fn last(&self) {
        if let Some(last) = self.option_walker.last() {
            self.focus_active_descendant(last);
        }
    }

    pub fn next(&self) {
        if let Some(next) = self.option_walker.next() {
            self.focus_active_descendant(next);
        }
    }

    pub fn prev(&self) {
        if let Some(prev) = self.option_walker.prev() {
            self.focus_active_descendant(prev);
        }
    }

    pub fn blur(&self) {
        self.blur_active_descendant();
    }

    pub fn active(&self) -> Option<HtmlElement> {
        let active = self.active.borrow();
        active.as_ref().map(|active| active.clone())
    }

    pub fn active_id_signal(&self) -> Signal<Option<String>> {
        self.active_id.into()
    }

    pub fn find(&self, predicate: impl Fn(String) -> bool) -> Option<String> {
        let target = self.option_walker.find(predicate)?;
        let id = target.id();
        self.focus_active_descendant(target);
        Some(id)
    }
}
