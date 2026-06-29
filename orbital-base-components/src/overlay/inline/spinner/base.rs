use leptos::prelude::*;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SpinnerSize {
    ExtraTiny,
    Tiny,
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
    Huge,
}

impl SpinnerSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExtraTiny => "extra-tiny",
            Self::Tiny => "tiny",
            Self::ExtraSmall => "extra-small",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::ExtraLarge => "extra-large",
            Self::Huge => "huge",
        }
    }
}

#[component]
pub fn BaseSpinner(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] size: Signal<SpinnerSize>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let id = StoredValue::new(uuid::Uuid::new_v4().to_string());
    let spinner_label = label;
    let children_flag = children.is_some();
    let labelledby = move || {
        spinner_label.with(|label| {
            if label.is_some() || children_flag {
                Some(id.get_value())
            } else {
                None
            }
        })
    };

    view! {
        <div
            class=move || {
                let mut parts = vec![
                    "orbital-spinner".to_string(),
                    format!("orbital-spinner--{}", size.get().as_str()),
                ];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role="progressbar"
            aria-labelledby=labelledby
        >
            <span class="orbital-spinner__spinner" aria-hidden="true">
                <span class="orbital-spinner__spinner-tail"></span>
            </span>
            {label.get().map(|text| view! {
                <span class="orbital-spinner__label" id=id.get_value()>{text}</span>
            })}
            {children.map(|children| view! {
                <span class="orbital-spinner__label" id=id.get_value()>{children()}</span>
            })}
        </div>
    }
}
