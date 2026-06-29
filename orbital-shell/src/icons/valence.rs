use leptos::prelude::*;

/// Valence icon component
///
/// Returns an SVG icon that can be dropped into any component. Uses currentColor for stroke, so it will inherit the text color.
#[component]
pub fn Valence(
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
                <line x1="14" y1="26" x2="22" y2="20"/>
                <line x1="22" y1="20" x2="30" y2="24"/>
                <line x1="30" y1="24" x2="38" y2="16"/>
                <line x1="30" y1="24" x2="38" y2="28"/>
                <line x1="14" y1="26" x2="12" y2="34"/>
                <circle cx="14" cy="26" r="4"/>
                <circle cx="22" cy="20" r="3.5"/>
                <circle cx="30" cy="24" r="6"/>
                <circle cx="38" cy="16" r="4"/>
                <circle cx="38" cy="28" r="4"/>
                <circle cx="12" cy="34" r="3"/>
            </g>
        </svg>
    }
}
