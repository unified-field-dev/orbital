use icondata_core::Icon;
use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};
#[cfg(feature = "hydrate")]
use orbital_base_components::NavigationInjection;
use orbital_core_components::{NavigationItem, NavigationItemConfig};

use crate::nav::use_route_active;

fn preserve_env_query_suffix(search: &str) -> String {
    let trimmed = search.trim_start_matches('?');
    if trimmed.is_empty() {
        return String::new();
    }
    for pair in trimmed.split('&') {
        let mut it = pair.splitn(2, '=');
        let key = it.next().unwrap_or("");
        if key.eq_ignore_ascii_case("env") {
            let value = it.next().unwrap_or("");
            return format!("?env={value}");
        }
    }
    String::new()
}

/// Route-aware wrapper around [`NavigationItem`] for Leptos Router apps.
///
/// Syncs selection with the active route and navigates client-side on click.
#[component]
pub fn NavigationLink(
    #[prop(into)] path: String,
    #[prop(into)] value: String,
    icon: Icon,
    #[prop(default = false)] exact: bool,
    #[prop(optional, into)] test_id: Option<String>,
    children: Children,
) -> impl IntoView {
    #[cfg(feature = "hydrate")]
    let nav = NavigationInjection::expect_context();
    let active = use_route_active(&path, exact);

    #[cfg(feature = "hydrate")]
    {
        let value_for_effect = value.clone();
        Effect::new(move |_| {
            if active.get() {
                nav.selected_value().set(Some(value_for_effect.clone()));
            }
        });
    }

    #[cfg(not(feature = "hydrate"))]
    let _ = active;

    let location = use_location();
    let navigate = use_navigate();
    let navigate_store = StoredValue::new(navigate);
    let nav_path = path.clone();
    let href = Signal::derive({
        let nav_path = nav_path.clone();
        move || nav_path.clone()
    });
    let item_value = Signal::derive(move || value.clone());
    let on_click = Callback::new(move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        let from_query = location
            .query
            .get_untracked()
            .get_str("env")
            .filter(|v| !v.is_empty())
            .map(|v| format!("?env={v}"));
        let suffix = from_query
            .unwrap_or_else(|| preserve_env_query_suffix(&location.search.get_untracked()));
        let dest = format!("{}{}", nav_path, suffix);
        navigate_store.with_value(|navigate| {
            navigate(&dest, Default::default());
        });
    });

    view! {
        <div data-testid=test_id.unwrap_or_default()>
            <NavigationItem config=NavigationItemConfig::from_signal(item_value).with_href(href).with_on_click(on_click) icon=icon>
                {children()}
            </NavigationItem>
        </div>
    }
}

/// Backward-compatible alias for [`NavigationLink`].
pub use NavigationLink as NavLink;
