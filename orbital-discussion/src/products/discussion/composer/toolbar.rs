use leptos::{html, prelude::*};
use orbital_core_components::{Button, ButtonAppearance, ButtonType};

#[cfg(feature = "hydrate")]
use crate::{
    add_attachment_draft, use_discussion_composer, validate_attachment_metadata,
    DiscussionAttachmentDraft,
};
use crate::{can_submit, use_discussion, DiscussionAttachmentValidation, DiscussionFeatures};

/// Send and attach button row for the composer.
#[component]
pub fn DiscussionComposerToolbar(
    value: Signal<String>,
    disabled: Signal<bool>,
    #[prop(default = None)] attachment_validation: Option<DiscussionAttachmentValidation>,
    #[prop(default = None)] on_attachment_reject: Option<Callback<Vec<(String, String)>, ()>>,
) -> impl IntoView {
    #[cfg(not(feature = "hydrate"))]
    let _ = (&attachment_validation, &on_attachment_reject);
    let ctx = use_discussion();
    #[cfg(feature = "hydrate")]
    let composer = use_discussion_composer();
    let send_disabled = Memo::new(move |_| !can_submit(&value.get(), disabled.get()));
    let show_attach = Memo::new(move |_| ctx.features.contains(DiscussionFeatures::ATTACHMENTS));
    let file_input_ref = NodeRef::<html::Input>::new();

    let on_attach_click = move |_| {
        if let Some(input) = file_input_ref.get() {
            input.click();
        }
    };

    #[cfg(feature = "hydrate")]
    let validation = StoredValue::new(attachment_validation);
    #[cfg(feature = "hydrate")]
    let on_reject = StoredValue::new(on_attachment_reject);

    let on_file_change = move |ev: leptos::ev::Event| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::JsCast;

            let Some(input) = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            else {
                return;
            };

            let Some(file_list) = input.files() else {
                return;
            };

            let mut rejections = Vec::new();
            let validation = validation.get_value();

            for index in 0..file_list.length() {
                let Some(file) = file_list.item(index) else {
                    continue;
                };

                let name = file.name();
                let mime = if file.type_().is_empty() {
                    None
                } else {
                    Some(file.type_())
                };
                let size_bytes = Some(file.size() as u64);
                let current_count = composer.attachment_drafts.get_untracked().len();

                match validate_attachment_metadata(
                    &name,
                    mime.as_deref(),
                    size_bytes,
                    current_count,
                    validation.as_ref(),
                ) {
                    Ok(()) => {
                        let id = format!("att-{}-{}", chrono::Utc::now().timestamp_millis(), index);
                        let mut draft = DiscussionAttachmentDraft::new(id, name);
                        draft.mime = mime;
                        draft.size_bytes = size_bytes;
                        add_attachment_draft(composer.attachment_drafts, draft);
                    }
                    Err(reason) => {
                        rejections.push((file.name(), reason.to_string()));
                    }
                }
            }

            if !rejections.is_empty() {
                if let Some(cb) = on_reject.get_value().as_ref() {
                    cb.run(rejections);
                }
            }

            input.set_value("");
        }

        #[cfg(not(feature = "hydrate"))]
        {
            let _ = ev;
        }
    };

    view! {
        <div class="orbital-discussion__composer-toolbar">
            <Show when=move || show_attach.get()>
                <>
                    <input
                        node_ref=file_input_ref
                        type="file"
                        multiple=true
                        style="display: none;"
                        on:change=on_file_change
                        data-testid="discussion-composer-file-input"
                    />
                    <span data-testid="discussion-composer-attach">
                        <Button
                            appearance=ButtonAppearance::Secondary
                            button_type=ButtonType::Button
                            icon=icondata::AiPaperClipOutlined
                            on:click=on_attach_click
                            attr:aria-label="Attach file"
                        >
                            "Attach"
                        </Button>
                    </span>
                </>
            </Show>
            <span data-testid="discussion-composer-send">
                <Button
                    appearance=ButtonAppearance::Primary
                    button_type=ButtonType::Submit
                    disabled=Signal::derive(move || send_disabled.get())
                >
                    "Send"
                </Button>
            </span>
        </div>
    }
}
