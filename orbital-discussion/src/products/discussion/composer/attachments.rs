use leptos::prelude::*;
use orbital_base_components::Handler;
use orbital_core_components::{Tag, TagAppearance, TagSize};

use crate::{remove_attachment_draft, use_discussion, use_discussion_composer, DiscussionFeatures};

/// Draft attachment chips above the composer toolbar.
#[component]
pub fn DiscussionComposerAttachments() -> impl IntoView {
    let ctx = use_discussion();
    if !ctx.features.contains(DiscussionFeatures::ATTACHMENTS) {
        return view! { <span></span> }.into_any();
    }

    let composer = use_discussion_composer();

    view! {
        <Show when=move || !composer.attachment_drafts.get().is_empty()>
            <div class="orbital-discussion__composer-attachments" data-testid="discussion-composer-attachments">
                <For
                    each=move || composer.attachment_drafts.get()
                    key=|draft| draft.id.clone()
                    children=move |draft| {
                        let id = draft.id.clone();
                        let name = draft.name.clone();
                        let testid = format!("discussion-composer-attachment-{id}");
                        view! {
                            <Tag
                                class="orbital-discussion__composer-attachment-chip".to_string()
                                appearance=Signal::derive(|| TagAppearance::Outline)
                                size=Signal::derive(|| TagSize::Small)
                                dismissible=Signal::derive(|| true)
                                on_dismiss=Handler::on(move |_| {
                                    remove_attachment_draft(composer.attachment_drafts, &id);
                                })
                                attr:data-testid=testid
                            >
                                {name}
                            </Tag>
                        }
                    }
                />
            </div>
        </Show>
    }
    .into_any()
}
