use ::wasm_bindgen::{prelude::Closure, JsCast};
use leptos::ev;
use web_sys::EventTarget;

pub fn add_event_listener<E>(
    target: impl Into<EventTarget>,
    event: E,
    cb: impl Fn(E::EventType) + 'static,
) -> EventListenerHandle
where
    E: ev::EventDescriptor + 'static,
    E::EventType: JsCast,
{
    add_event_listener_untyped(target, &event.name(), move |e| {
        cb(e.unchecked_into::<E::EventType>())
    })
}

pub fn add_event_listener_capture<E>(
    target: impl Into<EventTarget>,
    event: E,
    cb: impl Fn(E::EventType) + 'static,
) -> EventListenerHandle
where
    E: ev::EventDescriptor + 'static,
    E::EventType: JsCast,
{
    add_event_listener_untyped_capture(target, &event.name(), move |e| {
        cb(e.unchecked_into::<E::EventType>())
    })
}

pub struct EventListenerHandle(Box<dyn FnOnce() + Send + Sync>);

impl std::fmt::Debug for EventListenerHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EventListenerHandle").finish()
    }
}

impl EventListenerHandle {
    pub fn remove(self) {
        (self.0)();
    }
}

fn add_event_listener_untyped(
    target: impl Into<EventTarget>,
    event_name: &str,
    cb: impl Fn(web_sys::Event) + 'static,
) -> EventListenerHandle {
    fn wel(
        target: EventTarget,
        cb: Box<dyn FnMut(web_sys::Event)>,
        event_name: &str,
    ) -> EventListenerHandle {
        let cb = Closure::wrap(cb);
        _ = target.add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref());

        EventListenerHandle({
            let event_name = event_name.to_string();
            let cb = send_wrapper::SendWrapper::new(cb);
            let target = send_wrapper::SendWrapper::new(target);
            Box::new(move || {
                let _ = target
                    .remove_event_listener_with_callback(&event_name, cb.as_ref().unchecked_ref());
            })
        })
    }

    wel(target.into(), Box::new(cb), event_name)
}

fn add_event_listener_untyped_capture(
    target: impl Into<EventTarget>,
    event_name: &str,
    cb: impl Fn(web_sys::Event) + 'static,
) -> EventListenerHandle {
    fn wel(
        target: EventTarget,
        cb: Box<dyn FnMut(web_sys::Event)>,
        event_name: &str,
    ) -> EventListenerHandle {
        let cb = Closure::wrap(cb);
        _ = target.add_event_listener_with_callback_and_bool(
            event_name,
            cb.as_ref().unchecked_ref(),
            true,
        );

        EventListenerHandle({
            let event_name = event_name.to_string();
            let cb = send_wrapper::SendWrapper::new(cb);
            let target = send_wrapper::SendWrapper::new(target);
            Box::new(move || {
                let _ = target.remove_event_listener_with_callback_and_bool(
                    &event_name,
                    cb.as_ref().unchecked_ref(),
                    true,
                );
            })
        })
    }

    wel(target.into(), Box::new(cb), event_name)
}

pub fn get_scroll_parent_node(node: &web_sys::Node) -> Option<web_sys::Node> {
    use leptos::prelude::*;
    use wasm_bindgen::JsCast;

    let parent_node = node.parent_node()?;

    let node_type = parent_node.node_type();
    if node_type == web_sys::Node::ELEMENT_NODE {
        let el = parent_node.clone().dyn_into::<web_sys::Element>().unwrap();
        if let Some((overflow, overflow_x, overflow_y)) = get_overflow(&el) {
            let overflow = format!("{overflow}{overflow_x}{overflow_y}");
            if overflow.contains("auto")
                || overflow.contains("scroll")
                || overflow.contains("overlay")
            {
                return Some(parent_node);
            }
        }
    } else if node_type == web_sys::Node::DOCUMENT_NODE {
        return Some(document().into());
    }

    get_scroll_parent_node(&parent_node)
}

fn get_overflow(parent_element: &web_sys::Element) -> Option<(String, String, String)> {
    use leptos::prelude::*;

    let Ok(Some(css_style_declaration)) = window().get_computed_style(parent_element) else {
        return None;
    };
    let Ok(overflow) = css_style_declaration.get_property_value("overflow") else {
        return None;
    };
    let Ok(overflow_x) = css_style_declaration.get_property_value("overflowX") else {
        return None;
    };
    let Ok(overflow_y) = css_style_declaration.get_property_value("overflowY") else {
        return None;
    };
    Some((overflow, overflow_x, overflow_y))
}

pub fn on_click_outside<EF, CF>(els: EF, on_click: CF)
where
    EF: Fn() -> Option<Vec<web_sys::Element>> + 'static,
    CF: Fn() + 'static,
{
    #[cfg(any(feature = "hydrate", not(feature = "ssr")))]
    {
        use leptos::prelude::*;
        use std::rc::Rc;

        let els = Rc::new(els);
        let on_click = Rc::new(on_click);

        let handle_pointer = {
            let els = els.clone();
            let on_click = on_click.clone();
            window_event_listener(::leptos::ev::pointerdown, move |ev| {
                dispatch_outside_click(&ev, els.as_ref(), on_click.as_ref());
            })
        };
        let handle_mouse = {
            let els = els.clone();
            let on_click = on_click.clone();
            window_event_listener(::leptos::ev::mousedown, move |ev| {
                dispatch_outside_click(&ev, els.as_ref(), on_click.as_ref());
            })
        };
        on_cleanup(move || {
            handle_pointer.remove();
            handle_mouse.remove();
        });
    }
    #[cfg(all(feature = "ssr", not(feature = "hydrate")))]
    {
        let _ = els;
        let _ = on_click;
    }
}

#[cfg(any(feature = "hydrate", not(feature = "ssr")))]
fn dispatch_outside_click<EF, CF>(ev: &web_sys::Event, els: &EF, on_click: &CF)
where
    EF: Fn() -> Option<Vec<web_sys::Element>>,
    CF: Fn(),
{
    let Some(els) = els() else {
        return;
    };
    if !els.is_empty() {
        let composed_path = ev.composed_path();
        if els.iter().any(|el| composed_path.includes(el, 0)) {
            return;
        }
    }
    on_click();
}
