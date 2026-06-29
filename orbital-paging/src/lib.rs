//! Shared paging primitives for Orbital apps.
//!
//! This crate provides [`Page`] and [`PageRequest`] — the canonical wire types
//! for offset/limit pagination across all [`leptos`] server functions. Because it
//! has no UI or framework dependencies, any crate in the workspace can depend
//! on it without introducing cycles.
//!
//! When the **`leptos`** feature is enabled, this crate also provides
//! [`use_paged_infinite_scroll`] — a reusable hook that encapsulates the
//! 4-signal + `use_infinite_scroll` callback pattern for paginated lists.
//!
//! # Example
//!
//! ```rust
//! use orbital_paging::{Page, PageRequest};
//!
//! let request = PageRequest::new(0, 10);
//! assert_eq!(request.offset, 0);
//! assert_eq!(request.limit, 10);
//!
//! let page: Page<String> = Page {
//!     items: vec!["a".into(), "b".into()],
//!     has_more: false,
//!     total_count: Some(2),
//!     next_request_offset: None,
//! };
//! assert!(!page.has_more);
//! ```

use orbital_data::DataValue;
use serde::{Deserialize, Serialize};

#[cfg(feature = "leptos")]
mod infinite_scroll;
#[cfg(feature = "leptos")]
pub use infinite_scroll::*;

/// Sort direction on the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirectionWire {
    Asc,
    Desc,
}

/// Single sort column on the wire.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SortParam {
    pub field: String,
    pub direction: SortDirectionWire,
}

/// How multiple filter rules combine on the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilterLogicWire {
    #[default]
    And,
    Or,
}

/// Single filter rule on the wire.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterRuleParam {
    pub field: String,
    pub operator: String,
    pub value: DataValue,
}

/// Structured filter query on the wire.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterQuery {
    pub items: Vec<FilterRuleParam>,
    pub logic: FilterLogicWire,
}

/// Client-to-server pagination and query parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageRequest {
    /// Number of items to skip.
    pub offset: u32,
    /// Maximum number of items to return.
    pub limit: u32,
    /// Optional multi-column sort (server-side).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<SortParam>>,
    /// Optional structured filter (server-side).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterQuery>,
    /// Optional quick-search tokens (server-side).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quick_search: Option<String>,
}

impl PageRequest {
    /// Create a new page request with offset and limit only.
    pub fn new(offset: u32, limit: u32) -> Self {
        Self {
            offset,
            limit,
            sort: None,
            filter: None,
            quick_search: None,
        }
    }

    /// Attach sort, filter, and quick-search query parameters.
    pub fn with_query(
        offset: u32,
        limit: u32,
        sort: Option<Vec<SortParam>>,
        filter: Option<FilterQuery>,
        quick_search: Option<String>,
    ) -> Self {
        Self {
            offset,
            limit,
            sort,
            filter,
            quick_search,
        }
    }

    /// Convenience: is this the first page?
    pub fn is_first_page(&self) -> bool {
        self.offset == 0
    }
}

/// A single page of results returned by a server function.
///
/// `T` is the item type (e.g. `NotificationDto`). The struct derives
/// `Serialize` and `Deserialize` so it crosses the Leptos server boundary
/// transparently.
///
/// * `has_more` — `true` when more items exist beyond this page.
/// * `total_count` — optionally present on the **first page** (`offset == 0`)
///   to give the client the full result-set size without a separate call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page<T> {
    /// The items in this page.
    pub items: Vec<T>,
    /// Whether additional pages exist after this one.
    pub has_more: bool,
    /// Total number of matching items (typically provided only on the first
    /// page to avoid repeated count queries).
    pub total_count: Option<u64>,
    /// When set, the next `(offset, limit)` fetch should use **this** value as
    /// `offset` instead of `accumulated_items.len()`.
    ///
    /// Use when the server applies `offset`/`limit` at the **database row**
    /// layer but returns **fewer** items after filtering (e.g. resolving a
    /// join skips some rows). Without this, the client would skip the wrong
    /// slice and can show a short first page plus a premature “end of list”.
    #[serde(default)]
    pub next_request_offset: Option<u32>,
}

impl<T> Page<T> {
    /// Build a `Page` from a result set fetched with `limit + 1` rows.
    ///
    /// If `raw_items` contains more than `limit` entries the extra row is
    /// removed and `has_more` is set to `true`.
    pub fn from_oversized(mut raw_items: Vec<T>, limit: u32, total_count: Option<u64>) -> Self {
        let has_more = raw_items.len() as u32 > limit;
        if has_more {
            raw_items.truncate(limit as usize);
        }
        Self {
            items: raw_items,
            has_more,
            total_count,
            next_request_offset: None,
        }
    }

    /// Create an empty page (no items, no more pages).
    pub fn empty() -> Self {
        Self {
            items: Vec::new(),
            has_more: false,
            total_count: Some(0),
            next_request_offset: None,
        }
    }

    /// Number of items in this page.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether this page contains zero items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_oversized_trims_and_sets_has_more() {
        let items: Vec<u32> = (1..=11).collect(); // 11 items, limit 10
        let page = Page::from_oversized(items, 10, Some(25));
        assert_eq!(page.items.len(), 10);
        assert!(page.has_more);
        assert_eq!(page.total_count, Some(25));
    }

    #[test]
    fn from_oversized_no_extra_item() {
        let items: Vec<u32> = (1..=10).collect(); // exactly 10
        let page = Page::from_oversized(items, 10, None);
        assert_eq!(page.items.len(), 10);
        assert!(!page.has_more);
        assert_eq!(page.total_count, None);
    }

    #[test]
    fn page_request_first_page() {
        assert!(PageRequest::new(0, 10).is_first_page());
        assert!(!PageRequest::new(10, 10).is_first_page());
    }

    #[test]
    fn page_request_legacy_deserialize() {
        let json = r#"{"offset":0,"limit":10}"#;
        let req: PageRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.offset, 0);
        assert_eq!(req.limit, 10);
        assert!(req.sort.is_none());
        assert!(req.filter.is_none());
        assert!(req.quick_search.is_none());
    }

    #[test]
    fn empty_page() {
        let page: Page<String> = Page::empty();
        assert!(page.is_empty());
        assert!(!page.has_more);
        assert_eq!(page.total_count, Some(0));
    }
}
