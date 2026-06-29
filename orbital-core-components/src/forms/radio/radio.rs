use leptos::prelude::*;
use orbital_base_components::BaseRadio;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::forms::radio_group::radio_group_styles;

/// Radio marks one choice within a [`RadioGroup`](crate::RadioGroup) where only a single
/// option can be selected — deployment tier, billing plan, or permission level.
///
/// **Radio + RadioGroup pair:** always bind selection on [`RadioGroup`](crate::RadioGroup) (`Option<String>`).
/// Each [`Radio`](crate::Radio) supplies a `value` and `label`. Preview slug `radio` documents both components.
///
/// Give each `Radio` a distinct `value` and `label`. Wrap the group in [`Field`](crate::Field) when the set requires a visible heading or validation.
///
/// # When to use
///
/// - Mutually exclusive choices (plan tier, shipping method, environment) - Short option lists where every choice should remain visible - Form fields where exactly one value is required
///
/// # API notes
///
/// - Bind selection on [`RadioGroup`](crate::RadioGroup) with `Option<String>` — `None` means no selection yet.
/// - Set [`RadioGroupLayout::Horizontal`] on [`RadioGroup`](crate::RadioGroup) for compact horizontal rows.
/// - Each [`Radio`](crate::Radio) supplies a distinct `value` and visible `label`.
///
/// # Usage
///
/// 1. Create an `Option<String>` signal and pass it to [`RadioGroup`](crate::RadioGroup) via `bind`.
/// 2. Add one [`Radio`](crate::Radio) per option with a unique `value` and visible `label`.
/// 3. Wrap in [`Field`](crate::Field) when the group needs a field label or validation messaging.
/// 4. Wrap preview examples in a native element with `data-testid` for E2E selectors.
///
/// # Best Practices
///
/// ## Do's
///
/// * Always use [`RadioGroup`](crate::RadioGroup) as the parent container * Provide a visible `label` on each option for accessible naming * Use `Field` + `RadioGroupRule::required` when a selection is mandatory
///
/// ## Don'ts
///
/// * Do not use for multi-select — prefer [`Checkbox`](crate::Checkbox) * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Default selected
/// One option starts checked when the bound signal holds its value.
/// <!-- preview -->
/// ```rust
/// use crate::{Radio, RadioGroup};
/// let choice = RwSignal::new(Some("a".to_string()));
/// view! {
///     <div data-testid="radio-preview">
///     <div data-testid="radio-group-preview">
///         <RadioGroup bind=choice>
///             <Radio value="a" label="Option A" />
///             <Radio value="b" label="Option B" />
///         </RadioGroup>
///     </div>
///     </div>
/// }
/// ```
///
/// ## Starts unselected
/// No option is checked when the bound signal is `None`.
/// <!-- preview -->
/// ```rust
/// use crate::{Radio, RadioGroup};
/// let choice = RwSignal::new(None::<String>);
/// view! {
///     <div data-testid="radio-group-unselected">
///         <RadioGroup bind=choice>
///             <Radio value="one" label="One" />
///             <Radio value="two" label="Two" />
///         </RadioGroup>
///     </div>
/// }
/// ```
///
/// ## Click changes selection
/// Clicking another option moves the checked state within the group.
/// <!-- preview -->
/// ```rust
/// use crate::{Radio, RadioGroup};
/// let choice = RwSignal::new(Some("cat".to_string()));
/// view! {
///     <div data-testid="radio-group-click">
///         <RadioGroup bind=choice>
///             <Radio value="cat" label="Cat" />
///             <Radio value="dog" label="Dog" />
///         </RadioGroup>
///     </div>
/// }
/// ```
///
/// ## Field wrapper integration
/// [`Field`](crate::Field) supplies the group label; each [`Radio`](crate::Radio) keeps its option label.
/// <!-- preview -->
/// ```rust
/// use crate::{Field, Radio, RadioGroup};
/// let choice = RwSignal::new(Some("daily".to_string()));
/// view! {
///     <div data-testid="radio-group-field">
///         <Field label="Digest frequency" name="frequency">
///             <RadioGroup bind=choice>
///                 <Radio value="daily" label="Daily" />
///                 <Radio value="weekly" label="Weekly" />
///             </RadioGroup>
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Disabled fieldset
/// A disabled fieldset prevents changing the current selection.
/// <!-- preview -->
/// ```rust
/// use crate::{Radio, RadioGroup};
/// let choice = RwSignal::new(Some("stable".to_string()));
/// view! {
///     <div data-testid="radio-group-disabled">
///         <fieldset disabled>
///             <RadioGroup bind=choice>
///                 <Radio value="stable" label="Stable" />
///                 <Radio value="beta" label="Beta" />
///             </RadioGroup>
///         </fieldset>
///     </div>
/// }
/// ```
///
/// ## Horizontal layout
/// Compact row of options using [`RadioGroupLayout::Horizontal`].
/// <!-- preview -->
/// ```rust
/// use crate::{Radio, RadioGroup, RadioGroupLayout};
/// let choice = RwSignal::new(Some("a".to_string()));
/// view! {
///     <div data-testid="radio-group-horizontal">
///         <RadioGroup bind=choice layout=RadioGroupLayout::Horizontal>
///             <Radio value="a" label="Option A" />
///             <Radio value="b" label="Option B" />
///             <Radio value="c" label="Option C" />
///         </RadioGroup>
///     </div>
/// }
/// ```
///
/// ## Required validation
/// Required rule on the group; arrow keys move selection between options.
/// <!-- preview -->
/// ```rust
/// use crate::{Field, Radio, RadioGroup, RadioGroupBind, RadioGroupRule};
/// let choice = RwSignal::new(None::<String>);
/// let required = Signal::from(true);
/// view! {
///     <div data-testid="radio-group-required">
///         <Field label="Environment" name="environment" required=true>
///             <RadioGroup
///                 bind=RadioGroupBind {
///                     value: choice.into(),
///                     rules: vec![RadioGroupRule::required(required)],
///                     ..Default::default()
///                 }
///             >
///                 <Radio value="dev" label="Development" />
///                 <Radio value="prod" label="Production" />
///             </RadioGroup>
///         </Field>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "radio",
    preview_label = "Radio",
    preview_icon = icondata::AiCheckCircleOutlined,
)]
#[component]
pub fn Radio(
    /// Additional CSS class names merged onto the root `orbital-radio` span.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Value submitted when this option is selected; must be unique within the group.
    #[prop(optional, into)]
    value: String,
    /// Visible caption rendered beside the indicator; also used as the accessible name.
    #[prop(optional, into)]
    label: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orbital-radio", radio_group_styles());
    let class = MaybeProp::derive(move || {
        let mut parts = vec!["orbital-radio".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        Some(parts.join(" "))
    });

    view! {
        <BaseRadio class=class value=value label=label />
    }
}
