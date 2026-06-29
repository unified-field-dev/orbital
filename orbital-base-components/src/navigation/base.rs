use leptos::prelude::*;

use super::types::{NavigationDensity, NavigationMode};

/// Headless navigation root — structure and data attributes only.
#[component]
pub fn BaseNavigation(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] data_testid: MaybeProp<String>,
    #[prop(default = NavigationDensity::Standard)] density: NavigationDensity,
    #[prop(default = NavigationMode::Inline)] mode: NavigationMode,
    #[prop(default = true.into())] open: Signal<bool>,
    #[prop(default = false.into())] collapsed: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let root_class = Memo::new(move |_| {
        let extra = class.get().unwrap_or_default();
        let mut parts = vec![
            "orbital-navigation".to_string(),
            density.modifier_class().to_string(),
            mode.modifier_class().to_string(),
        ];
        if collapsed.get() {
            parts.push("orbital-navigation--collapsed".to_string());
        }
        if !open.get() {
            parts.push("orbital-navigation--closed".to_string());
        }
        if !extra.is_empty() {
            parts.push(extra);
        }
        parts.join(" ")
    });

    if let Some(test_id) = data_testid.get() {
        view! {
            <nav
                class=root_class
                data-testid=test_id
                data-navigation-density=density.as_data()
                data-navigation-open=move || open.get().to_string()
                data-navigation-collapsed=move || collapsed.get().to_string()
            >
                {children()}
            </nav>
        }
        .into_any()
    } else {
        view! {
            <nav
                class=root_class
                data-navigation-density=density.as_data()
                data-navigation-open=move || open.get().to_string()
                data-navigation-collapsed=move || collapsed.get().to_string()
            >
                {children()}
            </nav>
        }
        .into_any()
    }
}
