//! [`DateRangeCalendar`] — two-month range selection panel.

mod styles;

use chrono::{Datelike, Months, NaiveDate};
use leptos::prelude::*;
use orbital_base_components::{
    build_month_grid, format_unix, DatetimeFormat, DatetimeTimezone, GridDayKind,
};
use orbital_core_components::{
    calendar_styles, Button, ButtonAppearance, ButtonGroup, CalendarDayProps, CalendarDayRenderer,
    CalendarWeekdayHeader,
};
use orbital_macros::component_doc;
use orbital_theme::use_theme_options;

use crate::shared::{
    apply_range_click, day_range_role, is_day_disabled, layout_root_classes, picker_style_sheet,
    range_to_selection, selection_to_range, today_for_timezone, DayRangeRole, RangeSelection,
};

use super::field_types::{DateRangeCalendarAppearance, DateRangeCalendarBind};
use styles::date_range_calendar_styles;

use crate::DateTimeRange;

/// Inline dual-month calendar for selecting a date range bound to [`DateTimeRange`].
///
/// DateRangeCalendar renders one or two month grids with range highlighting, hover preview,
/// and click-to-complete selection. Range pickers compose this panel behind a field trigger.
/// Document range surfaces with [`DatePickerFeatures::RANGE_PICKERS`] — there is no runtime
/// license check.
///
/// # When to use
///
/// - Popover or inline panels for booking and reporting date ranges
/// - Building blocks inside [`DateRangePicker`](crate::DateRangePicker)
///
/// # Usage
///
/// 1. Bind `Option<DateTimeRange>` via [`DateRangeCalendarBind`].
/// 2. Set `appearance.calendars` to `1` or `2` month panels.
/// 3. Wrap preview examples in a native element with `data-testid`.
///
/// # Best Practices
///
/// ## Do's
///
/// * Enable range pickers in product docs with [`DatePickerFeatures::RANGE_PICKERS`]
/// * Wrap previews with `data-testid` on a native element
///
/// ## Don'ts
///
/// * Do not use for single-day selection — prefer [`DateCalendar`](crate::DateCalendar)
/// * Do not put `data-testid` on the component itself
///
/// # Examples
///
/// ## Dual month range
/// Two month panels with bind readout for E2E.
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use orbital_base_components::ToUnixSeconds;
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="date-range-calendar-preview">
///         <PickerPreviewKnobs />
///         <DateRangeCalendar bind=value />
///         <div data-testid="date-range-calendar-preview-VALUE">{move || match value.get() {
///             Some(range) => [range.start.to_unix_seconds().to_string(), range.end.to_unix_seconds().to_string()].join(","),
///             None => "none".to_string(),
///         }}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## Bind readout
/// Completing a two-click range updates the bound [`DateTimeRange`].
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use orbital_base_components::ToUnixSeconds;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="DRC-02">
///         <DateRangeCalendar bind=value />
///         <div data-testid="DRC-02-VALUE">{move || match value.get() {
///             Some(range) => [range.start.to_unix_seconds().to_string(), range.end.to_unix_seconds().to_string()].join(","),
///             None => "none".to_string(),
///         }}</div>
///     </PickerPreviewExample>
/// }
/// ```
///
/// ## Single month panel
/// Compact layouts can show one calendar month.
/// <!-- preview -->
/// ```rust
/// use crate::DateTimeRange;
/// use crate::preview::PickerPreviewExample;
/// let value = RwSignal::new(None::<DateTimeRange>);
/// view! {
///     <PickerPreviewExample data_testid="DRC-03">
///         <DateRangeCalendar bind=value appearance=DateRangeCalendarAppearance { calendars: Signal::from(1), ..Default::default() } />
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-range-calendar",
    preview_label = "Date Range Calendar",
    preview_icon = icondata::AiCalendarOutlined,
)]
#[component]
pub fn DateRangeCalendar(
    /// Selected range binding.
    #[prop(optional, into)]
    bind: DateRangeCalendarBind,
    /// Timezone, constraints, and panel count.
    #[prop(optional, into)]
    appearance: DateRangeCalendarAppearance,
    /// Optional CSS class merged onto the layout root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let DateRangeCalendarBind { value } = bind;
    let DateRangeCalendarAppearance {
        timezone,
        min_date,
        max_date,
        disabled,
        calendars,
        day,
    } = appearance;

    let theme_options = use_theme_options();
    let value = StoredValue::new(value);
    let selection = RwSignal::new(RangeSelection::default());
    let hover_date = RwSignal::new(None::<NaiveDate>);
    let anchor_month = RwSignal::new(today_for_timezone(DatetimeTimezone::Local));
    let last_external = RwSignal::new(None::<Option<DateTimeRange>>);

    Effect::new(move |_| {
        let current = value.with_value(|v| v.get());
        if last_external.get_untracked() == Some(current.clone()) {
            return;
        }
        last_external.set(Some(current.clone()));
        selection.set(range_to_selection(current));
        if let Some(start) = selection.get_untracked().start {
            anchor_month.set(start);
        }
    });

    let root_class = move || {
        let mut parts = vec![layout_root_classes(theme_options.get().density)];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    };

    let previous_month = move |_| {
        anchor_month.update(|date| {
            if let Some(prev) = date.checked_sub_months(Months::new(1)) {
                *date = prev;
            }
        });
    };

    let next_month = move |_| {
        anchor_month.update(|date| {
            if let Some(next) = date.checked_add_months(Months::new(1)) {
                *date = next;
            }
        });
    };

    view! {
        <style>{date_range_calendar_styles()}</style>
        <style>{calendar_styles()}</style>
        <style>{picker_style_sheet()}</style>
        <div class=root_class data-orbital-picker="">
            <div class="orb-picker-range-calendar">
                <div class="orb-picker-range-calendar__nav">
                    <ButtonGroup>
                        <Button
                            appearance=ButtonAppearance::Secondary
                            on_click=Callback::new(previous_month)
                        >
                            "Previous"
                        </Button>
                        <Button
                            appearance=ButtonAppearance::Secondary
                            on_click=Callback::new(next_month)
                        >
                            "Next"
                        </Button>
                    </ButtonGroup>
                </div>
                {move || {
                    let panel_count = calendars.get().clamp(1, 2) as usize;
                    let anchor = anchor_month.get();
                    (0..panel_count)
                        .map(|offset| {
                            let month_date = anchor
                                .checked_add_months(Months::new(offset as u32))
                                .unwrap_or(anchor);
                            view! {
                                <RangeMonthPanel
                                    year=month_date.year()
                                    month=month_date.month()
                                    selection=selection
                                    hover_date=hover_date
                                    timezone=timezone
                                    min_date=min_date
                                    max_date=max_date
                                    disabled=disabled
                                    value=value
                                    day=day.clone()
                                />
                            }
                        })
                        .collect_view()
                }}
            </div>
        </div>
    }
}

#[component]
fn RangeMonthPanel(
    year: i32,
    month: u32,
    selection: RwSignal<RangeSelection>,
    hover_date: RwSignal<Option<NaiveDate>>,
    timezone: Signal<DatetimeTimezone>,
    min_date: Signal<Option<orbital_base_components::OrbitalDateTime>>,
    max_date: Signal<Option<orbital_base_components::OrbitalDateTime>>,
    disabled: Signal<bool>,
    value: StoredValue<orbital_base_components::OptionBind<DateTimeRange>>,
    day: Option<CalendarDayRenderer>,
) -> impl IntoView {
    let month_grid = Memo::new(move |_| build_month_grid(year, month));
    let day_renderer = StoredValue::new(day);
    let focused_date = RwSignal::new(
        NaiveDate::from_ymd_opt(year, month, 1)
            .unwrap_or_else(|| today_for_timezone(DatetimeTimezone::Local)),
    );

    view! {
        <div class="orb-picker-range-calendar__panel">
            <div class="orbital-calendar">
                <div class="orbital-calendar__header">
                    <span class="orbital-calendar__header-title">
                        {NaiveDate::from_ymd_opt(year, month, 1)
                            .map(|d| d.format("%B %Y").to_string())
                            .unwrap_or_default()}
                    </span>
                </div>
                <CalendarWeekdayHeader />
                <div
                    class="orbital-calendar__dates"
                    role="grid"
                    aria-label="Date range calendar"
                    tabindex="-1"
                >
                    {move || {
                        let tz = timezone.get();
                        let min = min_date.get();
                        let max = max_date.get();
                        let is_disabled_panel = disabled.get();
                        let current_selection = selection.get();
                        let hover = hover_date.get();
                        let today = today_for_timezone(tz);
                        let renderer = day_renderer.get_value();
                        let focus = focused_date.get();
                        month_grid
                            .get()
                            .into_iter()

                            .map(|day_cell| {
                                let is_other_month = day_cell.kind != GridDayKind::Current;
                                let is_today = day_cell.date == today;
                                let is_disabled =
                                    is_disabled_panel || is_day_disabled(day_cell.date, min, max);
                                let role =
                                    day_range_role(day_cell.date, current_selection, hover);
                                let is_selected = matches!(
                                    role,
                                    DayRangeRole::RangeStart
                                        | DayRangeRole::RangeEnd
                                        | DayRangeRole::InRange
                                );
                                let day_label = format_unix(
                                    day_cell.unix_secs,
                                    DatetimeFormat::IsoDate,
                                    DatetimeTimezone::Utc,
                                );
                                let date = day_cell.date;
                                let on_select = Callback::new(move |_| {
                                    if is_disabled {
                                        return;
                                    }
                                    let next = apply_range_click(selection.get_untracked(), date);
                                    selection.set(next);
                                    focused_date.set(date);
                                    if next.is_complete() {
                                        if let Some(range) =
                                            selection_to_range(next, timezone.get_untracked())
                                        {
                                            value.with_value(|v| v.set(Some(range)));
                                        }
                                    } else {
                                        value.with_value(|v| v.set(None));
                                    }
                                });
                                let on_focus = Callback::new(move |_| focused_date.set(date));
                                let on_mouse_enter = {
                                    let date = day_cell.date;
                                    move |_| {
                                        if is_disabled {
                                            return;
                                        }
                                        hover_date.set(Some(date));
                                    }
                                };
                                let tabindex = if is_disabled {
                                    "-1"
                                } else if focus == day_cell.date {
                                    "0"
                                } else {
                                    "-1"
                                };
                                let props = CalendarDayProps {
                                    date,
                                    day_number: day_cell.date.day(),
                                    weekday_label: None,
                                    selected: is_selected,
                                    today: is_today,
                                    other_month: is_other_month,
                                    disabled: is_disabled,
                                    aria_label: day_label,
                                    tabindex,
                                    on_select,
                                    on_focus,
                                };

                                if let Some(render) = renderer.clone() {
                                    render(props).into_any()
                                } else {
                                    view! {
                                        <div
                                            class="orbital-calendar-item"
                                            class=("orbital-calendar-item--other-month", is_other_month)
                                            class=("orbital-calendar-item--today", is_today)
                                            class=("orbital-calendar-item--disabled", is_disabled)
                                            class=("orbital-calendar-item--range-start", role == DayRangeRole::RangeStart)
                                            class=("orbital-calendar-item--range-end", role == DayRangeRole::RangeEnd)
                                            class=("orbital-calendar-item--in-range", role == DayRangeRole::InRange)
                                            class=("orbital-calendar-item--preview-range", role == DayRangeRole::PreviewRange)
                                            role="gridcell"
                                            tabindex=tabindex
                                            aria-label=props.aria_label.clone()
                                            aria-selected=is_selected
                                            aria-disabled=is_disabled
                                            on:click=move |_| props.on_select.run(())
                                            on:focus=move |_| props.on_focus.run(())
                                            on:mouseenter=on_mouse_enter
                                        >
                                            <div class="orbital-calendar-item__header">
                                                <span class="orbital-calendar-item__header-day">
                                                    {day_cell.date.day()}
                                                </span>
                                            </div>
                                            <div class="orbital-calendar-item__bar"></div>
                                        </div>
                                    }
                                    .into_any()
                                }
                            })
                            .collect_view()
                    }}
                </div>
            </div>
        </div>
    }
}
