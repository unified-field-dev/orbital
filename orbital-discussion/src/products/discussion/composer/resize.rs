#[cfg(feature = "hydrate")]
pub fn auto_resize(el: &web_sys::HtmlTextAreaElement) {
    let style = el.style();
    let _ = style.set_property("height", "auto");
    let height = el.scroll_height();
    let _ = style.set_property("height", &format!("{height}px"));
}
