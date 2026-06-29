use leptos::{html, prelude::*};

use super::{
    base::{merge_config, AnchoredOverlay},
    config::PopoverVariant,
};
use crate::overlay::{
    appearance::OverlayAppearance,
    panel::OverlayPanelSize,
    panel_surface::OverlaySurface,
    placement::Placement,
    trigger::{OverlayTrigger, OverlayTriggerType},
    visibility::OverlayLifecycle,
};

#[derive(Clone, Default)]
pub struct PopoverEvents {
    pub on_open: Option<Callback<()>>,
    pub on_close: Option<Callback<()>>,
}

#[component]
pub fn BasePopover<TTrigger>(
    #[prop(optional)] trigger_type: Option<OverlayTriggerType>,
    #[prop(optional)] placement: Option<Placement>,
    #[prop(optional)] appearance: Option<OverlayAppearance>,
    #[prop(optional)] size: Option<OverlayPanelSize>,
    #[prop(optional)] events: Option<PopoverEvents>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = None)] mount: Option<NodeRef<html::Div>>,
    overlay_trigger: OverlayTrigger<TTrigger>,
    children: Children,
) -> impl IntoView
where
    TTrigger: AddAnyAttr + IntoView + Send + 'static,
{
    let appearance = appearance.unwrap_or(OverlayAppearance::Default);
    let mut config = merge_config(
        PopoverVariant::config(),
        trigger_type,
        placement,
        Some(appearance),
        None,
        None,
    );
    let _panel_size = size.or(config.panel_size);
    if let Some(size) = size {
        config.panel_size = Some(size);
    }

    let lifecycle = events
        .map(|events| OverlayLifecycle {
            on_open: events.on_open,
            on_close: events.on_close,
        })
        .unwrap_or_default();

    let shell_class = Signal::derive(move || {
        let mut classes = vec!["orbital-popover-shell".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                classes.push(extra);
            }
        }
        classes.join(" ")
    });

    view! {
        <AnchoredOverlay
            config=config
            overlay_trigger=overlay_trigger
            lifecycle=lifecycle
            mount=mount
        >
            <OverlaySurface class=shell_class>
                {children()}
            </OverlaySurface>
        </AnchoredOverlay>
    }
}
