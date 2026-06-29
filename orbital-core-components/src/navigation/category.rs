use leptos::context::Provider;
use leptos::either::Either;
use leptos::prelude::*;
use leptos::tachys::view::any_view::IntoAny;
use orbital_base_components::{NavigationCategoryInjection, NavigationInjection};

use super::styles::navigation_styles;
use crate::{Button, ButtonAppearance, Icon};

/// Category header slot for [`NavigationCategory`].
#[slot]
pub struct NavigationCategoryHeader {
    #[prop(optional, into)]
    pub class: MaybeProp<String>,
    #[prop(optional, into)]
    pub icon: MaybeProp<icondata_core::Icon>,
    /// Tooltip label when the navigation rail is collapsed.
    #[prop(optional, into)]
    pub tooltip: MaybeProp<String>,
    /// Custom icon view when `icon` is not sufficient.
    #[prop(optional)]
    pub custom_icon: Option<Callback<(), leptos::prelude::AnyView>>,
    /// Top-level preview catalog section folder styling.
    #[prop(default = false)]
    pub section_folder: bool,
    /// Nesting depth for nested group folders (adds inline-start padding).
    #[prop(default = 0)]
    pub depth: u8,
    pub children: Children,
}

/// Expandable navigation category with sub-items.
#[component]
pub fn NavigationCategory(
    #[prop(into)] value: Signal<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    navigation_category_header: NavigationCategoryHeader,
    children: Children,
) -> impl IntoView {
    let nav = NavigationInjection::expect_context();

    let open = Memo::new({
        let nav = nav.clone();
        move |_| value.with(|value| nav.is_category_open(value))
    });
    let is_selected_category = Memo::new({
        let nav = nav.clone();
        move |_| value.with(|value| nav.is_selected_category(value))
    });

    let on_toggle = move |_| {
        nav.on_request_category_toggle(value.get_untracked());
    };

    let (_, class_names) = navigation_styles();

    let NavigationCategoryHeader {
        class: header_class,
        icon: header_icon,
        tooltip,
        custom_icon,
        section_folder,
        depth: header_depth,
        children: header_children,
    } = navigation_category_header;

    let button_class = Memo::new(move |_| {
        let mut parts = vec![class_names.category_header.to_string()];
        if section_folder {
            parts.push(class_names.category_header_section_folder.to_string());
        }
        if is_selected_category.get() {
            parts.push(class_names.category_header_selected.to_string());
        }
        if let Some(extra) = header_class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    let aria_label = Memo::new(move |_| {
        tooltip
            .get()
            .unwrap_or_else(|| "Navigation category".to_string())
    });

    let chevron_class = Memo::new(move |_| {
        if open.get() {
            format!(
                "{} {}",
                class_names.category_chevron, class_names.category_chevron_open
            )
        } else {
            class_names.category_chevron.to_string()
        }
    });

    let header_style = Memo::new(move |_| {
        if section_folder || header_depth == 0 {
            None
        } else {
            let indent = 14 + header_depth as u32 * 16;
            Some(format!("padding-inline-start: {indent}px"))
        }
    });

    view! {
        <Provider value=NavigationCategoryInjection { value }>
            <div
                class=format!("{} orbital-navigation__category-root", class_names.category_root)
                data-testid="navigation-category"
            >
                <button
                    class=move || format!("{} orbital-navigation__category-header-expanded", button_class.get())
                    style=move || header_style.get()
                    type="button"
                    on:click=on_toggle
                    aria-expanded=move || if open.get() { "true" } else { "false" }
                >
                    {move || {
                        if let Some(ci) = custom_icon {
                            Either::Left(view! { <span class=class_names.category_icon>{ci.run(())}</span> }.into_any())
                        } else if let Some(icon) = header_icon.get() {
                            Either::Right(view! {
                                <span class=class_names.category_icon>
                                    <Icon icon=icon width="20px" height="20px" />
                                </span>
                            }.into_any())
                        } else {
                            Either::Right(().into_any())
                        }
                    }}
                    <span class=class_names.category_label>{header_children()}</span>
                    <span class=move || chevron_class.get() aria-hidden="true">
                        <Icon
                            icon=icondata::AiRightOutlined
                            width="20px"
                            height="20px"
                        />
                    </span>
                </button>
                <div
                    class=format!("{} orbital-navigation__category-collapsed-trigger", class_names.collapsed_trigger)
                    data-testid="navigation-category-collapsed-trigger"
                >
                    {move || {
                        if let Some(ci) = custom_icon {
                            view! {
                                <Button appearance=ButtonAppearance::Subtle attr:aria-label=aria_label>
                                    {ci.run(())}
                                </Button>
                            }.into_any()
                        } else {
                            view! {
                                <Button
                                    appearance=ButtonAppearance::Subtle
                                    icon=header_icon.get().unwrap_or(icondata::AiAppstoreOutlined)
                                    attr:aria-label=aria_label
                                />
                            }.into_any()
                        }
                    }}
                </div>
                <div
                    class=format!("{} orbital-navigation__category-subitems", class_names.category_subitems)
                    data-testid="navigation-category-subitems"
                >
                    {children()}
                </div>
            </div>
        </Provider>
    }
}
