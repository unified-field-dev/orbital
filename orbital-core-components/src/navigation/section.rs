use leptos::either::Either;
use leptos::prelude::*;
use orbital_base_components::NavigationInjection;
use orbital_motion::{OrbitalPresence, PresenceMotion};

use super::styles::navigation_styles;
use crate::Icon;

/// App branding row at the top of the navigation body.
#[component]
pub fn NavigationAppItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] icon: MaybeProp<icondata_core::Icon>,
    #[prop(optional, into)] href: Option<Signal<String>>,
    children: Children,
) -> impl IntoView {
    let (_, class_names) = navigation_styles();
    let item_class = Memo::new(move |_| {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            class_names.item_app.to_string()
        } else {
            format!("{} {extra}", class_names.item_app)
        }
    });

    let content = move || {
        view! {
            {move || icon.get().map(|icon| view! { <Icon icon=icon width="20px" height="20px" /> })}
            {children()}
        }
    };

    if let Some(href) = href {
        Either::Left(view! {
            <a class=item_class href=move || href.get()>{content()}</a>
        })
    } else {
        Either::Right(view! {
            <div class=item_class>{content()}</div>
        })
    }
}

/// Non-interactive section label between navigation groups.
///
/// `depth` controls inline-start indent only; typography is not tied to nesting level.
#[component]
pub fn NavigationSectionHeader(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = 1)] depth: u8,
    #[prop(default = false)] band: bool,
    children: Children,
) -> impl IntoView {
    let (_, class_names) = navigation_styles();
    let header_class = Memo::new(move |_| {
        let mut parts = vec![class_names.section_header.to_string()];
        if band {
            parts.push(class_names.section_header_band.to_string());
        }
        let extra = class.get().unwrap_or_default();
        if !extra.is_empty() {
            parts.push(extra);
        }
        parts.join(" ")
    });
    let indent_px = move || 8 + (depth.saturating_sub(1) as u32) * 16;

    view! {
        <div
            class=header_class
            role="presentation"
            style=move || format!("padding-inline-start: {}px", indent_px())
        >
            {children()}
        </div>
    }
}

/// Strong divider between navigation groups.
#[component]
pub fn NavigationDivider(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    let (_, class_names) = navigation_styles();
    let divider_class = Memo::new(move |_| {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            class_names.divider.to_string()
        } else {
            format!("{} {extra}", class_names.divider)
        }
    });

    view! {
        <hr class=divider_class role="separator" />
    }
}

/// Wraps sub-items under a [`NavigationCategory`].
#[component]
pub fn NavigationSubItemGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    use leptos::context::Provider;
    use orbital_base_components::NavigationSubItemGroupInjection;

    let nav = NavigationInjection::expect_context();
    let category = orbital_base_components::NavigationCategoryInjection::use_context();
    let (_, class_names) = navigation_styles();

    let open = Memo::new(move |_| {
        if nav.collapsed.get() && category.is_some() {
            return true;
        }
        category.is_none_or(|c| c.value.with(|value| nav.is_category_open(value)))
    });

    let group_class = Memo::new(move |_| {
        let mut parts = vec![class_names.sub_item_group.to_string()];
        let extra = class.get().unwrap_or_default();
        if !extra.is_empty() {
            parts.push(extra);
        }
        parts.join(" ")
    });
    let open_signal = Signal::derive(move || open.get());
    let motion = Signal::from(PresenceMotion::collapse());

    view! {
        <Provider value=NavigationSubItemGroupInjection>
            <OrbitalPresence show=open_signal motion=motion>
                <div class=group_class>{children()}</div>
            </OrbitalPresence>
        </Provider>
    }
}
