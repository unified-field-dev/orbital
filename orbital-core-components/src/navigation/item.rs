use leptos::either::Either;
use leptos::prelude::*;
use orbital_base_components::{
    BaseNavigationItem, BaseNavigationItemConfig, NavigationCategoryInjection, NavigationInjection,
    NavigationSubItemGroupInjection,
};

use super::item_config::NavigationItemConfig;
use super::styles::navigation_styles;
use crate::Icon;

fn item_selected(
    nav: NavigationInjection,
    value: Signal<String>,
    in_sub_group: bool,
) -> Memo<bool> {
    Memo::new(move |_| {
        let selected = value.with(|value| nav.collection.selection.is_selected(value));

        if selected && in_sub_group {
            if let Some(category) = NavigationCategoryInjection::use_context() {
                let cat_value = category.value.get_untracked();
                if !nav.is_selected_category(&cat_value) {
                    nav.selected_category_value.set(Some(cat_value));
                }
            }
        } else if selected {
            if let Some(category) = NavigationCategoryInjection::use_context() {
                let cat_value = category.value.get_untracked();
                if nav
                    .selected_category_value
                    .with_untracked(|s| s.as_deref() != Some(cat_value.as_str()))
                {
                    nav.selected_category_value.set(Some(cat_value));
                }
            }
        }

        selected
    })
}

#[component]
fn NavigationItemInner(
    class: MaybeProp<String>,
    icon: MaybeProp<icondata_core::Icon>,
    config: NavigationItemConfig,
    sub_item: bool,
    children: Children,
) -> impl IntoView {
    let NavigationItemConfig {
        value,
        href,
        target,
        badge,
        disabled,
        depth,
        on_click,
    } = config;

    let nav = NavigationInjection::expect_context();
    let in_sub_group = NavigationSubItemGroupInjection::use_context().is_some();
    let (_, class_names) = navigation_styles();

    let on_activate = Callback::new({
        let nav = nav.clone();
        move |ev: leptos::ev::MouseEvent| {
            if disabled.get_untracked() {
                return;
            }
            if let Some(handler) = on_click {
                handler.run(ev);
                return;
            }
            let val = value.get_untracked();
            nav.collection.selection.select_item(
                val,
                false,
                Some(true),
                None,
                Some(&nav.collection.registry),
            );
            nav.sync_selected_from_collection();
            nav.selected_category_value
                .set(NavigationCategoryInjection::use_context().map(|c| c.value.get_untracked()));
        }
    });

    let selected = item_selected(nav.clone(), value, in_sub_group);

    let item_class = Memo::new(move |_| {
        let mut parts = vec![class_names.item.to_string()];
        if sub_item {
            parts.push(class_names.item_sub.to_string());
        }
        if selected.get() {
            parts.push(class_names.item_selected.to_string());
        }
        if disabled.get() {
            parts.push(class_names.item_disabled.to_string());
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    let item_style = Memo::new(move |_| {
        if sub_item && depth > 0 {
            let indent = 14 + depth as u32 * 16;
            Some(format!("padding-inline-start: {indent}px"))
        } else {
            None
        }
    });

    let host = move || {
        view! {
            {move || icon.get().map(|icon| view! { <Icon icon=icon width="20px" height="20px" /> })}
            {children()}
            {badge.clone().map(|b| view! { <span class=class_names.item_badge>{b}</span> })}
        }
    };

    view! {
        <BaseNavigationItem config=BaseNavigationItemConfig::new(
            Signal::derive(move || value.get()),
            on_activate,
        )
            .with_label(Signal::derive(move || value.get()))
            .with_depth(depth as usize)
            .with_disabled(disabled)
            .with_selected(Signal::derive(move || selected.get()))>
            {match href {
                Some(href) => Either::Left(view! {
                    <a
                        class=move || item_class.get()
                        style=move || item_style.get()
                        href=move || href.get()
                        target=move || target.get()
                    >
                        {host()}
                    </a>
                }),
                None => Either::Right(view! {
                    <button
                        class=move || item_class.get()
                        style=move || item_style.get()
                        type="button"
                        disabled=move || disabled.get()
                    >
                        {host()}
                    </button>
                }),
            }}
        </BaseNavigationItem>
    }
}

/// Top-level navigation link or button.
#[component]
pub fn NavigationItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] icon: MaybeProp<icondata_core::Icon>,
    #[prop(into)] config: NavigationItemConfig,
    children: Children,
) -> impl IntoView {
    view! {
        <NavigationItemInner class icon config sub_item=false>
            {children()}
        </NavigationItemInner>
    }
}

/// Indented navigation link under a category.
#[component]
pub fn NavigationSubItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] icon: MaybeProp<icondata_core::Icon>,
    #[prop(into)] config: NavigationItemConfig,
    children: Children,
) -> impl IntoView {
    let mut config = config;
    if config.depth == 0 {
        config.depth = 1;
    }

    view! {
        <NavigationItemInner class icon config sub_item=true>
            {children()}
        </NavigationItemInner>
    }
}
