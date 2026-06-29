use icondata_core::Icon as IconData;
use leptos::prelude::*;
use orbital::components::{Button, ButtonAppearance, Tooltip};

const GITHUB_SPONSOR_URL: &str = "https://github.com/sponsors/deathbreakfast";
const GITHUB_REPO_URL: &str = "https://github.com/unified-field-dev/orbital";
const PATREON_URL: &str = "https://www.patreon.com/cw/u4202798";

fn open_external(url: &'static str) {
    #[cfg(target_arch = "wasm32")]
    if let Some(window) = leptos::web_sys::window() {
        let _ = window.open_with_url_and_target(url, "_blank");
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = url;
}

/// Sponsor, GitHub, and Patreon icon buttons for the catalog AppBar trailing region.
#[component]
pub fn PreviewToolbarLinks() -> impl IntoView {
    view! {
        <PreviewToolbarIconButton
            href=GITHUB_SPONSOR_URL
            icon=icondata::BiDonateHeartRegular
            label="GitHub Sponsors"
            test_id="preview-toolbar-link-sponsor"
        />
        <PreviewToolbarIconButton
            href=GITHUB_REPO_URL
            icon=icondata::BiGithub
            label="GitHub repository"
            test_id="preview-toolbar-link-github"
        />
        <PreviewToolbarIconButton
            href=PATREON_URL
            icon=icondata::BiPatreon
            label="Patreon"
            test_id="preview-toolbar-link-patreon"
        />
    }
}

#[component]
fn PreviewToolbarIconButton(
    href: &'static str,
    icon: IconData,
    label: &'static str,
    test_id: &'static str,
) -> impl IntoView {
    view! {
        <div data-testid=test_id aria-label=label>
            <Tooltip content=label>
                <Button
                    appearance=ButtonAppearance::Transparent
                    icon=icon
                    on_click=Callback::new(move |_| open_external(href))
                />
            </Tooltip>
        </div>
    }
}
