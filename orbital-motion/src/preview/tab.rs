//! Tab strip for motion preview pages (mirrors core `TabList` / `Tab` styling).

use leptos::prelude::*;

const TAB_LIST_CSS: &str = include_str!("tab-list.css");
const TAB_CSS: &str = include_str!("tab.css");

#[component]
pub fn PreviewTabList(selected_value: RwSignal<String>, children: Children) -> impl IntoView {
    provide_context(selected_value);

    view! {
        <style>{TAB_LIST_CSS}</style>
        <style>{TAB_CSS}</style>
        <div class="orbital-tab-list" role="tablist">
            {children()}
        </div>
    }
}

#[component]
pub fn PreviewTab(#[prop(into)] value: String, children: Children) -> impl IntoView {
    let selected_value = expect_context::<RwSignal<String>>();
    let value = StoredValue::new(value);

    let selected =
        Memo::new(move |_| selected_value.with(|selected| value.with_value(|v| v == selected)));

    let tab_class = Memo::new(move |_| {
        if selected.get() {
            "orbital-tab orbital-tab--selected".to_string()
        } else {
            "orbital-tab".to_string()
        }
    });

    let on_select = move |_| {
        selected_value.set(value.get_value());
    };

    view! {
        <button
            class=tab_class
            role="tab"
            aria-selected=move || if selected.get() { "true" } else { "false" }
            on:click=on_select
        >
            <span class="orbital-tab__content">{children()}</span>
        </button>
    }
}
