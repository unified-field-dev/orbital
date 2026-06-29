//! Shortcuts guide — preset chips on [`DateCalendar`].

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Add preset shortcut chips above calendar panels for one-click date selection.
///
/// Shortcut presets appear as a horizontal bar of Orbital [`Button`](orbital_core_components::Button)
/// controls. Selecting a chip commits an [`OrbitalDateTime`] to the picker bind immediately —
/// useful for common reporting ranges without opening the grid.
///
/// # When to use
///
/// - Dashboard date filters (Today, Yesterday, This week)
/// - Reporting UIs where users pick the same relative dates repeatedly
/// - Range pickers that need quick anchors before fine-tuning on the calendar
///
/// # Usage
///
/// 1. Build a shortcut list with helpers such as [`today_and_yesterday_shortcuts`].
/// 2. Pass the list to `appearance.shortcuts` on [`DateCalendar`](crate::DateCalendar) or popover pickers that support shortcuts.
/// 3. Match `appearance.timezone` on the picker to the timezone used when constructing shortcuts.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep labels short and action-oriented ("Today", "Yesterday", "Last 7 days")
/// * Rebuild shortcuts when the active timezone changes so "today" resolves correctly
/// * Limit visible presets to the most common ranges — use a menu for long tail options
///
/// ## Don'ts
///
/// * Do not mix shortcut timezones with a different picker `appearance.timezone`
/// * Do not rely on shortcuts alone when users need arbitrary historical dates — keep the calendar available
///
/// # Examples
///
/// ## This week preset
/// Today and Yesterday shortcut chips update the calendar bind immediately.
/// <!-- preview -->
/// ```rust
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// use crate::{today_and_yesterday_shortcuts, DateCalendar, DateCalendarAppearance, DateCalendarBind};
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, ToUnixSeconds};
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// let tz = DatetimeTimezone::Local;
/// let shortcuts = Signal::from(today_and_yesterday_shortcuts(tz));
/// view! {
///     <PickerPreviewExample data_testid="date-pickers-shortcuts-preview">
///         <PickerPreviewKnobs />
///         <DateCalendar
///             bind=value
///             appearance=DateCalendarAppearance {
///                 timezone: Signal::from(tz),
///                 shortcuts,
///                 ..Default::default()
///             }
///         />
///         <div data-testid="date-pickers-shortcuts-value">{move || value.get().map(|v| v.to_unix_seconds().to_string()).unwrap_or_else(|| "none".to_string())}</div>
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-pickers-shortcuts",
    preview_label = "Shortcuts",
    preview_icon = icondata::AiCalendarOutlined,
)]
#[component]
pub fn DatePickersShortcutsGuide() -> impl IntoView {
    view! { () }
}
