use leptos::prelude::*;

/// Gluon icon component
///
/// Returns an SVG icon that can be dropped into any component. Uses currentColor for stroke, so it will inherit the text color.
#[component]
pub fn Gluon(
    /// Optional class name for the SVG
    #[prop(optional)]
    class: Option<&'static str>,
    /// Optional width, defaults to 48
    #[prop(optional)]
    width: Option<&'static str>,
    /// Optional height, defaults to 48
    #[prop(optional)]
    height: Option<&'static str>,
) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 48 48"
            width=width.unwrap_or("48")
            height=height.unwrap_or("48")
            class=class
        >
            <g fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M6 22c4-8 32-8 36 0"/>
                <path d="M6 26c4 8 32 8 36 0"/>
                <circle cx="24" cy="36.5" r="2.5"/>
                <path d="M10 16l8-4 8 4v8l-8 4-8-4z"/>
                <path d="M18 12v8l8-4"/>
                <path d="M10 16l8 4 8-4"/>
                <line x1="13" y1="20" x2="17" y2="20"/>
                <line x1="13" y1="23.5" x2="18" y2="23.5"/>
                <path d="M20 10l8-4 8 4v8l-8 4-8-4z"/>
                <path d="M28 6v8l8-4"/>
                <path d="M20 10l8 4 8-4"/>
                <path d="M30 16l8-4 8 4v8l-8 4-8-4z"/>
                <path d="M38 12v8l8-4"/>
                <path d="M30 16l8 4 8-4"/>
                <line x1="33" y1="20" x2="37" y2="20"/>
                <line x1="33" y1="23.5" x2="38" y2="23.5"/>
            </g>
        </svg>
    }
}
