//! Custom field slot with picker context (DP-31).

use leptos::prelude::*;
use orbital_core_components::{Field, Input, InputAppearance, InputBind};
use orbital_macros::component_doc;

use crate::use_picker_field;
use orbital_base_components::{format_datetime, DatetimeFormat};

#[component]
pub fn CustomRangeSummaryField() -> impl IntoView {
    let ctx = use_picker_field();
    let text = RwSignal::new(String::new());
    Effect::new(move |_| {
        let display = ctx
            .value
            .get()
            .map(|range| {
                format!(
                    "{} – {}",
                    format_datetime(range.start, DatetimeFormat::IsoDate),
                    format_datetime(range.end, DatetimeFormat::IsoDate)
                )
            })
            .unwrap_or_else(|| "Select a date range".to_string());
        text.set(display);
    });

    view! {
        <Field label="Travel dates" name="travel_range">
            <Input
                bind=InputBind { value: text.into(), ..Default::default() }
                appearance=InputAppearance {
                    readonly: Signal::from(true),
                    disabled: ctx.disabled,
                    ..Default::default()
                }
            />
        </Field>
    }
}

/// Replace the default segmented field with a custom control via [`use_picker_field`].
///
/// [`PickerFieldSlot`](crate::PickerFieldSlot) swaps the built-in range input for any child
/// component that reads picker context through [`use_picker_field`]. The hook exposes the
/// bound value, disabled state, and format signals so custom summaries stay in sync with
/// the calendar popover.
///
/// # When to use
///
/// - Read-only summary inputs that open a calendar on focus or click elsewhere
/// - Branded range displays (for example "Jan 4 – Jan 10" in a single line)
/// - Flows where the default segmented mask does not match your design system
///
/// # Usage
///
/// 1. Implement a field component that calls [`use_picker_field`] inside a [`PickerFieldSlot`](crate::PickerFieldSlot) child.
/// 2. Mirror `ctx.value` and `ctx.disabled` into your control (typically a readonly [`Input`](orbital_core_components::Input)).
/// 3. Pass the slot as a child of [`DateRangePicker`](crate::DateRangePicker) or other range pickers that support field replacement.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep the custom field readonly when the calendar owns editing
/// * Format display text with [`format_datetime`] using the range endpoints' timezones
/// * Wrap custom inputs in [`Field`] for labeling and validation
///
/// ## Don'ts
///
/// * Do not call [`use_picker_field`] outside a picker field slot — context will be missing
/// * Do not write directly to the bind signal from the summary without updating the calendar selection
///
/// # Examples
///
/// ## Custom read-only summary
/// Replace the default segmented range field with a summary input bound through picker context.
/// <!-- preview -->
/// ```rust
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// use crate::{CustomRangeSummaryField, DateRangePicker, DateTimeRange, PickerFieldSlot};
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="date-pickers-custom-field-preview">
///         <PickerPreviewKnobs />
///         <DateRangePicker bind=value>
///             <PickerFieldSlot slot>
///                 <CustomRangeSummaryField />
///             </PickerFieldSlot>
///         </DateRangePicker>
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-pickers-custom-field",
    preview_label = "Custom Field",
    preview_icon = icondata::AiFormOutlined,
)]
#[component]
pub fn DatePickersCustomFieldGuide() -> impl IntoView {
    view! { () }
}
