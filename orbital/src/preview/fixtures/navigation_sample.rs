use leptos::prelude::*;
use orbital_core_components::{
    Navigation, NavigationBody, NavigationCategory, NavigationCategoryHeader, NavigationConfig,
    NavigationItem, NavigationSubItem, NavigationSubItemGroup,
};

pub fn sample_navigation_view() -> impl IntoView {
    let selected = RwSignal::new(Some("home".to_string()));
    let open = RwSignal::new(vec!["tools".to_string()]);

    view! {
        <Navigation config=NavigationConfig::new().with_selected_value(selected).with_open_categories(open).with_collapsible(true)>
            <NavigationBody slot>
                <NavigationItem config="home" icon=icondata::AiHomeOutlined>"Home"</NavigationItem>
                <NavigationCategory value="tools">
                    <NavigationCategoryHeader slot icon=icondata::AiToolOutlined tooltip="Tools">
                        "Tools"
                    </NavigationCategoryHeader>
                    <NavigationSubItemGroup>
                        <NavigationSubItem config="a">"Tool A"</NavigationSubItem>
                        <NavigationSubItem config="b">"Tool B"</NavigationSubItem>
                    </NavigationSubItemGroup>
                </NavigationCategory>
            </NavigationBody>
        </Navigation>
    }
}
