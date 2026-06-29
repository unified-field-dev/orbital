use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PresenceStatus {
    #[default]
    Available,
    Away,
    Busy,
    Offline,
    OutOfOffice,
    Unknown,
}

impl PresenceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Away => "away",
            Self::Busy => "busy",
            Self::Offline => "offline",
            Self::OutOfOffice => "out-of-office",
            Self::Unknown => "unknown",
        }
    }

    pub fn aria_label(&self) -> &'static str {
        match self {
            Self::Available => "Available",
            Self::Away => "Away",
            Self::Busy => "Busy",
            Self::Offline => "Offline",
            Self::OutOfOffice => "Out of office",
            Self::Unknown => "Unknown",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PresenceBadgeSize {
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

impl PresenceBadgeSize {
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
pub fn BasePresenceBadge(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] status: Signal<PresenceStatus>,
    #[prop(optional, into)] size: Signal<PresenceBadgeSize>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let standalone = children.is_none();

    view! {
        <span
            class=move || {
                let mut parts = vec![
                    "orbital-presence-badge".to_string(),
                    format!("orbital-presence-badge--{}", status.get().as_str()),
                    format!("orbital-presence-badge--{}", size.get().as_str()),
                ];
                if standalone {
                    parts.push("orbital-presence-badge--standalone".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
        >
            {children.map(|c| c())}
            <span
                class=move || {
                    format!(
                        "orbital-presence-badge__indicator orbital-presence-badge__indicator--{}",
                        status.get().as_str()
                    )
                }
                role="img"
                aria-label=move || status.get().aria_label()
            />
        </span>
    }
}
