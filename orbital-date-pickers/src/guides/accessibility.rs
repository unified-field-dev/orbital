//! Accessibility guide — grid roles, labels, keyboard nav (DP-04).

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Keyboard, ARIA, and labeling patterns for Orbital date and time pickers.
///
/// Calendar grids expose `role="grid"` with roving tabindex and arrow-key navigation.
/// Segmented fields use `role="spinbutton"` with `aria-valuenow` on numeric segments.
/// Always wrap pickers in [`Field`](orbital_core_components::Field) so labels associate
/// with the first focusable segment or calendar control.
///
/// # When to use
///
/// - Any production form that must meet WCAG keyboard and labeling requirements
/// - Flows where users navigate dates exclusively from the keyboard
/// - Custom picker layouts that still need correct roles on calendar and field surfaces
///
/// # Usage
///
/// 1. Wrap each picker in [`Field`] with a visible `label` and stable `name`.
/// 2. Prefer built-in [`DateField`](crate::DateField) and [`DateCalendar`](crate::DateCalendar) over raw HTML inputs.
/// 3. Test arrow keys on calendar grids and Tab/Shift+Tab through segmented fields.
/// 4. Verify screen readers announce the label, current segment value, and validation errors from [`Field`].
///
/// # Best Practices
///
/// ## Do's
///
/// * Provide a visible label for every picker — placeholder text is not a substitute
/// * Keep calendar and field pickers on separate labeled fields when both appear on one page
/// * Use [`Field`] validation states so errors are announced with `aria-invalid`
///
/// ## Don'ts
///
/// * Do not put `data-testid` on picker internals — wrap with a native element for E2E targets
/// * Do not remove keyboard handlers from custom day renderers without restoring grid navigation
///
/// # Accessibility reference
///
/// | Surface | Roles / behavior |
/// |---------|------------------|
/// | [`DateCalendar`](crate::DateCalendar) | `role="grid"`, arrow keys move between days |
/// | [`DateField`](crate::DateField) | `role="spinbutton"` segments, arrow keys increment |
/// | [`Field`](orbital_core_components::Field) | Associates `<label>` with the first control |
///
/// # Examples
///
/// ## Labeled field and keyboard calendar
/// A segmented date field and inline calendar, each wrapped in a labeled [`Field`].
/// <!-- preview -->
/// ```rust
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// use crate::{DateCalendar, DateCalendarBind, DateField, DateFieldBind};
/// use orbital_core_components::Field;
/// use orbital_base_components::OrbitalDateTime;
/// let calendar_value = RwSignal::new(None::<OrbitalDateTime>);
/// let field_value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="date-pickers-a11y-preview">
///         <PickerPreviewKnobs />
///         <Field label="Event date" name="event_date">
///             <DateField bind=field_value />
///         </Field>
///         <Field label="Pick on calendar" name="calendar_date">
///             <DateCalendar bind=calendar_value />
///         </Field>
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-pickers-a11y",
    preview_label = "Accessibility",
    preview_icon = icondata::AiEyeOutlined,
)]
#[component]
pub fn DatePickersAccessibilityGuide() -> impl IntoView {
    view! { () }
}
