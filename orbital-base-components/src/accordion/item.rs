use leptos::prelude::*;

use orbital_motion::{resolve_presence_motion, MotionSlot, OrbitalPresence, PresenceMotion};

use super::base::AccordionInjection;

#[slot]
pub struct AccordionHeader {
    pub children: Children,
}

#[component]
pub fn BaseAccordionItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into)] value: Signal<String>,
    accordion_header: AccordionHeader,
    #[prop(optional)] motion: MotionSlot,
    children: Children,
) -> impl IntoView {
    let AccordionInjection {
        open_items,
        multiple,
        collapsible,
    } = AccordionInjection::expect_context();

    let is_show_panel = Memo::new(move |_| {
        let items = open_items.get();
        let v = value.get();
        items.contains(&v)
    });

    let on_click = move |_| {
        let is_open = is_show_panel.get_untracked();
        let v = value.get_untracked();
        open_items.update(|items| {
            if is_open {
                if collapsible || multiple {
                    items.remove(&v);
                }
            } else {
                if !multiple {
                    items.clear();
                }
                items.insert(v);
            }
        });
    };

    let motion = resolve_presence_motion(motion, PresenceMotion::collapse());

    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-accordion-item".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
        >
            <div class="orbital-accordion-header">
                <button
                    class="orbital-accordion-header__button"
                    aria-expanded=move || is_show_panel.get().to_string()
                    type="button"
                    on:click=on_click
                >
                    <span class="orbital-accordion-header__expand-icon" aria-hidden="true">
                        <svg
                            fill="currentColor"
                            aria-hidden="true"
                            width="1em"
                            height="1em"
                            viewBox="0 0 20 20"
                            style=move || {
                                if is_show_panel.get() {
                                    "transform: rotate(90deg)"
                                } else {
                                    "transform: rotate(0deg)"
                                }
                            }
                        >
                            <path
                                d="M7.65 4.15c.2-.2.5-.2.7 0l5.49 5.46c.21.22.21.57 0 .78l-5.49 5.46a.5.5 0 0 1-.7-.7L12.8 10 7.65 4.85a.5.5 0 0 1 0-.7Z"
                                fill="currentColor"
                            ></path>
                        </svg>
                    </span>
                    {(accordion_header.children)()}
                </button>
            </div>
            <OrbitalPresence show=Signal::derive(move || is_show_panel.get()) motion=motion>
                <div class="orbital-accordion-panel">
                    {children()}
                </div>
            </OrbitalPresence>
        </div>
    }
}
