use leptos::prelude::*;

use crate::shared::{hour_hand_degrees, minute_hand_degrees};

use super::dial::ClockView;

#[component]
pub fn ClockPointer(
    view: Signal<ClockView>,
    draft_hour_24: Signal<u32>,
    draft_minute: Signal<u32>,
    ampm: Signal<bool>,
) -> impl IntoView {
    view! {
        {move || {
            let current_view = view.get();
            let (degrees, has_thumb) = match current_view {
                ClockView::Hours => (hour_hand_degrees(draft_hour_24.get(), ampm.get()), true),
                ClockView::Minutes => {
                    let minute = draft_minute.get();
                    (minute_hand_degrees(minute), minute.is_multiple_of(5))
                }
            };

            view! {
                <div
                    class="orb-picker-time-clock__pointer"
                    style:transform=format!("rotateZ({degrees}deg)")
                    aria-hidden="true"
                >
                    <div class="orb-picker-time-clock__pointer-line" />
                    {has_thumb.then(|| view! {
                        <div class="orb-picker-time-clock__pointer-thumb" />
                    })}
                </div>
            }
        }}
    }
}
