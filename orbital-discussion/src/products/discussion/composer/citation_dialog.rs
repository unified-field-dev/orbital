use leptos::prelude::*;
use orbital_base_components::InputType;
use orbital_core_components::{
    Button, ButtonAppearance, ButtonType, Dialog, DialogActions, DialogBody, DialogContent,
    DialogSurface, DialogTitle, Field, Input, InputAppearance,
};

/// Modal for entering citation title and URL before inserting a draft chip + ref token.
#[component]
pub fn DiscussionComposerCitationDialog(
    open: RwSignal<bool>,
    on_add: Callback<(String, String), ()>,
    disabled: Signal<bool>,
) -> impl IntoView {
    let title = RwSignal::new(String::new());
    let url = RwSignal::new(String::new());

    Effect::new(move |_| {
        if !open.get() {
            title.set(String::new());
            url.set(String::new());
        }
    });

    let on_cancel = {
        let open = open;
        move |_| open.set(false)
    };

    let on_confirm = {
        let open = open;
        let on_add = on_add;
        move |_| {
            let url_val = url.get().trim().to_string();
            if url_val.is_empty() {
                return;
            }
            let title_val = title.get().trim().to_string();
            let title_val = if title_val.is_empty() {
                title_from_url(&url_val)
            } else {
                title_val
            };
            on_add.run((title_val, url_val));
            open.set(false);
        }
    };

    let confirm_disabled = Memo::new(move |_| disabled.get() || url.get().trim().is_empty());

    view! {
        <Dialog open=open>
            <DialogSurface>
                <DialogBody>
                    <div
                        class="orbital-discussion__composer-citation-dialog"
                        data-testid="discussion-composer-citation-dialog"
                    >
                        <DialogTitle>"Add citation"</DialogTitle>
                        <DialogContent>
                            <div class="orbital-discussion__composer-citation-dialog-fields">
                            <Field label="Title">
                                <Input
                                    bind=title
                                    appearance=InputAppearance {
                                        disabled,
                                        placeholder: "Optional — defaults to URL host".into(),
                                        ..Default::default()
                                    }
                                    attr:data-testid="discussion-composer-citation-dialog-title"
                                />
                            </Field>
                            <Field label="URL">
                                <Input
                                    bind=url
                                    appearance=InputAppearance {
                                        disabled,
                                        placeholder: "https://example.com/doc".into(),
                                        input_type: Signal::from(InputType::Url),
                                        ..Default::default()
                                    }
                                    attr:data-testid="discussion-composer-citation-dialog-url"
                                />
                            </Field>
                        </div>
                    </DialogContent>
                    <DialogActions>
                        <Button
                            appearance=ButtonAppearance::Secondary
                            button_type=ButtonType::Button
                            disabled=disabled
                            on:click=on_cancel
                        >
                            "Cancel"
                        </Button>
                        <Button
                            appearance=ButtonAppearance::Primary
                            button_type=ButtonType::Button
                            disabled=confirm_disabled
                            on:click=on_confirm
                            attr:data-testid="discussion-composer-citation-dialog-add"
                        >
                            "Add citation"
                        </Button>
                    </DialogActions>
                    </div>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}

fn title_from_url(url: &str) -> String {
    url.trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()
        .unwrap_or(url)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_from_url_uses_host() {
        assert_eq!(
            title_from_url("https://orbital.dev/docs/design"),
            "orbital.dev"
        );
    }
}
