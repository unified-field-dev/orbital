use leptos::{context::Provider, prelude::*};

use super::{SkeletonInjection, SkeletonItemShape, SkeletonItemSize};

#[component]
pub fn BaseSkeleton(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] size: Option<Signal<SkeletonItemSize>>,
    #[prop(optional, into)] shape: Option<Signal<SkeletonItemShape>>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-skeleton".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role="progressbar"
            aria-busy="true"
        >
            <Provider value=SkeletonInjection { size, shape }>{children()}</Provider>
        </div>
    }
}

#[component]
pub fn BaseSkeletonItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] width: MaybeProp<String>,
    #[prop(optional, into)] height: MaybeProp<String>,
    #[prop(optional, into)] size: Option<Signal<SkeletonItemSize>>,
    #[prop(optional, into)] shape: Option<Signal<SkeletonItemShape>>,
) -> impl IntoView {
    let context = SkeletonInjection::use_context();

    let modifier_class = Signal::derive(move || {
        let resolved_size = size
            .map(|signal| signal.get())
            .or_else(|| {
                context
                    .as_ref()
                    .and_then(|ctx| ctx.size.map(|signal| signal.get()))
            })
            .unwrap_or_default();

        let resolved_shape = shape
            .map(|signal| signal.get())
            .or_else(|| {
                context
                    .as_ref()
                    .and_then(|ctx| ctx.shape.map(|signal| signal.get()))
            })
            .unwrap_or_default();

        format!(
            "orbital-skeleton-item--{} orbital-skeleton-item--size-{}",
            resolved_shape.as_str(),
            resolved_size.px()
        )
    });

    view! {
        <div
            class=move || {
                let mut parts = vec![
                    "orbital-skeleton-item".to_string(),
                    modifier_class.get(),
                ];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            style=move || {
                let mut parts = Vec::new();
                if let Some(w) = width.get() {
                    parts.push(format!("width: {w}"));
                }
                if let Some(h) = height.get() {
                    parts.push(format!("height: {h}"));
                }
                parts.join("; ")
            }
            aria-hidden="true"
        ></div>
    }
}
