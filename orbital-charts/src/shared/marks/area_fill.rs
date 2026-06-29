//! Area fill path with optional gradient.

use leptos::prelude::*;

/// Filled area path under a line.
#[component]
pub fn AreaFill(
    /// SVG path `d` attribute (closed area).
    #[prop(into)]
    d: Signal<String>,
    /// Series color for solid or gradient top stop.
    #[prop(into)]
    series_color: Signal<String>,
    /// Gradient definition id.
    gradient_id: String,
    /// When true, fill with vertical gradient to transparent.
    #[prop(default = false)]
    use_gradient: bool,
    /// Fill opacity for stacked areas.
    #[prop(default = 0.35)]
    fill_opacity: f64,
) -> impl IntoView {
    let gid = gradient_id.clone();
    let fill = Signal::derive(move || {
        if use_gradient {
            format!("url(#{gid})")
        } else {
            series_color.get()
        }
    });

    view! {
        {use_gradient.then(|| {
            let grad = gradient_id.clone();
            view! {
                <defs>
                    <linearGradient id=grad x1="0" y1="0" x2="0" y2="1">
                        <stop offset="0%" stop-color=move || series_color.get() />
                        <stop offset="100%" stop-color=move || series_color.get() stop-opacity="0" />
                    </linearGradient>
                </defs>
            }
        })}
        <path
            class="orb-area-fill"
            d=move || d.get()
            fill=move || fill.get()
            fill-opacity=fill_opacity
        />
    }
}

/// Gradient definition id for a series.
pub fn area_gradient_id(series_id: &str) -> String {
    format!("orb-area-grad-{series_id}")
}
