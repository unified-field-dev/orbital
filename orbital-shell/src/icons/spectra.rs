use leptos::prelude::*;

/// Spectra icon component
///
/// Returns an SVG icon that can be dropped into any component. Uses currentColor for stroke, so it will inherit the text color.
#[component]
pub fn Spectra(
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
                <line x1="6" y1="24" x2="17" y2="24"/>
                <path d="M18 14l10 10-10 10z"/>
                <line x1="28" y1="24" x2="42" y2="18"/>
                <line x1="28" y1="24" x2="42" y2="24"/>
                <line x1="28" y1="24" x2="42" y2="30"/>
                <circle cx="42" cy="18" r="1.5"/>
                <circle cx="42" cy="24" r="1.5"/>
                <circle cx="42" cy="30" r="1.5"/>
                <line x1="8" y1="34" x2="14" y2="34"/>
                <line x1="16" y1="34" x2="18" y2="34"/>
            </g>
        </svg>
    }
}
