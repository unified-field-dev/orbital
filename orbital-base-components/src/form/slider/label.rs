use leptos::prelude::*;

use super::base::SliderInjection;

/// Positioned label marker for [`BaseSlider`](super::base::BaseSlider).
#[component(transparent)]
pub fn BaseSliderLabel(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into)] value: Signal<f64>,
    children: Children,
) -> impl IntoView {
    let slider = SliderInjection::expect_context();

    let style = move || {
        let offset = (value.get() - slider.min.get()) / (slider.max.get() - slider.min.get());
        if slider.vertical.get() {
            format!(
                "bottom: calc({offset} * (100% - var(--orbital-slider__thumb--size)) + var(--orbital-slider__thumb--size) / 2)"
            )
        } else {
            format!(
                "left: calc({offset} * (100% - var(--orbital-slider__thumb--size)) + var(--orbital-slider__thumb--size) / 2)"
            )
        }
    };

    view! {
        <div class=move || {
            let mut parts = vec![
                "orbital-slider-label".to_string(),
                if slider.vertical.get() {
                    "orbital-slider-label--vertical".to_string()
                } else {
                    "orbital-slider-label--horizontal".to_string()
                },
            ];
            if let Some(extra) = class.get() {
                if !extra.is_empty() {
                    parts.push(extra);
                }
            }
            parts.join(" ")
        } style=style>
            {children()}
        </div>
    }
}
