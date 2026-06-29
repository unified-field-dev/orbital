mod counter_badge;
mod styles;

pub use counter_badge::CounterBadge;

#[cfg(feature = "preview")]
pub use counter_badge::{
    COUNTERBADGE_DESCRIPTION, COUNTERBADGE_DOC, COUNTERBADGE_PREVIEW_REGISTRATION,
    COUNTERBADGE_PROPS,
};
