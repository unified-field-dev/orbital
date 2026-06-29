use leptos::prelude::*;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ImageShape {
    Circular,
    Rounded,
    #[default]
    Square,
}

impl ImageShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Circular => "circular",
            Self::Rounded => "rounded",
            Self::Square => "square",
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ImageFit {
    None,
    Contain,
    Cover,
    #[default]
    Fill,
    ScaleDown,
}

impl ImageFit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Contain => "contain",
            Self::Cover => "cover",
            Self::Fill => "fill",
            Self::ScaleDown => "scale-down",
        }
    }
}

#[component]
pub fn BaseImage(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] src: MaybeProp<String>,
    #[prop(optional, into)] alt: MaybeProp<String>,
    #[prop(optional, into)] width: MaybeProp<String>,
    #[prop(optional, into)] height: MaybeProp<String>,
    #[prop(optional, into)] shape: Signal<ImageShape>,
    #[prop(optional, into)] block: Signal<bool>,
    #[prop(optional, into)] shadow: Signal<bool>,
    #[prop(optional, into)] fit: Signal<ImageFit>,
) -> impl IntoView {
    view! {
        <img
            class=move || {
                let mut parts = vec![
                    "orbital-image".to_string(),
                    format!("orbital-image--{}", shape.get().as_str()),
                    format!("orbital-image--fit-{}", fit.get().as_str()),
                ];
                if block.get() {
                    parts.push("orbital-image--block".to_string());
                }
                if shadow.get() {
                    parts.push("orbital-image--shadow".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            src=move || src.get()
            alt=move || alt.get()
            width=move || width.get()
            height=move || height.get()
        />
    }
}
