//! Interactive playground with format, timezone, and density knobs (DP-33).

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Interactive sandbox for picker format, timezone, and density combinations.
///
/// Use this page to verify how [`DatePicker`](orbital_core_components::DatePicker) masks,
/// timezone signals, and theme density interact before wiring the same props into production
/// forms. Controls map directly to `appearance` fields on the bound picker.
///
/// # When to use
///
/// - Spiking locale and timezone combinations during plugin integration
/// - QA checks after changing [`DatetimeLocale`](crate::DatetimeLocale) defaults
/// - Demos where stakeholders toggle US vs ISO masks and local vs UTC display
///
/// # Usage
///
/// 1. Bind `Option<OrbitalDateTime>` on a labeled [`Field`](orbital_core_components::Field).
/// 2. Drive `appearance.format` and `appearance.timezone` from local signals (Select, Switch, etc.).
/// 3. Include [`PickerPreviewKnobs`](crate::preview::PickerPreviewKnobs) to exercise compact/default/spacious density.
/// 4. Add a readout below the picker when debugging formatted output vs unix storage.
///
/// # Best Practices
///
/// ## Do's
///
/// * Mirror production `DatetimeFormat` and `DatetimeTimezone` values in playground controls
/// * Wrap pickers in [`Field`] even in sandboxes — matches real form composition
/// * Use [`PickerPreviewControls`](crate::preview::PickerPreviewControls) to lay out inline knobs
///
/// ## Don'ts
///
/// * Do not treat playground wiring as production validation — pair with [`DatePickerRule`] tests for bounds
/// * Do not hard-code timezone strings — map selects to [`DatetimeTimezone`] enum values
///
/// # Examples
///
/// ## Knobs playground
/// Adjust density, format, and timezone; toggle 12h display via the readout label.
/// <!-- preview -->
/// ```rust
/// use crate::preview::{PickerPreviewControls, PickerPreviewExample, PickerPreviewKnobs};
/// use crate::{DateCalendar, DateCalendarAppearance, DateCalendarBind};
/// use orbital_core_components::{Field, Select, SelectAppearance, SelectBind, Switch, SwitchBind};
/// use orbital_base_components::{DatetimeFormat, DatetimeTimezone, OrbitalDateTime, format_datetime, ToUnixSeconds};
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// let format = RwSignal::new("us".to_string());
/// let timezone = RwSignal::new("local".to_string());
/// let use_24h = RwSignal::new(false);
/// let picker_format = Signal::derive(move || if format.get() == "iso" { DatetimeFormat::IsoDate } else { DatetimeFormat::UsDate });
/// let picker_timezone = Signal::derive(move || if timezone.get() == "utc" { DatetimeTimezone::Utc } else { DatetimeTimezone::Local });
/// view! {
///     <PickerPreviewExample data_testid="date-pickers-playground-preview">
///         <PickerPreviewKnobs />
///         <PickerPreviewControls>
///             <Field label="Format" name="playground_format">
///                 <Select bind=SelectBind::from(format) appearance=SelectAppearance { default_value: Some("us".to_string()), ..Default::default() }>
///                     <option value="us">"US"</option>
///                     <option value="iso">"ISO"</option>
///                 </Select>
///             </Field>
///             <Field label="Timezone" name="playground_timezone">
///                 <Select bind=SelectBind::from(timezone) appearance=SelectAppearance { default_value: Some("local".to_string()), ..Default::default() }>
///                     <option value="local">"Local"</option>
///                     <option value="utc">"UTC"</option>
///                 </Select>
///             </Field>
///             <Switch bind=SwitchBind::from(use_24h) label="24-hour readout" />
///         </PickerPreviewControls>
///         <Field label="Playground calendar" name="playground_calendar">
///             <DateCalendar
///                 bind=DateCalendarBind { value: value.into(), ..Default::default() }
///                 appearance=DateCalendarAppearance { timezone: picker_timezone, ..Default::default() }
///             />
///         </Field>
///         <div data-testid="date-pickers-playground-readout">
///             {move || match value.get() {
///                 Some(dt) => if use_24h.get() {
///                     format!("{} (unix {})", format_datetime(dt, DatetimeFormat::Time24), dt.to_unix_seconds())
///                 } else {
///                     format!("{} (unix {})", format_datetime(dt, picker_format.get()), dt.to_unix_seconds())
///                 },
///                 None => "none".to_string(),
///             }}
///         </div>
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-pickers-playground",
    preview_label = "Playground",
    preview_icon = icondata::AiExperimentOutlined,
)]
#[component]
pub fn DatePickersPlaygroundGuide() -> impl IntoView {
    view! { () }
}
