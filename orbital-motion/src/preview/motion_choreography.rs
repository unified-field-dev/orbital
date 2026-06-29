use super::components::PreviewButton;
use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::atom::SlideFrom;
use crate::callback::MotionElementCallback;
use crate::group::{OrbitalPresenceGroup, OrbitalPresenceGroupItem};
use crate::tokens::MotionDuration;
use crate::PresenceMotion;

use super::demo::demo_tile_styles;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ChoreographyPreset {
    Fade,
    SlideBottom,
    Collapse,
}

impl ChoreographyPreset {
    fn motion(self) -> PresenceMotion {
        match self {
            Self::Fade => PresenceMotion::fade(),
            Self::SlideBottom => PresenceMotion::slide(SlideFrom::Bottom),
            Self::Collapse => PresenceMotion::collapse(),
        }
    }
}

#[derive(Clone)]
struct DemoItem {
    id: usize,
    show: RwSignal<bool>,
}

/// Staggered list choreography — keyed tiles enter and leave with incremental delays via [`OrbitalPresenceGroup`](crate::OrbitalPresenceGroup).
///
/// Orbital **choreography** means spacing enter delays across list items so additions and removals feel sequential rather than simultaneous. Use the demo controls to switch presets (`Fade`, `SlideBottom`, `Collapse`) and tune stagger duration. For production lists, wrap keyed [`OrbitalPresenceGroupItem`](crate::OrbitalPresenceGroupItem) children inside [`OrbitalPresenceGroup`](crate::OrbitalPresenceGroup).
///
/// # Examples
///
/// ## Choreography stagger
/// <!-- preview -->
/// ```rust
/// use crate::preview::MotionChoreographyStaggerDemo;
/// view! {
///     <div data-testid="motion-choreography-stagger">
///         <MotionChoreographyStaggerDemo />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Motion",
    preview_slug = "motion-choreography-stagger",
    preview_label = "Choreography Stagger",
    preview_icon = icondata::AiOrderedListOutlined,
)]
#[component]
pub fn MotionChoreographyStaggerDemo() -> impl IntoView {
    inject_style("orbital-motion-demo", demo_tile_styles());

    let items = RwSignal::new(
        [1usize, 2, 3]
            .into_iter()
            .map(|id| DemoItem {
                id,
                show: RwSignal::new(true),
            })
            .collect::<Vec<_>>(),
    );
    let next_id = RwSignal::new(4usize);
    let stagger = RwSignal::new(MotionDuration::Normal);
    let preset = RwSignal::new(ChoreographyPreset::Fade);
    let motion = Signal::derive(move || preset.get().motion().with_duration(stagger.get()));

    view! {
        <div data-testid="motion-choreography-stagger">
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
            <p data-testid="motion-choreography-readout" style="margin: 0 0 12px;">
                {move || format!(
                    "duration={:?} stagger_step={}",
                    stagger.get(),
                    stagger.get().ms(),
                )}
            </p>
            <div style="display: flex; gap: 8px; margin-bottom: 12px;">
                <PreviewButton on_click=Callback::new(move |_| stagger.set(MotionDuration::UltraFast))>"Fast (50ms)"</PreviewButton>
                <PreviewButton on_click=Callback::new(move |_| stagger.set(MotionDuration::Normal))>"Normal (200ms)"</PreviewButton>
                <PreviewButton on_click=Callback::new(move |_| stagger.set(MotionDuration::Slow))>"Slow (300ms)"</PreviewButton>
            </div>
            <div style="display: flex; gap: 8px; margin-bottom: 12px;">
                <PreviewButton on_click=Callback::new(move |_| preset.set(ChoreographyPreset::Fade))>"Fade"</PreviewButton>
                <PreviewButton on_click=Callback::new(move |_| preset.set(ChoreographyPreset::SlideBottom))>"Slide"</PreviewButton>
                <PreviewButton on_click=Callback::new(move |_| preset.set(ChoreographyPreset::Collapse))>"Collapse"</PreviewButton>
            </div>
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
                                    data-testid=format!("motion-choreography-tile-{item_id}")
                                >
                                    {item_id}
                                </div>
                            </OrbitalPresenceGroupItem>
                        }
                    }
                </ForEnumerate>
            </OrbitalPresenceGroup>
        </div>
    }
}
