use leptos::prelude::*;
use orbital_core_components::Button;

use crate::{navigate_focus_to, use_discussion, use_discussion_locale, DiscussionReplyGraph};

/// Drill-in affordance when nested replies exceed the visible depth cap.
#[component]
pub fn DiscussionReplyShowMore(
    reply_id: String,
    hidden_count: u32,
    child_count: usize,
    graph: Memo<DiscussionReplyGraph>,
) -> impl IntoView {
    let ctx = use_discussion();
    let locale = use_discussion_locale();
    let label_count = if hidden_count > 0 {
        hidden_count
    } else {
        child_count.max(1) as u32
    };
    let test_id = format!("discussion-show-more-{reply_id}");
    let button_label = Memo::new(move |_| locale.get().show_more_replies(label_count));

    view! {
        <div class="orbital-discussion__show-more">
            <Button
                class="orbital-discussion__show-more-button".to_string()
                attr:aria-label=move || button_label.get()
                on:click=move |_| {
                    navigate_focus_to(ctx, &reply_id, &graph.get_untracked());
                }
            >
                {move || button_label.get()}
            </Button>
            <span data-testid=test_id style="display:none" />
        </div>
    }
}
