use leptos::prelude::*;
use orbital_base_components::NavigationInjection;

use super::styles::navigation_styles;
use super::NavigationDivider;
use crate::{Button, ButtonAppearance, Flex, FlexAlign, FlexGap};

/// Built-in collapse toggle for [`Navigation`](crate::Navigation) footer.
#[component]
pub fn NavigationCollapseToggle() -> impl IntoView {
    let nav = NavigationInjection::expect_context();

    let (_, class_names) = navigation_styles();

    let toggle = Callback::new(move |_: leptos::ev::MouseEvent| {
        nav.collapsed.update(|v| *v = !*v);
    });

    view! {
        <div class=class_names.collapse_footer data-testid="navigation-collapse-toggle">
            <NavigationDivider />
            <Flex align=FlexAlign::Center gap=FlexGap::Small>
                <Button
                    appearance=ButtonAppearance::Subtle
                    icon=icondata::AiMenuFoldOutlined
                    on_click=toggle
                >
                    {move || if nav.collapsed.get() { ().into_any() } else { view! { "Collapse" }.into_any() }}
                </Button>
            </Flex>
        </div>
    }
}
