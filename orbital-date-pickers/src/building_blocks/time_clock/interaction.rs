use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::shared::{
    get_hours_from_point, get_minutes_from_point, to_twenty_four_hour, DEFAULT_MARKER_SIZE,
};

use super::dial::ClockView;

#[component]
pub fn ClockInteractionOverlay(
    view: RwSignal<ClockView>,
    draft_hour_24: RwSignal<u32>,
    draft_minute: RwSignal<u32>,
    is_pm: RwSignal<bool>,
    ampm: Signal<bool>,
    minute_step: Signal<u32>,
    disabled: Signal<bool>,
    on_minute_selected: Callback<(u32, u32)>,
) -> impl IntoView {
    let dragging = RwSignal::new(false);

    let set_time_from_event = move |ev: leptos::ev::PointerEvent| {
        if disabled.get_untracked() {
            return;
        }

        let Some(target) = ev
            .current_target()
            .and_then(|node| node.dyn_into::<web_sys::Element>().ok())
        else {
            return;
        };

        let rect = target.get_bounding_client_rect();
        let offset_x = ev.client_x() as f64 - rect.left();
        let offset_y = ev.client_y() as f64 - rect.top();
        let face_size = rect.width().max(rect.height());

        match view.get_untracked() {
            ClockView::Hours => {
                let display_hour = get_hours_from_point(
                    offset_x,
                    offset_y,
                    face_size,
                    DEFAULT_MARKER_SIZE,
                    ampm.get_untracked(),
                );
                let hour_24 = if ampm.get_untracked() {
                    to_twenty_four_hour(display_hour, is_pm.get_untracked())
                } else {
                    display_hour
                };
                draft_hour_24.set(hour_24);
            }
            ClockView::Minutes => {
                let minute = get_minutes_from_point(
                    offset_x,
                    offset_y,
                    face_size,
                    minute_step.get_untracked(),
                );
                draft_minute.set(minute);
            }
        }
    };

    view! {
        <div
            class=move || {
                if dragging.get() {
                    "orb-picker-time-clock__overlay orb-picker-time-clock__overlay--dragging"
                } else {
                    "orb-picker-time-clock__overlay"
                }
            }
            on:pointerdown=move |ev: leptos::ev::PointerEvent| {
                if disabled.get_untracked() {
                    return;
                }
                dragging.set(true);
                if let Some(target) = ev.current_target() {
                    let _ = target
                        .unchecked_ref::<web_sys::Element>()
                        .set_pointer_capture(ev.pointer_id());
                }
                set_time_from_event(ev);
            }
            on:pointermove=move |ev: leptos::ev::PointerEvent| {
                if dragging.get_untracked() {
                    set_time_from_event(ev);
                }
            }
            on:pointerup=move |ev: leptos::ev::PointerEvent| {
                if !dragging.get_untracked() {
                    return;
                }
                dragging.set(false);
                if let Some(target) = ev.current_target() {
                    let _ = target
                        .unchecked_ref::<web_sys::Element>()
                        .release_pointer_capture(ev.pointer_id());
                }
                set_time_from_event(ev);

                match view.get_untracked() {
                    ClockView::Hours => view.set(ClockView::Minutes),
                    ClockView::Minutes => {
                        let hour = draft_hour_24.get_untracked();
                        let minute = draft_minute.get_untracked();
                        on_minute_selected.run((hour, minute));
                        view.set(ClockView::Hours);
                    }
                }
            }
            on:pointercancel=move |ev: leptos::ev::PointerEvent| {
                dragging.set(false);
                if let Some(target) = ev.current_target() {
                    let _ = target
                        .unchecked_ref::<web_sys::Element>()
                        .release_pointer_capture(ev.pointer_id());
                }
            }
        />
    }
}
