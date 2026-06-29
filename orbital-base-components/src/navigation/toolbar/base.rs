use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToolbarSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ToolbarSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

#[component]
pub fn BaseToolbar(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] vertical: bool,
    #[prop(optional, into)] size: Signal<ToolbarSize>,
    children: Children,
) -> impl IntoView {
    let orientation = move || if vertical { "vertical" } else { "horizontal" };

    view! {
        <div
            class=move || {
                let mut parts = vec![
                    "orbital-toolbar".to_string(),
                    format!("orbital-toolbar--{}", size.get().as_str()),
                    format!("orbital-toolbar--{}", orientation()),
                ];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role="toolbar"
            aria-orientation=orientation
        >
            {children()}
        </div>
    }
}
