//! Reduced Motion utilities for respecting user preferences.
//!
//! Provides hooks and utilities to detect and respect `prefers-reduced-motion` media query.

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Hook to detect if the user prefers reduced motion.
///
/// Returns a `ReadSignal<bool>` that is `true` when the user prefers reduced motion.
///
/// Example:
/// ```rust,ignore
/// let prefers_reduced = use_reduced_motion();
/// view! {
///     <div class=move || if prefers_reduced.get() { "no-transition" } else { "transition" }>
///         "Content"
///     </div>
/// }
/// ```
#[component_doc]
pub fn use_reduced_motion() -> ReadSignal<bool> {
    let (prefers_reduced, set_prefers_reduced) = signal(false);

    // Silence unused warnings on non-wasm targets (SSR/native builds).
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
