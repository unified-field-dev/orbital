//! Introduction page demos wrapped in component-doc example cards.

use leptos::prelude::*;
use orbital::components::{
    Body1, ComponentPreviewCard, Flex, FlexGap, FlexWrap, FormHint, FormLabel, Material,
    MaterialElevation, MaterialVariant, SectionTitle, SpacingSize,
};

use super::token_demos::{
    BorderRadiusVisual, ColorSemanticVisual, ColorSwatchVisual, FontFamilyVisual,
    FontSizeRampVisual, FontWeightVisual, ShadowRampVisual, SpacingBarsVisual,
    SpacingProximityVisual, StrokeWidthVisual, TypographyRampVisual,
};

const SPACING_RAMP_CODE: &str = r#"use orbital::components::{ComponentPreviewCard, Flex, FlexAlign, SpacingSize};
use orbital_preview_app::preview::token_demos::SpacingBarsVisual;

view! {
    <ComponentPreviewCard title="Spacing ramp" code=SPACING_RAMP_CODE>
        <SpacingBarsVisual />
    </ComponentPreviewCard>
}"#;

const SPACING_PROXIMITY_CODE: &str = r#"use orbital::components::{Flex, FlexGap, FlexWrap, SpacingSize};
// Group related controls with Size80 (8px); separate sections with Size320 (32px).
view! {
    <Flex wrap=FlexWrap::Wrap gap=SpacingSize::Size240.flex_gap()>
        <Flex gap=SpacingSize::Size80.flex_gap()>{/* tight group */}</Flex>
        <Flex gap=SpacingSize::Size320.flex_gap()>{/* loose group */}</Flex>
    </Flex>
}"#;

const BORDER_RADIUS_CODE: &str = r#"use orbital::components::Box;
use orbital_base_components::{BorderRadius, SpacingInset, StrokeWidth, ThemeColor};

view! {
    <Box
        width="72px"
        padding=SpacingInset::uniform_px(24)
        background=ThemeColor::BrandBackground
        radius=BorderRadius::Medium
        border_color=ThemeColor::NeutralStroke1
        border_width=StrokeWidth::Thin
    />
}"#;

const STROKE_WIDTH_CODE: &str = r#"use orbital_base_components::StrokeWidth;

let line = format!(
    "border-top: {} solid var(--orb-color-border-default);",
    StrokeWidth::Thicker.css_var()
);"#;

const COLOR_SWATCHES_CODE: &str = r#"use orbital_base_components::ThemeColor;

// Typed theme colors — no raw var(--...) strings in component props.
ThemeColor::NeutralBackground1.css_var()  // -> "var(--orb-color-surface-canvas)"
ThemeColor::BrandBackground.name()        // -> "--orb-color-brand-bg""#;

const COLOR_SEMANTIC_CODE: &str = r#"use orbital::components::{MessageBar, MessageBarBody, MessageBarIntent, MessageBarLayout};

view! {
    <MessageBar intent=MessageBarIntent::Success layout=MessageBarLayout::Singleline>
        <MessageBarBody>"Success — changes saved"</MessageBarBody>
    </MessageBar>
}"#;

const ELEVATION_CODE: &str = r#"use orbital::components::Box;
use orbital_base_components::{BorderRadius, Shadow, SpacingInset, ThemeColor};

view! {
    <Box
        padding=SpacingInset::all_m()
        background=ThemeColor::NeutralBackground1
        shadow=Shadow::Shadow4
        radius=BorderRadius::Medium
    />
}"#;

const TYPOGRAPHY_RAMP_CODE: &str = r#"use orbital::components::{Body1, Caption1, Display, Title3};

view! {
    <>
        <Caption1>"Caption1 — metadata"</Caption1>
        <Body1 block=true>"Body1 — default reading text"</Body1>
        <Title3>"Title3 — page titles"</Title3>
        <Display>"Display"</Display>
    </>
}"#;

const FONT_FAMILIES_CODE: &str = r#"use orbital_base_components::FontFamily;

// Theme tokens: Base, Numeric, Monospace, Display (League Spartan, League Mono, Orbitron)
format!("font-family: {};", FontFamily::Display.css_var())"#;

const TYPOGRAPHY_FORM_CODE: &str = r#"use orbital::components::{FormHint, FormLabel, SectionTitle};

view! {
    <>
        <FormLabel>"Email address"</FormLabel>
        <FormHint>"We'll never share your email."</FormHint>
        <SectionTitle>"Account settings"</SectionTitle>
    </>
}"#;

const GRADIENT_HOST: &str =
    "padding: 16px; background: linear-gradient(135deg, #1A6F94 0%, #6B3FA0 100%);";

const TILE_INNER: &str = "padding: 12px; min-width: 72px; box-sizing: border-box;";

/// Spacing ramp with horizontal bars and Show code.
#[component]
pub fn SpacingRampDemo() -> impl IntoView {
    view! {
        <ComponentPreviewCard
            title="Spacing ramp"
            description="Each bar width matches the SpacingSize pixel value. Use SpacingSize in Flex gap and SpacingInset for padding."
            code=SPACING_RAMP_CODE
        >
            <SpacingBarsVisual />
        </ComponentPreviewCard>
    }
}

/// Tight vs loose grouping demo card.
#[component]
pub fn SpacingProximityDemo() -> impl IntoView {
    view! {
        <ComponentPreviewCard
            title="Spacing proximity"
            description="Small gaps group related controls; larger gaps separate sections."
            code=SPACING_PROXIMITY_CODE
        >
            <SpacingProximityVisual />
        </ComponentPreviewCard>
    }
}

/// Border radius token tiles.
#[component]
pub fn BorderRadiusDemo() -> impl IntoView {
    view! {
        <ComponentPreviewCard
            title="Border radii"
            description="Use BorderRadius on Box and other typed surface props."
            code=BORDER_RADIUS_CODE
        >
            <BorderRadiusVisual />
        </ComponentPreviewCard>
    }
}

/// Stroke width lines demo.
#[component]
pub fn StrokeWidthDemo() -> impl IntoView {
    view! {
        <ComponentPreviewCard
            title="Stroke widths"
            description="Progressively thicker rules using StrokeWidth tokens."
            code=STROKE_WIDTH_CODE
        >
            <StrokeWidthVisual />
        </ComponentPreviewCard>
    }
}

/// Neutral and status color swatches.
#[component]
pub fn ColorTokenSwatches(#[prop(into)] mono_class: &'static str) -> impl IntoView {
    view! {
        <ComponentPreviewCard
            title="Color tokens"
            description="ThemeColor maps every Orbital palette slot to a typed enum with css_var() and name()."
            code=COLOR_SWATCHES_CODE
        >
            <ColorSwatchVisual mono_class=mono_class />
        </ComponentPreviewCard>
    }
}

/// Semantic MessageBar color pairing.
#[component]
pub fn ColorSemanticDemo() -> impl IntoView {
    view! {
        <ComponentPreviewCard
            title="Semantic color"
            description="Status colors always ship with descriptive text via MessageBar intents."
            code=COLOR_SEMANTIC_CODE
        >
            <ColorSemanticVisual />
        </ComponentPreviewCard>
    }
}

/// Shadow / elevation ramp.
#[component]
pub fn ElevationMatrixDemo() -> impl IntoView {
    view! {
        <ComponentPreviewCard
            title="Elevation ramp"
            description="Shadow tokens from resting (--orb-elev-raised-sm) through modal (--orb-elev-modal)."
            code=ELEVATION_CODE
        >
            <ShadowRampVisual />
        </ComponentPreviewCard>
    }
}

#[component]
fn MaterialLabeledTile(label: &'static str, inner_style: &'static str) -> impl IntoView {
    view! {
        <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
            <div style=format!("{TILE_INNER} {inner_style}")>
                <Body1 block=true>{label}</Body1>
            </div>
        </Material>
    }
}

/// Solid material on neutral background steps.
#[component]
pub fn MaterialSolidDemo() -> impl IntoView {
    view! {
        <div
            data-testid="intro-material-solid"
            style="padding: 16px; background: var(--orb-color-surface-subtle); border-radius: var(--orb-radius-md);"
        >
            <Flex wrap=FlexWrap::Wrap gap=FlexGap::Medium>
                <MaterialLabeledTile
                    label="Shell ground (Bg3)"
                    inner_style="background: var(--orb-color-surface-subtle);"
                />
                <MaterialLabeledTile
                    label="Content canvas (Bg1)"
                    inner_style="background: var(--orb-color-surface-canvas);"
                />
            </Flex>
        </div>
    }
}

#[component]
fn VariantTile(variant: MaterialVariant, label: &'static str) -> impl IntoView {
    view! {
        <Material variant=variant elevation=MaterialElevation::Resting>
            <div style=TILE_INNER>
                <Body1 block=true>{label}</Body1>
            </div>
        </Material>
    }
}

#[component]
pub fn MaterialFrostDemo() -> impl IntoView {
    view! {
        <div data-testid="intro-material-frost" style=GRADIENT_HOST>
            <Flex wrap=FlexWrap::Wrap gap=FlexGap::Medium>
                <VariantTile variant=MaterialVariant::Frost label="Frost" />
                <VariantTile variant=MaterialVariant::Shell label="Shell" />
            </Flex>
        </div>
    }
}

#[component]
pub fn MaterialScrimDemo() -> impl IntoView {
    view! {
        <div data-testid="intro-material-scrim" style=GRADIENT_HOST>
            <Material variant=MaterialVariant::Scrim elevation=MaterialElevation::Resting>
                <div style="padding: 24px 16px; width: 100%; box-sizing: border-box;">
                    <Body1 block=true>"Scrim — blocks interaction with content beneath"</Body1>
                </div>
            </Material>
        </div>
    }
}

#[component]
pub fn MaterialVariantRecap() -> impl IntoView {
    view! {
        <div data-testid="intro-material-recap" style=GRADIENT_HOST>
            <Flex wrap=FlexWrap::Wrap gap=FlexGap::Medium>
                <VariantTile variant=MaterialVariant::Solid label="Solid" />
                <VariantTile variant=MaterialVariant::Frost label="Frost" />
                <VariantTile variant=MaterialVariant::Shell label="Shell" />
                <VariantTile variant=MaterialVariant::Scrim label="Scrim" />
            </Flex>
        </div>
    }
}

/// Typography preset ramp.
#[component]
pub fn TypographyRampDemo() -> impl IntoView {
    view! {
        <ComponentPreviewCard
            title="Typography ramp"
            description="Preset components (Caption1, Body1, Title3, Display) map to the Orbital type scale."
            code=TYPOGRAPHY_RAMP_CODE
        >
            <TypographyRampVisual />
        </ComponentPreviewCard>
    }
}

/// Font families and weights.
#[component]
pub fn TypographyFontsDemo() -> impl IntoView {
    view! {
        <ComponentPreviewCard
            title="Font families"
            description="League Spartan (Base and Numeric), League Mono (Monospace), and Orbitron (Display) — self-hosted OFL fonts via FontFamily theme tokens."
            code=FONT_FAMILIES_CODE
        >
            <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
                <FontFamilyVisual />
                <FontWeightVisual />
                <FontSizeRampVisual />
            </Flex>
        </ComponentPreviewCard>
    }
}

/// Form-specific typography helpers.
#[component]
pub fn TypographyFormHelpersDemo() -> impl IntoView {
    view! {
        <ComponentPreviewCard
            title="Form typography"
            description="FormLabel, FormHint, and SectionTitle are convenience wrappers around the type ramp."
            code=TYPOGRAPHY_FORM_CODE
        >
            <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
                <div data-testid="intro-typography-form-helpers">
                    <Flex
                        vertical=true
                        gap=SpacingSize::Size80.flex_gap()
                        padding=SpacingSize::Size160.inset()
                    >
                        <FormLabel>"Email address"</FormLabel>
                        <FormHint>"We'll never share your email."</FormHint>
                        <SectionTitle>"Account settings"</SectionTitle>
                    </Flex>
                </div>
            </Material>
        </ComponentPreviewCard>
    }
}
