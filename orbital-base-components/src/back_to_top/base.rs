use leptos::{ev, html, prelude::*};
use send_wrapper::SendWrapper;

use crate::floating_button::{BaseFloatingButton, FloatingButtonVariant};
use crate::overlay::dom_events::{add_event_listener, EventListenerHandle};
use crate::Handler;
use orbital_motion::{resolve_presence_motion, MotionSlot, OrbitalPresence, PresenceMotion};

use super::scroll_parent::{is_viewport_scroll_element, scroll_parent_element};

/// Headless back-to-top control that tracks the nearest scrollport and floats fixed on the viewport.
#[component]
pub fn BaseBackToTop(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = 40.into(), into)] right: Signal<i32>,
    #[prop(default = 40.into(), into)] bottom: Signal<i32>,
    #[prop(default = 180.into(), into)] visibility_height: Signal<i32>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] testid: MaybeProp<String>,
    #[prop(default = FloatingButtonVariant::Rounded.into(), into)] variant: Signal<
        FloatingButtonVariant,
    >,
    #[prop(optional)] motion: MotionSlot,
    children: Children,
) -> impl IntoView {
    let placeholder_ref = NodeRef::<html::Div>::new();
    let is_show = RwSignal::new(false);
    let scroll_top = RwSignal::new(0);
    let scroll_ready = RwSignal::new(false);

    Effect::new(move |_| {
        is_show.set(scroll_top.get() > visibility_height.get());
    });

    let scroll_to_top = StoredValue::new(None::<Box<dyn Fn() + Send + Sync>>);
    let scroll_handle = StoredValue::new(None::<EventListenerHandle>);

    Effect::new(move |_| {
        if scroll_ready.get_untracked() {
            return;
        }

        let Some(placeholder_el) = placeholder_ref.get() else {
            return;
        };

        let scroll_el = scroll_parent_element(&placeholder_el);
        let viewport = is_viewport_scroll_element(&scroll_el);

        {
            let scroll_el = SendWrapper::new(scroll_el.clone());
            scroll_to_top.set_value(Some(Box::new(move || {
                let options = web_sys::ScrollToOptions::new();
                options.set_top(0.0);
                options.set_behavior(web_sys::ScrollBehavior::Smooth);
                scroll_el.scroll_to_with_scroll_to_options(&options);
            })));
        }

        let read_scroll_top = {
            let scroll_el = scroll_el.clone();
            move || scroll_el.scroll_top()
        };

        scroll_top.set(read_scroll_top());

        let scroll_el_for_listener = scroll_el.clone();
        let handle = if viewport {
            let document = document();
            add_event_listener(window(), ev::scroll, move |_| {
                scroll_top.set(
                    document
                        .document_element()
                        .map(|el| el.scroll_top())
                        .unwrap_or(0),
                );
            })
        } else {
            add_event_listener(scroll_el_for_listener, ev::scroll, move |_| {
                scroll_top.set(read_scroll_top());
            })
        };
        scroll_handle.set_value(Some(handle));
        scroll_ready.set(true);
    });

    on_cleanup(move || {
        scroll_handle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.remove();
            }
        });
    });

    let on_click = Handler::with(move |_: ev::MouseEvent| {
        scroll_to_top.with_value(|scroll_to_top| {
            if let Some(scroll_to_top) = scroll_to_top {
                scroll_to_top();
            }
        });
    });

    let button_class = Memo::new(move |_| {
        let mut parts = vec![
            "orbital-floating-button".to_string(),
            format!("orbital-floating-button--{}", variant.get().as_str()),
            "orbital-floating-button--primary".to_string(),
            "orbital-floating-button--large".to_string(),
            "orbital-floating-button--fixed".to_string(),
            "orbital-back-to-top".to_string(),
        ];
        if let Some(extra) = class.get() {
            let extra = extra.trim();
            if !extra.is_empty() {
                parts.push(extra.to_string());
            }
        }
        parts.join(" ")
    });

    let button_style =
        Memo::new(move |_| format!("right: {}px; bottom: {}px", right.get(), bottom.get()));

    let motion = resolve_presence_motion(motion, PresenceMotion::fade_scale());

    view! {
        <div style="display: none" class="orbital-back-to-top-placeholder" node_ref=placeholder_ref></div>
        <OrbitalPresence show=is_show motion=motion appear=false>
            <BaseFloatingButton
                class=button_class
                style=button_style
                aria_label=aria_label
                on_click=on_click
                testid=testid
            >
                {children()}
            </BaseFloatingButton>
        </OrbitalPresence>
    }
}
