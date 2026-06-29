use leptos::prelude::*;

/// Phonon icon component
///
/// Returns an SVG icon that can be dropped into any component. Uses currentColor for stroke, so it will inherit the text color.
#[component]
pub fn Phonon(
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
                <circle cx="16" cy="26" r="3"/>
                <circle cx="24" cy="24" r="3"/>
                <circle cx="32" cy="26" r="3"/>
                <circle cx="24" cy="30" r="3"/>
                <path d="M10 36c1.5-4 10.5-4 12 0"/>
                <path d="M18 36c1.5-4 10.5-4 12 0"/>
                <path d="M26 36c1.5-4 10.5-4 12 0"/>
                <path d="M8 16c0-3 3-5 6-5h4c3 0 6 2 6 5s-3 5-6 5h-2l-3 3v-3h-1c-3 0-6-2-6-5z"/>
                <path d="M30 12h6c3 0 6 2 6 5s-3 5-6 5h-2l-3 3v-3h-1c-3 0-6-2-6-5s3-5 6-5z"/>
                <circle cx="34" cy="17" r="0.8"/>
                <circle cx="37" cy="17" r="0.8"/>
            </g>
        </svg>
    }
}
