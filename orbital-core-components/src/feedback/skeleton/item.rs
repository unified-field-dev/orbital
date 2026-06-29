use leptos::prelude::*;
use orbital_base_components::{BaseSkeletonItem, SkeletonItemShape, SkeletonItemSize};

/// Shimmer block inside a [`Skeleton`](super::skeleton::Skeleton) placeholder region.
#[component]
pub fn SkeletonItem(
    /// Optional CSS class on the shimmer element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Optional explicit width (CSS value).
    #[prop(optional, into)]
    width: MaybeProp<String>,
    /// Optional explicit height (CSS value).
    #[prop(optional, into)]
    height: MaybeProp<String>,
    /// Placeholder height in pixels. Inherits from parent [`Skeleton`] when omitted.
    #[prop(optional, into)]
    size: Option<Signal<SkeletonItemSize>>,
    /// Placeholder geometry. Inherits from parent [`Skeleton`] when omitted.
    #[prop(optional, into)]
    shape: Option<Signal<SkeletonItemShape>>,
) -> impl IntoView {
    view! {
        <BaseSkeletonItem class=class width=width height=height nostrip:size=size nostrip:shape=shape />
    }
}
