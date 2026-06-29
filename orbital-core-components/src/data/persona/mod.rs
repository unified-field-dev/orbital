mod persona;
mod styles;
mod types;

pub use persona::Persona;
pub use types::{
    PersonaConfig, PersonaPrimaryText, PersonaQuaternaryText, PersonaSecondaryText, PersonaSize,
    PersonaTertiaryText, PersonaTextAlignment, PersonaTextPosition,
};

#[cfg(feature = "preview")]
pub use persona::{PERSONA_DESCRIPTION, PERSONA_DOC, PERSONA_PREVIEW_REGISTRATION, PERSONA_PROPS};
