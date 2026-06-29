use leptos::prelude::*;
use orbital_base_components::{BackdropMode, OpenBind};

use crate::BackdropConfig;

use super::types::SpotlightBackdrop;

/// Map spotlight backdrop options to core [`BackdropConfig`].
pub fn spotlight_backdrop_config(
    open: OpenBind,
    backdrop: SpotlightBackdrop,
    anchor_id: Signal<Option<String>>,
) -> Option<(BackdropConfig, Option<Callback<leptos::ev::MouseEvent>>)> {
    let show = open.signal();
    match backdrop {
        SpotlightBackdrop::None => None,
        SpotlightBackdrop::Dim { dismiss_on_click } => {
            let on_click = dismiss_on_click.then(|| {
                let open = open;
                Callback::new(move |_: leptos::ev::MouseEvent| open.set(false))
            });
            Some((BackdropConfig::new(show), on_click))
        }
        SpotlightBackdrop::Spotlight {
            padding,
            dismiss_on_click,
        } => {
            let on_click = dismiss_on_click.then(|| {
                let open = open;
                Callback::new(move |_: leptos::ev::MouseEvent| open.set(false))
            });
            Some((
                BackdropConfig::new(show).with_mode(BackdropMode::Spotlight { anchor_id, padding }),
                on_click,
            ))
        }
    }
}
