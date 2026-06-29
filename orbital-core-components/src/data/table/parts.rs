use leptos::prelude::*;
use orbital_base_components::{
    BaseTableBody, BaseTableCell, BaseTableCellLayout, BaseTableHeader, BaseTableHeaderCell,
    BaseTableRow,
};
use orbital_style::inject_style;

use super::styles::table_styles;
use super::types::{TableCellLayoutConfig, TableHeaderCellConfig};

static TABLE_STYLES: std::sync::Once = std::sync::Once::new();

fn ensure_table_styles() {
    TABLE_STYLES.call_once(|| inject_style("orbital-table", table_styles()));
}

#[component]
pub fn TableHeader(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    ensure_table_styles();
    view! { <BaseTableHeader class=class>{children()}</BaseTableHeader> }
}

#[component]
pub fn TableHeaderCell(
    #[prop(optional, into)] config: TableHeaderCellConfig,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(default = None)] colspan: Option<u32>,
    #[prop(default = None)] rowspan: Option<u32>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    ensure_table_styles();
    let on_resize_end = config.on_resize_end;
    let on_autosize = config.on_autosize;
    view! {
        <BaseTableHeaderCell
            class=class
            style=style
            resizable=config.resizable
            min_width=config.min_width
            max_width=config.max_width
            colspan=colspan
            rowspan=rowspan
            on_resize_end=on_resize_end
            on_autosize=on_autosize
        >
            {children.map(|c| c())}
        </BaseTableHeaderCell>
    }
}

#[component]
pub fn TableBody(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    ensure_table_styles();
    view! { <BaseTableBody class=class>{children()}</BaseTableBody> }
}

#[component]
pub fn TableRow(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    ensure_table_styles();
    view! { <BaseTableRow class=class>{children()}</BaseTableRow> }
}

#[component]
pub fn TableCell(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(default = None)] colspan: Option<u32>,
    #[prop(default = None)] rowspan: Option<u32>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    ensure_table_styles();
    view! {
        <BaseTableCell class=class style=style colspan=colspan rowspan=rowspan>
            {children.map(|c| c())}
        </BaseTableCell>
    }
}

#[component]
pub fn TableCellLayout(
    #[prop(optional, into)] config: TableCellLayoutConfig,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    ensure_table_styles();
    view! {
        <BaseTableCellLayout class=class truncate=Signal::from(config.truncate)>
            {children()}
        </BaseTableCellLayout>
    }
}
