use leptos::prelude::*;

use super::types::{AppBarDensity, AppBarPosition};

/// Headless application header — semantic shell chrome without theme styling.
#[component]
pub fn BaseAppBar(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = AppBarPosition::Static)] position: AppBarPosition,
    #[prop(default = AppBarDensity::Standard)] density: AppBarDensity,
    children: Children,
) -> impl IntoView {
    let root_class = Memo::new(move |_| {
        let extra = class.get().unwrap_or_default();
        let mut parts = vec![
            "orbital-app-bar".to_string(),
            position.modifier_class().to_string(),
            density.modifier_class().to_string(),
        ];
        if !extra.is_empty() {
            parts.push(extra);
        }
        parts.join(" ")
    });

    view! {
        <header
            class=root_class
            role="banner"
            data-testid="app-bar"
            data-app-bar-position=position.as_data()
            data-app-bar-density=density.as_data()
        >
            {children()}
        </header>
    }
}
