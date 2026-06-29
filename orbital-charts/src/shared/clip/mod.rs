//! SVG clip path helpers for plot overflow control.
//!
//! Wrap plot marks in [`PlotClip`] per [`ChartCompositionOrder`]: grid renders beneath the clip,
//! plot children inside the clip stack bottom-to-top by child order.

use leptos::prelude::*;

use crate::context::use_drawing_area;

/// Defines a plot-area clip path and wraps children in a clipped `<g>`.
///
/// Provide a unique `id` per chart instance (for example `orb-clip-mixed-chart`).
#[component]
pub fn PlotClip(
    /// Unique SVG clip path id referenced by `clip-path="url(#id)"`.
    #[prop(into)]
    id: String,
    /// Plot layers to clip to the drawing area bounds.
    children: Children,
) -> impl IntoView {
    let area = use_drawing_area();
    let clip_ref = format!("url(#{id})");

    view! {
        <defs>
            <clipPath id=id.clone() attr:data-orb-chart-clip="">
                <rect x=0.0 y=0.0 width=area.plot_width height=area.plot_height />
            </clipPath>
        </defs>
        <g clip-path=clip_ref>
            {children()}
        </g>
    }
}
