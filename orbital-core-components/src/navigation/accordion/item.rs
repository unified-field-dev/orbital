use leptos::prelude::*;
use orbital_base_components::BaseAccordionItem;
use orbital_motion::MotionSlot;
use orbital_style::inject_style;

use super::styles::accordion_styles;

static ACCORDION_STYLES: std::sync::Once = std::sync::Once::new();

fn ensure_accordion_styles() {
    ACCORDION_STYLES.call_once(|| inject_style("orbital-accordion", accordion_styles()));
}

pub use orbital_base_components::AccordionHeader;

/// One expandable panel inside [`Accordion`](crate::Accordion).
///
/// Set `value` to a stable id included in the parent's `open_items` set. Pass optional
/// `motion: MotionSlot` to override the default expand/collapse animation for this panel.
#[component]
pub fn AccordionItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into)] value: Signal<String>,
    accordion_header: AccordionHeader,
    #[prop(optional)] motion: MotionSlot,
    children: Children,
) -> impl IntoView {
    ensure_accordion_styles();
    view! {
        <BaseAccordionItem class=class value=value accordion_header=accordion_header motion=motion>
            {children()}
        </BaseAccordionItem>
    }
}
