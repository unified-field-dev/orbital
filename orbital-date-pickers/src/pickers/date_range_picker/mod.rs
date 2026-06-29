//! [`DateRangePicker`] — range field with popover calendar panel.

mod styles;

use leptos::prelude::*;
use orbital_base_components::OverlayDismiss;
use orbital_core_components::{
    Icon, Popover, PopoverPosition, PopoverSize, PopoverTrigger, PopoverTriggerType,
};
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use crate::building_blocks::{
    DateRangeCalendar, DateRangeCalendarAppearance, DateRangeCalendarBind, DateRangeField,
    DateRangeFieldAppearance, DateRangeFieldBind, DateRangePickerAppearance, DateRangePickerBind,
};
use crate::shared::{
    layout_root_classes, picker_style_sheet, DateRangePickerSlots, OpenTriggerSlot,
    PickerFieldContext, PickerFieldSlot, PickerFieldSlotHost,
};
use styles::date_range_picker_styles;

/// Range field with an anchored dual-month calendar panel, bound to [`DateTimeRange`].
///
/// DateRangePicker composes [`DateRangeField`](crate::DateRangeField) and
/// [`DateRangeCalendar`](crate::DateRangeCalendar) in a click popover. See
/// See the crate README for when to pick range pickers vs fields.
///
/// # When to use
///
/// - Flight or hotel booking flows that need a visible calendar span
/// - Report filters where users pick start and end days in one control
///
/// # Usage
///
/// 1. Wrap the tree in [`DatetimeLocale`](crate::DatetimeLocale) when timezone or format vary.
/// 2. Bind `Option<DateTimeRange>` through [`DateRangePickerBind`].
/// 3. Document the feature with [`DatePickerFeatures::RANGE_PICKERS`] — there is no runtime license check.
///
/// # Best Practices
///
/// ## Do's
///
/// - Wrap in [`Field`](orbital_core_components::Field) with a clear label like "Travel dates".
///
/// ## Don'ts
///
/// - Do not submit until both endpoints are set — `None` means an incomplete range.
///
/// # Examples
///
/// ## Flight booking range
/// Default range field with two-month calendar panel and bind readout.
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use orbital_base_components::ToUnixSeconds;
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="date-range-picker-preview">
///         <PickerPreviewKnobs />
///         <DateRangePicker bind=value />
///         <div data-testid="date-range-picker-preview-VALUE">{move || match value.get() {
///             Some(range) => [range.start.to_unix_seconds().to_string(), range.end.to_unix_seconds().to_string()].join(","),
///             None => "none".to_string(),
///         }}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## Bind readout
/// Completing a calendar range updates the bound value.
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use orbital_base_components::ToUnixSeconds;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="DRP-02">
///         <DateRangePicker bind=value />
///         <div data-testid="DRP-02-VALUE">{move || match value.get() {
///             Some(range) => [range.start.to_unix_seconds().to_string(), range.end.to_unix_seconds().to_string()].join(","),
///             None => "none".to_string(),
///         }}</div>
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-range-picker",
    preview_label = "Date Range Picker",
    preview_icon = icondata::AiCalendarOutlined,
)]
#[component]
pub fn DateRangePicker(
    /// Value binding for the range field and calendar.
    #[prop(optional, into)]
    bind: DateRangePickerBind,
    /// Field format, panel count, and popover options.
    #[prop(optional, into)]
    appearance: DateRangePickerAppearance,
    /// Optional CSS class on the layout wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Optional custom range field slot replacing the default [`DateRangeField`].
    #[prop(optional)]
    picker_field_slot: Option<PickerFieldSlot>,
    /// Optional custom open button slot replacing the calendar icon.
    #[prop(optional)]
    open_trigger_slot: Option<OpenTriggerSlot>,
) -> impl IntoView {
    let DateRangePickerBind { value, id, name } = bind;
    let DateRangePickerAppearance {
        format,
        timezone,
        disabled,
        calendars,
        close_on_select,
        placement,
    } = appearance;

    let theme_options = use_theme_options();
    let value_stored = StoredValue::new(value);

    let slots = DateRangePickerSlots::from_slot_props(picker_field_slot, open_trigger_slot);
    let field_children = StoredValue::new(slots.field.map(|slot| slot.children));
    let open_trigger_children = StoredValue::new(slots.open_trigger.map(|slot| slot.children));

    let field_context = Signal::derive(move || PickerFieldContext {
        value: Signal::derive(move || value_stored.with_value(|v| v.get())),
        disabled,
        format,
        timezone,
    });
    let calendar_appearance = DateRangeCalendarAppearance {
        timezone,
        min_date: Signal::from(None),
        max_date: Signal::from(None),
        disabled,
        calendars,
        day: None,
    };

    let root_class = move || {
        let mut parts = vec![layout_root_classes(theme_options.get().density)];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    };

    view! {
        <style>{date_range_picker_styles()}</style>
        <style>{picker_style_sheet()}</style>
        <div class=root_class data-orbital-picker="">
            <Popover
                trigger_type=PopoverTriggerType::Click
                position=placement_to_popover_position(placement.get_untracked())
                size=Signal::from(PopoverSize::Large)
            >
                <PopoverTrigger slot>
                    <div class="orb-picker-range-picker__trigger">
                        {move || {
                            if let Some(children) = field_children.get_value() {
                                let children = children.clone();
                                view! {
                                    <PickerFieldSlotHost context=field_context children=children />
                                }
                                .into_any()
                            } else {
                                view! {
                                    <DateRangeField
                                        bind=DateRangeFieldBind {
                                            value: value_stored.with_value(|v| v.clone()),
                                            id,
                                            name,
                                        }
                                        appearance=DateRangeFieldAppearance { format, timezone, disabled }
                                    />
                                }
                                .into_any()
                            }
                        }}
                        {move || {
                            if let Some(children) = open_trigger_children.get_value() {
                                let children = children.clone();
                                children().into_any()
                            } else {
                                view! {
                                    <button
                                        type="button"
                                        class="orb-picker-range-picker__open-btn"
                                        aria-label="Open calendar"
                                        disabled=move || disabled.get()
                                    >
                                        <Icon icon=icondata::AiCalendarOutlined />
                                    </button>
                                }
                                .into_any()
                            }
                        }}
                    </div>
                </PopoverTrigger>
                <DateRangePickerPanel
                    value=value_stored
                    appearance=calendar_appearance
                    close_on_select=close_on_select
                />
            </Popover>
        </div>
    }
}

#[component]
fn DateRangePickerPanel(
    value: StoredValue<orbital_base_components::OptionBind<crate::DateTimeRange>>,
    appearance: DateRangeCalendarAppearance,
    close_on_select: Signal<bool>,
) -> impl IntoView {
    let dismiss = use_context::<OverlayDismiss>();
    let prev_complete = RwSignal::new(None::<bool>);

    Effect::new(move |_| {
        if !close_on_select.get() {
            return;
        }
        let complete = value.with_value(|v| v.get()).is_some();
        if complete && prev_complete.get_untracked() == Some(false) {
            if let Some(dismiss) = dismiss {
                dismiss.close.run(());
            }
        }
        prev_complete.set(Some(complete));
    });

    view! {
        <div class="orb-picker-range-picker__panel">
            <DateRangeCalendar
                bind=DateRangeCalendarBind {
                    value: value.with_value(|v| v.clone()),
                }
                appearance=appearance
            />
        </div>
    }
}

fn placement_to_popover_position(placement: orbital_base_components::Placement) -> PopoverPosition {
    use orbital_base_components::Placement;
    match placement {
        Placement::Top => PopoverPosition::Top,
        Placement::Bottom => PopoverPosition::Bottom,
        Placement::Left => PopoverPosition::Left,
        Placement::Right => PopoverPosition::Right,
        Placement::TopStart => PopoverPosition::TopStart,
        Placement::TopEnd => PopoverPosition::TopEnd,
        Placement::LeftStart => PopoverPosition::LeftStart,
        Placement::LeftEnd => PopoverPosition::LeftEnd,
        Placement::RightStart => PopoverPosition::RightStart,
        Placement::RightEnd => PopoverPosition::RightEnd,
        Placement::BottomStart => PopoverPosition::BottomStart,
        Placement::BottomEnd => PopoverPosition::BottomEnd,
    }
}
