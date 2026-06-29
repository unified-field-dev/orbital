//! DOM element callbacks for transition lifecycle hooks.

use leptos::prelude::*;

/// Callback invoked with the transitioning [`web_sys::HtmlElement`].
pub type MotionElementCallback = Callback<web_sys::HtmlElement>;
