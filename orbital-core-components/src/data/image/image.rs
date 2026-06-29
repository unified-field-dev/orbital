use leptos::prelude::*;
use orbital_base_components::BaseImage;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::image_styles;
use super::types::ImageConfig;

/// Framed image with shape, fit, shadow, and dimension controls.
///
/// Always provide meaningful `config.alt` when the image conveys information. For circular profile photos use [`Avatar`](crate::Avatar); for card hero regions use [`CardMedia`](crate::CardMedia).
///
/// # When to use
///
/// - Thumbnails, hero images, and content illustrations - Profile or product photos with shape and fit presets - Block-width images inside cards and article layouts
///
/// # Usage
///
/// 1. Set `config.src` and `config.alt` for accessible images. 2. Use `config.shape` for circular, rounded, or square corners. 3. Set `config.fit` to control object-fit within a bounded frame. 4. Enable `config.shadow` for elevated card-style imagery.
///
/// # Best Practices
///
/// ## Do's
///
/// * Provide descriptive `config.alt` for informative images * Set explicit `config.width` and `config.height` to prevent layout shift * Use `config.fit=ImageFit::Cover` for uniform thumbnail grids
///
/// ## Don'ts
///
/// * Do not use decorative images with misleading alt text * Do not stretch images without an appropriate `config.fit` preset
///
/// # Examples
///
/// ## Default image
/// Square image with default fill fit — suitable for thumbnails and content blocks.
/// <!-- preview -->
/// ```rust
/// use crate::{Image, ImageConfig};
/// view! {
///     <div data-testid="image-preview">
///         <Image config=ImageConfig::framed(
///             "https://picsum.photos/200",
///             "Sample landscape",
///             "200px",
///             "120px",
///         ) />
///     </div>
/// }
/// ```
///
/// ## Rounded shape
/// Rounded corners suit cards and inline content thumbnails.
/// <!-- preview -->
/// ```rust
/// use crate::{Image, ImageConfig, ImageShape};
/// view! {
///     <div data-testid="image-rounded">
///         <Image config=ImageConfig::framed(
///             "https://picsum.photos/80",
///             "Rounded thumbnail",
///             "80px",
///             "80px",
///         ).with_shape(ImageShape::Rounded) />
///     </div>
/// }
/// ```
///
/// ## Shadow elevation
/// Shadow treatment adds depth for featured or card imagery.
/// <!-- preview -->
/// ```rust
/// use crate::{Image, ImageConfig};
/// view! {
///     <div data-testid="image-shadow">
///         <Image config=ImageConfig::framed(
///             "https://picsum.photos/120",
///             "Elevated photo",
///             "120px",
///             "80px",
///         ).with_shadow_flag() />
///     </div>
/// }
/// ```
///
/// ## Fit cover and contain
/// Object-fit presets control cropping within a fixed frame.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, FlexGap, Image, ImageConfig, ImageFit};
/// view! {
///     <div data-testid="image-fit">
///         <Flex gap=FlexGap::Medium>
///             <div data-testid="image-fit-cover">
///                 <Image config=ImageConfig::framed(
///                     "https://picsum.photos/200",
///                     "Cover fit",
///                     "80px",
///                     "80px",
///                 ).with_fit(ImageFit::Cover) />
///             </div>
///             <div data-testid="image-fit-contain">
///                 <Image config=ImageConfig::framed(
///                     "https://picsum.photos/200",
///                     "Contain fit",
///                     "80px",
///                     "80px",
///                 ).with_fit(ImageFit::Contain) />
///             </div>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Block width
/// Block images span the full width of their container.
/// <!-- preview -->
/// ```rust
/// use crate::{Image, ImageConfig};
/// view! {
///     <div data-testid="image-block" style="width: 240px;">
///         <Image config=ImageConfig::block(
///             "https://picsum.photos/400/120",
///             "Full-width banner",
///         ) />
///     </div>
/// }
/// ```
///
/// ## Theme border token
/// Image borders use stroke tokens from the Orbital theme provider.
/// <!-- preview -->
/// ```rust
/// use crate::{Image, ImageConfig};
/// view! {
///     <div data-testid="image-theme">
///         <Image config=ImageConfig::framed(
///             "https://picsum.photos/100",
///             "Themed frame",
///             "100px",
///             "100px",
///         ) />
///     </div>
/// }
/// ```
///
/// ## Bounded frame
/// Explicit width and height constrain layout while fit controls cropping.
/// <!-- preview -->
/// ```rust
/// use crate::{Image, ImageConfig, ImageFit};
/// view! {
///     <div data-testid="image-bounded">
///         <Image config=ImageConfig::framed(
///             "https://picsum.photos/300",
///             "Bounded frame",
///             "160px",
///             "100px",
///         ).with_fit(ImageFit::Cover) />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "image",
    preview_label = "Image",
    preview_icon = icondata::AiPictureOutlined,
)]
#[component]
pub fn Image(
    /// Optional CSS class on the image element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Source, dimensions, shape, and fit settings grouped together.
    #[prop(optional, into)]
    config: ImageConfig,
) -> impl IntoView {
    inject_style("orbital-image", image_styles());

    let src = MaybeProp::from(config.src);
    let alt = MaybeProp::from(config.alt);
    let width = MaybeProp::from(config.width);
    let height = MaybeProp::from(config.height);
    let shape = Signal::from(config.shape);
    let block = Signal::from(config.block);
    let shadow = Signal::from(config.shadow);
    let fit = Signal::from(config.fit);

    view! {
        <BaseImage
            class=class
            src=src
            alt=alt
            width=width
            height=height
            shape=shape
            block=block
            shadow=shadow
            fit=fit
        />
    }
}
