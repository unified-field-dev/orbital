pub use orbital_base_components::{ImageFit, ImageShape};

/// Display configuration for [`crate::Image`].
#[derive(Clone, Default)]
pub struct ImageConfig {
    /// Image URL.
    pub src: Option<String>,
    /// Accessible description of the image content.
    pub alt: Option<String>,
    /// CSS width (e.g. `"200px"`).
    pub width: Option<String>,
    /// CSS height.
    pub height: Option<String>,
    /// `Circular`, `Rounded`, or `Square`.
    pub shape: ImageShape,
    /// When true, image spans the container width.
    pub block: bool,
    /// Elevated shadow treatment.
    pub shadow: bool,
    /// Object-fit preset (`Contain`, `Cover`, `Fill`, …).
    pub fit: ImageFit,
}

impl ImageConfig {
    pub fn src(src: impl Into<String>, alt: impl Into<String>) -> Self {
        Self {
            src: Some(src.into()),
            alt: Some(alt.into()),
            ..Default::default()
        }
    }

    pub fn framed(
        src: impl Into<String>,
        alt: impl Into<String>,
        width: impl Into<String>,
        height: impl Into<String>,
    ) -> Self {
        Self {
            src: Some(src.into()),
            alt: Some(alt.into()),
            width: Some(width.into()),
            height: Some(height.into()),
            ..Default::default()
        }
    }

    pub fn rounded(src: impl Into<String>, alt: impl Into<String>) -> Self {
        Self {
            src: Some(src.into()),
            alt: Some(alt.into()),
            shape: ImageShape::Rounded,
            ..Default::default()
        }
    }

    pub fn with_shadow(src: impl Into<String>, alt: impl Into<String>) -> Self {
        Self {
            src: Some(src.into()),
            alt: Some(alt.into()),
            shadow: true,
            ..Default::default()
        }
    }

    pub fn cover(src: impl Into<String>, alt: impl Into<String>) -> Self {
        Self {
            src: Some(src.into()),
            alt: Some(alt.into()),
            fit: ImageFit::Cover,
            ..Default::default()
        }
    }

    pub fn block(src: impl Into<String>, alt: impl Into<String>) -> Self {
        Self {
            src: Some(src.into()),
            alt: Some(alt.into()),
            block: true,
            ..Default::default()
        }
    }

    pub fn with_shape(mut self, shape: ImageShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn with_shadow_flag(mut self) -> Self {
        self.shadow = true;
        self
    }

    pub fn with_fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }
}
