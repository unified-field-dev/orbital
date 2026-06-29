//! Orbital Date Pickers product overview (DP-01).

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Hub for the Orbital Date Pickers plugin — component family, first picker setup, and common questions.
///
/// See the crate README for picker and field selection.
/// For resource scheduling (week grid or Gantt lanes), use **Scheduling** previews
/// (`SchedulerCalendar` / `SchedulerTimeline` in `orbital-scheduler`) — not form pickers.
///
/// # When to use
///
/// - Onboarding a new form or settings page that needs date, time, or range entry
/// - Answering setup questions before picking a specific picker component
///
/// # Usage
///
/// Wrap the tree in [`DatetimeLocale`] and bind [`OrbitalDateTime`] through a labeled [`Field`](orbital_core_components::Field)
/// or a picker component from this crate.
///
/// # Best Practices
///
/// ## Why OrbitalDateTime instead of unix seconds?
///
/// Public APIs bind `Option<OrbitalDateTime>` with explicit [`DatetimeTimezone`]. Convert at boundaries via
/// [`ToUnixSeconds`](orbital_base_components::ToUnixSeconds) or [`ToIso8601`](orbital_base_components::ToIso8601).
///
/// ## Do I need an adapter?
///
/// No. Use [`DatetimeLocale`] plus [`DatetimeFormat`](orbital_base_components::DatetimeFormat). Orbital does not ship
/// LocalizationProvider or dayjs adapters.
///
/// ## SSR and hydration
///
/// Wrap pickers in [`DatetimeLocale`] on server and client with matching timezone and format signals.
///
/// # Examples
///
/// ## Picker family
/// Core plugin fields and pickers sharing one [`OrbitalDateTime`] bind model.
/// <!-- preview -->
/// ```rust
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// use orbital_core_components::{Flex, FlexGap, FlexWrap};
/// use crate::{DateCalendar, DateField, DateTimePicker, TimeField};
/// use orbital_base_components::OrbitalDateTime;
/// let date = RwSignal::new(None::<OrbitalDateTime>);
/// let time = RwSignal::new(None::<OrbitalDateTime>);
/// let datetime = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="date-pickers-overview-preview">
///         <PickerPreviewKnobs />
///         <Flex gap=FlexGap::Medium wrap=FlexWrap::Wrap>
///             <DateField bind=date />
///             <TimeField bind=time />
///         </Flex>
///         <DateCalendar bind=date />
///         <DateTimePicker bind=datetime />
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## Minimal DatePicker
/// Wrap the tree in [`DatetimeLocale`] and bind [`OrbitalDateTime`] through a labeled [`Field`].
/// <!-- preview -->
/// ```rust
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// use crate::DatetimeLocale;
/// use orbital_core_components::{DatePicker, DatePickerBind, Field};
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime};
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="date-pickers-overview-getting-started-preview">
///         <PickerPreviewKnobs />
///         <DatetimeLocale default_timezone=Signal::from(DatetimeTimezone::Local)>
///             <Field label="Event date" name="event_date">
///                 <DatePicker bind=value />
///             </Field>
///         </DatetimeLocale>
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-pickers-overview",
    preview_label = "Overview",
    preview_icon = icondata::AiCalendarOutlined,
)]
#[component]
pub fn DatePickersOverviewGuide() -> impl IntoView {
    view! { () }
}
