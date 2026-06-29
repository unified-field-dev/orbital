use leptos::prelude::*;

/// Orbital icon component
///
/// Returns an SVG icon that can be dropped into any component. Uses currentColor for stroke, so it will inherit the text color.
#[component]
pub fn Orbital(
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
                <circle cx="24" cy="24" r="14"/>
                <ellipse cx="24" cy="24" rx="20" ry="8" transform="rotate(-15 24 24)"/>
                <path d="M10 22c4 3 10 5 14 5" opacity="0.9"/>
                <path d="M25 27c5 0 10-2 13-4" opacity="0.9"/>
                <rect x="18" y="16" width="8" height="4" rx="1.5"/>
                <rect x="28" y="21" width="7" height="4" rx="1.5"/>
                <rect x="16" y="26" width="9" height="5" rx="1.5"/>
                <circle cx="10" cy="18" r="1.5"/>
                <circle cx="40" cy="30" r="1.5"/>
            </g>
        </svg>
    }
}
