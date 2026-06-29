use leptos::prelude::*;
use orbital_core_components::{Skeleton, SkeletonItem, SkeletonItemShape, SkeletonItemSize};

/// Card-shaped skeleton placeholders shown while the host loads replies.
#[component]
pub fn DiscussionThreadLoadingOverlay() -> impl IntoView {
    view! {
        <div class="orbital-discussion__loading" data-testid="discussion-thread-loading">
            <For
                each=|| 0..3
                key=|index| *index
                children=|_| view! {
                    <div class="orbital-discussion__loading-row">
                        <Skeleton>
                            <SkeletonItem
                                size=Signal::derive(|| SkeletonItemSize::S32)
                                shape=Signal::derive(|| SkeletonItemShape::Circle)
                            />
                            <div class="orbital-discussion__loading-lines">
                                <SkeletonItem size=Signal::derive(|| SkeletonItemSize::S16) />
                                <SkeletonItem size=Signal::derive(|| SkeletonItemSize::S16) />
                                <div style="width: 60%;">
                                    <SkeletonItem size=Signal::derive(|| SkeletonItemSize::S16) />
                                </div>
                            </div>
                        </Skeleton>
                    </div>
                }
            />
        </div>
    }
}
