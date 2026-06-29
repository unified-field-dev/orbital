use leptos::prelude::*;
use orbital_base_components::Handler;
use orbital_core_components::Tag;

use crate::{remove_citation_draft, use_discussion, use_discussion_composer, DiscussionFeatures};

/// Draft citation chips above the composer action row.
#[component]
pub fn DiscussionComposerCitations() -> impl IntoView {
    let ctx = use_discussion();
    let composer = use_discussion_composer();
    let show = Memo::new(move |_| {
        ctx.features.contains(DiscussionFeatures::CITATIONS)
            && !composer.citation_drafts.get().is_empty()
    });

    view! {
        <Show when=move || show.get()>
            <div
                class="orbital-discussion__composer-citations"
                data-testid="discussion-composer-citations"
            >
                <For
                    each=move || composer.citation_drafts.get()
                    key=|citation| citation.id.clone()
                    children=move |citation| {
                        let id = citation.id.clone();
                        let label = citation.title.clone();
                        let on_dismiss = {
                            let id = id.clone();
                            Handler::on(move |_| remove_citation_draft(composer.citation_drafts, &id))
                        };
                        view! {
                            <span data-testid=format!("discussion-composer-citation-{}", id)>
                                <Tag
                                    dismissible=Signal::derive(|| true)
                                    on_dismiss=on_dismiss
                                >
                                    {label}
                                </Tag>
                            </span>
                        }
                    }
                />
            </div>
        </Show>
    }
}
