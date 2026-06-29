use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike};
use leptos::{children::ToChildren, ev, prelude::*};

use crate::{
    form::{DatetimeFormat, FieldInjection, OptionBind},
    overlay::{positioning::AnchorWidth, AnchoredPanel, AnchoredPositioner, Placement},
};

pub fn format_time_value(unix_secs: i64, format: DatetimeFormat) -> String {
    local_from_unix(unix_secs)
        .map(|dt| dt.format(pattern_for(format)).to_string())
        .unwrap_or_default()
}

pub fn parse_time_input(input: &str, format: DatetimeFormat) -> Option<NaiveTime> {
    NaiveTime::parse_from_str(input.trim(), pattern_for(format)).ok()
}

pub fn to_panel_time(unix_secs: i64) -> Option<NaiveTime> {
    local_from_unix(unix_secs).map(|dt| dt.time())
}

pub fn now_time() -> NaiveTime {
    Local::now().time()
}

pub fn normalize_reference_date(reference_date: Option<i64>) -> NaiveDate {
    reference_date
        .and_then(date_from_unix)
        .unwrap_or_else(|| Local::now().date_naive())
}

pub fn compose_time_unix(
    time: NaiveTime,
    current_value: Option<i64>,
    reference_date: Option<i64>,
) -> Option<i64> {
    let base_date = current_value
        .and_then(date_from_unix)
        .unwrap_or_else(|| normalize_reference_date(reference_date));
    let naive = NaiveDateTime::new(base_date, time);
    Local
        .from_local_datetime(&naive)
        .single()
        .map(|dt| dt.timestamp())
}

fn local_from_unix(secs: i64) -> Option<DateTime<Local>> {
    Local.timestamp_opt(secs, 0).single()
}

fn date_from_unix(secs: i64) -> Option<NaiveDate> {
    local_from_unix(secs).map(|dt| dt.date_naive())
}

fn pattern_for(format: DatetimeFormat) -> &'static str {
    match format {
        DatetimeFormat::Time24 => "%H:%M:%S",
        DatetimeFormat::Time12 => "%I:%M:%S %p",
        DatetimeFormat::IsoDate => "%H:%M:%S",
        DatetimeFormat::UsDate => "%H:%M:%S",
    }
}

/// Headless time picker with anchored scroll columns.
#[component]
pub fn BaseTimePicker(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: OptionBind<i64>,
    #[prop(optional, into, default = Signal::from(DatetimeFormat::Time12))] format: Signal<
        DatetimeFormat,
    >,
    #[prop(optional, into)] reference_date: Signal<Option<i64>>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let format = StoredValue::new(normalize_time_format(format.get_untracked()));
    let value = StoredValue::new(value);
    let reference_date = StoredValue::new(reference_date);
    let open = RwSignal::new(false);
    let draft = RwSignal::new(
        value
            .get_value()
            .get_untracked()
            .and_then(to_panel_time)
            .unwrap_or_else(now_time),
    );

    Effect::new(move |_| {
        if open.get() {
            return;
        }
        if let Some(secs) = value.get_value().get() {
            if let Some(time) = to_panel_time(secs) {
                draft.set(time);
            }
        }
    });

    let show_label = Signal::derive(move || {
        value
            .get_value()
            .get()
            .map(|secs| format_time_value(secs, format.get_value()))
            .unwrap_or_else(|| "--:--:--".to_string())
    });

    let open_panel = move |_| {
        if disabled.get_untracked() {
            return;
        }
        let next = value
            .get_value()
            .get_untracked()
            .and_then(to_panel_time)
            .unwrap_or_else(now_time);
        draft.set(next);
        open.set(true);
    };

    let set_now = move |_| draft.set(now_time());
    let commit = move |_| {
        let Some(draft_time) = draft.try_get_untracked() else {
            return;
        };
        if let Some(unix_secs) = compose_time_unix(
            draft_time,
            value.get_value().get_untracked(),
            reference_date.get_value().get_untracked(),
        ) {
            value.get_value().set(Some(unix_secs));
        }
        open.set(false);
    };

    view! {
        <AnchoredPositioner panel=AnchoredPanel {
            show: open.read_only().into(),
            width: Some(AnchorWidth::Target),
            placement: Placement::BottomStart,
            auto_height: false,
            arrow: None,
            motion: None,
            children: ToChildren::to_children(move || {
                let hour =
                    Signal::derive(move || draft.try_get().map(|time| time.hour()).unwrap_or(0));
                let minute =
                    Signal::derive(move || draft.try_get().map(|time| time.minute()).unwrap_or(0));
                let second =
                    Signal::derive(move || draft.try_get().map(|time| time.second()).unwrap_or(0));
                let is_12 = format.get_value() == DatetimeFormat::Time12;
                let display_hour = Signal::derive(move || to_display_hour(hour.get(), is_12));
                let period = Signal::derive(move || if hour.get() >= 12 { "PM" } else { "AM" });

                view! {
                    <div class="orbital-time-picker-panel" role="dialog" aria-label="Time picker">
                        <div class="orbital-time-picker-panel__columns">
                            <div class="orbital-time-picker-panel__column">
                                <div class="orbital-time-picker-panel__scroll">
                                    {move || {
                                        if is_12 {
                                            (1..=12)
                                                .map(|h| {
                                                    let selected = Signal::derive(move || display_hour.get() == h);
                                                    view! {
                                                        <button
                                                            type="button"
                                                            class="orbital-time-picker-panel__item"
                                                            class=("orbital-time-picker-panel__item--selected", move || selected.get())
                                                            on:click=move |_| {
                                                                let Some(t) = draft.try_get_untracked() else {
                                                                    return;
                                                                };
                                                                let minute = t.minute();
                                                                let second = t.second();
                                                                let next_h =
                                                                    to_twenty_four_hour(h, t.hour() >= 12);
                                                                if let Some(next) =
                                                                    NaiveTime::from_hms_opt(next_h, minute, second)
                                                                {
                                                                    draft.set(next);
                                                                }
                                                            }
                                                        >
                                                            {format!("{h:02}")}
                                                        </button>
                                                    }
                                                })
                                                .collect_view()
                                                .into_any()
                                        } else {
                                            (0..24)
                                                .map(|h| {
                                                    let selected = Signal::derive(move || hour.get() == h);
                                                    view! {
                                                        <button
                                                            type="button"
                                                            class="orbital-time-picker-panel__item"
                                                            class=("orbital-time-picker-panel__item--selected", move || selected.get())
                                                            on:click=move |_| {
                                                                let Some(t) = draft.try_get_untracked() else {
                                                                    return;
                                                                };
                                                                let minute = t.minute();
                                                                let second = t.second();
                                                                if let Some(next) =
                                                                    NaiveTime::from_hms_opt(h, minute, second)
                                                                {
                                                                    draft.set(next);
                                                                }
                                                            }
                                                        >
                                                            {format!("{h:02}")}
                                                        </button>
                                                    }
                                                })
                                                .collect_view()
                                                .into_any()
                                        }
                                    }}
                                </div>
                            </div>
                            <div class="orbital-time-picker-panel__column">
                                <div class="orbital-time-picker-panel__scroll">
                                    {(0..60)
                                        .map(|m| {
                                            let selected = Signal::derive(move || minute.get() == m);
                                            view! {
                                                <button
                                                    type="button"
                                                    class="orbital-time-picker-panel__item"
                                                    class=("orbital-time-picker-panel__item--selected", move || selected.get())
                                                    on:click=move |_| {
                                                        let Some(t) = draft.try_get_untracked() else {
                                                            return;
                                                        };
                                                        let hour = t.hour();
                                                        let second = t.second();
                                                        if let Some(next) =
                                                            NaiveTime::from_hms_opt(hour, m, second)
                                                        {
                                                            draft.set(next);
                                                        }
                                                    }
                                                >
                                                    {format!("{m:02}")}
                                                </button>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            </div>
                            <div class="orbital-time-picker-panel__column">
                                <div class="orbital-time-picker-panel__scroll">
                                    {(0..60)
                                        .map(|s| {
                                            let selected = Signal::derive(move || second.get() == s);
                                            view! {
                                                <button
                                                    type="button"
                                                    class="orbital-time-picker-panel__item"
                                                    class=("orbital-time-picker-panel__item--selected", move || selected.get())
                                                    on:click=move |_| {
                                                        let Some(t) = draft.try_get_untracked() else {
                                                            return;
                                                        };
                                                        let hour = t.hour();
                                                        let minute = t.minute();
                                                        if let Some(next) =
                                                            NaiveTime::from_hms_opt(hour, minute, s)
                                                        {
                                                            draft.set(next);
                                                        }
                                                    }
                                                >
                                                    {format!("{s:02}")}
                                                </button>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            </div>
                            {if is_12 {
                                Some(view! {
                                    <div class="orbital-time-picker-panel__column orbital-time-picker-panel__column--period">
                                        <div class="orbital-time-picker-panel__scroll">
                                            <button
                                                type="button"
                                                class="orbital-time-picker-panel__item"
                                                class=("orbital-time-picker-panel__item--selected", move || period.get() == "AM")
                                                on:click=move |_| {
                                                    let Some(t) = draft.try_get_untracked() else {
                                                        return;
                                                    };
                                                    let h = t.hour();
                                                    if h >= 12 {
                                                        if let Some(next) = NaiveTime::from_hms_opt(
                                                            h - 12,
                                                            t.minute(),
                                                            t.second(),
                                                        ) {
                                                            draft.set(next);
                                                        }
                                                    }
                                                }
                                            >
                                                "AM"
                                            </button>
                                            <button
                                                type="button"
                                                class="orbital-time-picker-panel__item"
                                                class=("orbital-time-picker-panel__item--selected", move || period.get() == "PM")
                                                on:click=move |_| {
                                                    let Some(t) = draft.try_get_untracked() else {
                                                        return;
                                                    };
                                                    let h = t.hour();
                                                    if h < 12 {
                                                        if let Some(next) = NaiveTime::from_hms_opt(
                                                            h + 12,
                                                            t.minute(),
                                                            t.second(),
                                                        ) {
                                                            draft.set(next);
                                                        }
                                                    }
                                                }
                                            >
                                                "PM"
                                            </button>
                                        </div>
                                    </div>
                                })
                            } else {
                                None
                            }}
                        </div>
                        <div class="orbital-time-picker-panel__actions">
                            <button type="button" class="orbital-time-picker-panel__action" on:click=set_now>
                                "Now"
                            </button>
                            <button type="button" class="orbital-time-picker-panel__action orbital-time-picker-panel__action--primary" on:click=commit>
                                "OK"
                            </button>
                        </div>
                    </div>
                }
            }),
        }>
            <div class=move || {
                let mut parts = vec!["orbital-time-picker".to_string()];
                if disabled.get() {
                    parts.push("orbital-time-picker--disabled".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }>
                <button
                    id=id
                    type="button"
                    class="orbital-time-picker__trigger"
                    aria-haspopup="dialog"
                    aria-expanded=move || open.get().to_string()
                    disabled=move || disabled.get().then_some("")
                    on:click=open_panel
                    on:keydown=move |ev: ev::KeyboardEvent| {
                        if ev.key() == "Escape" {
                            open.set(false);
                        }
                    }
                >
                    {move || show_label.get()}
                </button>
                <input
                    type="hidden"
                    name=move || name.get()
                    value=move || value.get_value().get().map(|v| v.to_string()).unwrap_or_default()
                />
            </div>
        </AnchoredPositioner>
    }
}

fn normalize_time_format(format: DatetimeFormat) -> DatetimeFormat {
    match format {
        DatetimeFormat::Time24 | DatetimeFormat::Time12 => format,
        _ => DatetimeFormat::Time12,
    }
}

fn to_display_hour(hour: u32, is_12: bool) -> u32 {
    if !is_12 {
        return hour;
    }
    let hour = hour % 12;
    if hour == 0 {
        12
    } else {
        hour
    }
}

fn to_twenty_four_hour(hour_12: u32, is_pm: bool) -> u32 {
    match (hour_12 % 12, is_pm) {
        (0, false) => 0,
        (0, true) => 12,
        (h, false) => h,
        (h, true) => h + 12,
    }
}
