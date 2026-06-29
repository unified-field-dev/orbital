//! Height measurement hooks for [`crate::PresenceMotion::collapse`] transitions.

/// Begin enter: animate from `max-height: 0` to the measured content height.
pub fn on_enter(el: &web_sys::HtmlElement) {
    let style = el.style();
    let height = el.offset_height();
    let _ = style.set_property("max-height", "0");
    let _ = el.offset_width();
    let _ = style.set_property("transition", "");
    let _ = style.set_property("max-height", &format!("{height}px"));
    let _ = el.offset_width();
}

/// After enter: allow natural height growth.
pub fn on_after_enter(el: &web_sys::HtmlElement) {
    let _ = el.style().set_property("max-height", "");
}

/// Before leave: pin height so the close transition can shrink from it.
pub fn on_before_leave(el: &web_sys::HtmlElement) {
    let _ = el
        .style()
        .set_property("max-height", &format!("{}px", el.offset_height()));
    let _ = el.offset_width();
}

/// Begin leave: animate to `max-height: 0`.
pub fn on_leave(el: &web_sys::HtmlElement) {
    let _ = el.style().set_property("max-height", "0");
    let _ = el.offset_width();
}

/// After leave: clear inline height overrides.
pub fn on_after_leave(el: &web_sys::HtmlElement) {
    let _ = el.style().set_property("max-height", "");
}
