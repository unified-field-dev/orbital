use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::prelude::*;
use std::time::Duration;

use super::provider::{ToastRecord, ToastRecordContent};
use super::trigger::ToastItemContext;
use super::{BaseToast, BaseToastBody, BaseToastFooter, BaseToastTitle};

fn schedule_dismiss(
    handle_store: StoredValue<Option<TimeoutHandle>>,
    remaining_ms: i32,
    on_dismiss: Callback<()>,
) {
    handle_store.update_value(|handle| {
        if let Some(h) = handle.take() {
            h.clear();
        }
        if remaining_ms <= 0 {
            return;
        }
        *handle = set_timeout_with_handle(
            move || on_dismiss.run(()),
            Duration::from_millis(remaining_ms as u64),
        )
        .ok();
    });
}

#[component]
pub fn BaseToastContainer(record: ToastRecord, on_dismiss: Callback<String>) -> impl IntoView {
    let toast_id = record.id.clone();
    let timeout = record.timeout;
    let pause_on_hover = record.pause_on_hover;
    let intent = record.intent;
    let content = record.content;

    let dismiss_self = Callback::new({
        let toast_id = toast_id.clone();
        let on_dismiss = on_dismiss;
        move |()| on_dismiss.run(toast_id.clone())
    });

    provide_context(ToastItemContext {
        dismiss: dismiss_self,
    });

    let timer_handle = StoredValue::new(None::<TimeoutHandle>);
    let remaining_ms = StoredValue::new(timeout);
    let timer_started_at = StoredValue::new(0.0_f64);

    let refresh_remaining = move || {
        if timer_started_at.get_value() <= 0.0 {
            return remaining_ms.get_value();
        }
        let elapsed = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now() - timer_started_at.get_value())
            .unwrap_or(0.0) as i32;
        (remaining_ms.get_value() - elapsed).max(0)
    };

    let start_timer = move |ms: i32| {
        if ms <= 0 {
            return;
        }
        remaining_ms.set_value(ms);
        timer_started_at.set_value(
            web_sys::window()
                .and_then(|w| w.performance())
                .map(|p| p.now())
                .unwrap_or(0.0),
        );
        schedule_dismiss(timer_handle, ms, dismiss_self);
    };

    let pause_timer = move || {
        let left = refresh_remaining();
        remaining_ms.set_value(left);
        timer_started_at.set_value(0.0);
        timer_handle.update_value(|handle| {
            if let Some(h) = handle.take() {
                h.clear();
            }
        });
    };

    Effect::new(move |_| {
        if timeout <= 0 {
            return;
        }
        let delay_handle = StoredValue::new(None::<TimeoutHandle>);
        delay_handle.set_value(
            set_timeout_with_handle(move || start_timer(timeout), Duration::from_millis(50)).ok(),
        );
        on_cleanup(move || {
            delay_handle.update_value(|handle| {
                if let Some(h) = handle.take() {
                    h.clear();
                }
            });
            timer_handle.update_value(|timer| {
                if let Some(h) = timer.take() {
                    h.clear();
                }
            });
        });
    });

    let on_mouse_enter = move |_| {
        if pause_on_hover && timeout > 0 {
            pause_timer();
        }
    };

    let on_mouse_leave = move |_| {
        if pause_on_hover && timeout > 0 {
            let left = remaining_ms.get_value();
            if left > 0 {
                start_timer(left);
            }
        }
    };

    let toast_body = match content {
        ToastRecordContent::Text {
            title,
            body,
            footer_actions,
        } => view! {
            <BaseToast intent=Signal::from(intent)>
                <BaseToastTitle>{title}</BaseToastTitle>
                {body.map(|body| view! {
                    <BaseToastBody>{body}</BaseToastBody>
                })}
                {(!footer_actions.is_empty()).then(|| {
                    view! {
                        <BaseToastFooter>
                            {footer_actions.into_iter().map(|action| {
                                let label = action.label.clone();
                                let dismiss = action.dismiss;
                                let on_click = action.on_click;
                                let dismiss_self = dismiss_self;
                                view! {
                                    <button
                                        type="button"
                                        class="orbital-toast-footer__action"
                                        on:click=move |_| {
                                            if let Some(cb) = on_click {
                                                cb.run(());
                                            }
                                            if dismiss {
                                                dismiss_self.run(());
                                            }
                                        }
                                    >
                                        {label}
                                    </button>
                                }.into_any()
                            }).collect_view()}
                        </BaseToastFooter>
                    }
                })}
            </BaseToast>
        }
        .into_any(),
        ToastRecordContent::View(view) => view.with_value(|render| render()).into_any(),
    };

    view! {
        <div
            class="orbital-toaster-container"
            on:mouseenter=on_mouse_enter
            on:mouseleave=on_mouse_leave
        >
            {toast_body}
        </div>
    }
}
