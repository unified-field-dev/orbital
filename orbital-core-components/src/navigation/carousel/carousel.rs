use std::time::Duration;

use leptos::html;
use leptos::prelude::*;
use orbital_base_components::{
    BaseCarousel, BaseCarouselSlide, BaseCarouselViewport, CarouselState, CarouselStateInjection,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::carousel_styles;

/// `Carousel` cycles through slides inside a [`CarouselViewport`]. Set `active_index` or
/// `default_active_index`, optionally enable `autoplay`, and add [`CarouselStepper`] for dots or arrows.
/// Prefer manual navigation for accessibility; pause autoplay in app logic when users need control.
///
/// # When to use
///
/// - Hero banners, campaigns, and landing-page highlights - Product or feature tours with a small set of full-width panels - Image-forward galleries where one slide should dominate the viewport
///
/// # Usage
///
/// 1. Wrap slides in [`Carousel`] and set `default_active_index` or control `active_index`. 2. Add [`CarouselViewport`] containing one [`CarouselSlide`] per panel. 3. Add [`CarouselStepper`] for previous/next buttons and page indicators, or drive navigation via [`CarouselStateInjection`].
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep slide counts modest (typically 3–7 panels) so users can orient themselves * Give each slide a title or description for screen-reader context * Use `wrap=false` when the carousel should stop at the first or last slide * Respect `prefers-reduced-motion` — autoplay is disabled automatically in reduced-motion styles
///
/// ## Don'ts
///
/// * Do not use for long lists — prefer scroll regions, tables, or pagination * Do not autoplay critical information without a pause control * Do not nest interactive controls that trap focus inside each slide without a clear tab order
///
/// # Examples
///
/// ## Default carousel
/// Hero-style slides with titles, supporting text, and bottom stepper controls.
/// <!-- preview -->
/// ```rust
/// use crate::{Carousel, CarouselSlide, CarouselStepper, CarouselViewport};
/// view! {
///     <div data-testid="carousel-preview">
///         <Carousel default_active_index=0 wrap=true>
///             <CarouselViewport>
///                 <CarouselSlide>
///                     <div class="orbital-carousel__slide-panel">
///                         <p class="orbital-carousel__slide-eyebrow">"Introducing"</p>
///                         <h3 class="orbital-carousel__slide-title">"Orbital design system"</h3>
///                         <p class="orbital-carousel__slide-body">"Compose accessible experiences with shared tokens, motion, and components."</p>
///                     </div>
///                 </CarouselSlide>
///                 <CarouselSlide>
///                     <div class="orbital-carousel__slide-panel orbital-carousel__slide-panel--neutral">
///                         <p class="orbital-carousel__slide-eyebrow">"Build faster"</p>
///                         <h3 class="orbital-carousel__slide-title">"Preview-driven workflows"</h3>
///                         <p class="orbital-carousel__slide-body">"Iterate on components in isolation before shipping to product surfaces."</p>
///                     </div>
///                 </CarouselSlide>
///                 <CarouselSlide>
///                     <div class="orbital-carousel__slide-panel orbital-carousel__slide-panel--accent">
///                         <p class="orbital-carousel__slide-eyebrow">"Ship confidently"</p>
///                         <h3 class="orbital-carousel__slide-title">"Accessible by default"</h3>
///                         <p class="orbital-carousel__slide-body">"Carousels expose slide semantics, live regions, and keyboard-friendly steppers."</p>
///                     </div>
///                 </CarouselSlide>
///             </CarouselViewport>
///             <CarouselStepper />
///         </Carousel>
///     </div>
/// }
/// ```
///
/// ## Controlled index
/// Read and drive the active slide from application state via [`CarouselStateInjection`].
/// <!-- preview -->
/// ```rust
/// use crate::{Carousel, CarouselSlide, CarouselStepper, CarouselViewport};
/// use leptos::prelude::*;
/// use orbital_base_components::CarouselStateInjection;
/// view! {
///     <div data-testid="carousel-next-prev">
///         <Carousel default_active_index=0 wrap=false>
///             <CarouselViewport>
///                 <CarouselSlide>
///                     <div class="orbital-carousel__slide-panel">
///                         <p class="orbital-carousel__slide-eyebrow">"Step 1"</p>
///                         <h3 class="orbital-carousel__slide-title">"Connect your data"</h3>
///                         <p class="orbital-carousel__slide-body">"Import records from your existing tools to get started quickly."</p>
///                     </div>
///                 </CarouselSlide>
///                 <CarouselSlide>
///                     <div class="orbital-carousel__slide-panel orbital-carousel__slide-panel--neutral">
///                         <p class="orbital-carousel__slide-eyebrow">"Step 2"</p>
///                         <h3 class="orbital-carousel__slide-title">"Invite your team"</h3>
///                         <p class="orbital-carousel__slide-body">"Collaborate on the same live workspace with role-based access."</p>
///                     </div>
///                 </CarouselSlide>
///                 <CarouselSlide>
///                     <div class="orbital-carousel__slide-panel orbital-carousel__slide-panel--accent">
///                         <p class="orbital-carousel__slide-eyebrow">"Step 3"</p>
///                         <h3 class="orbital-carousel__slide-title">"Publish insights"</h3>
///                         <p class="orbital-carousel__slide-body">"Share dashboards and reports with stakeholders in one click."</p>
///                     </div>
///                 </CarouselSlide>
///             </CarouselViewport>
///             <CarouselStepper />
///             <p data-testid="carousel-active-index">
///                 "Active slide: "
///                 {move || (CarouselStateInjection::expect_context().active_index() + 1).to_string()}
///             </p>
///         </Carousel>
///     </div>
/// }
/// ```
///
/// ## Wrap navigation
/// With `wrap=true`, advancing past the last slide returns to the first—ideal for looping highlights.
/// <!-- preview -->
/// ```rust
/// use crate::{Carousel, CarouselSlide, CarouselStepper, CarouselViewport};
/// use leptos::prelude::*;
/// use orbital_base_components::CarouselStateInjection;
/// view! {
///     <div data-testid="carousel-wrap">
///         <Carousel default_active_index=2 wrap=true>
///             <CarouselViewport>
///                 <CarouselSlide>
///                     <div class="orbital-carousel__slide-panel">
///                         <h3 class="orbital-carousel__slide-title">"Alpha release"</h3>
///                         <p class="orbital-carousel__slide-body">"Foundation components and tokens."</p>
///                     </div>
///                 </CarouselSlide>
///                 <CarouselSlide>
///                     <div class="orbital-carousel__slide-panel orbital-carousel__slide-panel--neutral">
///                         <h3 class="orbital-carousel__slide-title">"Beta program"</h3>
///                         <p class="orbital-carousel__slide-body">"Expanded inputs, navigation, and feedback."</p>
///                     </div>
///                 </CarouselSlide>
///                 <CarouselSlide>
///                     <div class="orbital-carousel__slide-panel orbital-carousel__slide-panel--accent">
///                         <h3 class="orbital-carousel__slide-title">"General availability"</h3>
///                         <p class="orbital-carousel__slide-body">"Production-ready patterns for product teams."</p>
///                     </div>
///                 </CarouselSlide>
///             </CarouselViewport>
///             <CarouselStepper />
///             <p data-testid="carousel-wrap-index">
///                 "Index: "
///                 {move || CarouselStateInjection::expect_context().active_index().to_string()}
///             </p>
///         </Carousel>
///     </div>
/// }
/// ```
///
/// ## Autoplay
/// Automatically advance slides on an interval—use sparingly for ambient marketing content.
/// <!-- preview -->
/// ```rust
/// use crate::{Carousel, CarouselSlide, CarouselStepper, CarouselViewport};
/// view! {
///     <div data-testid="carousel-autoplay">
///         <Carousel default_active_index=0 wrap=true autoplay=true autoplay_interval=4000>
///             <CarouselViewport>
///                 <CarouselSlide>
///                     <div class="orbital-carousel__slide-panel">
///                         <h3 class="orbital-carousel__slide-title">"Always-on support"</h3>
///                         <p class="orbital-carousel__slide-body">"Autoplay advances every four seconds when motion is allowed."</p>
///                     </div>
///                 </CarouselSlide>
///                 <CarouselSlide>
///                     <div class="orbital-carousel__slide-panel orbital-carousel__slide-panel--neutral">
///                         <h3 class="orbital-carousel__slide-title">"Guided onboarding"</h3>
///                         <p class="orbital-carousel__slide-body">"Pair autoplay with clear stepper indicators so users stay oriented."</p>
///                     </div>
///                 </CarouselSlide>
///             </CarouselViewport>
///             <CarouselStepper />
///         </Carousel>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "carousel",
    preview_label = "Carousel",
    preview_icon = icondata::AiSlidersOutlined,
)]
#[component]
pub fn Carousel(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Enables automatic slide advancement.
    #[prop(optional)]
    autoplay: bool,
    /// Interval between autoplay advances in milliseconds.
    #[prop(optional)]
    autoplay_interval: Option<i32>,
    /// Whether navigation wraps from last to first slide.
    #[prop(optional, default = true)]
    wrap: bool,
    /// Initial active slide index (uncontrolled).
    #[prop(optional, default = 0)]
    default_active_index: i32,
    /// Controlled active slide index.
    #[prop(optional, into)]
    active_index: Option<Signal<Option<i32>>>,
    /// Fired when the active slide changes.
    #[prop(optional)]
    on_active_index_change: Option<Callback<i32>>,
    /// Slide viewport and navigation children.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-carousel", carousel_styles());

    let wrap_signal = Signal::derive(move || wrap);
    let state = CarouselState::new(default_active_index, wrap_signal, on_active_index_change);
    provide_context(CarouselStateInjection(state));

    let initial_index = StoredValue::new(default_active_index);
    let initial_synced = StoredValue::new(false);
    let initial_sync_timer = StoredValue::new(None::<TimeoutHandle>);
    Effect::new(move |_| {
        if initial_synced.get_value() {
            return;
        }
        let count = state.slide_count();
        if count <= 0 {
            return;
        }

        initial_sync_timer.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
        });

        if let Ok(handle) = set_timeout_with_handle(
            move || {
                if initial_synced.get_value() {
                    return;
                }
                let count = state.slide_count();
                if count <= 0 {
                    return;
                }
                let target = initial_index.get_value();
                if target >= count {
                    return;
                }
                state.go_to(target);
                initial_synced.set_value(true);
            },
            Duration::from_millis(50),
        ) {
            initial_sync_timer.set_value(Some(handle));
        }
    });

    on_cleanup(move || {
        initial_sync_timer.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
        });
    });

    if let Some(active_index) = active_index {
        Effect::new(move |_| {
            if let Some(index) = active_index.get() {
                state.go_to(index);
            }
        });
    }

    if autoplay {
        let interval_ms = autoplay_interval.unwrap_or(5000).max(250) as u64;
        let autoplay_handle = StoredValue::new(None::<TimeoutHandle>);

        Effect::new(move |_| {
            autoplay_handle.update_value(|handle| {
                if let Some(handle) = handle.take() {
                    handle.clear();
                }
            });

            if !autoplay {
                return;
            }

            fn schedule(
                state: CarouselState,
                handle_store: StoredValue<Option<TimeoutHandle>>,
                interval_ms: u64,
            ) {
                if let Ok(handle) = set_timeout_with_handle(
                    move || {
                        state.next();
                        schedule(state, handle_store, interval_ms);
                    },
                    Duration::from_millis(interval_ms),
                ) {
                    handle_store.set_value(Some(handle));
                }
            }

            schedule(state, autoplay_handle, interval_ms);
        });
    }

    view! {
        <BaseCarousel class=class>
            {children()}
        </BaseCarousel>
    }
}

/// Scroll-snap viewport that tracks the active slide.
#[component]
pub fn CarouselViewport(
    /// Optional CSS class on the viewport element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Slide children.
    children: Children,
) -> impl IntoView {
    let state = CarouselStateInjection::expect_context();
    let viewport_ref = NodeRef::<html::Div>::new();
    let active_index = state.active_index_signal();

    Effect::new(move |_| {
        let index = active_index.get();
        if let Some(viewport) = viewport_ref.get() {
            let width = viewport.client_width();
            viewport.set_scroll_left(index * width);
        }
    });

    view! {
        <BaseCarouselViewport class=class node_ref=viewport_ref>
            {children()}
        </BaseCarouselViewport>
    }
}

/// A single slide inside a [`Carousel`].
#[component]
pub fn CarouselSlide(
    /// Optional CSS class on the slide element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Slide content.
    children: Children,
) -> impl IntoView {
    let state = CarouselStateInjection::expect_context();
    let index = state.register_slide();
    let active_index = state.active_index_signal();
    let active = Signal::derive(move || active_index.get() == index);

    view! {
        <BaseCarouselSlide class=class index=index active=active>
            {children()}
        </BaseCarouselSlide>
    }
}
