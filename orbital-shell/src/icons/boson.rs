use leptos::prelude::*;

/// Boson icon component
///
/// Returns an SVG icon that can be dropped into any component. Uses currentColor for stroke, so it will inherit the text color. Represents the async work engine with a lightning-bolt-through-gear motif.
#[component]
pub fn Boson(
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
                // Gear outline
                <circle cx="24" cy="24" r="14"/>
                <circle cx="24" cy="24" r="6"/>
                // Gear teeth (top, right, bottom, left, and diagonals)
                <line x1="24" y1="4" x2="24" y2="10"/>
                <line x1="24" y1="38" x2="24" y2="44"/>
                <line x1="4" y1="24" x2="10" y2="24"/>
                <line x1="38" y1="24" x2="44" y2="24"/>
                <line x1="9.9" y1="9.9" x2="14.1" y2="14.1"/>
                <line x1="33.9" y1="33.9" x2="38.1" y2="38.1"/>
                <line x1="9.9" y1="38.1" x2="14.1" y2="33.9"/>
                <line x1="33.9" y1="14.1" x2="38.1" y2="9.9"/>
                // Lightning bolt through center
                <polyline points="22,16 20,24 26,23 24,32"/>
            </g>
        </svg>
    }
}
