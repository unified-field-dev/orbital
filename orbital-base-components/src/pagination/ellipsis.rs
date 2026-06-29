use std::cmp::min;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaginationItem {
    DotLeft,
    DotRight,
    Number(usize),
}

pub fn pagination_items(page: usize, count: usize, sibling_count: usize) -> Vec<PaginationItem> {
    let total_page_numbers = sibling_count + 5;
    if total_page_numbers >= count {
        return range(1, count);
    }

    let left_sibling_index = if page > sibling_count + 1 {
        page - sibling_count
    } else {
        1
    };
    let right_sibling_index = min(page + sibling_count, count);
    let should_show_left_dots = left_sibling_index > 2;
    let should_show_right_dots = right_sibling_index < count - 2;

    if !should_show_left_dots && should_show_right_dots {
        let left_item_count = 3 + 2 * sibling_count;
        let mut left_range = range(1, left_item_count);
        left_range.push(PaginationItem::DotRight);
        left_range.push(PaginationItem::Number(count));
        left_range
    } else if should_show_left_dots && !should_show_right_dots {
        let right_item_count = 3 + 2 * sibling_count;
        let mut right_range = range(count - right_item_count + 1, count);
        let mut ret = vec![PaginationItem::Number(1), PaginationItem::DotLeft];
        ret.append(&mut right_range);
        ret
    } else {
        let mut middle_range = range(left_sibling_index, right_sibling_index);
        let mut items = vec![PaginationItem::Number(1), PaginationItem::DotLeft];
        items.append(&mut middle_range);
        items.append(&mut vec![
            PaginationItem::DotRight,
            PaginationItem::Number(count),
        ]);
        items
    }
}

fn range(start: usize, end: usize) -> Vec<PaginationItem> {
    (start..=end).map(PaginationItem::Number).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_page_count_returns_full_range() {
        assert_eq!(
            pagination_items(1, 5, 1),
            vec![
                PaginationItem::Number(1),
                PaginationItem::Number(2),
                PaginationItem::Number(3),
                PaginationItem::Number(4),
                PaginationItem::Number(5),
            ]
        );
    }

    #[test]
    fn middle_page_shows_both_dots() {
        let items = pagination_items(10, 20, 1);
        assert!(items.contains(&PaginationItem::DotLeft));
        assert!(items.contains(&PaginationItem::DotRight));
        assert!(items.contains(&PaginationItem::Number(1)));
        assert!(items.contains(&PaginationItem::Number(20)));
    }

    #[test]
    fn early_page_shows_right_dots_only() {
        let items = pagination_items(2, 20, 1);
        assert!(!items.contains(&PaginationItem::DotLeft));
        assert!(items.contains(&PaginationItem::DotRight));
    }

    #[test]
    fn late_page_shows_left_dots_only() {
        let items = pagination_items(19, 20, 1);
        assert!(items.contains(&PaginationItem::DotLeft));
        assert!(!items.contains(&PaginationItem::DotRight));
    }
}
