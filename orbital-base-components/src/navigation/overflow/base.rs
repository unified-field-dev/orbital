use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OverflowDirection {
    #[default]
    Horizontal,
    Vertical,
    Both,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OverflowAxes {
    #[default]
    Horizontal,
    Vertical,
    Both,
}

#[derive(Clone, Debug, Default)]
pub struct OverflowChangeData {
    pub has_overflow: bool,
}

#[component]
pub fn BaseOverflow(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] overflow_direction: Signal<OverflowDirection>,
    #[prop(optional, into)] has_overflow: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let mut parts = vec![
                    "orbital-overflow".to_string(),
                    format!(
                        "orbital-overflow--{}",
                        direction_as_str(overflow_direction.get())
                    ),
                ];
                if has_overflow.get() {
                    parts.push("orbital-overflow--clipped".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            data-overflow=move || has_overflow.get().to_string()
        >
            {children()}
        </div>
    }
}

fn direction_as_str(direction: OverflowDirection) -> &'static str {
    match direction {
        OverflowDirection::Horizontal => "horizontal",
        OverflowDirection::Vertical => "vertical",
        OverflowDirection::Both => "both",
    }
}
