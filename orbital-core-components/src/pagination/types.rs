use leptos::prelude::*;
use orbital_base_components::SignalModel;

/// Configuration for [`Pagination`].
#[derive(Clone)]
pub struct PaginationConfig {
    /// Current page (1-indexed).
    pub page: SignalModel<usize>,
    /// Total number of pages in the set.
    pub page_count: Signal<usize>,
    /// Number of page buttons shown on each side of the current page before ellipsis.
    pub sibling_count: Signal<usize>,
}

impl PaginationConfig {
    pub fn new(page: SignalModel<usize>, page_count: Signal<usize>) -> Self {
        Self {
            page,
            page_count,
            sibling_count: Signal::derive(|| 1),
        }
    }

    pub fn with_sibling_count(mut self, sibling_count: Signal<usize>) -> Self {
        self.sibling_count = sibling_count;
        self
    }
}
