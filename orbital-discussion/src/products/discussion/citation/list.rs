use leptos::prelude::*;
use orbital_core_components::Divider;

use crate::DiscussionCitation;
use crate::DiscussionReply;

use super::row::DiscussionCitationRow;

/// Divider section listing structured citations on a reply card.
#[component]
pub fn DiscussionCitationList(
    reply: DiscussionReply,
    citations: Vec<DiscussionCitation>,
) -> impl IntoView {
    if citations.is_empty() {
        return view! { <span></span> }.into_any();
    }

    let reply_for_rows = reply.clone();

    view! {
        <section class="orbital-discussion__citations" data-testid="discussion-citation-list">
            <Divider />
            <h4 class="orbital-discussion__citations-heading">"Citations"</h4>
            <For
                each=move || {
                    citations
                        .iter()
                        .enumerate()
                        .map(|(i, c)| (i + 1, c.clone()))
                        .collect::<Vec<_>>()
                }
                key=|(_, citation)| citation.id.clone()
                children=move |(index, citation)| {
                    view! {
                        <DiscussionCitationRow
                            reply=reply_for_rows.clone()
                            citation=citation
                            index=index
                        />
                    }
                }
            />
        </section>
    }
    .into_any()
}
