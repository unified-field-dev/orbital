//! [`DateTimeRangePicker`] — datetime range field with dual datetime picker panels.

use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use crate::building_blocks::{
    DateTimeRangeField, DateTimeRangeFieldAppearance, DateTimeRangeFieldBind,
    DateTimeRangePickerAppearance, DateTimeRangePickerBind,
};
use crate::pickers::DateTimePicker;
use crate::shared::{
    datetime_range_picker_row_class, layout_root_classes, picker_style_sheet, use_range_coordinator,
};
use crate::{DateTimePickerAppearance, DateTimePickerBind};

/// Datetime range field with side-by-side start/end [`DateTimePicker`](crate::DateTimePicker)
/// panels, bound to [`DateTimeRange`].
///
/// See the crate README for range control selection.
///
/// # When to use
///
/// - Meeting slot booking with full date and time on both endpoints
/// - Maintenance windows that span partial days
///
/// # Usage
///
/// 1. Wrap in [`DatetimeLocale`](crate::DatetimeLocale) for timezone and format defaults.
/// 2. Bind `Option<DateTimeRange>` through [`DateTimeRangePickerBind`].
/// 3. Tune [`DateTimeRangePickerAppearance`] for date masks and 12/24-hour time columns.
///
/// # Best Practices
///
/// ## Do's
///
/// - Validate `range.end` is after `range.start` before submit — the picker allows partial entry while editing.
///
/// ## Don'ts
///
/// - Do not store unix seconds in the bind — use [`DateTimeRange`] with [`OrbitalDateTime`] endpoints.
///
/// # Examples
///
/// ## Meeting slot range
/// Default US date + 12-hour time range with bind readout.
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use orbital_base_components::ToUnixSeconds;
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="date-time-range-picker-preview">
///         <PickerPreviewKnobs />
///         <DateTimeRangePicker bind=value />
///         <div data-testid="date-time-range-picker-preview-VALUE">{move || match value.get() {
///             Some(range) => [range.start.to_unix_seconds().to_string(), range.end.to_unix_seconds().to_string()].join(","),
///             None => "none".to_string(),
///         }}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## ISO date and 24-hour time
/// Combined pickers with ISO date masks and 24-hour time columns.
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="DTRP-02">
///         <DateTimeRangePicker bind=value appearance=DateTimeRangePickerAppearance::iso_time24() />
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-time-range-picker",
    preview_label = "DateTime Range Picker",
    preview_icon = icondata::AiCalendarOutlined,
)]
#[component]
pub fn DateTimeRangePicker(
    /// Value binding for the combined datetime range pickers.
    #[prop(optional, into)]
    bind: DateTimeRangePickerBind,
    /// Date format, time format, timezone, and disabled state.
    #[prop(optional, into)]
    appearance: DateTimeRangePickerAppearance,
    /// Optional CSS class on the layout wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let DateTimeRangePickerBind { value, id, name } = bind;
    let DateTimeRangePickerAppearance {
        date_format,
        time_format,
        timezone,
        disabled,
    } = appearance;

    let theme_options = use_theme_options();
    let value_stored = StoredValue::new(value);
    let coordinator = use_range_coordinator(value_stored.with_value(|v| v.clone()));

    let field_bind = DateTimeRangeFieldBind {
        value: value_stored.with_value(|v| v.clone()),
        id,
        name,
    };
    let field_appearance = DateTimeRangeFieldAppearance {
        date_format,
        time_format,
        timezone,
        disabled,
    };

    let picker_appearance_start = DateTimePickerAppearance {
        date_format,
        time_format,
        timezone,
        disabled,
        ..Default::default()
    };
    let picker_appearance_end = DateTimePickerAppearance {
        date_format,
        time_format,
        timezone,
        disabled,
        ..Default::default()
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
        <style>{picker_style_sheet()}</style>
        <div class=root_class data-orbital-picker="">
            <DateTimeRangeField bind=field_bind appearance=field_appearance />
            <div class=datetime_range_picker_row_class()>
                <DateTimePicker
                    bind=DateTimePickerBind { value: coordinator.start.into(), id, name }
                    appearance=picker_appearance_start
                />
                <DateTimePicker
                    bind=DateTimePickerBind { value: coordinator.end.into(), ..Default::default() }
                    appearance=picker_appearance_end
                />
            </div>
        </div>
    }
}
