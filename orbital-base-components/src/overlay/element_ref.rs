use leptos::html::ElementType;
use send_wrapper::SendWrapper;
use std::ops::Deref;

/// Any DOM element reference type for generic `NodeRef` attachment.
#[derive(Debug, Clone)]
pub struct AnyElement {
    el: SendWrapper<web_sys::Element>,
}

impl ElementType for AnyElement {
    type Output = web_sys::Element;

    const TAG: &'static str = "";

    const SELF_CLOSING: bool = false;

    const ESCAPE_CHILDREN: bool = false;

    const NAMESPACE: Option<&'static str> = None;

    fn tag(&self) -> &str {
        ""
    }
}

impl Deref for AnyElement {
    type Target = web_sys::Element;

    fn deref(&self) -> &Self::Target {
        &self.el
    }
}

/// Any HTML element reference type for generic `NodeRef` attachment.
#[derive(Debug, Clone)]
pub struct AnyHtmlElement {
    el: SendWrapper<web_sys::HtmlElement>,
}

impl ElementType for AnyHtmlElement {
    type Output = web_sys::HtmlElement;

    const TAG: &'static str = "";

    const SELF_CLOSING: bool = false;

    const ESCAPE_CHILDREN: bool = false;

    const NAMESPACE: Option<&'static str> = None;

    fn tag(&self) -> &str {
        ""
    }
}

impl Deref for AnyHtmlElement {
    type Target = web_sys::HtmlElement;

    fn deref(&self) -> &Self::Target {
        &self.el
    }
}
