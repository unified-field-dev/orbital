//! SVG gradient and pattern definition helpers.

use leptos::prelude::*;

/// Register a linear gradient in chart SVG defs.
#[component]
pub fn ChartLinearGradient(
    /// Unique gradient id referenced by series fill.
    #[prop(into)]
    id: String,
    /// Gradient start color.
    #[prop(into)]
    from: String,
    /// Gradient end color.
    #[prop(into)]
    to: String,
) -> impl IntoView {
    view! {
        <linearGradient id=id x1="0%" y1="0%" x2="0%" y2="100%">
            <stop offset="0%" stop-color=from />
            <stop offset="100%" stop-color=to />
        </linearGradient>
    }
}

/// SVG defs container for gradients and patterns.
#[component]
pub fn ChartDefs(children: Children) -> impl IntoView {
    view! {
        <defs>{children()}</defs>
    }
}
