use leptos::prelude::*;

use super::injection::FloatingActionsMenuInjection;

#[component]
pub fn BaseFloatingActionsMenuItem(
    #[prop(into)] tooltip: String,
    #[prop(optional)] on_click: Option<Callback<()>>,
    #[prop(optional, into)] testid: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let ctx = FloatingActionsMenuInjection::expect_context();
    let aria_label = tooltip.clone();
    let tooltip_text = tooltip;
    let on_click = move |_| {
        if let Some(on_click) = on_click.as_ref() {
            on_click.run(());
        }
        ctx.open.set(false);
    };

    view! {
        <div
            class="orbital-floating-actions-menu__item"
            style:display=move || if ctx.open.get() { "flex" } else { "none" }
        >
            <span
                class="orbital-floating-actions-menu__tooltip"
                style:display=move || {
                    if ctx.open.get() && ctx.persistent_tooltips.get() {
                        "inline"
                    } else {
                        "none"
                    }
                }
            >
                {tooltip_text.clone()}
            </span>
            <button
                type="button"
                class="orbital-floating-actions-menu__action"
                aria-label=aria_label.clone()
                title=tooltip_text
                data-testid=move || testid.get()
                on:click=on_click
            >
                {children()}
            </button>
        </div>
    }
}
