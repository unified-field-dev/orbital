use crate::Handler;
use leptos::{ev, prelude::*};

#[cfg(any(feature = "hydrate", not(feature = "ssr")))]
thread_local! {
    static STACK: std::cell::RefCell<Vec<uuid::Uuid>> = Default::default();
}

/// Registers Escape-to-close when active; only the topmost trap receives the event.
#[component]
pub fn FocusTrap(
    disabled: bool,
    #[prop(into)] active: Signal<bool>,
    #[prop(into)] on_esc: Handler<ev::KeyboardEvent>,
    children: Children,
) -> impl IntoView {
    #[cfg(any(feature = "hydrate", not(feature = "ssr")))]
    if !disabled {
        use leptos::leptos_dom::helpers::WindowListenerHandle;
        let esc_handle = StoredValue::new(None::<WindowListenerHandle>);
        let id = StoredValue::new(uuid::Uuid::new_v4());

        let is_current_active =
            move || STACK.with_borrow(|stack| id.with_value(|id| stack.last() == Some(id)));
        let deactivate = move || {
            esc_handle.update_value(|handle| {
                if let Some(handle) = handle.take() {
                    handle.remove();
                }
            });
            STACK.with_borrow_mut(|stack| stack.retain(|value| id.with_value(|id| id != value)));
        };

        Effect::new(move |prev: Option<bool>| {
            let is_active = active.get();
            if is_active && !prev.unwrap_or(false) {
                let on_esc = on_esc.clone();
                let handle = window_event_listener(ev::keydown, move |e| {
                    if &e.code() == "Escape" && is_current_active() {
                        on_esc.run(e);
                    }
                });
                esc_handle.set_value(Some(handle));
                STACK.with_borrow_mut(|stack| {
                    stack.push(id.get_value());
                });
            } else {
                deactivate();
            }

            is_active
        });

        on_cleanup(move || {
            deactivate();
        });
    }

    #[cfg(all(feature = "ssr", not(feature = "hydrate")))]
    {
        let _ = disabled;
        let _ = active;
        let _ = on_esc;
    }

    children()
}
