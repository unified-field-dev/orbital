//! Navigation-oriented hooks shared by Orbital shell components.
//!
//! These helpers are intentionally small and router-focused. They let UI components
//! such as side navigation or tabs reactively determine whether a
//! route should be considered active.
//!
//! ## Example
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use orbital::nav::use_route_active;
//!
//! #[component]
//! fn CounterNavLink() -> impl IntoView {
//!     let is_active = use_route_active("/counter", false);
//!
//!     view! {
//!         <a class:active=move || is_active.get() href="/counter">
//!             "Counter"
//!         </a>
//!     }
//! }
//! ```

use leptos::prelude::*;
use leptos_router::hooks::use_location;

/// Reactive hook that returns whether the given path matches the current location.
///
/// When `exact` is true, only an exact match counts. When `exact` is false (default), any path that starts with the given prefix matches.
///
/// The hook normalises the current pathname by stripping a trailing slash so that `/counter` and `/counter/` are treated identically.
///
/// This is primarily intended for navigation items that should stay highlighted while the user browses nested routes under the same section.
pub fn use_route_active(path: &str, exact: bool) -> Memo<bool> {
    let location = use_location();
    let path = path.to_string();
    Memo::new(move |_| {
        let raw = location.pathname.get();
        let normalized = if raw.len() > 1 {
            raw.trim_end_matches('/').to_string()
        } else {
            raw
        };
        if exact {
            normalized == path
        } else {
            normalized.starts_with(&path)
        }
    })
}
