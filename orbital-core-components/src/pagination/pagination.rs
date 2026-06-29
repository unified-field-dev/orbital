use icondata::AiLeftOutlined;
use icondata::AiRightOutlined;
use leptos::{either::Either, prelude::*};
use orbital_base_components::{pagination_items, PaginationItem};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::pagination_styles;
use super::types::PaginationConfig;
use crate::{Button, ButtonAppearance};

/// Page navigation control with previous/next and numbered page buttons.
///
/// Use for list and table footers when the parent owns the current page signal. For offset/limit table paging, prefer [`orbital::components::Paginator`].
///
/// # When to use
///
/// - Standalone page controls when you already have `page` and `page_count` - Table footers — pair with [`orbital::components::Paginator`] for offset/limit bridges
///
/// # Usage
///
/// 1. Bind `config.page` to a `SignalModel<usize>` starting at `1`. 2. Provide `config.page_count` as a reactive total page count. 3. Optionally tune `config.sibling_count` for ellipsis density on large page counts.
///
/// # Best Practices
///
/// - Keep page 1-indexed to match Paginator and table conventions. - Disable prev on page 1 and next on the last page (handled automatically). - Use primary appearance on the active page button for clear selection.
///
/// # Examples
///
/// ## Ten pages, page 1 selected
/// Previous disabled; page 1 uses primary appearance.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{Pagination, PaginationConfig};
/// use orbital_base_components::SignalModel;
/// let page = RwSignal::new(1usize);
/// let page_count = Signal::derive(|| 10usize);
/// view! {
///     <div data-testid="pagination-preview" style="display: flex; justify-content: center; width: 100%;">
///         <Pagination config=PaginationConfig::new(SignalModel::from(page), page_count) />
///     </div>
/// }
/// ```
///
/// ## Three pages
/// Compact control for small datasets.
/// <!-- preview -->
/// ```rust
/// use crate::{Pagination, PaginationConfig};
/// use orbital_base_components::SignalModel;
/// let page = RwSignal::new(1usize);
/// let page_count = Signal::derive(|| 3usize);
/// view! {
///     <div data-testid="pagination-three">
///         <Pagination config=PaginationConfig::new(SignalModel::from(page), page_count) />
///     </div>
/// }
/// ```
///
/// ## Click page 3
/// Page signal updates and page 3 becomes primary.
/// <!-- preview -->
/// ```rust
/// use crate::{Pagination, PaginationConfig};
/// use orbital_base_components::SignalModel;
/// let page = RwSignal::new(1usize);
/// let page_count = Signal::derive(|| 5usize);
/// view! {
///     <div data-testid="pagination-click">
///         <Pagination config=PaginationConfig::new(SignalModel::from(page), page_count) />
///         <span data-testid="pagination-click-page">{move || page.get()}</span>
///     </div>
/// }
/// ```
///
/// ## Ellipsis with many pages
/// Middle page on a 20-page control shows `"..."` and first/last buttons.
/// <!-- preview -->
/// ```rust
/// use crate::{Pagination, PaginationConfig};
/// use orbital_base_components::SignalModel;
/// let page = RwSignal::new(10usize);
/// let page_count = Signal::derive(|| 20usize);
/// view! {
///     <div data-testid="pagination-ellipsis">
///         <Pagination config=PaginationConfig::new(SignalModel::from(page), page_count) />
///     </div>
/// }
/// ```
///
/// ## Theme surfaces
/// Active page button resolves theme background tokens.
/// <!-- preview -->
/// ```rust
/// use crate::{Pagination, PaginationConfig};
/// use orbital_base_components::SignalModel;
/// let page = RwSignal::new(2usize);
/// let page_count = Signal::derive(|| 4usize);
/// view! {
///     <div data-testid="pagination-theme">
///         <Pagination config=PaginationConfig::new(SignalModel::from(page), page_count) />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "pagination",
    preview_label = "Pagination",
    preview_icon = icondata::AiOrderedListOutlined,
)]
#[component]
pub fn Pagination(
    /// Current page, total page count, sibling window, and navigation callbacks.
    config: PaginationConfig,
    /// Extra CSS class names merged onto the pagination control row.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orbital-pagination", pagination_styles());

    let page = config.page.clone();
    let page_count = config.page_count;
    let sibling_count = config.sibling_count;

    let no_next = Memo::new({
        let page = page.clone();
        move |_| page.get() >= page_count.get()
    });
    let no_previous = Memo::new({
        let page = page.clone();
        move |_| page.get() <= 1
    });

    let on_click_previous = {
        let page = page.clone();
        Callback::new(move |_| page.update(|val| *val = val.saturating_sub(1)))
    };

    let on_click_next = {
        let page = page.clone();
        Callback::new(move |_| page.update(|val| *val = val.saturating_add(1)))
    };

    view! {
        <div class=move || {
            let mut parts = vec!["orbital-pagination".to_string()];
            if let Some(c) = class.get() {
                parts.push(c);
            }
            parts.join(" ")
        }>
            <Button
                class="orbital-pagination-item"
                on_click=on_click_previous
                icon=AiLeftOutlined
                disabled=Signal::derive(move || no_previous.get())
                attr:aria-label="Previous page"
            />
            {move || {
                pagination_items(page.get(), page_count.get(), sibling_count.get())
                    .into_iter()
                    .map(|item| match item {
                        PaginationItem::Number(nb) => {
                            let page = page.clone();
                            Either::Left(view! {
                                <Button
                                    class="orbital-pagination-item"
                                    appearance=Memo::new({
                                        let page = page.clone();
                                        move |_| {
                                            if page.get() == nb {
                                                ButtonAppearance::Primary
                                            } else {
                                                ButtonAppearance::Secondary
                                            }
                                        }
                                    })
                                    on_click=Callback::new({
                                        let page = page.clone();
                                        move |_| {
                                            if page.get() != nb {
                                                page.set(nb);
                                            }
                                        }
                                    })
                                >
                                    {nb}
                                </Button>
                            })
                        }
                        PaginationItem::DotLeft | PaginationItem::DotRight => {
                            Either::Right(view! { <div class="orbital-pagination-item">"..."</div> })
                        }
                    })
                    .collect_view()
            }}
            <Button
                class="orbital-pagination-item"
                on_click=on_click_next
                icon=AiRightOutlined
                disabled=Signal::derive(move || no_next.get())
                attr:aria-label="Next page"
            />
        </div>
    }
}
