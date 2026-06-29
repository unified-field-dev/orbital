//! OrbitalInfiniteScroll component for paginated infinite-scroll lists.
//!
//! Wraps [`use_paged_infinite_scroll`]
//! with standardised loading, empty, and end-of-list UI states so every
//! paginated view in the platform gets consistent behaviour out of the box.

use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_paging::{page_fetch_from_tuple, use_paged_infinite_scroll, Page};
use serde::{de::DeserializeOwned, Serialize};
use std::future::Future;
use turf::inline_style_sheet_values;

use super::{Body1, Caption1};
use crate::primitives::*;

// ── Slots ────────────────────────────────────────────────────────────────────

/// Slot for customising the empty-state view shown when no items exist after the first page loads.
///
/// When omitted the component renders a default `<MessageBar intent=Info>"No items found."</MessageBar>`.
///
/// # Example
///
/// ```rust,ignore
/// <OrbitalInfiniteScrollEmptyView slot>
///     <MessageBar intent=MessageBarIntent::Info>"No scores yet."</MessageBar>
/// </OrbitalInfiniteScrollEmptyView>
/// ```
#[slot]
pub struct OrbitalInfiniteScrollEmptyView {
    children: ChildrenFn,
}

/// Slot for customising the initial loading view shown before the first page arrives.
///
/// When omitted the component renders a centered `<Body1>` with `loading_message`
/// (default `"Loading…"`).
///
/// # Example
///
/// ```rust,ignore
/// <OrbitalInfiniteScrollLoadingView slot>
///     <MyTableSkeleton />
/// </OrbitalInfiniteScrollLoadingView>
/// ```
#[slot]
pub struct OrbitalInfiniteScrollLoadingView {
    children: ChildrenFn,
}

/// Slot for customising the end-of-list indicator shown after every page has been loaded.
///
/// The slot content is placed inside a centered container with `EndIndicator` styling.  When omitted the component renders a default `<Caption1>"End of list"</Caption1>`.
///
/// # Example
///
/// ```rust,ignore
/// <OrbitalInfiniteScrollEndView slot>
///     <Caption1>"End of leaderboard"</Caption1>
/// </OrbitalInfiniteScrollEndView>
/// ```
#[slot]
pub struct OrbitalInfiniteScrollEndView {
    children: ChildrenFn,
}

// ── Component ────────────────────────────────────────────────────────────────

/// Load-more list wrapper with standardized loading, empty, and end-of-list states.
///
/// Wraps [`use_paged_infinite_scroll`] and manages the scroll container internally. The caller provides a `fetch` function and children that receive accumulated items via `let:items`.
///
/// **States:** initial loading ([`OrbitalInfiniteScrollLoadingView`] slot) → empty ([`OrbitalInfiniteScrollEmptyView`] slot) → content → loading more → end ([`OrbitalInfiniteScrollEndView`] slot).
///
/// Prefer [`Paginator`](crate::components::Paginator) when the total count is known and footer page buttons fit the UX — users need to jump to a specific page.
///
/// **Best practice:** reset the underlying query when filters change so stale pages are not appended. For empty UX, use the empty slot with [`EmptyState`](crate::components::EmptyState) instead of a one-off message.
///
/// # Example
///
/// ```rust,ignore
/// let fetch_items = |offset, limit| get_items_page(offset, limit);
///
/// view! {
///     <OrbitalInfiniteScroll
///         page_size=10
///         fetch=fetch_items
///         max_height="600px"
///         let:items
///     >
///         <OrbitalInfiniteScrollEmptyView slot>
///             <MessageBar intent=MessageBarIntent::Info>
///                 "Nothing here yet."
///             </MessageBar>
///         </OrbitalInfiniteScrollEmptyView>
///         <OrbitalInfiniteScrollEndView slot>
///             <Caption1>"You've seen it all!"</Caption1>
///         </OrbitalInfiniteScrollEndView>
///         <For
///             each=move || items.get()
///             key=|item| item.id.clone()
///             let:item
///         >
///             <ItemCard item=item />
///         </For>
///     </OrbitalInfiniteScroll>
/// }
/// ```
///
/// # Examples
///
/// ## Default list
/// Card-style rows with custom empty and end slots.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::preview::fixtures::mock_fetch_items;
/// use MessageBarIntent;
///
/// let fetch_default = |offset: u32, limit: u32| mock_fetch_items(offset, limit);
/// let empty_view = || view! {
///     <MessageBar intent=MessageBarIntent::Info>
///         "The list is empty — nothing to show."
///     </MessageBar>
/// };
/// let end_view = || view! { <Caption1>"You have reached the end!"</Caption1> };
///
/// view! {
///     <div data-testid="infinite-scroll-preview">
///     <OrbitalInfiniteScroll
///         page_size=5
///         fetch=fetch_default
///         max_height="300px"
///         let:items
///     >
///         <OrbitalInfiniteScrollEmptyView slot>
///             {empty_view()}
///         </OrbitalInfiniteScrollEmptyView>
///         <OrbitalInfiniteScrollEndView slot>
///             {end_view()}
///         </OrbitalInfiniteScrollEndView>
///         <For
///             each=move || items.get()
///             key=|item| item.id
///             let:item
///         >
///             <div style="padding: 8px 0; border-bottom: 1px solid var(--orb-color-border-default)">
///                 <Body1>{item.title.clone()}</Body1>
///                 <Caption1>{item.description.clone()}</Caption1>
///             </div>
///         </For>
///     </OrbitalInfiniteScroll>
///     </div>
/// }
/// ```
///
/// ## Table layout
/// Infinite scroll inside a table with row numbers.
/// <!-- preview -->
/// ```rust
/// use crate::preview::fixtures::mock_fetch_items;
///
/// let fetch_table = |offset: u32, limit: u32| mock_fetch_items(offset, limit);
///
/// view! {
///     <OrbitalInfiniteScroll
///         page_size=5
///         fetch=fetch_table
///         max_height="250px"
///         let:items
///     >
///         <OrbitalInfiniteScrollEndView slot>
///             <Caption1>"End of list"</Caption1>
///         </OrbitalInfiniteScrollEndView>
///         <Table>
///             <TableHeader>
///                 <TableRow>
///                     <TableHeaderCell>"#"</TableHeaderCell>
///                     <TableHeaderCell>"Title"</TableHeaderCell>
///                     <TableHeaderCell>"ID"</TableHeaderCell>
///                 </TableRow>
///             </TableHeader>
///             <TableBody>
///                 <ForEnumerate
///                     each=move || items.get()
///                     key=|item| item.id
///                     let(idx, item)
///                 >
///                     <TableRow>
///                         <TableCell>
///                             <TableCellLayout>{move || idx.get() + 1}</TableCellLayout>
///                         </TableCell>
///                         <TableCell>
///                             <TableCellLayout>{item.title.clone()}</TableCellLayout>
///                         </TableCell>
///                         <TableCell>
///                             <TableCellLayout>{item.id}</TableCellLayout>
///                         </TableCell>
///                     </TableRow>
///                 </ForEnumerate>
///             </TableBody>
///         </Table>
///     </OrbitalInfiniteScroll>
/// }
/// ```
///
/// ## Minimal
/// Built-in default empty and end slots.
/// <!-- preview -->
/// ```rust
/// use crate::preview::fixtures::mock_fetch_items;
///
/// let fetch_minimal = |offset: u32, limit: u32| mock_fetch_items(offset, limit);
///
/// view! {
///     <OrbitalInfiniteScroll
///         page_size=5
///         fetch=fetch_minimal
///         max_height="250px"
///         let:items
///     >
///         <For
///             each=move || items.get()
///             key=|item| item.id
///             let:item
///         >
///             <div style="padding: 8px 0; border-bottom: 1px solid var(--orb-color-border-default)">
///                 <Body1>{item.title.clone()}</Body1>
///             </div>
///         </For>
///     </OrbitalInfiniteScroll>
/// }
/// ```
#[component_doc(
    category = "Layout",
    preview_slug = "infinite-scroll",
    preview_label = "Infinite Scroll",
    preview_icon = icondata::AiUnorderedListOutlined,
)]
#[component]
pub fn OrbitalInfiniteScroll<T, Fut, F, IV, EF>(
    /// Number of items per page (passed as `limit` to the fetch function).
    page_size: u32,
    /// Async fetch function: `(offset, limit) -> Result<Page<T>, ServerFnError>`. Typically a server function call.
    fetch: F,
    /// Render callback that receives the accumulated items signal. Supports `let:items` syntax — place `<For>` or `<ForEnumerate>` as children to render individual items in any layout (table, card grid, plain list, etc.).
    children: EF,
    /// Optional slot: custom empty-state view. Pass `<OrbitalInfiniteScrollEmptyView slot>` as a child to override the default `MessageBar`.
    #[prop(optional)]
    orbital_infinite_scroll_empty_view: Option<OrbitalInfiniteScrollEmptyView>,
    /// Optional slot: custom initial loading view. Pass `<OrbitalInfiniteScrollLoadingView slot>` as a child to override the default centered loading text.
    #[prop(optional)]
    orbital_infinite_scroll_loading_view: Option<OrbitalInfiniteScrollLoadingView>,
    /// Optional slot: custom end-of-list view. Pass `<OrbitalInfiniteScrollEndView slot>` as a child to override the default `Caption1` label.
    #[prop(optional)]
    orbital_infinite_scroll_end_view: Option<OrbitalInfiniteScrollEndView>,
    /// Maximum height of the scrollable area (CSS value). Defaults to `"400px"`.
    #[prop(optional, into)]
    max_height: Option<String>,
    /// Loading text shown before the first page arrives. Defaults to `"Loading…"`.
    #[prop(optional, into)]
    loading_message: Option<String>,
) -> impl IntoView
where
    T: Clone + Serialize + DeserializeOwned + Send + Sync + PartialEq + 'static,
    Fut: Future<Output = Result<Page<T>, ServerFnError>> + Send + 'static,
    F: Fn(u32, u32) -> Fut + Send + Sync + Clone + 'static,
    IV: IntoView + 'static,
    EF: Fn(RwSignal<Vec<T>>) -> IV + 'static,
{
    let loading_msg = StoredValue::new(loading_message.unwrap_or_else(|| "Loading\u{2026}".into()));
    let max_h = max_height.unwrap_or_else(|| "400px".into());

    // ── Hook wiring ──────────────────────────────────────────────────────
    let scroll_el = NodeRef::<leptos::html::Div>::new();
    let refresh = RwSignal::new(0u32);
    let paged = use_paged_infinite_scroll(
        scroll_el,
        page_size,
        refresh.into(),
        page_fetch_from_tuple(fetch),
    );

    let items = paged.items;
    let has_more = paged.has_more;
    let ever_loaded = paged.ever_loaded;
    let is_loading = paged.loading;

    // ── Render the user-provided children once ──────────────────────────
    // The returned view contains reactive <For>/<ForEnumerate> that updates
    // as items accumulate.  We hide it via CSS when the list is empty to
    // avoid Show/FnOnce lifecycle issues.
    let items_view = children(items);

    // ── Styles ───────────────────────────────────────────────────────────
    let (style_sheet, class_names) = inline_style_sheet_values! {
        // Scroll container with Orbital-themed thin scrollbar.
        .ScrollContainer {
            overflow-y: auto;
            padding: var(--orb-space-block-sm) var(--orb-space-inline-sm);
            scrollbar-width: thin;
            scrollbar-color: var(--orb-color-text-quaternary) transparent;
        }

        .ScrollContainer::-webkit-scrollbar {
            width: 6px;
        }

        .ScrollContainer::-webkit-scrollbar-thumb {
            background-color: var(--orb-color-text-quaternary);
            border-radius: var(--orb-radius-circular);
        }

        .ScrollContainer::-webkit-scrollbar-track {
            background: transparent;
        }

        .StatusCenter {
            display: flex;
            justify-content: center;
            padding: var(--orb-space-block-md, 12px);
        }

        .EndIndicator {
            display: flex;
            justify-content: center;
            padding: var(--orb-space-block-sm, 8px);
        }
    };

    // Copy class names into owned Strings for use inside reactive closures.
    let status_center = class_names.status_center.to_string();
    let end_indicator = class_names.end_indicator.to_string();

    // Reactively toggle content visibility via CSS `display`.
    let content_style = Memo::new(move |_| {
        if items.with(|i| i.is_empty()) {
            "display:none"
        } else {
            ""
        }
    });

    view! {
        <style>{style_sheet}</style>
        <div
            node_ref=scroll_el
            class=class_names.scroll_container
            style=format!("max-height: {max_h}")
            data-testid="orbital-infinite-scroll"
        >
            // ── Empty / initial-loading state ────────────────────────────
            {
                let sc = status_center.clone();
                move || {
                    let is_empty = items.with(|i| i.is_empty());
                    let loaded = ever_loaded.get();
                    let loading = is_loading.get();
                    let more = has_more.get();

                    // When empty and loaded, show empty slot. Skip the !loading check when
                    // has_more is false — the scroll hook can briefly set loading=true when
                    // the container is short (e.g. 0 items), then bail. That spurious blip
                    // would otherwise cause flashing between empty and blank.
                    if is_empty && loaded && (!loading || !more) {
                        // Use the caller's empty slot if provided,
                        // otherwise fall back to a default MessageBar.
                        if let Some(ref slot) = orbital_infinite_scroll_empty_view {
                            (slot.children)().into_any()
                        } else {
                            view! {
                                <MessageBar intent=MessageBarIntent::Info>
                                    "No items found."
                                </MessageBar>
                            }
                            .into_any()
                        }
                    } else if is_empty && !loaded {
                        if let Some(ref slot) = orbital_infinite_scroll_loading_view {
                            (slot.children)().into_any()
                        } else {
                            view! {
                                <div class=sc.clone()>
                                    <Body1>{loading_msg.get_value()}</Body1>
                                </div>
                            }
                            .into_any()
                        }
                    } else {
                        ().into_any()
                    }
                }
            }

            // ── Content area ─────────────────────────────────────────────
            // Always in the DOM; hidden via `display:none` when empty so
            // the reactive ForEnumerate/For inside doesn't need
            // re-creation.
            <div style=move || content_style.get()>
                {items_view}
            </div>

            // ── Loading-more / end-of-list state ─────────────────────────
            {
                let sc = status_center.clone();
                let ei = end_indicator.clone();
                move || {
                    let loading = is_loading.get();
                    let more = has_more.get();
                    let loaded = ever_loaded.get();
                    let is_empty = items.with(|i| i.is_empty());

                    if loading && more {
                        view! {
                            <div class=sc.clone()>
                                <Spinner size=SpinnerSize::Tiny />
                            </div>
                        }
                        .into_any()
                    } else if !more && loaded && !is_empty {
                        // Use the caller's end slot inside the centered
                        // end-indicator wrapper, or fall back to a default
                        // Caption1 label.
                        let inner = if let Some(ref slot) = orbital_infinite_scroll_end_view {
                            (slot.children)().into_any()
                        } else {
                            view! {
                                <Caption1>"End of list"</Caption1>
                            }
                            .into_any()
                        };
                        view! {
                            <div class=ei.clone()>
                                {inner}
                            </div>
                        }
                        .into_any()
                    } else {
                        ().into_any()
                    }
                }
            }
        </div>
    }
}
