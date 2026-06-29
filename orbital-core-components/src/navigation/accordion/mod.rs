mod accordion;
mod item;
mod styles;

pub use accordion::Accordion;
pub use item::{AccordionHeader, AccordionItem};

#[cfg(feature = "preview")]
pub use accordion::{
    ACCORDION_DESCRIPTION, ACCORDION_DOC, ACCORDION_PREVIEW_REGISTRATION, ACCORDION_PROPS,
};
