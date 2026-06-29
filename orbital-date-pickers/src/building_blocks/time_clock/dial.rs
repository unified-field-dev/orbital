use leptos::prelude::*;
use orbital_core_components::{Button, ButtonAppearance, ButtonGroup, ButtonShape, ButtonSize};

use crate::shared::{
    hour_markers, is_minute_step_disabled, marker_position_style, minute_display_markers,
    snap_minute, tick_endpoints, to_twelve_hour, to_twenty_four_hour, DEFAULT_FACE_SIZE,
};

use super::clock_number::ClockNumber;
use super::interaction::ClockInteractionOverlay;
use super::pointer::ClockPointer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClockView {
    Hours,
    Minutes,
}

#[component]
pub fn TimeClockDial(
    view: RwSignal<ClockView>,
    draft_hour_24: RwSignal<u32>,
    draft_minute: RwSignal<u32>,
    is_pm: RwSignal<bool>,
    ampm: Signal<bool>,
    minute_step: Signal<u32>,
    disabled: Signal<bool>,
    on_minute_selected: Callback<(u32, u32)>,
) -> impl IntoView {
    view! {
        <div class="orb-picker-time-clock__face">
            <svg class="orb-picker-time-clock__svg" viewBox="0 0 260 260" aria-hidden="true">
                <circle class="orb-picker-time-clock__circle" cx="130" cy="130" r="125" />
                {(0..12).map(|index| {
                    let (x1, y1, x2, y2) = tick_endpoints(index, DEFAULT_FACE_SIZE);
                    view! {
                        <line
                            class="orb-picker-time-clock__tick"
                            x1=x1
                            y1=y1
                            x2=x2
                            y2=y2
                        />
                    }
                }).collect_view()}
                <circle class="orb-picker-time-clock__center" cx="130" cy="130" r="4" />
            </svg>

            <ClockPointer
                view=Signal::derive(move || view.get())
                draft_hour_24=Signal::derive(move || draft_hour_24.get())
                draft_minute=Signal::derive(move || draft_minute.get())
                ampm=ampm
            />

            {move || {
                match view.get() {
                    ClockView::Hours => hour_numbers_view(
                        ampm,
                        disabled,
                        draft_hour_24,
                        is_pm,
                        view,
                    )
                    .into_any(),
                    ClockView::Minutes => minute_numbers_view(
                        minute_step,
                        disabled,
                        draft_hour_24,
                        draft_minute,
                        view,
                        on_minute_selected,
                    )
                    .into_any(),
                }
            }}

            <ClockInteractionOverlay
                view=view
                draft_hour_24=draft_hour_24
                draft_minute=draft_minute
                is_pm=is_pm
                ampm=ampm
                minute_step=minute_step
                disabled=disabled
                on_minute_selected=on_minute_selected
            />
        </div>
        {move || {
            ampm.get().then(|| {
                view! {
                    <div class="orb-picker-time-clock__meridiem">
                        <ButtonGroup>
                            <Button
                                appearance=Signal::derive(move || {
                                    if is_pm.get() {
                                        ButtonAppearance::Secondary
                                    } else {
                                        ButtonAppearance::Primary
                                    }
                                })
                                shape=Signal::from(ButtonShape::Rounded)
                                size=Signal::from(ButtonSize::Small)
                                disabled=disabled
                                on_click=Callback::new(move |_| {
                                    if is_pm.get() {
                                        is_pm.set(false);
                                        let (display, _) = to_twelve_hour(draft_hour_24.get());
                                        draft_hour_24.set(to_twenty_four_hour(display, false));
                                    }
                                })
                            >
                                "AM"
                            </Button>
                            <Button
                                appearance=Signal::derive(move || {
                                    if is_pm.get() {
                                        ButtonAppearance::Primary
                                    } else {
                                        ButtonAppearance::Secondary
                                    }
                                })
                                shape=Signal::from(ButtonShape::Rounded)
                                size=Signal::from(ButtonSize::Small)
                                disabled=disabled
                                on_click=Callback::new(move |_| {
                                    if !is_pm.get() {
                                        is_pm.set(true);
                                        let (display, _) = to_twelve_hour(draft_hour_24.get());
                                        draft_hour_24.set(to_twenty_four_hour(display, true));
                                    }
                                })
                            >
                                "PM"
                            </Button>
                        </ButtonGroup>
                    </div>
                }
            })
        }}
    }
}

fn hour_numbers_view(
    ampm: Signal<bool>,
    disabled: Signal<bool>,
    draft_hour_24: RwSignal<u32>,
    is_pm: RwSignal<bool>,
    view: RwSignal<ClockView>,
) -> impl IntoView {
    let markers = hour_markers(ampm.get_untracked());
    let count = markers.len();
    markers
        .into_iter()
        .enumerate()
        .map(|(index, marker)| {
            let (left, top) = marker_position_style(index, count, false);
            let display = if ampm.get_untracked() {
                marker.to_string()
            } else {
                format!("{marker:02}")
            };
            let aria_label = format!("{display} hours");
            view! {
                <ClockNumber
                    label=display.clone()
                    selected=Signal::derive(move || {
                        let selected = draft_hour_24.get();
                        if ampm.get() {
                            to_twelve_hour(selected).0 == marker
                        } else {
                            selected == marker
                        }
                    })
                    disabled=disabled
                    left=left
                    top=top
                    aria_label=aria_label
                    on_select=Callback::new(move |_| {
                        let hour_24 = if ampm.get_untracked() {
                            to_twenty_four_hour(marker, is_pm.get_untracked())
                        } else {
                            marker
                        };
                        draft_hour_24.set(hour_24);
                        view.set(ClockView::Minutes);
                    })
                />
            }
        })
        .collect_view()
}

fn minute_numbers_view(
    minute_step: Signal<u32>,
    disabled: Signal<bool>,
    draft_hour_24: RwSignal<u32>,
    draft_minute: RwSignal<u32>,
    view: RwSignal<ClockView>,
    on_minute_selected: Callback<(u32, u32)>,
) -> impl IntoView {
    let markers = minute_display_markers();
    let count = markers.len();
    markers
        .into_iter()
        .enumerate()
        .map(|(index, minute)| {
            let (left, top) = marker_position_style(index, count, false);
            let label = format!("{minute:02}");
            let aria_label = format!("{label} minutes");
            view! {
                <ClockNumber
                    label=label.clone()
                    selected=Signal::derive(move || {
                        snap_minute(draft_minute.get(), 5) == minute
                    })
                    disabled=Signal::derive(move || {
                        disabled.get()
                            || is_minute_step_disabled(minute, minute_step.get())
                    })
                    left=left
                    top=top
                    aria_label=aria_label
                    on_select=Callback::new(move |_| {
                        draft_minute.set(minute);
                        on_minute_selected.run((draft_hour_24.get_untracked(), minute));
                        view.set(ClockView::Hours);
                    })
                />
            }
        })
        .collect_view()
}
