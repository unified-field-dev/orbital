use leptos::prelude::*;
use orbital_base_components::{
    BaseSlider, BaseSliderLabel, FieldInjection, Rule, SliderRuleTrigger,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::slider_styles;
use super::types::{SliderAppearance, SliderBind};

/// Slider adjusts an approximate numeric value along a continuous or stepped range —
/// volume, opacity, or filter bounds.
///
/// Bind an `f64` signal, set min/max/step in appearance, and add [`SliderLabel`](crate::SliderLabel) children when key values need captions. For exact integers, prefer [`NumericStepper`](crate::NumericStepper) or [`Input`](crate::Input). Dual-thumb range selection is not supported.
///
/// # When to use
///
/// - Volume, opacity, or percentage controls - Settings where approximate values are acceptable - Filters with continuous numeric ranges
///
/// # Usage
///
/// 1. Bind a `f64` signal via [`SliderBind`]. 2. Configure min, max, step, and orientation in [`SliderAppearance`]. 3. Add [`SliderLabel`] children to annotate key values on the track.
///
/// # Best Practices
///
/// ## Do's
///
/// * Show min/max labels when the range is not obvious * Use `step` when only discrete values are valid
///
/// ## Don'ts
///
/// * Do not use for precise numeric entry — prefer [`NumericStepper`](crate::NumericStepper) or [`Input`](crate::Input) * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Default
/// Basic slider with a 0-100 range.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(42.0);
/// view! {
///     <div data-testid="slider-preview">
///         <Slider bind=value />
///     </div>
/// }
/// ```
///
/// ## Min and max
/// Constrain the value between custom bounds.
/// <!-- preview -->
/// ```rust
/// use crate::{Slider, SliderAppearance};
/// let value = RwSignal::new(25.0);
/// view! {
///     <div data-testid="slider-range">
///         <Slider
///             bind=value
///             appearance=SliderAppearance {
///                 min: Signal::from(10.0),
///                 max: Signal::from(40.0),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
/// ## Step and stops
/// Snap to fixed increments and display stop marks.
/// <!-- preview -->
/// ```rust
/// use crate::{Slider, SliderAppearance};
/// let value = RwSignal::new(30.0);
/// view! {
///     <div data-testid="slider-step">
///         <Slider
///             bind=value
///             appearance=SliderAppearance {
///                 step: MaybeProp::from(10.0),
///                 show_stops: Signal::from(true),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
/// ## Stops off
/// Keep step snapping while hiding stop marks.
/// <!-- preview -->
/// ```rust
/// use crate::{Slider, SliderAppearance};
/// let value = RwSignal::new(40.0);
/// view! {
///     <div data-testid="slider-stops">
///         <Slider
///             bind=value
///             appearance=SliderAppearance {
///                 step: MaybeProp::from(20.0),
///                 show_stops: Signal::from(false),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
/// ## Vertical
/// Render vertically with the minimum value at the bottom.
/// <!-- preview -->
/// ```rust
/// use crate::{Slider, SliderAppearance};
/// let value = RwSignal::new(60.0);
/// view! {
///     <div data-testid="slider-vertical">
///         <Slider
///             bind=value
///             appearance=SliderAppearance {
///                 vertical: Signal::from(true),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
/// ## Labels
/// Annotate key points using `SliderLabel`.
/// <!-- preview -->
/// ```rust
/// use crate::{Slider, SliderLabel};
/// let value = RwSignal::new(50.0);
/// view! {
///     <div data-testid="slider-labels">
///         <Slider bind=value>
///             <SliderLabel value=Signal::from(0.0)>"0"</SliderLabel>
///             <SliderLabel value=Signal::from(50.0)>"50"</SliderLabel>
///             <SliderLabel value=Signal::from(100.0)>"100"</SliderLabel>
///         </Slider>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "slider",
    preview_label = "Slider",
    preview_icon = icondata::AiSlidersOutlined,
)]
#[component]
pub fn Slider(
    /// Value binding, field identity, and validation rules.
    #[prop(optional, into)]
    bind: SliderBind,
    /// Min/max range, step behavior, orientation, and style options.
    #[prop(optional, into)]
    appearance: SliderAppearance,
    /// Extra CSS class names merged onto the slider root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Slot children, typically one or more [`SliderLabel`].
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    inject_style("orbital-slider", slider_styles());

    let SliderBind {
        value,
        id,
        name,
        rules,
    } = bind;
    let SliderAppearance {
        min,
        max,
        step,
        show_stops,
        vertical,
        style,
    } = appearance;

    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let value = StoredValue::new(value);
    let validate = Rule::validate(rules, value, name);

    Effect::new(move |_| {
        let _ = value.get_value().get();
        validate.run(Some(SliderRuleTrigger::Input));
    });

    let root_class = Signal::derive(move || {
        let mut parts = Vec::new();
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    view! {
        <BaseSlider
            class=root_class
            style=style
            id=id
            name=name
            value=value.get_value()
            min=min
            max=max
            step=step
            show_stops=show_stops
            vertical=vertical
        >
            {children.map(|c| c())}
        </BaseSlider>
    }
}

#[component]
pub fn SliderLabel(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into)] value: Signal<f64>,
    children: Children,
) -> impl IntoView {
    view! {
        <BaseSliderLabel class=class value=value>
            {children()}
        </BaseSliderLabel>
    }
}
