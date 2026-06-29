use leptos::prelude::*;

use crate::overlay::FloatingPanel;

/// Popover panel body rendered inside the anchored overlay portal.
#[component]
pub fn PopoverPanel(#[prop(into)] class: Signal<String>, children: Children) -> impl IntoView {
    view! {
        <FloatingPanel class=class body_class="orbital-popover-body">
            {children()}
        </FloatingPanel>
    }
}
