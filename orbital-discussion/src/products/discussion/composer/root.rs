use leptos::prelude::*;
use orbital_core_components::{Card, CardContent, MaterialElevation, MaterialVariant};

use crate::{
    can_submit, select_reply_by_id, use_discussion, use_discussion_composer, ComposerReplyTarget,
    DiscussionAttachmentValidation, DiscussionComposerSubmit, DiscussionReply,
};

use super::{
    attachments::DiscussionComposerAttachments, citations::DiscussionComposerCitations,
    format_toolbar::DiscussionComposerFormatToolbar, input::DiscussionComposerInput,
    DiscussionComposerReplyBanner, DiscussionComposerToolbar,
};

/// Form orchestration for the default thread-bottom composer.
#[component]
pub fn DiscussionComposerRoot(
    reply_to: Signal<Option<String>>,
    set_reply_to: WriteSignal<Option<String>>,
    #[prop(default = None)] disabled: Option<Signal<bool>>,
    #[prop(default = None)] attachment_validation: Option<DiscussionAttachmentValidation>,
    #[prop(default = None)] on_attachment_reject: Option<Callback<Vec<(String, String)>, ()>>,
    composer_tools: ChildrenFn,
    composer_hint: ChildrenFn,
) -> impl IntoView {
    let ctx = use_discussion();
    let composer = use_discussion_composer();
    let disabled = disabled.unwrap_or_else(|| Signal::derive(|| false));

    let reply_target: Signal<Option<DiscussionReply>> = Signal::derive(move || {
        reply_to.get().and_then(|id| {
            let graph = ctx.graph.get();
            select_reply_by_id(&graph, &id).cloned()
        })
    });

    provide_context(ComposerReplyTarget {
        reply: reply_target,
    });

    let on_dismiss = Callback::new(move |()| {
        set_reply_to.set(None);
    });

    let submit = Callback::new(move |()| {
        let body = composer.draft.get_untracked();
        if !can_submit(&body, disabled.get_untracked()) {
            return;
        }
        let parent_id = reply_to.get_untracked();
        let payload = DiscussionComposerSubmit {
            body_markdown: body.trim().to_string(),
            parent_id,
            attachments: composer.attachment_drafts.get_untracked(),
            citations: composer.citation_drafts.get_untracked(),
        };
        ctx.events
            .with_value(|events| events.notify_submit(payload));
        composer.draft.set(String::new());
        composer.attachment_drafts.set(Vec::new());
        composer.citation_drafts.set(Vec::new());
        set_reply_to.set(None);
    });

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
            data-composer-disabled=move || disabled.get().then_some("true")
            on:submit=on_form_submit
            on:keydown=on_keydown
        >
            <Card variant=MaterialVariant::Outlined elevation=MaterialElevation::Flat>
                <CardContent class="orbital-discussion__composer-content".to_string()>
                    <DiscussionComposerReplyBanner reply_to=reply_target on_dismiss=on_dismiss />
                    <DiscussionComposerFormatToolbar disabled=disabled />
                    <DiscussionComposerInput value=composer.draft disabled=disabled />
                    <DiscussionComposerAttachments />
                    <DiscussionComposerCitations />
                    {(composer_tools)()}
                    {(composer_hint)()}
                    <DiscussionComposerToolbar
                        value=Signal::derive(move || composer.draft.get())
                        disabled=disabled
                        attachment_validation=attachment_validation.clone()
                        on_attachment_reject=on_attachment_reject
                    />
                </CardContent>
            </Card>
        </form>
    }
}
