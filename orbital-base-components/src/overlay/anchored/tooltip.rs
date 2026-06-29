use leptos::{html, prelude::*};

use super::{
    base::{merge_config, AnchoredOverlay},
    config::TooltipVariant,
};
use crate::overlay::{
    appearance::OverlayAppearance, panel_surface::OverlaySurface, placement::Placement,
};

#[component]
pub fn BaseTooltip<TTrigger>(
    #[prop(optional)] placement: Option<Placement>,
    #[prop(optional)] appearance: Option<OverlayAppearance>,
    #[prop(default = 0u64)] show_delay_ms: u64,
    #[prop(default = 100u64)] hide_delay_ms: u64,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = None)] mount: Option<NodeRef<html::Div>>,
    trigger: TypedChildren<TTrigger>,
    children: Children,
) -> impl IntoView
where
    TTrigger: AddAnyAttr + IntoView + Send + 'static,
{
    let appearance = appearance.unwrap_or(OverlayAppearance::Normal);
    let config = merge_config(
        TooltipVariant::config(),
        None,
        placement,
        Some(appearance),
        Some(show_delay_ms),
        Some(hide_delay_ms),
    );

    let shell_class = Signal::derive(move || {
        let mut classes = vec!["orbital-tooltip-shell".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                classes.push(extra);
            }
        }
        classes.join(" ")
    });

    view! {
        <AnchoredOverlay config=config trigger=trigger mount=mount>
            <OverlaySurface class=shell_class>
                {children()}
            </OverlaySurface>
        </AnchoredOverlay>
    }
}
