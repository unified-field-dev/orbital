use leptos::prelude::*;
use orbital_core_components::{Button, ButtonAppearance};

use crate::DiscussionReply;

/// Banner shown when replying to a specific author.
#[component]
pub fn DiscussionComposerReplyBanner(
    reply_to: Signal<Option<DiscussionReply>>,
    on_dismiss: Callback<(), ()>,
) -> impl IntoView {
    view! {
        <Show when=move || reply_to.get().is_some()>
            <div
                class="orbital-discussion__composer-reply-banner"
                data-testid="discussion-composer-reply-banner"
            >
                {move || {
                    reply_to
                        .get()
                        .map(|reply| {
                            view! {
                                <span class="orbital-discussion__composer-reply-banner-text">
                                    {format!("Replying to {}", reply.author.display_name)}
                                </span>
                                <Button
                                    appearance=ButtonAppearance::Subtle
                                    class="orbital-discussion__composer-reply-banner-dismiss".to_string()
                                    on:click=move |_| on_dismiss.run(())
                                >
                                    "Dismiss"
                                </Button>
                            }
                        })
                }}
            </div>
        </Show>
    }
}
