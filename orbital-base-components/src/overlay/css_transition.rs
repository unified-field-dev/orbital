use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos::tachys::html::style::style;

use crate::WebSysCallback;

/// **Deprecated:** prefer [`orbital_motion::OrbitalPresence`] with [`orbital_motion::PresenceMotion`] presets.
#[deprecated(since = "0.1.0", note = "use orbital_motion::OrbitalPresence instead")]
/// Legacy wrapper around [`leptos_transition_group::CSSTransition`].
#[component]
pub fn OrbitalCSSTransition<T>(
    #[prop(into)] show: Signal<bool>,
    #[prop(into)] name: Signal<String>,
    #[prop(optional)] appear: bool,
    #[prop(optional)] on_before_enter: Option<WebSysCallback>,
    #[prop(optional)] on_enter: Option<WebSysCallback>,
    #[prop(optional)] on_after_enter: Option<WebSysCallback>,
    #[prop(optional)] on_before_leave: Option<WebSysCallback>,
    #[prop(optional)] on_leave: Option<WebSysCallback>,
    #[prop(optional)] on_after_leave: Option<WebSysCallback>,
    children: TypedChildren<T>,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
{
    #[cfg(not(feature = "ssr"))]
    {
        use leptos_transition_group::CSSTransition;

        let before = on_before_enter;
        let on_before_enter_cb = move |el: web_sys::HtmlElement| {
            if let Some(cb) = before {
                cb.run(el);
            }
        };

        let enter = on_enter;
        let on_enter_cb = move |el: web_sys::HtmlElement| {
            if let Some(cb) = enter {
                cb.run(el);
            }
        };

        let after_enter = on_after_enter;
        let on_after_enter_cb = move |el: web_sys::HtmlElement| {
            if let Some(cb) = after_enter {
                cb.run(el);
            }
        };

        let before_leave = on_before_leave;
        let on_before_leave_cb = move |el: web_sys::HtmlElement| {
            if let Some(cb) = before_leave {
                cb.run(el);
            }
        };

        let leave = on_leave;
        let on_leave_cb = move |el: web_sys::HtmlElement| {
            if let Some(cb) = leave {
                cb.run(el);
            }
        };

        let after = on_after_leave;
        let on_after_leave_cb = move |el: web_sys::HtmlElement| {
            if let Some(cb) = after {
                cb.run(el);
            }
        };

        view! {
            <CSSTransition
                show=show
                name=name
                appear=appear
                on_before_enter=on_before_enter_cb
                on_enter=on_enter_cb
                on_after_enter=on_after_enter_cb
                on_before_leave=on_before_leave_cb
                on_leave=on_leave_cb
                on_after_leave=on_after_leave_cb
            >
                {children.into_inner()().into_inner()}
            </CSSTransition>
        }
    }

    #[cfg(feature = "ssr")]
    {
        let _ = (
            name,
            appear,
            on_before_enter,
            on_enter,
            on_after_enter,
            on_before_leave,
            on_leave,
            on_after_leave,
        );
        children.into_inner()().into_inner().add_any_attr(style((
            "display",
            if show.get_untracked() { "" } else { "none" },
        )))
    }
}
