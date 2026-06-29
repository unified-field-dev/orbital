use leptos::prelude::*;

/// Trigger slot for [`super::popover::SpotlightPopover`].
#[slot]
pub struct SpotlightTrigger {
    pub children: Children,
}

/// Title region in a spotlight surface.
#[slot]
pub struct SpotlightHeader {
    pub children: Children,
}

/// Main instructional copy.
#[slot]
pub struct SpotlightBody {
    pub children: Children,
}

/// Optional illustration or screenshot.
#[slot]
pub struct SpotlightMedia {
    pub children: Children,
}

/// Primary/secondary actions (Next, Dismiss, etc.).
#[slot]
pub struct SpotlightActions {
    pub children: Children,
}

/// Footer region for step indicators or secondary links.
#[slot]
pub struct SpotlightFooter {
    pub children: Children,
}

/// Backdrop behavior for [`super::tip::SpotlightTip`] and [`super::tour::tour::SpotlightTour`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SpotlightBackdrop {
    #[default]
    None,
    Dim {
        dismiss_on_click: bool,
    },
    Spotlight {
        padding: u32,
        dismiss_on_click: bool,
    },
}

impl SpotlightBackdrop {
    pub fn tour_default() -> Self {
        Self::Spotlight {
            padding: 8,
            dismiss_on_click: false,
        }
    }
}
