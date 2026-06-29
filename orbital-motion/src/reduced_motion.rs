//! Reduced-motion utilities for respecting user preferences.

use leptos::prelude::*;

/// Detect whether the user prefers reduced motion (`prefers-reduced-motion: reduce`).
pub fn use_reduced_motion() -> ReadSignal<bool> {
    let (prefers_reduced, set_prefers_reduced) = signal(false);

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = set_prefers_reduced;
    }

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::web_sys::window;
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::prelude::*;

        Effect::new(move |_| {
            if let Some(win) = window() {
                if let Ok(Some(media_query_list)) =
                    win.match_media("(prefers-reduced-motion: reduce)")
                {
                    let matches: bool = media_query_list.matches();
                    set_prefers_reduced.set(matches);

                    let closure = Closure::wrap(Box::new({
                        let set_prefers_reduced = set_prefers_reduced.clone();
                        let media_query_list = media_query_list.clone();
                        move |_| {
                            set_prefers_reduced.set(media_query_list.matches());
                        }
                    })
                        as Box<dyn FnMut(web_sys::Event)>);

                    let _ = media_query_list
                        .add_listener_with_opt_callback(Some(closure.as_ref().unchecked_ref()));
                    closure.forget();
                }
            }
        });
    }

    prefers_reduced
}
