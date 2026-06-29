use std::sync::Arc;

use orbital_paging::PageRequest;

/// Policy for server-driven [`DataTableSource::Server`](crate::types::DataTableSource::Server) fetches.
///
/// # Stale-response suppression
///
/// By default, every sort/filter/quick-search/offset/page-size change starts a new fetch
/// generation. When a slower response from an older generation completes, it is **dropped**
/// so rows and totals cannot be overwritten by stale data. This uses a generation counter
/// inside the table — user fetchers (typically Leptos server functions) do not need to
/// support HTTP abort or cancellation tokens.
///
/// # Dedupe
///
/// When [`Self::dedupe_key`] is set, consecutive fetches whose keys match the previous
/// request are skipped. The dedupe cache is cleared when filter or quick-search changes.
///
/// # `server_total` invalidation
///
/// - Filter or quick-search change clears [`server_total`](crate::types::DataTableTableState::server_total)
///   so the footer shows an estimated range until the next page-0 response supplies
///   [`Page::total_count`](orbital_paging::Page::total_count).
/// - Page &gt; 0 responses often omit `total_count`; the table retains the last known total
///   until filter/search clears it or offset 0 returns a fresh count.
/// - Sort and page-size changes reset offset but do not clear the total.
///
/// # Server + advanced features
///
/// Server-fetched pages are rendered as returned by the fetcher. Tree, row grouping, pivot,
/// and aggregation run in the client [`run_processed_pipeline`](crate::engine::run_processed_pipeline)
/// for **client** sources only — enable those features only when the server implements the
/// corresponding semantics.
///
/// # Deferred
///
/// Response caching (`ServerFetchPolicy::Cache(ttl)`) is deferred per design §11.2.
#[derive(Clone, Default)]
pub struct ServerFetchPolicy {
    /// When set, skip a fetch when this key equals the previous request's key.
    pub dedupe_key: Option<Arc<dyn Fn(&PageRequest) -> String + Send + Sync>>,
}

impl ServerFetchPolicy {
    /// Default policy: always invalidate stale in-flight responses; no dedupe.
    pub fn new() -> Self {
        Self::default()
    }
}
