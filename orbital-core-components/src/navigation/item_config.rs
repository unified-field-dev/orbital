use leptos::prelude::*;

/// Link/button settings for [`NavigationItem`](super::item::NavigationItem) and [`NavigationSubItem`](super::item::NavigationSubItem).
#[derive(Clone)]
pub struct NavigationItemConfig {
    pub value: Signal<String>,
    pub href: Option<Signal<String>>,
    pub target: MaybeProp<String>,
    pub badge: Option<String>,
    pub disabled: Signal<bool>,
    pub depth: u8,
    pub on_click: Option<Callback<leptos::ev::MouseEvent>>,
}

impl NavigationItemConfig {
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        Self {
            value: Signal::from(value),
            href: None,
            target: MaybeProp::default(),
            badge: None,
            disabled: Signal::derive(|| false),
            depth: 0,
            on_click: None,
        }
    }

    pub fn from_signal(value: Signal<String>) -> Self {
        Self {
            value,
            href: None,
            target: MaybeProp::default(),
            badge: None,
            disabled: Signal::derive(|| false),
            depth: 0,
            on_click: None,
        }
    }

    pub fn sub(value: impl Into<String>) -> Self {
        Self::new(value).with_depth(1)
    }

    pub fn with_href(mut self, href: Signal<String>) -> Self {
        self.href = Some(href);
        self
    }

    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = MaybeProp::from(target.into());
        self
    }

    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn with_disabled(mut self, disabled: Signal<bool>) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_depth(mut self, depth: u8) -> Self {
        self.depth = depth;
        self
    }

    pub fn with_on_click(mut self, on_click: Callback<leptos::ev::MouseEvent>) -> Self {
        self.on_click = Some(on_click);
        self
    }
}

impl From<&str> for NavigationItemConfig {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for NavigationItemConfig {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
