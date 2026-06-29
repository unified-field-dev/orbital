use leptos::prelude::*;

use crate::spacing::SpacingInset;

use super::types::{FlexAlign, FlexGap, FlexJustify, FlexWrap};

/// Headless flex container — layout structure only, no theme styling.
#[component]
pub fn BaseFlex(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] gap: FlexGap,
    #[prop(optional)] vertical: bool,
    #[prop(optional, into)] inline: Signal<bool>,
    #[prop(optional, into)] align: MaybeProp<FlexAlign>,
    #[prop(optional, into)] justify: MaybeProp<FlexJustify>,
    #[prop(optional, default = FlexWrap::NoWrap)] wrap: FlexWrap,
    #[prop(optional, default = false)] fill: bool,
    #[prop(optional, default = false)] full_width: bool,
    #[prop(optional, into)] padding: MaybeProp<SpacingInset>,
    #[prop(optional, into)] margin: MaybeProp<SpacingInset>,
    children: Children,
) -> impl IntoView {
    let layout = Memo::new(move |_| {
        let mut s = String::new();
        s.push_str(if inline.get() {
            "display: inline-flex;"
        } else {
            "display: flex;"
        });
        s.push_str(if vertical {
            "flex-direction: column;"
        } else {
            "flex-direction: row;"
        });
        s.push_str(&format!("flex-wrap: {};", wrap.as_str()));
        if fill {
            s.push_str("height: 100%;min-height: 0;");
        }
        if full_width {
            s.push_str("width: 100%;");
        }
        let gap_css = match gap {
            FlexGap::Small => "gap: 4px 8px;",
            FlexGap::Medium => "gap: 8px 12px;",
            FlexGap::Large => "gap: 12px 16px;",
            FlexGap::Size(size) => {
                s.push_str(&format!("gap: {size}px {size}px;"));
                ""
            }
            FlexGap::WH(width, height) => {
                s.push_str(&format!("gap: {width}px {height}px;"));
                ""
            }
        };
        if !gap_css.is_empty() {
            s.push_str(gap_css);
        }
        if let Some(align) = align.get() {
            s.push_str(&format!("align-items: {};", align.as_str()));
        }
        if let Some(justify) = justify.get() {
            s.push_str(&format!("justify-content: {};", justify.as_str()));
        }
        if let Some(padding) = padding.get() {
            s.push_str(&padding.padding_css());
        }
        if let Some(margin) = margin.get() {
            s.push_str(&margin.margin_css());
        }
        s
    });

    view! {
        <div
            class=move || {
                let extra = class.get().unwrap_or_default();
                if extra.is_empty() {
                    "orbital-flex".to_string()
                } else {
                    format!("orbital-flex {extra}")
                }
            }
            style=move || layout.get()
        >
            {children()}
        </div>
    }
}
