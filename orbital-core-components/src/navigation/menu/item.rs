use icondata_core::Icon;
use leptos::prelude::*;
use orbital_base_components::BaseMenuItem;

/// Command row inside a [`Menu`](super::menu::Menu).
#[component]
pub fn MenuItem<V>(
    /// Optional CSS class on the menu item.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Optional leading icon.
    #[prop(optional, into)]
    icon: MaybeProp<Icon>,
    /// Value passed to the menu `on_select` handler when chosen.
    value: V,
    /// When true, the item cannot be activated.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Item label text or inline elements.
    children: Children,
) -> impl IntoView
where
    V: Clone + Send + Sync + 'static,
{
    view! {
        <BaseMenuItem class=class icon=icon value=value disabled=disabled>
            {children()}
        </BaseMenuItem>
    }
}
