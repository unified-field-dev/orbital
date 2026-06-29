use leptos::prelude::*;
use orbital_paging::Page;
use serde::{Deserialize, Serialize};

/// Simple mock item for infinite-scroll preview demos.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MockItem {
    pub id: u32,
    pub title: String,
    pub description: String,
}

/// Total number of mock items in the preview dataset.
const MOCK_TOTAL: u32 = 23;

/// Simulates a paginated server function by slicing a static dataset.
pub async fn mock_fetch_items(offset: u32, limit: u32) -> Result<Page<MockItem>, ServerFnError> {
    let all: Vec<MockItem> = (1..=MOCK_TOTAL)
        .map(|i| MockItem {
            id: i,
            title: format!("Item {i}"),
            description: format!(
                "Description for item {i} — a short summary of what this entry contains."
            ),
        })
        .collect();

    let start = offset as usize;
    let end = ((offset + limit + 1) as usize).min(all.len());
    let slice = if start < all.len() {
        all[start..end].to_vec()
    } else {
        Vec::new()
    };

    let total_count = if offset == 0 {
        Some(MOCK_TOTAL as u64)
    } else {
        None
    };

    Ok(Page::from_oversized(slice, limit, total_count))
}
