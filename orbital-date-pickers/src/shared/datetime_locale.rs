use super::DatetimeLocaleStrings;
use leptos::prelude::*;
use orbital_base_components::{DatetimeFormat, DatetimeTimezone, OrbitalDateTime};
use orbital_macros::component_doc;

/// BCP-47 or Orbital locale identifier for datetime formatting context.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Locale(pub String);

impl Locale {
    pub fn new(tag: impl Into<String>) -> Self {
        Self(tag.into())
    }
}

impl From<&str> for Locale {
    fn from(tag: &str) -> Self {
        Self(tag.to_string())
    }
}

/// Locale, format, timezone, reference-date defaults, and localized chrome strings for picker subtrees.
#[derive(Clone, Debug, PartialEq)]
pub struct DatetimeLocaleContext {
    pub locale: Locale,
    pub strings: DatetimeLocaleStrings,
    pub default_format: DatetimeFormat,
    pub default_timezone: DatetimeTimezone,
    pub reference_date: OrbitalDateTime,
}

impl Default for DatetimeLocaleContext {
    fn default() -> Self {
        let locale = Locale::from("en-US");
        Self {
            strings: DatetimeLocaleStrings::for_tag(&locale.0),
            locale,
            default_format: DatetimeFormat::default(),
            default_timezone: DatetimeTimezone::Local,
            reference_date: OrbitalDateTime::utc_now(DatetimeTimezone::Local).start_of_day(),
        }
    }
}

#[derive(Clone)]
struct DatetimeLocaleInjection(Memo<DatetimeLocaleContext>);

fn default_locale_signal() -> Signal<Locale> {
    Signal::from(Locale::from("en-US"))
}

fn default_format_signal() -> Signal<DatetimeFormat> {
    Signal::from(DatetimeFormat::default())
}

fn default_timezone_signal() -> Signal<DatetimeTimezone> {
    Signal::from(DatetimeTimezone::Local)
}

fn default_reference_date_signal() -> Signal<OrbitalDateTime> {
    Signal::from(OrbitalDateTime::utc_now(DatetimeTimezone::Local).start_of_day())
}

/// Provides locale, format, timezone, and reference-date defaults to picker subtrees.
///
/// # Timezone
///
/// `default_timezone` supplies the fallback wall-clock zone for child pickers that do not set
/// `appearance.timezone` explicitly. Prefer explicit `appearance.timezone` on each picker when
/// the zone is user-controlled (e.g. UTC vs local).
///
/// # When to use
///
/// - Wrap date/time picker pages so child components share locale defaults
/// - Set a reference calendar day for time-only pickers
/// - Share defaults with `SchedulerCalendar` event dialogs in `orbital-scheduler` that compose [`DateTimePicker`](crate::DateTimePicker)
///
/// # Usage
///
/// Wrap picker content and read defaults via [`use_datetime_locale`].
///
/// # Best Practices
///
/// ## Do's
///
/// - Mount the same `DatetimeLocale` on server and client with matching signals for SSR.
///
/// ## Don'ts
///
/// - Do not confuse `Locale` (format tag) with scheduler toolbar strings — those use `SchedulerLocaleText` in `orbital-scheduler`.
///
/// # Examples
///
/// ## Default locale shell
/// Provides baseline locale context and a calendar using localized weekday headers.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::{DatetimeFormat, DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// use crate::DateCalendar;
/// let locale = Signal::from(Locale::from("fr-FR"));
/// let default_format = Signal::from(DatetimeFormat::IsoDate);
/// let default_timezone = Signal::from(DatetimeTimezone::Local);
/// let reference_date = Signal::from(
///     OrbitalDateTime::try_from_unix_seconds(1_735_689_600, DatetimeTimezone::Local).expect("valid"),
/// );
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// view! {
///     <PickerPreviewExample data_testid="datetime-locale-preview">
///         <PickerPreviewKnobs />
///         <DatetimeLocale
///             locale=locale
///             default_format=default_format
///             default_timezone=default_timezone
///             reference_date=reference_date
///         >
///             <DateCalendar bind=value />
///             <p data-testid="datetime-locale-weekday-sample">
///                 {move || use_datetime_locale_strings().weekday_header_labels()[0].clone()}
///             </p>
///         </DatetimeLocale>
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "datetime-locale",
    preview_label = "Datetime Locale",
    preview_icon = icondata::AiGlobalOutlined,
)]
#[component]
pub fn DatetimeLocale(
    /// BCP-47 or Orbital locale id for month names, weekday labels, first day of week.
    #[prop(default = default_locale_signal())]
    locale: Signal<Locale>,
    /// Default display/parse format when appearance does not override.
    #[prop(default = default_format_signal())]
    default_format: Signal<DatetimeFormat>,
    /// Default timezone for new values and parsing.
    #[prop(default = default_timezone_signal())]
    default_timezone: Signal<DatetimeTimezone>,
    /// Reference calendar day for time-only pickers.
    #[prop(default = default_reference_date_signal())]
    reference_date: Signal<OrbitalDateTime>,
    /// Subtree that consumes locale defaults.
    children: Children,
) -> impl IntoView {
    let context = Memo::new(move |_| {
        let tag = locale.get();
        DatetimeLocaleContext {
            locale: tag.clone(),
            strings: DatetimeLocaleStrings::for_tag(&tag.0),
            default_format: default_format.get(),
            default_timezone: default_timezone.get(),
            reference_date: reference_date.get(),
        }
    });
    provide_context(DatetimeLocaleInjection(context));
    children()
}

/// Returns the active [`DatetimeLocaleContext`] when inside [`DatetimeLocale`].
pub fn use_datetime_locale() -> DatetimeLocaleContext {
    use_context::<DatetimeLocaleInjection>()
        .map(|injection| injection.0.get())
        .unwrap_or_default()
}

/// Default timezone from the nearest [`DatetimeLocale`] provider.
pub fn use_default_timezone() -> DatetimeTimezone {
    use_datetime_locale().default_timezone
}

/// Returns localized picker chrome strings from the nearest [`DatetimeLocale`] provider.
pub fn use_datetime_locale_strings() -> DatetimeLocaleStrings {
    use_datetime_locale().strings
}

/// Reference calendar day from the nearest [`DatetimeLocale`] provider.
pub fn use_reference_date() -> OrbitalDateTime {
    use_datetime_locale().reference_date
}
