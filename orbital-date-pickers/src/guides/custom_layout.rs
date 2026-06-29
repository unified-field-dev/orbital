//! Custom picker layout slots (DP-30).

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Reorder picker panel sections and inject custom action bars with layout slots.
///
/// Popover and inline calendar pickers expose slot components such as
/// [`PickerLayoutActionBarSlot`](crate::PickerLayoutActionBarSlot) so product teams can
/// place shortcuts, calendars, and custom actions in a consistent vertical stack without
/// forking the picker implementation.
///
/// # When to use
///
/// - Calendars that need a clear action below the grid (Clear, Apply, Cancel)
/// - Layouts where shortcut chips must stay above the month grid but actions sit below
/// - Branded picker panels that add secondary controls without replacing core surfaces
///
/// # Usage
///
/// 1. Choose a picker that supports layout slots (for example [`DateCalendar`](crate::DateCalendar)).
/// 2. Pass shortcut presets through `appearance.shortcuts` when needed.
/// 3. Provide child content inside [`PickerLayoutActionBarSlot`] for buttons or links below the grid.
/// 4. Use Orbital [`Button`](orbital_core_components::Button) for actions — not raw HTML buttons.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep action bars to one primary action plus optional secondary actions
/// * Wire clear/reset handlers to the same bind signal the calendar uses
/// * Match shortcut timezone to `appearance.timezone`
///
/// ## Don'ts
///
/// * Do not replace the entire picker when you only need an extra row — use layout slots
/// * Do not stack so many sections that the popover exceeds the viewport on compact density
///
/// # Layout slots reference
///
/// | Slot | Description |
/// |------|-------------|
/// | [`PickerLayoutActionBarSlot`](crate::PickerLayoutActionBarSlot) | Row below the calendar grid for actions |
///
/// # Examples
///
/// ## Action bar below calendar
/// Shortcut presets stay above the grid; a custom action bar clears the selection below.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// use crate::{today_and_yesterday_shortcuts, DateCalendar, DateCalendarAppearance, DateCalendarBind, PickerLayoutActionBarSlot, DateTimeRange};
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime};
/// use orbital_core_components::{Button, ButtonAppearance};
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// let tz = DatetimeTimezone::Local;
/// view! {
///     <PickerPreviewExample data_testid="date-pickers-custom-layout-preview">
///         <PickerPreviewKnobs />
///         <DateCalendar
///             bind=value
///             appearance=DateCalendarAppearance {
///                 timezone: Signal::from(tz),
///                 shortcuts: Signal::from(today_and_yesterday_shortcuts(tz)),
///                 ..Default::default()
///             }
///         >
///             <PickerLayoutActionBarSlot slot>
///                 <span data-testid="date-pickers-custom-layout-clear">
///                     <Button appearance=ButtonAppearance::Secondary on_click=Callback::new(move |_| value.set(None))>"Clear selection"</Button>
///                 </span>
///             </PickerLayoutActionBarSlot>
///         </DateCalendar>
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-pickers-custom-layout",
    preview_label = "Custom Layout",
    preview_icon = icondata::AiLayoutOutlined,
)]
#[component]
pub fn DatePickersCustomLayoutGuide() -> impl IntoView {
    view! { () }
}
