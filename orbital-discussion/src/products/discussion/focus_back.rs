use leptos::prelude::*;
use orbital_core_components::Button;

use crate::{navigate_focus_back, use_discussion, use_discussion_locale, DiscussionFocus};

/// Go-back bar shown when focus is drilled into a reply branch.
#[component]
pub fn DiscussionFocusBack() -> impl IntoView {
    let ctx = use_discussion();
    let locale = use_discussion_locale();
    let visible = Memo::new(move |_| !matches!(ctx.focus.get(), DiscussionFocus::Root));
    let label = Memo::new(move |_| locale.get().go_back_label.clone());

    view! {
        <Show when=move || visible.get()>
            <div class="orbital-discussion__focus-back">
                <Button
                    class="orbital-discussion__focus-back-button".to_string()
                    attr:aria-label=move || label.get()
                    on:click=move |_| navigate_focus_back(ctx)
                >
                    {move || label.get()}
                </Button>
                <span data-testid="discussion-focus-back" style="display:none" />
            </div>
        </Show>
    }
}
