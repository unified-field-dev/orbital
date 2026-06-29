use leptos::prelude::*;

use super::injection::use_grid;

/// Computes grid-item offset margin-left for a given column span, offset, and horizontal gap.
pub fn grid_item_offset_margin(column: u16, offset: u16, x_gap: u16) -> String {
    if offset == 0 {
        return String::new();
    }
    format!(
        "margin-left: calc((100% - {}px) / {} * {} + {}px);",
        (column + offset - 1) * x_gap,
        column + offset,
        offset,
        offset * x_gap
    )
}

/// Computes grid-column span style for column + offset.
pub fn grid_item_column_span(column: u16, offset: u16) -> String {
    let span = column + offset;
    format!("grid-column: span {span} / span {span};")
}

#[component]
pub fn BaseGridItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = 1u16.into(), into)] column: Signal<u16>,
    #[prop(optional, into)] offset: Signal<u16>,
    children: Children,
) -> impl IntoView {
    let grid = use_grid();

    let style = Memo::new(move |_| {
        let offset_val = offset.get();
        let column_val = column.get();
        let x_gap = grid.x_gap.get();
        let mut style = grid_item_offset_margin(column_val, offset_val, x_gap);
        style.push_str(&grid_item_column_span(column_val, offset_val));
        style
    });

    view! {
        <div
            class=move || {
                let extra = class.get().unwrap_or_default();
                if extra.is_empty() {
                    "orbital-grid-item".to_string()
                } else {
                    format!("orbital-grid-item {extra}")
                }
            }
            style=move || style.get()
        >
            {children()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::{grid_item_column_span, grid_item_offset_margin};

    #[test]
    fn offset_margin_nonzero_when_offset_set() {
        let margin = grid_item_offset_margin(2, 1, 8);
        assert!(margin.contains("margin-left"));
        assert!(margin.contains("calc"));
    }

    #[test]
    fn offset_margin_empty_when_no_offset() {
        assert!(grid_item_offset_margin(2, 0, 8).is_empty());
    }

    #[test]
    fn column_span_includes_offset() {
        assert_eq!(grid_item_column_span(2, 1), "grid-column: span 3 / span 3;");
    }
}
