use leptos::prelude::*;

use super::{TagAppearance, TagSize};
use crate::icon::BaseIcon;

pub(crate) struct TagSurfaceClassOptions {
    pub appearance: TagAppearance,
    pub size: Option<TagSize>,
    pub dismissible: bool,
    pub with_media: bool,
    pub primary_action: bool,
    pub with_secondary: bool,
    pub secondary_action: bool,
    pub extra: Option<String>,
}

pub(crate) fn tag_surface_classes(options: TagSurfaceClassOptions) -> String {
    let TagSurfaceClassOptions {
        appearance,
        size,
        dismissible,
        with_media,
        primary_action,
        with_secondary,
        secondary_action,
        extra,
    } = options;

    let mut parts = if secondary_action {
        vec!["orbital-tag__dismiss".to_string()]
    } else {
        vec!["orbital-tag".to_string()]
    };

    parts.push(format!("orbital-tag--{}", appearance.as_str()));

    if dismissible {
        parts.push("orbital-tag--dismissible".to_string());
    }
    if with_media {
        parts.push("orbital-tag--with-media".to_string());
    }
    if primary_action {
        parts.push("orbital-tag--primary-action".to_string());
    }
    if with_secondary {
        parts.push("orbital-tag--with-secondary".to_string());
    }
    if secondary_action {
        parts.push("orbital-tag--secondary-action".to_string());
    }
    if let Some(size) = size {
        parts.push(format!("orbital-tag--{}", size.as_str()));
    }
    if let Some(extra) = extra {
        if !extra.is_empty() {
            parts.push(extra);
        }
    }

    parts.join(" ")
}

#[component]
pub fn TagMediaView(#[prop(into)] icon: icondata_core::Icon) -> impl IntoView {
    view! {
        <span class="orbital-tag__media" aria-hidden="true">
            <BaseIcon icon=icon width="1em" height="1em" />
        </span>
    }
}
