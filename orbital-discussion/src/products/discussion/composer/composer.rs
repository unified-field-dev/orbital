use leptos::prelude::*;
use orbital_core_components::{Card, CardContent, MaterialElevation, MaterialVariant};

use crate::{
    can_submit, ComposerContext, DiscussionAttachmentDraft, DiscussionAttachmentValidation,
    DiscussionComposerSubmit, DiscussionFeatures, DiscussionReply,
};

use super::{
    attachments::DiscussionComposerAttachments, citations::DiscussionComposerCitations,
    format_toolbar::DiscussionComposerFormatToolbar, input::DiscussionComposerInput,
    DiscussionComposerReplyBanner, DiscussionComposerToolbar,
};

/// Reply composer with controlled draft, reply banner, and submit flow.
#[component]
pub fn DiscussionComposer(
    /// Controlled composer draft text.
    value: Signal<String>,
    /// Fires when the draft text changes.
    on_input: Callback<String, ()>,
    /// Reply target for the banner and submit parent_id.
    reply_to: Signal<Option<DiscussionReply>>,
    /// Fires when the user dismisses the reply-to banner.
    #[prop(optional)]
    on_dismiss_reply: Option<Callback<(), ()>>,
    /// Fires when the user submits a non-empty draft.
    on_submit: Callback<DiscussionComposerSubmit, ()>,
    /// Disables input and send when true.
    #[prop(optional)]
    disabled: Option<Signal<bool>>,
    /// Draft attachments included on submit.
    #[prop(optional)]
    attachments: Option<Signal<Vec<DiscussionAttachmentDraft>>>,
    /// Feature flags for attach button and draft chips.
    #[prop(default = DiscussionFeatures::default())]
    features: DiscussionFeatures,
    #[prop(default = None)] attachment_validation: Option<DiscussionAttachmentValidation>,
    #[prop(default = None)] on_attachment_reject: Option<Callback<Vec<(String, String)>, ()>>,
    /// Optional tools and hint slot children.
    children: Children,
) -> impl IntoView {
    let disabled = disabled.unwrap_or_else(|| Signal::derive(|| false));
    let external_attachments = attachments;

    let composer_ctx = ComposerContext::new();
    let draft = composer_ctx.draft;
    provide_context(composer_ctx);
    Effect::new(move |_| {
        let external = value.get();
        if draft.get_untracked() != external {
            draft.set(external);
        }
    });
    Effect::new(move |_| {
        let current = draft.get();
        if value.get_untracked() != current {
            on_input.run(current);
        }
    });

    let on_dismiss = Callback::new(move |()| {
        if let Some(cb) = on_dismiss_reply.as_ref() {
            cb.run(());
        }
    });

    let submit = {
        let on_submit = on_submit;
        let reply_to = reply_to;
        let disabled = disabled;
        let features = features;
        let external_attachments = external_attachments;
        Callback::new(move |()| {
            let body = draft.get_untracked();
            if !can_submit(&body, disabled.get_untracked()) {
                return;
            }
            let parent_id = reply_to.get_untracked().map(|r| r.id);
            let attachments = if features.contains(DiscussionFeatures::ATTACHMENTS) {
                composer_ctx.attachment_drafts.get_untracked()
            } else {
                external_attachments
                    .as_ref()
                    .map(|signal| signal.get_untracked())
                    .unwrap_or_default()
            };
            on_submit.run(DiscussionComposerSubmit {
                body_markdown: body.trim().to_string(),
                parent_id,
                attachments,
                citations: if features.contains(DiscussionFeatures::CITATIONS) {
                    composer_ctx.citation_drafts.get_untracked()
                } else {
                    vec![]
                },
            });
            if features.contains(DiscussionFeatures::ATTACHMENTS) {
                composer_ctx.attachment_drafts.set(Vec::new());
            }
            if features.contains(DiscussionFeatures::CITATIONS) {
                composer_ctx.citation_drafts.set(Vec::new());
            }
        })
    };

    let on_form_submit = {
        let submit = submit;
        move |ev: leptos::ev::SubmitEvent| {
            ev.prevent_default();
            submit.run(());
        }
    };

    let on_keydown = {
        let submit = submit;
        move |ev: leptos::ev::KeyboardEvent| {
            if ev.key() == "Enter" && !ev.shift_key() && !ev.is_composing() {
                ev.prevent_default();
                submit.run(());
            }
        }
    };

    view! {
        <form
            class="orbital-discussion__composer"
            on:submit=on_form_submit
            on:keydown=on_keydown
        >
            <Card variant=MaterialVariant::Outlined elevation=MaterialElevation::Flat>
                <CardContent class="orbital-discussion__composer-content".to_string()>
                    <DiscussionComposerReplyBanner reply_to=reply_to on_dismiss=on_dismiss />
                    <DiscussionComposerFormatToolbar disabled=disabled />
                    <DiscussionComposerInput value=draft disabled=disabled />
                    <DiscussionComposerAttachments />
                    <DiscussionComposerCitations />
                    {children()}
                    <DiscussionComposerToolbar
                        value=Signal::derive(move || draft.get())
                        disabled=disabled
                        attachment_validation=attachment_validation.clone()
                        on_attachment_reject=on_attachment_reject
                    />
                </CardContent>
            </Card>
        </form>
    }
}
