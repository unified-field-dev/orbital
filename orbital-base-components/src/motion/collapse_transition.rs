#[allow(deprecated)]
use leptos::prelude::*;

use crate::{overlay::css_transition::OrbitalCSSTransition, WebSysCallback};

/// Height collapse animation for expandable panels (tree subtrees, accordions).
///
/// **Deprecated:** prefer [`orbital_motion::OrbitalPresence`] with [`orbital_motion::PresenceMotion::collapse`].
#[deprecated(
    since = "0.1.0",
    note = "use orbital_motion::OrbitalPresence with PresenceMotion::collapse instead"
)]
#[component]
pub fn BaseCollapseTransition<T>(
    #[prop(into)] show: Signal<bool>,
    #[prop(optional, into)] motion_name: MaybeProp<String>,
    children: TypedChildren<T>,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
{
    let on_enter = WebSysCallback::new(|el: web_sys::HtmlElement| {
        let style = el.style();
        let memorized_height = el.offset_height();
        let _ = style.set_property("max-height", "0");
        let _ = el.offset_width();
        let _ = style.set_property("transition", "");
        let _ = style.set_property("max-height", &format!("{memorized_height}px"));
        let _ = el.offset_width();
    });

    let on_after_enter = WebSysCallback::new(|el: web_sys::HtmlElement| {
        let _ = el.style().set_property("max-height", "");
    });

    let on_before_leave = WebSysCallback::new(|el: web_sys::HtmlElement| {
        let _ = el
            .style()
            .set_property("max-height", &format!("{}px", el.offset_height()));
        let _ = el.offset_width();
    });

    let on_leave = WebSysCallback::new(|el: web_sys::HtmlElement| {
        let _ = el.style().set_property("max-height", "0");
        let _ = el.offset_width();
    });

    let on_after_leave = WebSysCallback::new(|el: web_sys::HtmlElement| {
        let _ = el.style().set_property("max-height", "");
    });

    view! {
        <OrbitalCSSTransition
            show=show
            name=Signal::derive(move || {
                motion_name
                    .get()
                    .unwrap_or_else(|| "orbital-motion-collapse".to_string())
            })
            on_enter=on_enter
            on_after_enter=on_after_enter
            on_before_leave=on_before_leave
            on_leave=on_leave
            on_after_leave=on_after_leave
        >
            {children.into_inner()().into_inner()}
        </OrbitalCSSTransition>
    }
}
