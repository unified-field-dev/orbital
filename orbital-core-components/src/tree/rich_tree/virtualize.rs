use leptos::prelude::*;
use wasm_bindgen::JsCast;

use super::node::RichTreeNode;
use super::node_view::RichTreeNodeView;

pub(crate) const VIRTUALIZE_THRESHOLD: usize = 50;
const VIRTUAL_ROW_HEIGHT: f64 = 32.0;
const VIRTUAL_OVERSCAN: usize = 5;

#[component]
pub fn VirtualRichTreeChildren(nodes: Vec<RichTreeNode>) -> impl IntoView {
    let nodes = StoredValue::new(nodes);
    let scroll_top = RwSignal::new(0.0);

    let visible_range = Memo::new(move |_| {
        let all = nodes.get_value();
        let total = all.len();
        if total == 0 {
            return (0_usize, 0_usize, 0.0);
        }
        let top = scroll_top.get();
        let start = ((top / VIRTUAL_ROW_HEIGHT).floor() as usize).saturating_sub(VIRTUAL_OVERSCAN);
        let visible_count =
            ((400.0 / VIRTUAL_ROW_HEIGHT).ceil() as usize).saturating_add(VIRTUAL_OVERSCAN * 2);
        let end = (start + visible_count).min(total);
        (start, end, total as f64 * VIRTUAL_ROW_HEIGHT)
    });

    view! {
        <div
            class="orbital-tree-virtual-scroll"
            style="max-height: 400px; overflow: auto;"
            on:scroll=move |ev| {
                if let Some(el) = ev
                    .target()
                    .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok())
                {
                    scroll_top.set(el.scroll_top() as f64);
                }
            }
        >
            <div style=move || {
                let (_, _, height) = visible_range.get();
                format!("height: {height}px; position: relative;")
            }>
                <div style=move || {
                    let (start, _, _) = visible_range.get();
                    format!(
                        "position: absolute; top: {}px; left: 0; right: 0;",
                        start as f64 * VIRTUAL_ROW_HEIGHT
                    )
                }>
                    <For
                        each=move || {
                            let (start, end, _) = visible_range.get();
                            nodes
                                .get_value()
                                .into_iter()
                                .enumerate()
                                .filter_map(|(index, node)| {
                                    (index >= start && index < end).then_some(node)
                                })
                                .collect::<Vec<_>>()
                        }
                        key=|child| child.id.clone()
                        children=move |child| view! { <RichTreeNodeView node=child /> }
                    />
                </div>
            </div>
        </div>
    }
}
