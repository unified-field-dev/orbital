mod resize;

use leptos::{either::Either, ev, html, leptos_dom::helpers::WindowListenerHandle, prelude::*};

use resize::{clamp_column_width, column_width_style};

#[component]
pub fn BaseTableHeaderCell(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, default = false)] resizable: bool,
    #[prop(default = None)] min_width: Option<f64>,
    #[prop(default = None)] max_width: Option<f64>,
    #[prop(default = None)] colspan: Option<u32>,
    #[prop(default = None)] rowspan: Option<u32>,
    #[prop(default = None)] on_resize_end: Option<Callback<f64, ()>>,
    #[prop(default = None)] on_autosize: Option<Callback<(), ()>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let th_ref = NodeRef::<html::Th>::new();
    let pointer_down_x = RwSignal::new(0.0);
    let pointer_down_th_width = RwSignal::new(0.0);
    let last_width = RwSignal::new(0.0);
    let resize_width = RwSignal::new(None::<f64>);
    let listeners = StoredValue::new(Vec::<WindowListenerHandle>::new());

    let th_style = Signal::derive(move || {
        if let Some(width) = resize_width.get() {
            column_width_style(width, min_width, max_width)
        } else {
            style.get().unwrap_or_default()
        }
    });

    let begin_resize = move |client_x: f64| {
        pointer_down_x.set(client_x);
        let Some(th_el) = th_ref.get_untracked() else {
            return;
        };
        let width = th_el.get_bounding_client_rect().width();
        let width = clamp_column_width(width, min_width, max_width);
        if width <= 0.0 {
            return;
        }
        pointer_down_th_width.set(width);
        last_width.set(width);
        resize_width.set(Some(width));

        let on_pointer_move = window_event_listener(ev::pointermove, move |e: ev::PointerEvent| {
            let delta = f64::from(e.client_x()) - pointer_down_x.get_untracked();
            let new_width = clamp_column_width(
                pointer_down_th_width.get_untracked() + delta,
                min_width,
                max_width,
            );
            last_width.set(new_width);
            resize_width.set(Some(new_width));
        });
        let on_mouse_move = window_event_listener(ev::mousemove, move |e: ev::MouseEvent| {
            let delta = f64::from(e.client_x()) - pointer_down_x.get_untracked();
            let new_width = clamp_column_width(
                pointer_down_th_width.get_untracked() + delta,
                min_width,
                max_width,
            );
            last_width.set(new_width);
            resize_width.set(Some(new_width));
        });
        let on_pointer_up = window_event_listener(ev::pointerup, move |_| {
            let width = last_width.get_untracked();
            if let Some(cb) = on_resize_end {
                cb.run(width);
            }
            resize_width.set(Some(width));
            listeners.update_value(|value| {
                for handle in value.drain(..) {
                    handle.remove();
                }
            });
        });
        let on_mouse_up = window_event_listener(ev::mouseup, move |_| {
            let width = last_width.get_untracked();
            if let Some(cb) = on_resize_end {
                cb.run(width);
            }
            resize_width.set(Some(width));
            listeners.update_value(|value| {
                for handle in value.drain(..) {
                    handle.remove();
                }
            });
        });
        listeners.update_value(|value| {
            value.push(on_pointer_move);
            value.push(on_mouse_move);
            value.push(on_pointer_up);
            value.push(on_mouse_up);
        });
    };

    let on_pointer_down = move |e: ev::PointerEvent| {
        if !resizable {
            return;
        }
        e.prevent_default();
        e.stop_propagation();
        begin_resize(f64::from(e.client_x()));
    };

    let on_mouse_down = move |e: ev::MouseEvent| {
        if !resizable {
            return;
        }
        e.prevent_default();
        e.stop_propagation();
        begin_resize(f64::from(e.client_x()));
    };

    let on_dblclick = move |_| {
        if let Some(cb) = on_autosize {
            cb.run(());
        }
    };

    on_cleanup(move || {
        listeners.update_value(|value| {
            for handle in value.drain(..) {
                handle.remove();
            }
        });
    });

    view! {
        <th
            class=move || {
                let extra = class.get().unwrap_or_default();
                if extra.is_empty() {
                    "orbital-table-header-cell".to_string()
                } else {
                    format!("orbital-table-header-cell {extra}")
                }
            }
            style=move || th_style.get()
            colspan=colspan
            rowspan=rowspan
            node_ref=th_ref
        >
            <div class="orbital-table-header-cell__button" role="presentation">
                {if let Some(children) = children {
                    Either::Left(children())
                } else {
                    Either::Right(())
                }}
            </div>
            {if resizable {
                Either::Left(view! {
                    <span
                        class="orbital-table-header-cell__aside"
                        on:pointerdown=on_pointer_down
                        on:mousedown=on_mouse_down
                        on:dblclick=on_dblclick
                    >
                        <div
                            class="orbital-table-resize-handle"
                            role="separator"
                            aria-hidden="true"
                            on:pointerdown=on_pointer_down
                            on:mousedown=on_mouse_down
                            on:dblclick=on_dblclick
                        ></div>
                    </span>
                })
            } else {
                Either::Right(())
            }}
        </th>
    }
}
