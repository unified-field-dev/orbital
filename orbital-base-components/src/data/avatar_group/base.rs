use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarGroupLayout {
    #[default]
    Spread,
    Stack,
}

impl AvatarGroupLayout {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Spread => "spread",
            Self::Stack => "stack",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarGroupSize {
    Tiny,
    #[default]
    S24,
    S28,
    S32,
    S40,
    S56,
    S96,
}

impl AvatarGroupSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tiny => "tiny",
            Self::S24 => "24",
            Self::S28 => "28",
            Self::S32 => "32",
            Self::S40 => "40",
            Self::S56 => "56",
            Self::S96 => "96",
        }
    }

    pub fn px(&self) -> u8 {
        match self {
            Self::Tiny => 4,
            Self::S24 => 24,
            Self::S28 => 28,
            Self::S32 => 32,
            Self::S40 => 40,
            Self::S56 => 56,
            Self::S96 => 96,
        }
    }
}

#[component]
pub fn BaseAvatarGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] layout: Signal<AvatarGroupLayout>,
    #[prop(optional, into)] size: Signal<AvatarGroupSize>,
    #[prop(optional, into)] overflow: MaybeProp<u32>,
    children: Children,
) -> impl IntoView {
    view! {
        <span
            class=move || {
                let mut parts = vec![
                    "orbital-avatar-group".to_string(),
                    format!("orbital-avatar-group--{}", layout.get().as_str()),
                    format!("orbital-avatar-group--size-{}", size.get().as_str()),
                ];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            style=move || format!("--orbital-avatar-group-size: {}px;", size.get().px())
        >
            {children()}
            {move || {
                overflow.get().filter(|n| *n > 0).map(|n| {
                    view! {
                        <span class="orbital-avatar-group__overflow" aria-label=format!("{n} more")>
                            {format!("+{n}")}
                        </span>
                    }
                })
            }}
        </span>
    }
}
