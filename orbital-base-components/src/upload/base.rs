use leptos::{ev, html, prelude::*};

use crate::{overlay::dom_events::add_event_listener, Handler, UploadFileList};

/// Headless file upload — hidden input, trigger slot, and drag/drop forwarding.
#[component]
pub fn BaseUpload(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] accept: Signal<String>,
    #[prop(optional, into)] multiple: Signal<bool>,
    #[prop(into)] on_change: Handler<UploadFileList>,
    children: Children,
) -> impl IntoView {
    let input_ref = NodeRef::<html::Input>::new();
    let trigger_ref = NodeRef::<html::Div>::new();
    let is_trigger_dragover = RwSignal::new(false);

    Effect::new(move |_| {
        let Some(trigger_el) = trigger_ref.get() else {
            return;
        };
        let handle = add_event_listener(trigger_el, ev::click, move |_| {
            if let Some(input_el) = input_ref.get_untracked() {
                input_el.click();
            }
        });
        on_cleanup(move || handle.remove());
    });

    let dispatch_files = {
        let on_change = on_change.clone();
        move |files: UploadFileList| {
            on_change.run(files);
        }
    };

    let on_input_change = {
        let dispatch_files = dispatch_files.clone();
        move |_| {
            if let Some(input_el) = input_ref.get_untracked() {
                if let Some(files) = input_el.files() {
                    dispatch_files(files);
                }
                input_el.set_value("");
            }
        }
    };

    let on_trigger_drop = {
        let dispatch_files = dispatch_files.clone();
        move |event: ev::DragEvent| {
            event.prevent_default();
            if let Some(data) = event.data_transfer() {
                if let Some(files) = data.files() {
                    dispatch_files(files);
                }
            }
            is_trigger_dragover.set(false);
        }
    };

    let root_class = move || {
        let mut parts = vec!["orbital-upload".to_string()];
        if is_trigger_dragover.get() {
            parts.push("orbital-upload--drag-over".to_string());
        }
        if let Some(extra) = class.get() {
            let extra = extra.trim();
            if !extra.is_empty() {
                parts.push(extra.to_string());
            }
        }
        parts.join(" ")
    };

    view! {
        <div class=root_class>
            <input
                class="orbital-upload__input"
                data-testid="upload-input"
                aria-hidden="true"
                tabindex="-1"
                id=move || id.get()
                name=move || name.get()
                node_ref=input_ref
                type="file"
                accept=move || accept.get()
                multiple=move || multiple.get()
                on:change=on_input_change
            />
            <div
                class="orbital-upload__trigger"
                node_ref=trigger_ref
                on:drop=on_trigger_drop
                on:dragover=move |event: ev::DragEvent| {
                    event.prevent_default();
                    is_trigger_dragover.set(true);
                }
                on:dragenter=move |event: ev::DragEvent| event.prevent_default()
                on:dragleave=move |event: ev::DragEvent| {
                    event.prevent_default();
                    is_trigger_dragover.set(false);
                }
            >
                {children()}
            </div>
        </div>
    }
}
