use leptos::prelude::*;
use orbital_base_components::{BaseSkeleton, SkeletonItemShape, SkeletonItemSize};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::skeleton_item_styles;

/// `Skeleton` preserves layout while async data loads — stack [`SkeletonItem`] blocks to mirror text lines, avatars, or media rectangles.
///
/// Set defaults on `Skeleton` and override per item with `size`, `shape`, or explicit `width`/`height`. Remove placeholders as soon as real content is ready.
///
/// # When to use
///
/// - List rows, cards, or feeds where layout should not shift when data arrives
/// - Content areas that stay interactive while loading (unlike a blocking overlay)
///
/// Prefer [`Spinner`](crate::Spinner) for indeterminate action waits with no layout to preserve.
///
/// # Usage
///
/// 1. Wrap placeholder blocks in [`Skeleton`].
/// 2. Set default `size` and `shape` on the container; child [`SkeletonItem`] elements inherit when omitted.
/// 3. Override per item with explicit dimensions to match final content geometry.
/// 4. Swap the skeleton subtree for real content when the fetch completes.
///
/// # Examples
///
/// ## Text block skeleton
/// Skeleton placeholders mirror the shape of loading text so layout does not shift when real content arrives. Stack lines at approximate widths of the final copy.
/// <!-- preview -->
/// ```rust
/// use crate::{Skeleton, SkeletonItem, SkeletonItemSize};
/// view! {
///     <div data-testid="skeleton-preview" style="width: 100%; min-width: 240px;">
///         <Skeleton>
///             <SkeletonItem size=Signal::from(SkeletonItemSize::S16) />
///             <div style="width: 80%; margin-top: 8px;">
///                 <SkeletonItem size=Signal::from(SkeletonItemSize::S16) />
///             </div>
///         </Skeleton>
///     </div>
/// }
/// ```
///
/// ## Avatar row
/// Row layout mimics a list item with avatar and text lines while content loads.
/// <!-- preview -->
/// ```rust
/// use crate::{Skeleton, SkeletonItem, SkeletonItemShape, SkeletonItemSize};
/// view! {
///     <Skeleton>
///         <div data-testid="skeleton-row" style="display: flex; gap: 12px; align-items: center;">
///             <SkeletonItem
///                 shape=Signal::from(SkeletonItemShape::Circle)
///                 size=Signal::from(SkeletonItemSize::S40)
///             />
///             <div style="flex: 1;">
///                 <SkeletonItem size=Signal::from(SkeletonItemSize::S14) />
///                 <div style="width: 40%; margin-top: 8px;">
///                     <SkeletonItem size=Signal::from(SkeletonItemSize::S14) />
///                 </div>
///             </div>
///         </div>
///     </Skeleton>
/// }
/// ```
///
/// ## Card placeholder
/// Card skeleton combines a media block and title line for feed or grid placeholders.
/// <!-- preview -->
/// ```rust
/// use crate::{Skeleton, SkeletonItem, SkeletonItemSize};
/// view! {
///     <div data-testid="skeleton-card" style="width: 240px;">
///         <Skeleton>
///             <SkeletonItem size=Signal::from(SkeletonItemSize::S120) />
///             <div style="width: 70%; margin-top: 12px;">
///                 <SkeletonItem size=Signal::from(SkeletonItemSize::S16) />
///             </div>
///         </Skeleton>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Feedback",
    preview_slug = "skeleton",
    preview_label = "Skeleton",
    preview_icon = icondata::AiBorderOutlined,
)]
#[component]
pub fn Skeleton(
    /// Optional CSS class on the skeleton container (`role="progressbar"`).
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Default height for child [`SkeletonItem`] elements.
    #[prop(optional, into)]
    size: Option<Signal<SkeletonItemSize>>,
    /// Default shape for child [`SkeletonItem`] elements.
    #[prop(optional, into)]
    shape: Option<Signal<SkeletonItemShape>>,
    /// Placeholder regions composed from [`SkeletonItem`] elements.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-skeleton-item", skeleton_item_styles());

    view! {
        <BaseSkeleton class=class nostrip:size=size nostrip:shape=shape>
            {children()}
        </BaseSkeleton>
    }
}
