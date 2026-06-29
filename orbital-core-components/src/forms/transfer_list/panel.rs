use std::collections::HashSet;

use leptos::prelude::*;
use orbital_base_components::{selectable_ids, selected_count, toggle_all, TransferListItem};

use crate::Checkbox;

#[component]
fn TransferListRowCheckbox(
    id: String,
    label: String,
    disabled: bool,
    checked: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let row_checked = RwSignal::new(checked.get_untracked().contains(&id));
    let id_read = id.clone();
    Effect::new(move |_| {
        let in_set = checked.get().contains(&id_read);
        if row_checked.get_untracked() != in_set {
            row_checked.set(in_set);
        }
    });
    let id_write = id.clone();
    Effect::new(move |_| {
        if disabled {
            return;
        }
        let val = row_checked.get();
        checked.update(|set| {
            if val {
                set.insert(id_write.clone());
            } else {
                set.remove(&id_write);
            }
        });
    });

    view! {
        <li
            class="orbital-transfer-list__item"
            role="option"
            data-testid=format!("transfer-item-{id}")
        >
            <Checkbox checked=row_checked label=label disabled=Signal::from(disabled) />
        </li>
    }
}

#[component]
fn TransferListSelectAll(
    items: Signal<Vec<TransferListItem>>,
    checked: RwSignal<HashSet<String>>,
    selected: Memo<usize>,
    total: Memo<usize>,
) -> impl IntoView {
    let all_selected = Memo::new(move |_| {
        let selectable = selectable_ids(&items.get());
        let sel = selected_count(&items.get(), &checked.get());
        sel > 0 && sel == selectable.len() && !selectable.is_empty()
    });
    let header_checked = RwSignal::new(all_selected.get_untracked());
    Effect::new(move |_| {
        header_checked.set(all_selected.get());
    });
    let on_select_all = Callback::new(move |select: bool| {
        checked.update(|set| toggle_all(&items.get_untracked(), set, select));
    });

    let header_label = RwSignal::new(format!(
        "{}/{} selected",
        selected.get_untracked(),
        total.get_untracked()
    ));
    Effect::new(move |_| {
        header_label.set(format!("{}/{} selected", selected.get(), total.get()));
    });

    view! {
        <div class="orbital-transfer-list__header">
            <Checkbox
                class="orbital-transfer-list__select-all"
                checked=header_checked
                label=header_label
                on_change=on_select_all
            />
        </div>
    }
}

/// Styled transfer-list panel with Orbital checkboxes and listbox semantics.
#[component]
pub fn TransferListPanel(
    #[prop(optional, into)] title: MaybeProp<String>,
    items: Signal<Vec<TransferListItem>>,
    checked: RwSignal<HashSet<String>>,
    #[prop(default = false.into(), into)] enhanced: Signal<bool>,
    #[prop(optional, into)] list_testid: MaybeProp<String>,
) -> impl IntoView {
    let selected = Memo::new(move |_| selected_count(&items.get(), &checked.get()));
    let total = Memo::new(move |_| items.get().len());

    view! {
        <div class="orbital-transfer-list__panel">
            {move || title.get().map(|text| view! { <div class="orbital-transfer-list__title">{text}</div> })}
            <Show when=move || enhanced.get()>
                <TransferListSelectAll items=items checked=checked selected=selected total=total />
            </Show>
            <ul
                class="orbital-transfer-list__list"
                role="listbox"
                aria-multiselectable="true"
                data-testid=move || list_testid.get()
            >
                {move || {
                    items.get().into_iter().map(|item| {
                        view! {
                            <TransferListRowCheckbox
                                id=item.id.clone()
                                label=item.label.clone()
                                disabled=item.disabled
                                checked=checked
                            />
                        }
                    }).collect_view()
                }}
            </ul>
        </div>
    }
}
