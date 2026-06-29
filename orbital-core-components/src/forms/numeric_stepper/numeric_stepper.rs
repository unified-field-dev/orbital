use leptos::prelude::*;
use orbital_base_components::{
    BaseNumericStepper, FieldInjection, NumericStepperRuleTrigger, Rule,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::numeric_stepper_styles;
use super::types::{NumericStepperAppearance, NumericStepperBind};

/// NumericStepper edits a bounded integer with typed entry and +/- steppers — cart quantities,
/// retry limits, and discrete counters.
///
/// Bind an `i32` signal, clamp with min/max, and set step to match allowed increments.
/// For approximate values along a range, prefer [`Slider`](crate::Slider).
///
/// # API notes
///
/// - Bind `i32` values via [`NumericStepperBind`]; set min, max, and step in [`NumericStepperAppearance`].
/// - Display formatting is not built in — format values in the display layer when needed.
/// - Decimal and float precision are not supported — use [`Slider`](crate::Slider) or [`Input`](crate::Input) instead.
///
/// Validation: attach rules on `bind.rules` when wrapped in [`Field`](crate::Field).
///
/// # When to use
///
/// - Quantity selectors in forms and carts
/// - Integer settings with clear increment steps
/// - Compact numeric fields where +/- buttons aid precision
///
/// # Usage
///
/// 1. Bind an `i32` signal via [`NumericStepperBind`].
/// 2. Set min, max, step, and size in [`NumericStepperAppearance`].
/// 3. Wrap in [`Field`](crate::Field) when the control needs a label or validation.
///
/// # Best Practices
///
/// ## Do's
///
/// * Set sensible min/max bounds to prevent invalid values
/// * Use `step` matching the allowed increment granularity
///
/// ## Don'ts
///
/// * Do not use for free-form text — use [`Input`](crate::Input)
/// * Do not use for approximate or fractional values — use [`Slider`](crate::Slider) or [`Input`](crate::Input)
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Default
/// Basic counter with +/- controls.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(0);
/// view! {
///     <div data-testid="numeric-stepper-preview">
///         <NumericStepper bind=value />
///     </div>
/// }
/// ```
///
/// ## Bounded range
/// Clamp values between min and max.
/// <!-- preview -->
/// ```rust
/// use crate::{NumericStepper, NumericStepperAppearance};
/// let value = RwSignal::new(5);
/// view! {
///     <div data-testid="numeric-stepper-bounded">
///         <NumericStepper
///             bind=value
///             appearance=NumericStepperAppearance {
///                 min: Signal::from(0),
///                 max: Signal::from(10),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
/// ## Step
/// Increase and decrease by custom increments.
/// <!-- preview -->
/// ```rust
/// use crate::{NumericStepper, NumericStepperAppearance};
/// let value = RwSignal::new(10);
/// view! {
///     <div data-testid="numeric-stepper-step">
///         <NumericStepper
///             bind=value
///             appearance=NumericStepperAppearance {
///                 step: Signal::from(5),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
/// ## Disabled
/// Prevent direct typing and button interaction.
/// <!-- preview -->
/// ```rust
/// use crate::{NumericStepper, NumericStepperAppearance};
/// let value = RwSignal::new(3);
/// view! {
///     <div data-testid="numeric-stepper-disabled">
///         <NumericStepper
///             bind=value
///             appearance=NumericStepperAppearance {
///                 disabled: Signal::from(true),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
/// ## Small
/// Compact control for dense layouts.
/// <!-- preview -->
/// ```rust
/// use crate::{NumericStepper, NumericStepperAppearance, NumericStepperSize};
/// let value = RwSignal::new(7);
/// view! {
///     <div data-testid="numeric-stepper-small">
///         <NumericStepper
///             bind=value
///             appearance=NumericStepperAppearance {
///                 size: Signal::from(NumericStepperSize::Small),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "numeric-stepper",
    preview_label = "Numeric Stepper",
    preview_icon = icondata::AiPlusOutlined,
)]
#[component]
pub fn NumericStepper(
    /// Value binding, field identity, and validation rules.
    #[prop(optional, into)]
    bind: NumericStepperBind,
    /// Min/max range, step size, placeholder, disabled state, and visual size.
    #[prop(optional, into)]
    appearance: NumericStepperAppearance,
    /// Extra CSS class names merged onto the control root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orbital-numeric-stepper", numeric_stepper_styles());

    let NumericStepperBind {
        value,
        id,
        name,
        rules,
    } = bind;
    let NumericStepperAppearance {
        min,
        max,
        step,
        placeholder,
        disabled,
        size,
    } = appearance;

    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let value = StoredValue::new(value);
    let validate = Rule::validate(rules, value, name);

    Effect::new(move |_| {
        let _ = value.get_value().get();
        validate.run(Some(NumericStepperRuleTrigger::Change));
    });

    let root_class = Signal::derive(move || {
        let mut parts = vec![format!("orbital-numeric-stepper--{}", size.get().as_str())];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    view! {
        <BaseNumericStepper
            class=root_class
            id=id
            name=name
            value=value.get_value()
            min=min
            max=max
            step=step
            placeholder=placeholder
            disabled=disabled
        />
    }
}
