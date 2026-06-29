use std::sync::Arc;

use leptos::wasm_bindgen::JsCast;
use leptos::{ev, html, prelude::*};
use orbital_style::inject_style;
use web_sys::HtmlElement;

use super::styles::loading_bar_styles;
use crate::ComponentRef;

#[derive(Clone)]
pub(crate) struct LoadingBarRef {
    start: Arc<dyn Fn() + Send + Sync + 'static>,
    finish: Arc<dyn Fn() + Send + Sync + 'static>,
    error: Arc<dyn Fn() + Send + Sync + 'static>,
}

impl LoadingBarRef {
    pub fn start(&self) {
        (self.start)();
    }

    pub fn finish(&self) {
        (self.finish)();
    }

    pub fn error(&self) {
        (self.error)();
    }
}

fn set_style(el: &HtmlElement, property: &str, value: &str) {
    let _ = el.style().set_property(property, value);
}

fn set_style_important(el: &HtmlElement, property: &str, value: &str) {
    let _ = el
        .style()
        .set_property_with_priority(property, value, "important");
}

fn force_reflow(el: &HtmlElement) {
    let _ = el.offset_width();
}

fn freeze_then_fill(el: &HtmlElement, container: &HtmlElement) {
    let current = el.offset_width();
    let parent = container.offset_width();
    if parent > 0 && current > 0 {
        let pct = (current as f64 / parent as f64) * 100.0;
        set_style(el, "transition", "none");
        set_style(el, "max-width", &format!("{pct}%"));
        force_reflow(el);
    } else {
        set_style(el, "transition", "none");
        force_reflow(el);
    }
    set_style_important(el, "max-width", "100%");
    set_style_important(el, "width", "100%");
    set_style(el, "transition", "none");
    force_reflow(el);
}

#[component]
pub(crate) fn BaseLoadingBar(
    #[prop(optional)] comp_ref: ComponentRef<LoadingBarRef>,
) -> impl IntoView {
    inject_style("orbital-loading-bar", loading_bar_styles());

    let container_ref = NodeRef::<html::Div>::new();
    let loading_bar_ref = NodeRef::<html::Div>::new();

    let start = Arc::new(move || {
        let Some(container_el) = container_ref.get_untracked() else {
            return;
        };
        let Some(loading_bar_el) = loading_bar_ref.get_untracked() else {
            return;
        };
        let container_el: HtmlElement = container_el.unchecked_into();
        let loading_bar_el: HtmlElement = loading_bar_el.unchecked_into();

        let _ = container_el.style().set_property("display", "block");
        let _ = loading_bar_el.style().remove_property("background-color");
        set_style(&loading_bar_el, "transition", "none");
        set_style(&loading_bar_el, "max-width", "0");
        force_reflow(&loading_bar_el);
        set_style(&loading_bar_el, "transition", "max-width 4s linear");
        set_style(&loading_bar_el, "max-width", "80%");
    });

    let on_transitionend = move |_: ev::TransitionEvent| {};

    let finish = Arc::new(move || {
        let Some(container_el) = container_ref.get_untracked() else {
            return;
        };
        let Some(loading_bar_el) = loading_bar_ref.get_untracked() else {
            return;
        };
        let container_el: HtmlElement = container_el.unchecked_into();
        let loading_bar_el: HtmlElement = loading_bar_el.unchecked_into();

        let _ = container_el.style().set_property("display", "block");
        freeze_then_fill(&loading_bar_el, &container_el);
    });

    let error = Arc::new(move || {
        let Some(container_el) = container_ref.get_untracked() else {
            return;
        };
        let Some(loading_bar_el) = loading_bar_ref.get_untracked() else {
            return;
        };
        let container_el: HtmlElement = container_el.unchecked_into();
        let loading_bar_el: HtmlElement = loading_bar_el.unchecked_into();

        let _ = container_el.style().set_property("display", "block");
        if loading_bar_el
            .style()
            .get_property_value("max-width")
            .ok()
            .as_deref()
            != Some("100%")
        {
            set_style(&loading_bar_el, "transition", "none");
            set_style(&loading_bar_el, "max-width", "0");
            force_reflow(&loading_bar_el);
        }
        set_style(
            &loading_bar_el,
            "background-color",
            "var(--orb-color-status-danger-fg)",
        );
        freeze_then_fill(&loading_bar_el, &container_el);
    });

    comp_ref.load(LoadingBarRef {
        start,
        finish,
        error,
    });

    view! {
        <div
            class="orbital-loading-bar-container"
            style="display: none"
            node_ref=container_ref
        >
            <div
                class="orbital-loading-bar"
                node_ref=loading_bar_ref
                on:transitionend=on_transitionend
            ></div>
        </div>
    }
}
