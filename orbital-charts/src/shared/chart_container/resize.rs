//! Container size measurement via ResizeObserver.

use leptos::prelude::*;

/// Observe an element's content box and publish `(width, height)`.
pub fn use_container_size(
    node_ref: NodeRef<leptos::html::Div>,
    fallback: (f64, f64),
) -> ReadSignal<(f64, f64)> {
    let size = RwSignal::new(fallback);

    #[cfg(feature = "hydrate")]
    {
        use send_wrapper::SendWrapper;
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        Effect::new(move |_| {
            let Some(element) = node_ref.get() else {
                return;
            };
            let el: web_sys::Element = element.into();
            if el.client_width() > 0 && el.client_height() > 0 {
                size.set((el.client_width() as f64, el.client_height() as f64));
            }

            let size_signal = size;
            let callback = Closure::wrap(Box::new(move |entries: js_sys::Array| {
                let entry = entries.get(0);
                if let Ok(obs_entry) = entry.dyn_into::<web_sys::ResizeObserverEntry>() {
                    let rect = obs_entry.content_rect();
                    let w = rect.width();
                    let h = rect.height();
                    if w > 0.0 && h > 0.0 {
                        size_signal.set((w, h));
                    }
                }
            }) as Box<dyn FnMut(js_sys::Array)>);

            let observer = web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref())
                .expect("ResizeObserver should be available in hydrate builds");
            observer.observe(&el);
            callback.forget();

            let observer = SendWrapper::new(observer);
            on_cleanup(move || {
                observer.disconnect();
            });
        });
    }

    #[cfg(not(feature = "hydrate"))]
    {
        let _ = &node_ref;
    }

    size.read_only()
}
