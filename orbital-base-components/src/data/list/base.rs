use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListNavigationMode {
    Selectable,
    Nav,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListSelectionMode {
    Single,
    Multiselect,
}

#[component]
pub fn BaseList(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] navigation_mode: Signal<Option<ListNavigationMode>>,
    #[prop(optional, into)] selection_mode: Signal<Option<ListSelectionMode>>,
    children: Children,
) -> impl IntoView {
    let role = move || {
        if selection_mode.get().is_some() {
            "listbox"
        } else {
            "list"
        }
    };

    view! {
        <ul
            class=move || {
                let mut parts = vec!["orbital-list".to_string()];
                if let Some(mode) = navigation_mode.get() {
                    parts.push(format!("orbital-list--nav-{}", mode_as_str(mode)));
                }
                if let Some(mode) = selection_mode.get() {
                    parts.push(format!("orbital-list--select-{}", select_mode_as_str(mode)));
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role=role
        >
            {children()}
        </ul>
    }
}

fn mode_as_str(mode: ListNavigationMode) -> &'static str {
    match mode {
        ListNavigationMode::Selectable => "selectable",
        ListNavigationMode::Nav => "nav",
    }
}

fn select_mode_as_str(mode: ListSelectionMode) -> &'static str {
    match mode {
        ListSelectionMode::Single => "single",
        ListSelectionMode::Multiselect => "multiselect",
    }
}

#[component]
pub fn BaseListItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] selected: Signal<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    view! {
        <li
            class=move || {
                let mut parts = vec!["orbital-list__item".to_string()];
                if selected.get() {
                    parts.push("orbital-list__item--selected".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role="listitem"
            aria-selected=move || selected.get().then_some("true")
            tabindex=move || if selected.get() { "0" } else { "-1" }
            on:click=move |_| {
                if let Some(cb) = on_click {
                    cb.run(());
                }
            }
        >
            {children()}
        </li>
    }
}
