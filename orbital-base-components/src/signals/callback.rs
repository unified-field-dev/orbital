use leptos::prelude::Callback;

/// Leptos callback that receives a [`web_sys::HtmlElement`].
pub type WebSysCallback = Callback<web_sys::HtmlElement>;
