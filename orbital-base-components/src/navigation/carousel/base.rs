use leptos::html;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CarouselStepperLayout {
    #[default]
    Inline,
    Bottom,
}

#[component]
pub fn BaseCarousel(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-carousel".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role="region"
            aria-roledescription="carousel"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn BaseCarouselViewport(
    #[prop(optional, into)] class: MaybeProp<String>,
    node_ref: NodeRef<html::Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                let mut parts = vec!["orbital-carousel__viewport".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role="group"
            aria-live="polite"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn BaseCarouselSlide(
    #[prop(optional, into)] class: MaybeProp<String>,
    index: i32,
    #[prop(optional, into)] active: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-carousel__slide".to_string()];
                if active.get() {
                    parts.push("orbital-carousel__slide--active".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role="group"
            aria-roledescription="slide"
            aria-label=move || format!("Slide {}", index + 1)
            aria-hidden=move || (!active.get()).to_string()
            tabindex=move || if active.get() { "0" } else { "-1" }
            data-slide-index=index
        >
            {children()}
        </div>
    }
}

#[component]
pub fn BaseCarouselStepper(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] layout: Signal<CarouselStepperLayout>,
    children: Children,
) -> impl IntoView {
    view! {
        <nav
            class=move || {
                let mut parts = vec!["orbital-carousel__stepper".to_string()];
                parts.push(match layout.get() {
                    CarouselStepperLayout::Inline => "orbital-carousel__stepper--inline",
                    CarouselStepperLayout::Bottom => "orbital-carousel__stepper--bottom",
                }.to_string());
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            aria-label="Carousel stepper"
        >
            {children()}
        </nav>
    }
}
