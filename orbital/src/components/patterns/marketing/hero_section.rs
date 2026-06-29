use leptos::prelude::*;
use orbital_macros::component_doc;
use turf::inline_style_sheet_values;

use crate::components::{Body1, Caption1, Title3};
use crate::primitives::*;

/// CSS unit for height values
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeightUnit {
    /// Pixels
    Px,
    /// Viewport height
    Vh,
    /// Small viewport height (accounts for mobile browser UI)
    Svh,
    /// Em units (relative to font size)
    Em,
    /// Rem units (relative to root font size)
    Rem,
    /// Percentage
    Percent,
}

impl HeightUnit {
    fn as_str(self) -> &'static str {
        match self {
            HeightUnit::Px => "px",
            HeightUnit::Vh => "vh",
            HeightUnit::Svh => "svh",
            HeightUnit::Em => "em",
            HeightUnit::Rem => "rem",
            HeightUnit::Percent => "%",
        }
    }
}

/// CTA button configuration
#[derive(Clone, PartialEq)]
pub struct HeroCta {
    pub label: String,
    pub href: String,
    pub appearance: ButtonAppearance,
}

impl HeroCta {
    pub fn new(
        label: impl Into<String>,
        href: impl Into<String>,
        appearance: ButtonAppearance,
    ) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
            appearance,
        }
    }
}

/// Landing hero with title, subtitle, CTAs, and optional fine print.
///
/// Bundles a full-viewport marketing shell — gradient backdrop, centered typography, and a CTA row — that would take several primitives to recreate by hand (`Flex`, `Display` or `Title2`, `Button`, custom section CSS). Use this for above-the-fold landing blocks; for simpler in-page headers, compose typography and buttons directly.
///
/// Set `id` for in-page anchor navigation. `full_height=true` (default) uses `100svh` so the hero fills the viewport on mobile browsers with dynamic toolbars. Custom heights accept `HeightUnit::Px`, `Vh`, `Svh`, or `Rem`.
///
/// Background and text colors follow the active theme so contrast stays readable in light and dark mode.
///
/// Background parallax respects `prefers-reduced-motion` via the shared reduced-motion hook — test with OS accessibility settings enabled.
///
/// # Examples
///
/// ## Default
/// Full-height hero with title, subtitle, CTAs, and fine print.
/// <!-- preview -->
/// ```rust
/// let ctas = vec![
///     HeroCta::new("Sign up", "#", ButtonAppearance::Primary),
///     HeroCta::new("Log in", "#", ButtonAppearance::Secondary),
///     HeroCta::new("See demo", "#", ButtonAppearance::Subtle),
/// ];
///
/// view! {
///     <HeroSection
///         id="home".to_string()
///         title="Your New Software Product".to_string()
///         subtitle="Build faster with a human-friendly, enterprise-ready toolkit.".to_string()
///         ctas=ctas
///         fine_print="Free trial - No credit card - SSO ready".to_string()
///     />
/// }
/// ```
///
/// ## Full Height (Default)
/// Hero using the default full viewport height (`100svh`).
/// <!-- preview -->
/// ```rust
/// let ctas = vec![
///     HeroCta::new("Sign up", "#", ButtonAppearance::Primary),
///     HeroCta::new("Log in", "#", ButtonAppearance::Secondary),
/// ];
///
/// view! {
///     <HeroSection
///         id="full-height-example".to_string()
///         title="Full Height Hero".to_string()
///         subtitle="This hero section uses full viewport height (100svh) by default.".to_string()
///         ctas=ctas
///         full_height=true
///         fine_print="Default behavior - full_height=true".to_string()
///     />
/// }
/// ```
///
/// ## Custom Height Examples
/// Fixed pixel, viewport, and rem height variants.
/// <!-- preview -->
/// ```rust
/// let ctas_px = vec![
///     HeroCta::new("Sign up", "#", ButtonAppearance::Primary),
///     HeroCta::new("Log in", "#", ButtonAppearance::Secondary),
/// ];
/// let ctas_vh = ctas_px.clone();
/// let ctas_rem = ctas_px.clone();
///
/// view! {
///     <Flex vertical=true>
///         <HeroSection
///             id="px-example".to_string()
///             title="500px Height".to_string()
///             subtitle="Custom height using pixels.".to_string()
///             ctas=ctas_px
///             full_height=false
///             height=500.0
///             height_unit=HeightUnit::Px
///         />
///         <HeroSection
///             id="vh-example".to_string()
///             title="50vh Height".to_string()
///             subtitle="Custom height using viewport height units.".to_string()
///             ctas=ctas_vh
///             full_height=false
///             height=50.0
///             height_unit=HeightUnit::Vh
///         />
///         <HeroSection
///             id="rem-example".to_string()
///             title="30rem Height".to_string()
///             subtitle="Custom height using rem units.".to_string()
///             ctas=ctas_rem
///             full_height=false
///             height=30.0
///             height_unit=HeightUnit::Rem
///         />
///     </Flex>
/// }
/// ```
#[component_doc(
    category = "Patterns",
    preview_slug = "components/patterns/hero-section",
    preview_label = "Hero Section",
    preview_icon = icondata::AiRocketOutlined,
)]
#[component]
pub fn HeroSection(
    /// Section ID (for anchor links)
    #[prop(optional, default = "home".to_string())]
    id: String,
    /// Main hero title
    title: String,
    /// Hero subtitle/description
    subtitle: String,
    /// CTA buttons
    #[prop(optional, default = Vec::new())]
    ctas: Vec<HeroCta>,
    /// Fine print text (e.g. "Free trial • No credit card")
    #[prop(optional)]
    fine_print: Option<String>,
    /// Use full viewport height (100svh). Defaults to true. If true, this takes precedence over `height` and `height_unit`.
    #[prop(optional, default = true)]
    full_height: bool,
    /// Custom height value (e.g., 500 for 500px or 50 for 50vh) Only used if `full_height` is false.
    #[prop(optional)]
    height: Option<f64>,
    /// Unit for the custom height value. Defaults to `HeightUnit::Svh`. Only used if `full_height` is false and `height` is provided.
    #[prop(optional, default = HeightUnit::Svh)]
    height_unit: HeightUnit,
) -> impl IntoView {
    // Compute height CSS value
    let height_css = Signal::derive(move || {
        if full_height {
            "100svh".to_string()
        } else if let Some(h) = height {
            format!("{}{}", h, height_unit.as_str())
        } else {
            "100svh".to_string() // Default fallback
        }
    });

    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Layer {
            position: relative;
            padding: 80px clamp(16px, 4vw, 40px);
            display: grid;
            align-items: center;
            overflow: clip;
            isolation: isolate;
            transform: none;
        }

        .Layer::before {
            content: "";
            position: absolute;
            inset: 0;
            transform: translateZ(-0.6px) scale(1.6);
            z-index: 0;
            pointer-events: none;
            background:
                radial-gradient(60% 40% at 50% 18%, color-mix(in oklab, var(--orb-color-brand-bg) 22%, transparent), transparent 60%),
                radial-gradient(50% 50% at 18% 45%, color-mix(in oklab, var(--orb-color-brand-bg-subtle) 18%, transparent), transparent 55%),
                radial-gradient(45% 45% at 82% 55%, color-mix(in oklab, var(--orb-color-brand-fg) 12%, transparent), transparent 55%),
                linear-gradient(180deg, var(--orb-color-surface-canvas) 0%, var(--orb-color-surface-shell) 100%);
        }

        .LayerNoMotion::before {
            transform: none;
        }

        .Layer > * {
            position: relative;
            z-index: 1;
        }

        .HeroInner {
            max-width: 1000px;
            margin-inline: auto;
            text-align: center;
            transform: translateZ(0);
            color: var(--orb-color-text-primary);
        }

        .HeroFlex {
            align-items: center;
        }

        .HeroTitle {
            font-size: clamp(28px, 5vw, 56px);
            line-height: 1.1;
        }

        .HeroSub {
            margin: 12px 0 24px;
            font-size: clamp(16px, 2vw, 20px);
        }

        .CtaRow {
            display: flex;
            gap: 12px;
            justify-content: center;
            flex-wrap: wrap;
            margin-bottom: 8px;
        }

        @media (prefers-reduced-motion: reduce) {
            .Layer::before {
                transform: none;
            }
        }
    };

    let prefers_reduced = crate::components::motions::use_reduced_motion();

    let layer_class = Signal::derive(move || {
        let mut classes = vec![class_names.layer.to_string()];
        if prefers_reduced.get() {
            classes.push(class_names.layer_no_motion.to_string());
        }
        classes.join(" ")
    });

    view! {
        <style>{style_sheet}</style>
        <section
            class=layer_class
            id=id.clone()
            aria-label="Hero"
            style=move || format!("min-height: {}", height_css.get())
        >
            <div class=class_names.hero_inner>
                <Flex vertical=true class=class_names.hero_flex>
                    <Title3 tag=crate::components::TextTag::H1 class=class_names.hero_title>
                        {title.clone()}
                    </Title3>
                    <Body1 class=class_names.hero_sub>{subtitle.clone()}</Body1>
                    {if !ctas.is_empty() {
                        view! {
                            <div class=class_names.cta_row>
                                {ctas.into_iter().map(|cta| {
                                    view! {
                                        <Button appearance=cta.appearance size=ButtonSize::Large>
                                            <a href=cta.href.clone() style="text-decoration: none; color: inherit; display: block;">
                                                {cta.label}
                                            </a>
                                        </Button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    } else {
                        ().into_any()
                    }}
                    {if let Some(fp) = fine_print.clone() {
                        view! {
                            <Caption1>{fp}</Caption1>
                        }.into_any()
                    } else {
                        ().into_any()
                    }}
                </Flex>
            </div>
        </section>
    }
}
