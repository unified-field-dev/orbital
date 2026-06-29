use icondata_core::Icon as IconData;
use leptos::prelude::*;
use orbital_base_components::BaseFloatingActionsMenuItem;

use crate::Icon;

#[component]
pub fn FloatingActionsMenuItem(
    /// Tooltip and accessible name for the action.
    #[prop(into)]
    tooltip: String,
    /// Icon shown on the mini action button.
    #[prop(into)]
    icon: IconData,
    /// Called when the action is activated.
    #[prop(optional)]
    on_click: Option<Callback<()>>,
    /// Optional test hook for E2E.
    #[prop(optional, into)]
    testid: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <BaseFloatingActionsMenuItem tooltip=tooltip nostrip:on_click=on_click testid=testid>
            <Icon icon=icon />
        </BaseFloatingActionsMenuItem>
    }
}
