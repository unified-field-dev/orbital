//! Customization slots and picker field context for plugin pickers.

use std::sync::Arc;

use leptos::prelude::*;
use orbital_base_components::{DatetimeFormat, DatetimeTimezone};

use crate::DateTimeRange;

/// Context for custom field slot content inside a popover picker trigger row.
#[derive(Clone, Copy)]
pub struct PickerFieldContext {
    pub value: Signal<Option<DateTimeRange>>,
    pub disabled: Signal<bool>,
    pub format: Signal<DatetimeFormat>,
    pub timezone: Signal<DatetimeTimezone>,
}

#[derive(Clone, Copy)]
struct PickerFieldInjection(Signal<PickerFieldContext>);

/// Returns the active [`PickerFieldContext`] when inside a [`PickerFieldSlot`] on a range picker.
pub fn use_picker_field() -> PickerFieldContext {
    use_context::<PickerFieldInjection>()
        .expect("use_picker_field requires PickerFieldSlot on DateRangePicker")
        .0
        .get()
}

pub(crate) fn provide_picker_field(context: Signal<PickerFieldContext>) {
    provide_context(PickerFieldInjection(context));
}

/// Hosts custom picker field slot content with [`PickerFieldContext`].
#[component]
pub fn PickerFieldSlotHost(
    context: Signal<PickerFieldContext>,
    children: ChildrenFn,
) -> impl IntoView {
    provide_picker_field(context);
    children()
}

/// Collected layout slot children for [`crate::DateCalendar`].
#[derive(Default)]
pub struct PickerLayoutSlots {
    pub toolbar: Option<PickerLayoutToolbarSlot>,
    pub action_bar: Option<PickerLayoutActionBarSlot>,
}

impl PickerLayoutSlots {
    pub fn from_slot_props(
        toolbar: Option<PickerLayoutToolbarSlot>,
        action_bar: Option<PickerLayoutActionBarSlot>,
    ) -> Self {
        Self {
            toolbar,
            action_bar,
        }
    }
}

/// Collected trigger slots for [`crate::DateRangePicker`].
#[derive(Default)]
pub struct DateRangePickerSlots {
    pub field: Option<PickerFieldSlot>,
    pub open_trigger: Option<OpenTriggerSlot>,
}

impl DateRangePickerSlots {
    pub fn from_slot_props(
        field: Option<PickerFieldSlot>,
        open_trigger: Option<OpenTriggerSlot>,
    ) -> Self {
        Self {
            field,
            open_trigger,
        }
    }
}

/// Optional toolbar region above shortcuts on [`crate::DateCalendar`].
#[slot]
pub struct PickerLayoutToolbarSlot {
    pub(crate) children: ChildrenFn,
}

/// Optional action bar region below the calendar grid on [`crate::DateCalendar`].
#[slot]
pub struct PickerLayoutActionBarSlot {
    pub(crate) children: ChildrenFn,
}

/// Replaces the default range field in [`crate::DateRangePicker`]'s trigger row.
///
/// Use [`use_picker_field`] inside the slot to read bind state.
#[slot]
pub struct PickerFieldSlot {
    pub(crate) children: ChildrenFn,
}

/// Replaces the default calendar icon open button on [`crate::DateRangePicker`].
#[slot]
pub struct OpenTriggerSlot {
    pub(crate) children: ChildrenFn,
}

/// Type alias for custom day cell renderers forwarded to core [`orbital_core_components::Calendar`].
pub type CalendarDayRenderer =
    Arc<dyn Fn(orbital_core_components::CalendarDayProps) -> AnyView + Send + Sync>;

/// Type alias for custom month button renderers on core [`orbital_core_components::DatePicker`].
pub type CalendarMonthButtonRenderer =
    Arc<dyn Fn(orbital_core_components::CalendarMonthButtonProps) -> AnyView + Send + Sync>;
