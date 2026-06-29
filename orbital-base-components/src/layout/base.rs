use leptos::prelude::*;

use super::types::LayoutPosition;

/// Headless application shell root — structure only, no theme styling.
#[component]
pub fn BaseLayout(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] data_testid: MaybeProp<String>,
    #[prop(default = false)] overlay_header: bool,
    #[prop(default = LayoutPosition::Static)] position: LayoutPosition,
    children: Children,
) -> impl IntoView {
    let root_class = Memo::new(move |_| {
        let extra = class.get().unwrap_or_default();
        let mut parts = vec!["orbital-layout".to_string()];
        if overlay_header {
            parts.push("orbital-layout--overlay-header".to_string());
        }
        parts.push(position.modifier_class().to_string());
        if !extra.is_empty() {
            parts.push(extra);
        }
        parts.join(" ")
    });

    let root_style = move || style.get().unwrap_or_default();
    let overlay_header_attr = move || overlay_header.to_string();

    if let Some(test_id) = data_testid.get() {
        view! {
            <div
                class=root_class
                style=root_style
                data-testid=test_id
                data-layout-overlay-header=overlay_header_attr
                data-layout-position=position.as_data()
            >
                {children()}
            </div>
        }
        .into_any()
    } else {
        view! {
            <div
                class=root_class
                style=root_style
                data-layout-overlay-header=overlay_header_attr
                data-layout-position=position.as_data()
            >
                {children()}
            </div>
        }
        .into_any()
    }
}

/// Headless flex row body beneath the layout header region.
#[component]
pub fn BaseLayoutBody(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let extra = class.get().unwrap_or_default();
                if extra.is_empty() {
                    "orbital-layout__body".to_string()
                } else {
                    format!("orbital-layout__body {extra}")
                }
            }
            style=move || style.get().unwrap_or_default()
        >
            {children()}
        </div>
    }
}

/// Headless side navigation region.
#[component]
pub fn BaseLayoutSidebar(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <aside class=move || {
            let extra = class.get().unwrap_or_default();
            if extra.is_empty() {
                "orbital-layout__sidebar".to_string()
            } else {
                format!("orbital-layout__sidebar {extra}")
            }
        }>
            {children()}
        </aside>
    }
}

/// Headless scrollable main content region.
#[component]
pub fn BaseLayoutMain(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <main class=move || {
            let extra = class.get().unwrap_or_default();
            if extra.is_empty() {
                "orbital-layout__main".to_string()
            } else {
                format!("orbital-layout__main {extra}")
            }
        }>
            {children()}
        </main>
    }
}

/// Headless scrollable spacer matching overlay header height.
#[component]
pub fn BaseLayoutHeaderInset(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = 56)] height_px: u16,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let extra = class.get().unwrap_or_default();
                if extra.is_empty() {
                    "orbital-layout-header-inset".to_string()
                } else {
                    format!("orbital-layout-header-inset {extra}")
                }
            }
            role="presentation"
            aria-hidden="true"
            style=move || format!("height: {height_px}px; min-height: {height_px}px;")
        ></div>
    }
}
