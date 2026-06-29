use leptos::prelude::*;

/// Photon icon component
///
/// Returns an SVG icon that can be dropped into any component. Uses currentColor for stroke, so it will inherit the text color.
#[component]
pub fn Photon(
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
                <path d="M6 32c10-6 22-10 36-12"/>
                <path d="M8 36c10-6 22-10 34-12"/>
                <path d="M10 40c10-6 22-10 32-12"/>
                <circle cx="12" cy="26" r="1.5"/>
                <circle cx="18" cy="30" r="1.5"/>
                <circle cx="26" cy="26" r="1.5"/>
                <circle cx="34" cy="22" r="1.5"/>
                <rect x="22" y="34" width="6" height="3" rx="1.2"/>
            </g>
        </svg>
    }
}
