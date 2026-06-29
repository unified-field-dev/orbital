//! Preview registration for native core components.

mod component_doc_markdown;
mod component_doc_props;
mod preview_card;
mod preview_card_body;
mod preview_view;
mod registration;
pub mod static_registrations;

pub use component_doc_markdown::ComponentDocMarkdown;
pub use component_doc_props::ComponentDocProps;
pub use preview_card::ComponentPreviewCard;
pub use preview_card_body::OrbitalPreviewCardBody;
pub use preview_view::OrbitalComponentView;
pub use registration::PreviewRegistration;

pub(crate) mod components {

    pub use super::preview_card::ComponentPreviewCard;

    pub use super::preview_view::OrbitalComponentView;
}
