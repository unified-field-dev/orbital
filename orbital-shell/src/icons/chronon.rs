use leptos::prelude::*;

/// Chronon icon component
///
/// Returns an SVG icon that can be dropped into any component. Uses currentColor for stroke, so it will inherit the text color. Represents the cron scheduler with a clock and cyclic-arrow motif.
#[component]
pub fn Chronon(
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
                // Clock face
                <circle cx="24" cy="24" r="16"/>
                // Hour hand
                <line x1="24" y1="24" x2="24" y2="16"/>
                // Minute hand
                <line x1="24" y1="24" x2="30" y2="20"/>
                // Center dot
                <circle cx="24" cy="24" r="1.5"/>
                // Hour markers at 12, 3, 6, 9
                <line x1="24" y1="10" x2="24" y2="12"/>
                <line x1="38" y1="24" x2="36" y2="24"/>
                <line x1="24" y1="38" x2="24" y2="36"/>
                <line x1="10" y1="24" x2="12" y2="24"/>
                // Cyclic refresh arrow (top-right arc with arrowhead)
                <path d="M38 10c3.5 4 5 9 5 14"/>
                <polyline points="40,8 38,10 40,12"/>
            </g>
        </svg>
    }
}
