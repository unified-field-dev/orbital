use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use leptos::prelude::*;
use orbital_data::DataRecord;
use orbital_paging::{Page, PageRequest};

use crate::types::DataTableRowModel;

/// Async fetcher for server-driven paging.
pub type PageFetcher = Arc<
    dyn Fn(
            PageRequest,
        ) -> Pin<Box<dyn Future<Output = Result<Page<DataRecord>, ServerFnError>> + Send>>
        + Send
        + Sync,
>;

/// Data source: in-memory records or server fetcher.
#[derive(Clone)]
pub enum DataTableSource {
    /// In-memory records; engine sorts/filters/paginates locally.
    Client(RwSignal<Vec<DataTableRowModel>>),
    /// Server-driven; the same fetcher powers paged and infinite-scroll modes.
    ///
    /// The fetcher receives a [`PageRequest`] built from table sort, filter, quick-search,
    /// and pagination state. Rows returned are rendered as-is — tree expansion, row grouping,
    /// pivot, and client-side aggregation ([`run_processed_pipeline`](crate::engine::run_processed_pipeline))
    /// apply to [`Self::Client`] sources only. Enable [`DataTableFeatures::TREE_DATA`],
    /// [`DataTableFeatures::ROW_GROUPING`], or [`DataTableFeatures::PIVOTING`] with server
    /// paging only when the server implements those semantics.
    ///
    /// Pair with [`ServerFetchPolicy`] on [`DataTable`](crate::DataTable) for stale-response
    /// suppression and optional request dedupe.
    Server {
        fetcher: PageFetcher,
        page_size: u32,
    },
}

impl DataTableSource {
    pub fn client_rw(items: RwSignal<Vec<DataTableRowModel>>) -> Self {
        Self::Client(items)
    }

    /// Backward-compatible alias for [`Self::client_rw`].
    pub fn client_signal(items: RwSignal<Vec<DataTableRowModel>>) -> Self {
        Self::Client(items)
    }

    pub fn client_records(records: Vec<DataRecord>) -> Self {
        let rows: Vec<_> = records.into_iter().map(DataTableRowModel::new).collect();
        Self::Client(RwSignal::new(rows))
    }

    pub fn from_rows(rows: Vec<DataTableRowModel>) -> Self {
        Self::Client(RwSignal::new(rows))
    }

    pub fn is_server(&self) -> bool {
        matches!(self, Self::Server { .. })
    }

    pub fn server_page_size(&self) -> Option<u32> {
        match self {
            Self::Server { page_size, .. } => Some(*page_size),
            Self::Client(_) => None,
        }
    }

    pub fn server_fetcher(&self) -> Option<PageFetcher> {
        match self {
            Self::Server { fetcher, .. } => Some(fetcher.clone()),
            Self::Client(_) => None,
        }
    }

    pub fn client_items(&self) -> Option<RwSignal<Vec<DataTableRowModel>>> {
        match self {
            Self::Client(items) => Some(*items),
            Self::Server { .. } => None,
        }
    }
}

/// Pagination presentation mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PagingMode {
    /// Footer page controls via Pagination. Default.
    #[default]
    Paged,
    /// Load more rows when the user scrolls near the bottom.
    InfiniteScroll,
    /// No pagination chrome; render all rows (client) or single page fetch (server).
    None,
}
