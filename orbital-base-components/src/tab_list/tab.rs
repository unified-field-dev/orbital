use super::{TabListInjection, TabRegisterData};
use leptos::{html, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

#[component]
pub fn BaseTab(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into)] value: String,
    children: Children,
) -> impl IntoView {
    let tab_ref = NodeRef::<html::Button>::new();
    let tab_list = TabListInjection::expect_context();
    let value = StoredValue::new(value);
    tab_list.register(TabRegisterData {
        value: value.get_value(),
        tab_ref,
    });
    on_cleanup(move || {
        value.with_value(|v| tab_list.unregister(v));
    });

    let selected = Memo::new(move |_| {
        tab_list
            .selected_value
            .with(|selected_value| value.with_value(|value| value == selected_value))
    });

    let on_select = move |_| {
        tab_list.registered_tabs.with_untracked(|registered_tabs| {
            if let Some(previous_selected_tab) = tab_list
                .selected_value
                .with_untracked(|selected_value| registered_tabs.get(selected_value))
            {
                let tab_button = tab_ref.get_untracked().unwrap();
                let tab_el = tab_button.dyn_ref::<Element>().expect("tab button element");
                let selected_tab_rect = tab_el.get_bounding_client_rect();
                let previous_tab_button = previous_selected_tab.tab_ref.get_untracked().unwrap();
                let previous_tab_el = previous_tab_button
                    .dyn_ref::<Element>()
                    .expect("previous tab button element");
                let previous_selected_tab_rect = previous_tab_el.get_bounding_client_rect();

                let offset = previous_selected_tab_rect.x() - selected_tab_rect.x();
                let scale = previous_selected_tab_rect.width() / selected_tab_rect.width();

                let style = tab_button
                    .dyn_ref::<HtmlElement>()
                    .expect("tab button element")
                    .style();
                let _ = style.set_property(
                    "--orbital-tab__indicator--offset",
                    &format!("{offset:.0}px"),
                );
                let _ =
                    style.set_property("--orbital-tab__indicator--scale", &format!("{scale:.2}"));

                request_animation_frame(move || {
                    let _ = style.set_property("--orbital-tab__indicator--offset", "0px");
                    let _ = style.set_property("--orbital-tab__indicator--scale", "1");
                });
            }
        });
        tab_list.on_select(value.get_value());
    };

    let tab_class = Memo::new(move |_| {
        let mut parts = vec!["orbital-tab".to_string()];
        if selected.get() {
            parts.push("orbital-tab--selected".to_string());
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    view! {
        <button
            class=tab_class
            role="tab"
            aria-selected=move || if selected.get() { "true" } else { "false" }
            node_ref=tab_ref
            on:click=on_select
        >
            <span class="orbital-tab__content">{children()}</span>
        </button>
    }
}
