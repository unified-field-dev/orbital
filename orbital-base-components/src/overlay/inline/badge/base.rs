use leptos::prelude::*;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum BadgeAppearance {
    #[default]
    Filled,
    Tint,
    Outline,
    Ghost,
}

impl BadgeAppearance {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Filled => "filled",
            Self::Tint => "tint",
            Self::Outline => "outline",
            Self::Ghost => "ghost",
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum BadgeColor {
    #[default]
    Brand,
    Danger,
    Important,
    Informative,
    Severe,
    Subtle,
    Success,
    Warning,
}

impl BadgeColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Brand => "brand",
            Self::Danger => "danger",
            Self::Important => "important",
            Self::Informative => "informative",
            Self::Severe => "severe",
            Self::Subtle => "subtle",
            Self::Success => "success",
            Self::Warning => "warning",
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum BadgeSize {
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

impl BadgeSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExtraSmall => "extra-small",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::ExtraLarge => "extra-large",
        }
    }
}

#[component]
pub fn BaseBadge(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] appearance: Signal<BadgeAppearance>,
    #[prop(optional, into)] color: Signal<BadgeColor>,
    #[prop(optional, into)] size: Signal<BadgeSize>,
    children: Children,
) -> impl IntoView {
    view! {
        <span
            class=move || {
                let mut parts = vec![
                    "orbital-badge".to_string(),
                    format!("orbital-badge--{}", appearance.get().as_str()),
                    format!("orbital-badge--{}", color.get().as_str()),
                    format!("orbital-badge--{}", size.get().as_str()),
                ];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
        >
            {children()}
        </span>
    }
}
