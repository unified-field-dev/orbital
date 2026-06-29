use leptos::prelude::*;
use orbital::preview::PreviewRegistration;

use super::IntroductionPage;

#[component]
fn IntroductionPreview() -> impl IntoView {
    view! { <IntroductionPage /> }
}

static INTRODUCTION_PREVIEW_REGISTRATION: PreviewRegistration = PreviewRegistration {
    slug: "",
    label: "Introduction",
    section: "",
    section_priority: 0,
    category: "",
    category_priority: 0,
    category_default_collapsed: false,
    group: "",
    group_priority: 0,
    nav_item: true,
    icon: icondata::AiFileOutlined,
    render: || IntroductionPreview().into_any(),
};

pub fn introduction_preview_registration() -> &'static PreviewRegistration {
    &INTRODUCTION_PREVIEW_REGISTRATION
}
