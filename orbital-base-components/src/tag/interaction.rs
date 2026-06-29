use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::{context::Provider, ev, prelude::*};

use super::{
    dismiss::BaseTagDismissIcon,
    interaction_injection::InteractionTagInjection,
    surface::{tag_surface_classes, TagMediaView, TagSurfaceClassOptions},
    TagAppearance, TagGroupInjection, TagSize,
};
use crate::{icon::BaseIcon, Handler};

static NEXT_INTERACTION_TAG_ID: AtomicUsize = AtomicUsize::new(0);

#[component]
pub fn BaseInteractionTag(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] size: Option<Signal<TagSize>>,
    #[prop(optional, into)] appearance: Option<Signal<TagAppearance>>,
    children: Children,
) -> impl IntoView {
    let tag_group = TagGroupInjection::use_context();
    let size_class = size.or_else(|| tag_group.as_ref().map(|group| group.size));
    let appearance_class = appearance.or_else(|| tag_group.as_ref().map(|group| group.appearance));

    let resolved_size =
        Signal::derive(move || size_class.map(|size| size.get()).unwrap_or(TagSize::Medium));
    let resolved_appearance = Signal::derive(move || {
        appearance_class
            .map(|appearance| appearance.get())
            .unwrap_or(TagAppearance::Filled)
    });

    let id = NEXT_INTERACTION_TAG_ID.fetch_add(1, Ordering::Relaxed);
    let primary_id = StoredValue::new(format!("orbital-interaction-tag-primary-{id}"));
    let secondary_id = StoredValue::new(format!("orbital-interaction-tag-secondary-{id}"));

    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-interaction-tag".to_string()];
                parts.push(format!(
                    "orbital-interaction-tag--{}",
                    resolved_size.get().as_str()
                ));
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
        >
            <Provider value=InteractionTagInjection {
                primary_id,
                secondary_id,
                appearance: resolved_appearance,
                size: resolved_size,
            }>{children()}</Provider>
        </div>
    }
}

#[component]
pub fn BaseInteractionTagPrimary(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] has_secondary_action: Signal<bool>,
    #[prop(optional, into)] icon: MaybeProp<icondata_core::Icon>,
    children: Children,
) -> impl IntoView {
    let injection = InteractionTagInjection::use_context()
        .expect("BaseInteractionTagPrimary must be used inside BaseInteractionTag");

    let has_icon = move || icon.get().is_some();

    view! {
        <button
            id=move || injection.primary_id.get_value()
            class=move || {
                tag_surface_classes(TagSurfaceClassOptions {
                    appearance: injection.appearance.get(),
                    size: Some(injection.size.get()),
                    dismissible: false,
                    with_media: has_icon(),
                    primary_action: true,
                    with_secondary: has_secondary_action.get(),
                    secondary_action: false,
                    extra: class.get(),
                })
            }
        >
            {move || icon.get().map(|icon| view! { <TagMediaView icon=icon /> })}
            <span class="orbital-tag__primary-text">{children()}</span>
        </button>
    }
}

#[component]
pub fn BaseSecondaryActionTag(
    #[prop(into)] aria_label: String,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] icon: MaybeProp<icondata_core::Icon>,
    #[prop(optional, into)] on_click: Option<Handler<ev::MouseEvent>>,
) -> impl IntoView {
    let injection = InteractionTagInjection::use_context()
        .expect("BaseSecondaryActionTag must be used inside BaseInteractionTag");

    let primary_id = injection.primary_id;
    let secondary_id = injection.secondary_id;
    let aria_label = StoredValue::new(aria_label);

    view! {
        <button
            id=move || secondary_id.get_value()
            class=move || {
                tag_surface_classes(TagSurfaceClassOptions {
                    appearance: injection.appearance.get(),
                    size: Some(injection.size.get()),
                    dismissible: false,
                    with_media: false,
                    primary_action: false,
                    with_secondary: false,
                    secondary_action: true,
                    extra: class.get(),
                })
            }
            aria-label=move || aria_label.get_value()
            aria-labelledby=move || {
                format!(
                    "{} {}",
                    primary_id.get_value(),
                    secondary_id.get_value()
                )
            }
            on:click=move |event| {
                if let Some(on_click) = on_click.as_ref() {
                    on_click.run(event);
                }
            }
        >
            {move || {
                if let Some(icon) = icon.get() {
                    view! { <BaseIcon icon=icon width="1em" height="1em" /> }.into_any()
                } else {
                    view! { <BaseTagDismissIcon /> }.into_any()
                }
            }}
        </button>
    }
}
