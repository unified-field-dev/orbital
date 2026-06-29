mod carousel;
mod carousel_stepper;
mod styles;

pub use carousel::{Carousel, CarouselSlide, CarouselViewport};
pub use carousel_stepper::CarouselStepper;
pub use orbital_base_components::{CarouselState, CarouselStateInjection, CarouselStepperLayout};

#[cfg(feature = "preview")]
pub use carousel::CAROUSEL_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use carousel_stepper::CAROUSELSTEPPER_PREVIEW_REGISTRATION;
