use leptos::prelude::*;

/// Headless SVG icon from the icondata catalog.
///
/// Renders a bare `<svg>` with class `orbital-icon` and default `fill` of `currentColor`. Prefer [`orbital_core_components::Icon`] in product UI; use `BaseIcon` when building custom wrappers that should not add layout chrome (for example `display: inline-block`).
#[component]
pub fn BaseIcon(
    /// Icon glyph from the icondata catalog.
    #[prop(into)]
    icon: icondata_core::Icon,
    /// SVG width. Defaults to `1em` so the glyph tracks surrounding text size.
    #[prop(into, default = "1em".into())]
    width: MaybeProp<String>,
    /// SVG height. Defaults to `1em`.
    #[prop(into, default = "1em".into())]
    height: MaybeProp<String>,
    /// Extra classes merged onto `orbital-icon`.
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// Inline styles merged with any icondata-provided style.
    #[prop(into, optional)]
    style: MaybeProp<String>,
    /// Optional click handler on the SVG element.
    #[prop(optional)]
    on_click: Option<leptos::callback::UnsyncCallback<leptos::ev::MouseEvent>>,
) -> impl IntoView {
    let root_class = Memo::new(move |_| {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            "orbital-icon".to_string()
        } else {
            format!("orbital-icon {extra}")
        }
    });

    let svg_style = move || {
        let mut parts = Vec::new();
        if let Some(base) = icon.style {
            parts.push(base.to_string());
        }
        if let Some(extra) = style.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    };

    let on_click_handler = move |ev: leptos::ev::MouseEvent| {
        if let Some(handler) = on_click.as_ref() {
            handler.run(ev);
        }
    };

    view! {
        <svg
            class=move || root_class.get()
            style=svg_style
            x=icon.x
            y=icon.y
            width=move || width.get()
            height=move || height.get()
            viewBox=icon.view_box
            stroke-linecap=icon.stroke_linecap
            stroke-linejoin=icon.stroke_linejoin
            stroke-width=icon.stroke_width
            stroke=icon.stroke
            fill=icon.fill.unwrap_or("currentColor")
            inner_html=icon.data
            on:click=on_click_handler
        ></svg>
    }
}
