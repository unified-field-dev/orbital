use leptos::prelude::*;

use super::context::LayoutSidebarOpen;
use crate::{Button, ButtonAppearance};

/// Toggles [`Layout`](crate::Layout) sidebar visibility via [`LayoutSidebarOpen`] context.
#[component]
pub fn LayoutSidebarToggle(#[prop(optional, into)] aria_label: MaybeProp<String>) -> impl IntoView {
    let sidebar_open = LayoutSidebarOpen::expect_context();
    let label = Memo::new(move |_| {
        aria_label.get().unwrap_or_else(|| {
            if sidebar_open.0.get() {
                "Collapse navigation".to_string()
            } else {
                "Expand navigation".to_string()
            }
        })
    });

    view! {
        <div data-testid="layout-sidebar-toggle">
            <Button
                appearance=ButtonAppearance::Transparent
                icon=icondata::AiMenuOutlined
                attr:aria-label=label
                on_click=Callback::new(move |_: leptos::ev::MouseEvent| {
                    sidebar_open.0.update(|open| *open = !*open);
                })
            />
        </div>
    }
}
