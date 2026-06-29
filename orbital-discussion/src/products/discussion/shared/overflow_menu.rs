use leptos::prelude::*;
use orbital_core_components::{Button, ButtonAppearance, Menu, MenuItem, MenuTrigger};

use crate::DiscussionMenuItem;

/// Generic overflow menu for discussion row actions.
#[component]
pub fn DiscussionOverflowMenu(
    items: Vec<DiscussionMenuItem>,
    on_select: Callback<String, ()>,
    action_testid_prefix: String,
    aria_label: String,
) -> impl IntoView {
    if items.is_empty() {
        return view! { <span></span> }.into_any();
    }

    view! {
        <Menu on_select=move |action: String| on_select.run(action)>
            <MenuTrigger slot>
                <Button
                    appearance=ButtonAppearance::Subtle
                    icon=icondata::AiMoreOutlined
                    attr:aria-label=aria_label
                />
            </MenuTrigger>
            <For
                each=move || items.clone()
                key=|item| item.id.clone()
                children=move |item| {
                    let action_id = item.id.clone();
                    let testid = format!("{action_testid_prefix}-{}", item.id);
                    view! {
                        <MenuItem
                            value=action_id
                            disabled=item.disabled.unwrap_or(false)
                            attr:data-testid=testid
                        >
                            {item.label.clone()}
                        </MenuItem>
                    }
                }
            />
        </Menu>
    }
    .into_any()
}
