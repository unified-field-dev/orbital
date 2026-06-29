//! Interactive demo for [`crate::OrbitalPresenceGroup`] catalog page.

use leptos::prelude::*;
use orbital_style::inject_style;

use crate::callback::MotionElementCallback;
use crate::group::{OrbitalPresenceGroup, OrbitalPresenceGroupItem};
use crate::tokens::MotionDuration;
use crate::PresenceMotion;

use super::components::PreviewButton;
use super::demo::demo_tile_styles;

#[derive(Clone)]
struct DemoItem {
    id: usize,
    show: RwSignal<bool>,
}

/// Keyed list with staggered enter/exit — add and remove tiles to see the effect.
#[component]
pub fn OrbitalPresenceGroupDemo() -> impl IntoView {
    inject_style("orbital-motion-demo", demo_tile_styles());

    let items = RwSignal::new(Vec::<DemoItem>::new());
    let next_id = RwSignal::new(1usize);
    let motion = Signal::from(PresenceMotion::fade_scale());
    let stagger = Signal::from(MotionDuration::Normal);

    view! {
        <div data-testid="orbital-presence-group-preview">
            <p style="margin: 0 0 12px; color: var(--orb-color-text-secondary);">
                "Click Add item to mount keyed children with staggered fade-in. Remove items to see leave transitions."
            </p>
            <div style="display: flex; gap: 8px; margin-bottom: 12px;">
                <PreviewButton on_click=Callback::new(move |_| {
                    let id = next_id.get_untracked();
                    items.update(|list| {
                        list.push(DemoItem {
                            id,
                            show: RwSignal::new(true),
                        });
                    });
                    next_id.update(|n| *n += 1);
                })>"Add item"</PreviewButton>
                <PreviewButton on_click=Callback::new(move |_| {
                    items.with_untracked(|list| {
                        if let Some(item) = list.last() {
                            if item.show.get_untracked() {
                                item.show.set(false);
                            }
                        }
                    });
                })>"Remove item"</PreviewButton>
            </div>
            <div
                class="orbital-motion-presence-group-demo"
                style="display: flex; flex-direction: column; gap: 8px; min-height: 96px;"
            >
                <Show when=move || items.get().is_empty() fallback=|| ()>
                    <p
                        data-testid="orbital-presence-group-empty"
                        style="margin: 0; color: var(--orb-color-text-tertiary); font-size: var(--orb-type-size-sm);"
                    >
                        "No items yet — add one to start."
                    </p>
                </Show>
                <OrbitalPresenceGroup motion=motion stagger=stagger>
                    <ForEnumerate each=move || items.get() key=|item| item.id let(idx, item)>
                        {
                            let item_id = item.id;
                            let on_after_leave = MotionElementCallback::new(
                                move |_el: web_sys::HtmlElement| {
                                    items.update(|list| list.retain(|i| i.id != item_id));
                                },
                            );
                            view! {
                                <OrbitalPresenceGroupItem
                                    show=item.show.read_only()
                                    index=idx
                                    on_after_leave=on_after_leave
                                >
                                    <div
                                        class="orbital-motion-demo-shape"
                                        data-testid=format!("orbital-presence-group-tile-{item_id}")
                                    >
                                        {item_id}
                                    </div>
                                </OrbitalPresenceGroupItem>
                            }
                        }
                    </ForEnumerate>
                </OrbitalPresenceGroup>
            </div>
        </div>
    }
}
