use leptos::{either::Either, prelude::*};

#[component]
pub fn BaseTable(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <table
            class=move || {
                let extra = class.get().unwrap_or_default();
                if extra.is_empty() {
                    "orbital-table".to_string()
                } else {
                    format!("orbital-table {extra}")
                }
            }
        >
            {children()}
        </table>
    }
}

#[component]
pub fn BaseTableHeader(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <thead
            class=move || {
                let extra = class.get().unwrap_or_default();
                if extra.is_empty() {
                    "orbital-table-header".to_string()
                } else {
                    format!("orbital-table-header {extra}")
                }
            }
        >
            {children()}
        </thead>
    }
}

#[component]
pub fn BaseTableBody(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <tbody
            class=move || {
                let extra = class.get().unwrap_or_default();
                if extra.is_empty() {
                    "orbital-table-body".to_string()
                } else {
                    format!("orbital-table-body {extra}")
                }
            }
        >
            {children()}
        </tbody>
    }
}

#[component]
pub fn BaseTableRow(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <tr
            class=move || {
                let extra = class.get().unwrap_or_default();
                if extra.is_empty() {
                    "orbital-table-row".to_string()
                } else {
                    format!("orbital-table-row {extra}")
                }
            }
        >
            {children()}
        </tr>
    }
}

#[component]
pub fn BaseTableCell(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(default = None)] colspan: Option<u32>,
    #[prop(default = None)] rowspan: Option<u32>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <td
            class=move || {
                let extra = class.get().unwrap_or_default();
                if extra.is_empty() {
                    "orbital-table-cell".to_string()
                } else {
                    format!("orbital-table-cell {extra}")
                }
            }
            style=move || style.get()
            colspan=colspan
            rowspan=rowspan
        >
            {if let Some(children) = children {
                Either::Left(children())
            } else {
                Either::Right(())
            }}
        </td>
    }
}

#[component]
pub fn BaseTableCellLayout(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] truncate: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-table-cell-layout".to_string()];
                if truncate.get() {
                    parts.push("orbital-table-cell-layout--truncate".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
        >
            <div class="orbital-table-cell-layout__content">
                <span class="orbital-table-cell-layout__main">{children()}</span>
            </div>
        </div>
    }
}
