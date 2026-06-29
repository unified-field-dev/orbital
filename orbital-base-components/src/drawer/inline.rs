use leptos::prelude::*;
use orbital_motion::{resolve_presence_motion_derived, MotionSlot, OrbitalPresence};

use super::sizes::{drawer_presence_motion, drawer_size_css, DrawerPosition, DrawerSize};
use crate::overlay::open_bind::OpenBind;

#[component]
pub fn BaseInlineDrawer(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into)] open: OpenBind,
    #[prop(default = DrawerPosition::Left.into(), into)] position: Signal<DrawerPosition>,
    #[prop(default = DrawerSize::Small.into(), into)] size: Signal<DrawerSize>,
    #[prop(optional)] motion: MotionSlot,
    children: Children,
) -> impl IntoView {
    let open_drawer = RwSignal::new(open.get_untracked());

    Effect::new(move |_| {
        open_drawer.set(open.get());
    });

    let motion =
        resolve_presence_motion_derived(motion, move || drawer_presence_motion(position.get()));

    view! {
        <OrbitalPresence
            appear=open_drawer.get_untracked()
            show=open_drawer.read_only()
            motion=motion
        >
            <div
                class=move || {
                    let mut parts = vec![
                        "orbital-inline-drawer".to_string(),
                        format!(
                            "orbital-inline-drawer--position-{}",
                            position.get().as_str()
                        ),
                    ];
                    if let Some(extra) = class.get() {
                        if !extra.is_empty() {
                            parts.push(extra);
                        }
                    }
                    parts.join(" ")
                }
                style=(
                    "--orbital-drawer--size",
                    move || drawer_size_css(size.get(), position.get()),
                )
            >
                {children()}
            </div>
        </OrbitalPresence>
    }
}
