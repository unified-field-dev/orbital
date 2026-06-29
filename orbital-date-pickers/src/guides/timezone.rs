//! Timezone guide — UTC vs local wall-time display.

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Control how pickers interpret and display wall-clock time for the same UTC instant.
///
/// Each [`OrbitalDateTime`] stores a UTC instant plus an explicit [`DatetimeTimezone`].
/// Set `appearance.timezone` on pickers so parsing, labels, and scroll columns match the
/// timezone your users expect. When toggling display timezone in UI, re-tag the bound value
/// with [`OrbitalDateTime::from_instant`] so hours and calendar days stay consistent.
///
/// # When to use
///
/// - Scheduling flows where users switch between local and UTC views
/// - Admin tools displaying API timestamps in the viewer's local wall time
/// - Cross-region reporting where the same instant must read differently by zone
///
/// # Usage
///
/// 1. Wrap the tree in [`DatetimeLocale`](crate::DatetimeLocale) with a default timezone signal.
/// 2. Pass `appearance.timezone` to [`DateTimePicker`](crate::DateTimePicker), [`DatePicker`](orbital_core_components::DatePicker), or [`DigitalClock`](crate::DigitalClock).
/// 3. Keep the bound value's embedded timezone aligned with the active display mode when users switch zones.
/// 4. Format readouts with [`format_unix`](orbital_base_components::format_unix) when you need display-zone text independent of the stored tag.
///
/// # Best Practices
///
/// ## Do's
///
/// * Pair a datetime picker with a [`DigitalClock`](crate::DigitalClock) readout so hour shifts are obvious when toggling zones
/// * Store and transmit instants as unix seconds or ISO-8601 at API boundaries; apply timezone at display time
/// * Use [`DatetimeTimezone::FixedOffset`] when a product requires a stable offset independent of DST
///
/// ## Don'ts
///
/// * Do not assume the browser offset alone — always set `appearance.timezone` explicitly
/// * Do not change timezone on the bound value without preserving the same UTC instant
///
/// # Timezone reference
///
/// | Type | Description |
/// |------|-------------|
/// | [`DatetimeTimezone::Local`] | Browser/system local offset (respects DST) |
/// | [`DatetimeTimezone::Utc`] | UTC wall time (no offset) |
/// | [`DatetimeTimezone::FixedOffset`] | Fixed offset in seconds east of UTC |
///
/// # Examples
///
/// ## UTC vs local
/// Toggle timezone to see the same instant as local vs UTC wall time; the datetime picker and digital clock update together.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// use orbital_base_components::{DatetimeFormat, DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// use crate::preview::{PickerPreviewControls, PickerPreviewExample, PickerPreviewKnobs};
/// use crate::{DateTimePicker, DateTimePickerAppearance, DigitalClock, DigitalClockAppearance};
/// // 2025-01-01 15:30 UTC — afternoon in UTC, morning in US Pacific (UTC-8).
/// const PACIFIC: DatetimeTimezone = DatetimeTimezone::FixedOffset(-8 * 3600);
/// let value = RwSignal::new(Some(
///     OrbitalDateTime::try_from_unix_seconds(1_735_745_400_i64, PACIFIC)
///         .expect("valid datetime"),
/// ));
/// let timezone = RwSignal::new(PACIFIC);
/// let reference_date = Signal::derive(move || {
///     value
///         .get()
///         .map(|v| v.start_of_day())
///         .unwrap_or_else(|| OrbitalDateTime::utc_now(timezone.get()).start_of_day())
/// });
/// view! {
///     <PickerPreviewExample data_testid="date-pickers-timezone-preview">
///         <PickerPreviewKnobs />
///         <PickerPreviewControls>
///             <Button
///                 appearance=ButtonAppearance::Secondary
///                 attr:data-testid="date-pickers-timezone-local"
///                 on_click=Callback::new({
///                     move |_| {
///                         timezone.set(PACIFIC);
///                         if let Some(v) = value.get_untracked() {
///                             value.set(Some(OrbitalDateTime::from_instant(v.instant(), PACIFIC)));
///                         }
///                     }
///                 })
///             >
///                 "Pacific (UTC-8)"
///             </Button>
///             <Button
///                 appearance=ButtonAppearance::Secondary
///                 attr:data-testid="date-pickers-timezone-utc"
///                 on_click=Callback::new({
///                     move |_| {
///                         timezone.set(DatetimeTimezone::Utc);
///                         if let Some(v) = value.get_untracked() {
///                             value.set(Some(OrbitalDateTime::from_instant(v.instant(), DatetimeTimezone::Utc)));
///                         }
///                     }
///                 })
///             >
///                 "UTC"
///             </Button>
///         </PickerPreviewControls>
///         <DateTimePicker
///             bind=value
///             appearance=DateTimePickerAppearance {
///                 timezone: Signal::from(timezone),
///                 ..Default::default()
///             }
///         />
///         <div data-testid="date-pickers-timezone-clock">
///             <DigitalClock
///                 bind=value
///                 appearance=DigitalClockAppearance {
///                     timezone: Signal::from(timezone),
///                     reference_date,
///                     time_step: Signal::from(30),
///                     ..Default::default()
///                 }
///             />
///         </div>
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-pickers-timezone",
    preview_label = "Timezone",
    preview_icon = icondata::AiGlobalOutlined,
)]
#[component]
pub fn DatePickersTimezoneGuide() -> impl IntoView {
    view! { () }
}
