use leptos::prelude::*;

use super::anatomy::SpotlightAnatomyViews;
use super::types::{
    SpotlightActions, SpotlightBody, SpotlightFooter, SpotlightHeader, SpotlightMedia,
};

pub fn anatomy_from_slots(
    header: Option<SpotlightHeader>,
    body: Option<SpotlightBody>,
    media: Option<SpotlightMedia>,
    actions: Option<SpotlightActions>,
    footer: Option<SpotlightFooter>,
) -> SpotlightAnatomyViews {
    SpotlightAnatomyViews {
        header: header.map(|slot| (slot.children)().into_any()),
        body: body.map(|slot| (slot.children)().into_any()),
        media: media.map(|slot| (slot.children)().into_any()),
        actions: actions.map(|slot| (slot.children)().into_any()),
        footer: footer.map(|slot| (slot.children)().into_any()),
    }
}
