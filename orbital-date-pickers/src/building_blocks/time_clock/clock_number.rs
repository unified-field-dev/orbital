use leptos::prelude::*;

#[component]
pub fn ClockNumber(
    label: String,
    selected: Signal<bool>,
    disabled: Signal<bool>,
    left: String,
    top: String,
    aria_label: String,
    on_select: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="orb-picker-time-clock__number-slot" style:left=left style:top=top>
            <button
                type="button"
                class=move || {
                    let mut classes = vec!["orb-picker-time-clock__number".to_string()];
                    if selected.get() {
                        classes.push("orb-picker-time-clock__number--selected".to_string());
                    }
                    if disabled.get() {
                        classes.push("orb-picker-time-clock__number--disabled".to_string());
                    }
                    classes.join(" ")
                }
                aria-label=aria_label
                aria-selected=move || selected.get()
                disabled=move || disabled.get()
                on:click=move |ev: leptos::ev::MouseEvent| {
                    ev.stop_propagation();
                    if !disabled.get_untracked() {
                        on_select.run(());
                    }
                }
            >
                {label}
            </button>
        </div>
    }
}
