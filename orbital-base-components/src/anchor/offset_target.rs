use web_sys::Element;

#[cfg(any(feature = "hydrate", not(feature = "ssr")))]
use leptos::prelude::document;
#[cfg(any(feature = "hydrate", not(feature = "ssr")))]
use web_sys::DomRect;

/// Scroll container used to calculate anchor link offsets.
pub enum OffsetTarget {
    Selector(String),
    Element(Element),
}

impl From<&'static str> for OffsetTarget {
    fn from(value: &'static str) -> Self {
        Self::Selector(value.to_string())
    }
}

impl From<String> for OffsetTarget {
    fn from(value: String) -> Self {
        Self::Selector(value)
    }
}

impl From<Element> for OffsetTarget {
    fn from(value: Element) -> Self {
        Self::Element(value)
    }
}

#[cfg(any(feature = "hydrate", not(feature = "ssr")))]
impl OffsetTarget {
    pub(crate) fn element(&self) -> Option<Element> {
        match self {
            OffsetTarget::Selector(selector) => document().query_selector(selector).ok().flatten(),
            OffsetTarget::Element(el) => Some(el.clone()),
        }
    }

    pub(crate) fn bounding_client_rect(&self) -> Option<DomRect> {
        self.element().map(|el| el.get_bounding_client_rect())
    }
}

#[cfg(test)]
mod tests {
    use super::OffsetTarget;

    #[test]
    fn offset_target_from_static_str() {
        match OffsetTarget::from("#scroll") {
            OffsetTarget::Selector(value) => assert_eq!(value, "#scroll"),
            _ => panic!("expected selector variant"),
        }
    }
}
