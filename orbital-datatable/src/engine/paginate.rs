use crate::types::DataTableRowModel;

pub fn paginate_rows(
    rows: &[DataTableRowModel],
    page: usize,
    page_size: usize,
) -> Vec<DataTableRowModel> {
    if page_size == 0 {
        return rows.to_vec();
    }
    let start = page.saturating_mul(page_size);
    rows.iter().skip(start).take(page_size).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn paginate_second_page() {
        let rows: Vec<_> = (0..15)
            .map(|i| {
                DataTableRowModel::from_text_cells(
                    i.to_string(),
                    HashMap::from([("n".into(), i.to_string())]),
                )
            })
            .collect();
        let page = paginate_rows(&rows, 1, 10);
        assert_eq!(page.len(), 5);
        assert_eq!(page[0].id(), "10");
    }
}
