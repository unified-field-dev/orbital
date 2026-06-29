use leptos::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy)]
pub struct AccordionInjection {
    pub open_items: RwSignal<HashSet<String>>,
    pub multiple: bool,
    pub collapsible: bool,
}

impl AccordionInjection {
    pub fn expect_context() -> Self {
        expect_context::<Self>()
    }
}

#[component]
pub fn BaseAccordion(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = RwSignal::new(HashSet::new()), into)] open_items: RwSignal<HashSet<String>>,
    #[prop(optional)] multiple: bool,
    #[prop(optional)] collapsible: bool,
    children: Children,
) -> impl IntoView {
    provide_context(AccordionInjection {
        open_items,
        multiple,
        collapsible,
    });

    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-accordion".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
        >
            {children()}
        </div>
    }
}
