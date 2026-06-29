use chrono::{Datelike, Months};
use leptos::prelude::*;
use std::sync::Arc;

use crate::form::{
    build_month_grid, is_day_disabled, DatetimeTimezone, OptionBind, OrbitalDateTime,
    PickerShortcut, PickerShortcutsBar,
};
use crate::overlay::{AnchoredPanel, AnchoredPositioner, Placement};

use super::month::month_short_name;
use super::panel::{
    date_from_unix, month_heading, start_of_day_unix, today_for_timezone, DatePickerPanelMode,
};
use super::year::{build_years, year_panel_start};

const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

/// Props for a custom month button in [`BaseDatePicker`]'s month panel.
#[derive(Clone)]
pub struct MonthButtonRenderProps {
    pub month: u32,
    pub label: String,
    pub selected: bool,
    pub on_select: Callback<()>,
}

pub type MonthButtonRenderer = Arc<dyn Fn(MonthButtonRenderProps) -> AnyView + Send + Sync>;

fn default_month_button(props: MonthButtonRenderProps) -> impl IntoView {
    view! {
        <button
            type="button"
            class="orbital-date-picker__month-button"
            class=("orbital-date-picker__month-button--selected", props.selected)
            on:click=move |_| props.on_select.run(())
        >
            {props.label}
        </button>
    }
}

/// Headless anchored date picker panel and trigger wrapper.
#[component]
pub fn BaseDatePicker(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] panel_class: MaybeProp<String>,
    #[prop(optional, into)] value: OptionBind<i64>,
    #[prop(into)] timezone: Signal<DatetimeTimezone>,
    #[prop(optional, into)] min_date: Signal<Option<OrbitalDateTime>>,
    #[prop(optional, into)] max_date: Signal<Option<OrbitalDateTime>>,
    #[prop(optional, into)] shortcuts: Signal<Vec<PickerShortcut>>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] placement: Signal<Placement>,
    #[prop(optional)] on_select: Option<Callback<i64>>,
    #[prop(optional)] on_shortcut: Option<Callback<OrbitalDateTime>>,
    #[prop(default = None)] month_button: Option<MonthButtonRenderer>,
    children: Children,
) -> impl IntoView {
    let value = StoredValue::new(value);
    let is_open = RwSignal::new(false);
    let panel_mode = RwSignal::new(DatePickerPanelMode::Date);
    let month_button = StoredValue::new(month_button);
    let focused_date = RwSignal::new(
        value
            .get_value()
            .get_untracked()
            .and_then(|secs| date_from_unix(secs, timezone.get_untracked()))
            .unwrap_or_else(|| today_for_timezone(timezone.get_untracked())),
    );

    Effect::new(move |_| {
        if let Some(selected) = value.get_value().get() {
            if let Some(date) = date_from_unix(selected, timezone.get()) {
                focused_date.set(date);
            }
        }
    });

    let panel_classes = Signal::derive(move || {
        let mut parts = vec!["orbital-date-picker__panel".to_string()];
        if let Some(extra) = panel_class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    let open_panel = move |_| {
        if disabled.get() {
            return;
        }
        is_open.set(true);
    };

    view! {
        <AnchoredPositioner panel=AnchoredPanel {
            show: is_open.read_only().into(),
            width: None,
            placement: placement.get_untracked(),
            auto_height: false,
            arrow: None,
            motion: None,
            children: ToChildren::to_children(move || {
                view! {
                    <div class=panel_classes data-testid="date-picker-panel">
                        <div class="orbital-date-picker__panel-header">
                            <button
                                type="button"
                                class="orbital-date-picker__nav-button"
                                on:click=move |_| {
                                    match panel_mode.get() {
                                        DatePickerPanelMode::Date => {
                                            focused_date.update(|date| {
                                                if let Some(previous) = date
                                                    .checked_sub_months(Months::new(1))
                                                {
                                                    *date = previous;
                                                }
                                            });
                                        }
                                        DatePickerPanelMode::Month => {
                                            focused_date
                                                .update(|date| *date = *date - chrono::Days::new(365));
                                        }
                                        DatePickerPanelMode::Year => {
                                            focused_date
                                                .update(|date| *date = *date - chrono::Days::new(3650));
                                        }
                                    }
                                }
                                aria-label="Previous"
                            >
                                "Prev"
                            </button>
                            <button
                                type="button"
                                class="orbital-date-picker__title-button"
                                on:click=move |_| {
                                    panel_mode.set(match panel_mode.get() {
                                        DatePickerPanelMode::Date => DatePickerPanelMode::Month,
                                        DatePickerPanelMode::Month => DatePickerPanelMode::Year,
                                        DatePickerPanelMode::Year => DatePickerPanelMode::Date,
                                    });
                                }
                            >
                                {move || {
                                    let focused = focused_date.get();
                                    match panel_mode.get() {
                                        DatePickerPanelMode::Date => {
                                            month_heading(focused.year(), focused.month())
                                        }
                                        DatePickerPanelMode::Month => focused.year().to_string(),
                                        DatePickerPanelMode::Year => {
                                            let start = year_panel_start(focused.year());
                                            format!("{start} - {}", start + 11)
                                        }
                                    }
                                }}
                            </button>
                            <button
                                type="button"
                                class="orbital-date-picker__nav-button"
                                on:click=move |_| {
                                    match panel_mode.get() {
                                        DatePickerPanelMode::Date => {
                                            focused_date.update(|date| {
                                                if let Some(next) = date.checked_add_months(Months::new(1)) {
                                                    *date = next;
                                                }
                                            });
                                        }
                                        DatePickerPanelMode::Month => {
                                            focused_date
                                                .update(|date| *date = *date + chrono::Days::new(365));
                                        }
                                        DatePickerPanelMode::Year => {
                                            focused_date
                                                .update(|date| *date = *date + chrono::Days::new(3650));
                                        }
                                    }
                                }
                                aria-label="Next"
                            >
                                "Next"
                            </button>
                        </div>
                        <div class="orbital-date-picker__panel-body">
                            {move || {
                                let tz = timezone.get();
                                let focused = focused_date.get();
                                let selected = value
                                    .get_value()
                                    .get()
                                    .and_then(|secs| date_from_unix(secs, tz));

                                match panel_mode.get() {
                                    DatePickerPanelMode::Date => {
                                        let today = today_for_timezone(tz);
                                        let min = min_date.get();
                                        let max = max_date.get();
                                        let days = build_month_grid(focused.year(), focused.month());
                                        view! {
                                            <div class="orbital-date-picker__weekday-row">
                                                {WEEKDAYS
                                                    .into_iter()
                                                    .map(|name| view! { <span>{name}</span> })
                                                    .collect_view()}
                                            </div>
                                            <div class="orbital-date-picker__date-grid">
                                                {days
                                                    .into_iter()
                                                    .map(|day| {
                                                        let is_current_month =
                                                            day.kind == crate::form::GridDayKind::Current;
                                                        let is_selected = selected
                                                            .map(|selected_date| selected_date == day.date)
                                                            .unwrap_or(false);
                                                        let is_today = day.date == today;
                                                        let is_disabled = is_day_disabled(
                                                            day.date,
                                                            min,
                                                            max,
                                                        );
                                                        let on_click = {
                                                            let date = day.date;
                                                            let on_select = on_select;
                                                            move |_| {
                                                                if disabled.get_untracked()
                                                                    || is_disabled
                                                                {
                                                                    return;
                                                                }
                                                                focused_date.set(date);
                                                                if let Some(unix_secs) =
                                                                    start_of_day_unix(date, tz)
                                                                {
                                                                    value
                                                                        .with_value(|v| v.set(Some(unix_secs)));
                                                                    if let Some(on_select) = on_select
                                                                    {
                                                                        on_select.run(unix_secs);
                                                                    }
                                                                }
                                                                is_open.set(false);
                                                            }
                                                        };
                                                        view! {
                                                            <button
                                                                type="button"
                                                                class="orbital-date-picker__day-button"
                                                                class=(
                                                                    "orbital-date-picker__day-button--outside",
                                                                    !is_current_month,
                                                                )
                                                                class=(
                                                                    "orbital-date-picker__day-button--selected",
                                                                    is_selected,
                                                                )
                                                                class=(
                                                                    "orbital-date-picker__day-button--today",
                                                                    is_today,
                                                                )
                                                                class=(
                                                                    "orbital-date-picker__day-button--disabled",
                                                                    is_disabled,
                                                                )
                                                                disabled=is_disabled
                                                                on:click=on_click
                                                            >
                                                                {day.date.day().to_string()}
                                                            </button>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </div>
                                            {move || {
                                                if shortcuts.get().is_empty() {
                                                    ().into_any()
                                                } else {
                                                    let on_shortcut_cb = Callback::new(
                                                        move |dt: OrbitalDateTime| {
                                                            if disabled.get_untracked() {
                                                                return;
                                                            }
                                                            let tz = timezone.get_untracked();
                                                            if let Some(date) = dt.wall_date() {
                                                                focused_date.set(date);
                                                                if let Some(unix_secs) =
                                                                    start_of_day_unix(date, tz)
                                                                {
                                                                    value.with_value(|v| {
                                                                        v.set(Some(unix_secs))
                                                                    });
                                                                    if let Some(on_select) =
                                                                        on_select
                                                                    {
                                                                        on_select.run(unix_secs);
                                                                    }
                                                                    if let Some(on_shortcut) =
                                                                        on_shortcut
                                                                    {
                                                                        on_shortcut.run(dt);
                                                                    }
                                                                }
                                                            }
                                                            is_open.set(false);
                                                        },
                                                    );
                                                    view! {
                                                        <PickerShortcutsBar
                                                            shortcuts=shortcuts
                                                            disabled=disabled
                                                            on_select=on_shortcut_cb
                                                        />
                                                    }
                                                    .into_any()
                                                }
                                            }}
                                        }
                                            .into_any()
                                    }
                                    DatePickerPanelMode::Month => view! {
                                        <div class="orbital-date-picker__month-grid">
                                            {(1_u32..=12)
                                                .map(|month| {
                                                    let is_selected = focused.month() == month;
                                                    let label = month_short_name(month).to_string();
                                                    let on_select = Callback::new(move |_| {
                                                        focused_date.update(|date| {
                                                            if let Some(next) = date.with_month(month) {
                                                                *date = next;
                                                            }
                                                        });
                                                        panel_mode.set(DatePickerPanelMode::Date);
                                                    });
                                                    let props = MonthButtonRenderProps {
                                                        month,
                                                        label,
                                                        selected: is_selected,
                                                        on_select,
                                                    };
                                                    if let Some(renderer) = month_button.get_value().clone() {
                                                        renderer(props).into_any()
                                                    } else {
                                                        default_month_button(props).into_any()
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                    }
                                        .into_any(),
                                    DatePickerPanelMode::Year => {
                                        let start = year_panel_start(focused.year());
                                        let years = build_years(start);
                                        view! {
                                            <div class="orbital-date-picker__year-grid">
                                                {years
                                                    .into_iter()
                                                    .map(|year| {
                                                        let is_selected = focused.year() == year;
                                                        view! {
                                                            <button
                                                                type="button"
                                                                class="orbital-date-picker__year-button"
                                                                class=(
                                                                    "orbital-date-picker__year-button--selected",
                                                                    is_selected,
                                                                )
                                                                on:click=move |_| {
                                                                    focused_date.update(|date| {
                                                                        if let Some(next) = date
                                                                            .with_year(year)
                                                                        {
                                                                            *date = next;
                                                                        }
                                                                    });
                                                                    panel_mode.set(DatePickerPanelMode::Month);
                                                                }
                                                            >
                                                                {year.to_string()}
                                                            </button>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </div>
                                        }
                                            .into_any()
                                    }
                                }
                            }}
                        </div>
                    </div>
                }
            }),
        }>
            <div
                class=move || {
                    let mut parts = vec!["orbital-date-picker".to_string()];
                    if let Some(extra) = class.get() {
                        if !extra.is_empty() {
                            parts.push(extra);
                        }
                    }
                    parts.join(" ")
                }
                on:click=open_panel
            >
                {children()}
            </div>
        </AnchoredPositioner>
    }
}
