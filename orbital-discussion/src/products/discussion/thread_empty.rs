use leptos::prelude::*;

/// Built-in empty thread message when no custom [`DiscussionEmptyView`] slot is provided.
#[component]
pub fn DiscussionDefaultEmptyView() -> impl IntoView {
    view! {
        <div class="orbital-discussion__empty" data-testid="discussion-empty-default">
            <p class="orbital-discussion__empty-title">"No replies yet"</p>
            <p class="orbital-discussion__empty-hint">"Be the first to reply below."</p>
        </div>
    }
}
