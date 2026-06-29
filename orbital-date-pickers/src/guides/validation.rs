//! Validation guide — min/max [`DatePickerRule`] with Field.

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Constrain selectable dates with [`DatePickerRule`] and surface errors through [`Field`](orbital_core_components::Field).
///
/// Min and max bounds apply on blur and when completing segmented input. Mirror the same
/// bounds on `appearance.min_date` / `appearance.max_date` so the calendar disables out-of-range
/// days before the user commits an invalid value.
///
/// # When to use
///
/// - Booking or scheduling forms with policy date windows
/// - Admin tools that must reject dates outside a fiscal or contract period
/// - Any labeled field where validation messages should appear below the control
///
/// # Usage
///
/// 1. Define min and max as `Signal<Option<OrbitalDateTime>>`.
/// 2. Attach rules on [`DatePickerBind`](orbital_core_components::DatePickerBind): `DatePickerRule::min_date(min)` and `DatePickerRule::max_date(max)`.
/// 3. Pass the same signals to `appearance.min_date` and `appearance.max_date` on the picker.
/// 4. Wrap the picker in [`Field`] with a stable `name` for accessible error association.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep bind rules and appearance bounds in sync so keyboard entry and calendar clicks behave the same
/// * Use UTC or a fixed timezone for policy bounds that must not drift with DST
/// * Show the allowed range in helper text when users frequently hit validation errors
///
/// ## Don'ts
///
/// * Do not validate only on submit — picker rules run on blur for immediate feedback
/// * Do not set min/max on appearance without matching bind rules (or vice versa)
///
/// # Validation reference
///
/// | API | Description |
/// |-----|-------------|
/// | [`DatePickerRule::min_date`] | Rejects values before the bound day |
/// | [`DatePickerRule::max_date`] | Rejects values after the bound day |
/// | `appearance.min_date` / `max_date` | Disables calendar days outside the range |
///
/// # Examples
///
/// ## Out of range
/// Allowed range is **01/01/2025** through **01/01/2026** (US date mask, UTC bounds). Type a date outside that window — for example **01/01/2020** or **01/02/2027** — then tab away; the [`Field`](orbital_core_components::Field) shows a validation error on blur.
/// <!-- preview -->
/// ```rust
/// use orbital_core_components::{DatePicker, DatePickerAppearance, DatePickerBind, Field};
/// use orbital_base_components::{DatePickerRule, DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// let min = Signal::from(Some(
///     OrbitalDateTime::try_from_unix_seconds(1735689600_i64, DatetimeTimezone::Utc)
///         .expect("valid min"),
/// ));
/// let max = Signal::from(Some(
///     OrbitalDateTime::try_from_unix_seconds(1767225600_i64, DatetimeTimezone::Utc)
///         .expect("valid max"),
/// ));
/// view! {
///     <PickerPreviewExample data_testid="date-pickers-validation-preview">
///         <PickerPreviewKnobs />
///         <Field label="Event date" name="event_date">
///             <DatePicker
///                 bind=DatePickerBind {
///                     value: value.into(),
///                     rules: vec![DatePickerRule::min_date(min), DatePickerRule::max_date(max)],
///                     ..Default::default()
///                 }
///                 appearance=DatePickerAppearance { min_date: min, max_date: max, ..Default::default() }
///             />
///         </Field>
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-pickers-validation",
    preview_label = "Validation",
    preview_icon = icondata::AiCalendarOutlined,
)]
#[component]
pub fn DatePickersValidationGuide() -> impl IntoView {
    view! { () }
}
