//! Custom open trigger slot (DP-32).

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Replace the default calendar icon trigger with a custom open button.
///
/// Range and date popover pickers render a default icon button to open the panel.
/// Provide [`OpenTriggerSlot`](crate::OpenTriggerSlot) content to swap in a labeled
/// [`Button`](orbital_core_components::Button) or any other focusable control while
/// keeping popover wiring intact.
///
/// # When to use
///
/// - Range pickers that need a text label ("Choose dates") instead of an icon
/// - Forms where the open control must match secondary button styling
/// - Flows that open the calendar from a branded trigger elsewhere in the layout
///
/// # Usage
///
/// 1. Wrap the picker (for example [`DateRangePicker`](crate::DateRangePicker)) with an [`OpenTriggerSlot`] child.
/// 2. Place an Orbital [`Button`](orbital_core_components::Button) inside the slot — the picker attaches open handlers automatically.
/// 3. Use `ButtonAppearance::Secondary` for neutral triggers; reserve primary for form submit actions.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use descriptive button text ("Choose dates", "Select range")
/// * Keep the trigger focusable and visible — do not rely on hover-only affordances
/// * Match button density to surrounding form controls
///
/// ## Don'ts
///
/// * Do not use raw `<button>` elements — use Orbital [`Button`] for consistent focus rings and density
/// * Do not nest another popover trigger inside the slot without testing focus return
///
/// # Examples
///
/// ## Text open button
/// Replace the default calendar icon with a labeled secondary button.
/// <!-- preview -->
/// ```rust
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// use crate::{DateRangePicker, DateTimeRange, OpenTriggerSlot};
/// use orbital_core_components::{Button, ButtonAppearance};
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="date-pickers-custom-open-preview">
///         <PickerPreviewKnobs />
///         <DateRangePicker bind=value>
///             <OpenTriggerSlot slot>
///                 <Button
///                     appearance=ButtonAppearance::Secondary
///                     attr:data-testid="date-pickers-custom-open-button"
///                 >
///                     "Choose dates"
///                 </Button>
///             </OpenTriggerSlot>
///         </DateRangePicker>
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-pickers-custom-open",
    preview_label = "Custom Open Button",
    preview_icon = icondata::AiCalendarOutlined,
)]
#[component]
pub fn DatePickersCustomOpenGuide() -> impl IntoView {
    view! { () }
}
