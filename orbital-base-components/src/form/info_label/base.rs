use leptos::prelude::*;

use crate::form::{BaseLabel, LabelSize, LabelWeight};
use crate::icon::BaseIcon;
use crate::overlay::{
    BasePopover, OverlayAppearance, OverlayPanelSize, OverlayTrigger, OverlayTriggerType, Placement,
};

#[slot]
pub struct InfoLabelInfo {
    pub children: Children,
}

/// Headless info label composed from `BaseLabel` + `BasePopover` + `BaseIcon`.
///
/// Panel content is rendered without a Material surface; prefer the core `InfoLabel` control, which composes the orbital `Popover` so the panel gets `FloatingPanel` styling.
#[component]
pub fn BaseInfoLabel(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] size: Signal<LabelSize>,
    #[prop(optional, into)] weight: Signal<LabelWeight>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] html_for: MaybeProp<String>,
    #[prop(into)] icon: icondata_core::Icon,
    #[prop(optional)] trigger_type: OverlayTriggerType,
    #[prop(optional)] position: Placement,
    #[prop(optional)] appearance: OverlayAppearance,
    #[prop(optional)] panel_size: OverlayPanelSize,
    info_label_info: InfoLabelInfo,
    children: Children,
) -> impl IntoView {
    let size_class = Signal::derive(move || format!("orbital-label--{}", size.get().as_str()));
    let weight_class = Signal::derive(move || format!("orbital-label--{}", weight.get().as_str()));
    let wrapper_class = Signal::derive(move || {
        let mut parts = vec!["orbital-info-label".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    view! {
        <span class=wrapper_class>
            <BaseLabel
                class="orbital-info-label__label"
                size=size_class
                weight=weight_class
                label_size=size
                label_weight=weight
                required=required
                disabled=disabled
                attr_for=html_for
            >
                {children()}
            </BaseLabel>
            <BasePopover
                class="orbital-info-label__popover"
                trigger_type=trigger_type
                placement=position
                appearance=appearance
                size=panel_size
            >
                <OverlayTrigger slot>
                    <button
                        type="button"
                        class="orbital-info-label__info-button"
                        aria-label="More information"
                    >
                        <BaseIcon icon=icon />
                    </button>
                </OverlayTrigger>
                <div class="orbital-info-label__content">
                    {(info_label_info.children)()}
                </div>
            </BasePopover>
        </span>
    }
}
