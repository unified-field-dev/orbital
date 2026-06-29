use leptos::prelude::*;
use orbital_core_components::{Button, ButtonAppearance, ButtonType, Toolbar};

use crate::{
    add_citation_draft, apply_markdown_wrap, insert_at_caret, insert_markdown_image,
    insert_markdown_link, insert_markdown_prefix, use_discussion, use_discussion_composer,
    DiscussionCitation, DiscussionFeatures,
};

use super::citation_dialog::DiscussionComposerCitationDialog;

/// Markdown formatting toolbar above the composer textarea.
#[component]
pub fn DiscussionComposerFormatToolbar(disabled: Signal<bool>) -> impl IntoView {
    let ctx = use_discussion();
    let composer = use_discussion_composer();
    let show = Memo::new(move |_| ctx.features.contains(DiscussionFeatures::MARKDOWN));
    let citation_dialog_open = RwSignal::new(false);

    let on_bold = {
        let composer = composer;
        move |_| apply_markdown_wrap(composer.draft, composer, "**", "**")
    };
    let on_italic = {
        let composer = composer;
        move |_| apply_markdown_wrap(composer.draft, composer, "_", "_")
    };
    let on_link = {
        let draft = composer.draft;
        move |_| insert_markdown_link(draft, "link", "https://example.com")
    };
    let on_list = {
        let draft = composer.draft;
        move |_| insert_markdown_prefix(draft, "- ")
    };
    let on_code = {
        let composer = composer;
        move |_| apply_markdown_wrap(composer.draft, composer, "`", "`")
    };
    let on_citation_click = move |_| citation_dialog_open.set(true);
    let on_citation_add = {
        let composer = composer;
        Callback::new(move |(title, url): (String, String)| {
            let id = format!("cit-{}", chrono::Utc::now().timestamp_millis());
            let mut citation = DiscussionCitation::new(&id, title);
            citation.url = Some(url);
            add_citation_draft(composer.citation_drafts, citation);
            insert_at_caret(composer.draft, composer, &format!("[^{id}]"));
        })
    };
    let on_image = {
        let draft = composer.draft;
        move |_| insert_markdown_image(draft, "image", "https://orbital.dev/placeholder.png")
    };

    let show_citations = Memo::new(move |_| ctx.features.contains(DiscussionFeatures::CITATIONS));
    let show_attachments =
        Memo::new(move |_| ctx.features.contains(DiscussionFeatures::ATTACHMENTS));

    view! {
        <Show when=move || show.get()>
            <div
                class="orbital-discussion__composer-format-toolbar"
                data-testid="discussion-composer-format-toolbar"
            >
                <Toolbar>
                    <span data-testid="discussion-composer-format-bold">
                        <Button
                            appearance=ButtonAppearance::Subtle
                            button_type=ButtonType::Button
                            icon=icondata::AiBoldOutlined
                            disabled=disabled
                            on:click=on_bold
                            attr:aria-label="Bold"
                        />
                    </span>
                    <span data-testid="discussion-composer-format-italic">
                        <Button
                            appearance=ButtonAppearance::Subtle
                            button_type=ButtonType::Button
                            icon=icondata::AiItalicOutlined
                            disabled=disabled
                            on:click=on_italic
                            attr:aria-label="Italic"
                        />
                    </span>
                    <span data-testid="discussion-composer-format-link">
                        <Button
                            appearance=ButtonAppearance::Subtle
                            button_type=ButtonType::Button
                            icon=icondata::AiLinkOutlined
                            disabled=disabled
                            on:click=on_link
                            attr:aria-label="Insert link"
                        />
                    </span>
                    <span data-testid="discussion-composer-format-list">
                        <Button
                            appearance=ButtonAppearance::Subtle
                            button_type=ButtonType::Button
                            icon=icondata::AiUnorderedListOutlined
                            disabled=disabled
                            on:click=on_list
                            attr:aria-label="Bullet list"
                        />
                    </span>
                    <span data-testid="discussion-composer-format-code">
                        <Button
                            appearance=ButtonAppearance::Subtle
                            button_type=ButtonType::Button
                            icon=icondata::AiCodeOutlined
                            disabled=disabled
                            on:click=on_code
                            attr:aria-label="Inline code"
                        />
                    </span>
                    <Show when=move || show_citations.get()>
                        <span data-testid="discussion-composer-format-citation">
                            <Button
                                appearance=ButtonAppearance::Subtle
                                button_type=ButtonType::Button
                                icon=icondata::AiBookOutlined
                                disabled=disabled
                                on:click=on_citation_click
                                attr:aria-label="Insert citation"
                            />
                        </span>
                    </Show>
                    <Show when=move || show_attachments.get()>
                        <span data-testid="discussion-composer-format-image">
                            <Button
                                appearance=ButtonAppearance::Subtle
                                button_type=ButtonType::Button
                                icon=icondata::AiPictureOutlined
                                disabled=disabled
                                on:click=on_image
                                attr:aria-label="Insert image"
                            />
                        </span>
                    </Show>
                </Toolbar>
            </div>
            <Show when=move || show_citations.get()>
                <DiscussionComposerCitationDialog
                    open=citation_dialog_open
                    on_add=on_citation_add
                    disabled=disabled
                />
            </Show>
        </Show>
    }
}
