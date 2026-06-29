//! Reusable high-visibility token showcase visuals.

use leptos::prelude::*;
use orbital::components::{
    Body1, Box, Caption1, Caption1Strong, Flex, FlexAlign, FlexGap, FlexWrap, Material,
    MaterialElevation, MaterialVariant, MessageBar, MessageBarBody, MessageBarIntent,
    MessageBarLayout, SectionTitle, SpacingSize,
};
use orbital_base_components::{
    BorderRadius, FontFamily, FontSize, FontWeight, Shadow, SpacingInset, StrokeWidth, ThemeColor,
};

const BAR_HEIGHT: &str = "12px";

#[component]
pub fn SpacingBarsVisual() -> impl IntoView {
    const RAMP: [(&str, SpacingSize, &str); 6] = [
        ("Size40", SpacingSize::Size40, "4px"),
        ("Size80", SpacingSize::Size80, "8px"),
        ("Size120", SpacingSize::Size120, "12px"),
        ("Size160", SpacingSize::Size160, "16px"),
        ("Size240", SpacingSize::Size240, "24px"),
        ("Size320", SpacingSize::Size320, "32px"),
    ];

    view! {
        <div data-testid="intro-spacing-ramp">
            <Flex vertical=true gap=SpacingSize::Size120.flex_gap() align=FlexAlign::Stretch full_width=true>
                {RAMP
                    .into_iter()
                    .map(|(name, size, px)| {
                        let width = format!("{}px", size.px());
                        view! {
                            <Flex vertical=true gap=SpacingSize::Size40.flex_gap() align=FlexAlign::Stretch>
                                <Caption1>{format!("{name} | {px}")}</Caption1>
                                <div
                                    style=format!(
                                        "width: {width}; height: {BAR_HEIGHT}; background: var(--orb-color-brand-bg); border-radius: var(--orb-radius-sm);"
                                    )
                                ></div>
                            </Flex>
                        }
                    })
                    .collect_view()}
            </Flex>
        </div>
    }
}

#[component]
pub fn SpacingProximityVisual() -> impl IntoView {
    view! {
        <div data-testid="intro-spacing-proximity">
            <Flex wrap=FlexWrap::Wrap gap=SpacingSize::Size240.flex_gap() align=FlexAlign::Stretch>
                <ProximityGroup label="8px gap (Size80)" gap=SpacingSize::Size80 />
                <ProximityGroup label="32px gap (Size320)" gap=SpacingSize::Size320 />
            </Flex>
        </div>
    }
}

#[component]
fn ProximityGroup(label: &'static str, gap: SpacingSize) -> impl IntoView {
    view! {
        <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
            <Flex
                vertical=true
                gap=SpacingSize::Size120.flex_gap()
                padding=SpacingSize::Size160.inset()
                full_width=true
            >
                <Caption1>{label}</Caption1>
                <Flex gap=gap.flex_gap() wrap=FlexWrap::Wrap>
                    <SpacingTile label="A" />
                    <SpacingTile label="B" />
                    <SpacingTile label="C" />
                </Flex>
            </Flex>
        </Material>
    }
}

#[component]
fn SpacingTile(label: &'static str) -> impl IntoView {
    view! {
        <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
            <Box padding=SpacingInset::all_m()>
                <Body1 block=true>{label}</Body1>
            </Box>
        </Material>
    }
}

#[component]
pub fn BorderRadiusVisual() -> impl IntoView {
    const TILES: [(BorderRadius, &str); 7] = [
        (BorderRadius::None, "None"),
        (BorderRadius::Small, "Small"),
        (BorderRadius::Medium, "Medium"),
        (BorderRadius::Large, "Large"),
        (BorderRadius::XLarge, "XLarge"),
        (BorderRadius::Floating, "Floating"),
        (BorderRadius::Circular, "Circular"),
    ];

    view! {
        <div data-testid="intro-border-radius">
            <Flex wrap=FlexWrap::Wrap gap=SpacingSize::Size120.flex_gap() align=FlexAlign::FlexStart>
                {TILES
                    .into_iter()
                    .map(|(radius, label)| {
                        view! {
                            <Flex vertical=true align=FlexAlign::Center gap=SpacingSize::Size40.flex_gap()>
                                <Box
                                    width="72px"
                                    padding=SpacingInset::uniform_px(24)
                                    background=ThemeColor::BrandBackground
                                    radius=radius
                                    border_color=ThemeColor::NeutralStroke1
                                    border_width=StrokeWidth::Thin
                                ></Box>
                                <Caption1>{label}</Caption1>
                                <Caption1>{radius.name()}</Caption1>
                            </Flex>
                        }
                    })
                    .collect_view()}
            </Flex>
        </div>
    }
}

#[component]
pub fn StrokeWidthVisual() -> impl IntoView {
    const LINES: [(StrokeWidth, &str); 4] = [
        (StrokeWidth::Thin, "Thin"),
        (StrokeWidth::Thick, "Thick"),
        (StrokeWidth::Thicker, "Thicker"),
        (StrokeWidth::Thickest, "Thickest"),
    ];

    view! {
        <div data-testid="intro-stroke-width">
            <Flex vertical=true gap=SpacingSize::Size120.flex_gap() align=FlexAlign::Stretch full_width=true>
                {LINES
                    .into_iter()
                    .map(|(width, label)| {
                        view! {
                            <Flex vertical=true gap=SpacingSize::Size40.flex_gap() align=FlexAlign::Stretch>
                                <Caption1>{format!("{label} ({})", width.name())}</Caption1>
                                <div
                                    style=format!(
                                        "width: 100%; border-top: {} solid var(--orb-color-border-default);",
                                        width.css_var()
                                    )
                                ></div>
                            </Flex>
                        }
                    })
                    .collect_view()}
            </Flex>
        </div>
    }
}

#[component]
pub fn ColorSwatchVisual(#[prop(into)] mono_class: &'static str) -> impl IntoView {
    view! {
        <div data-testid="intro-color-swatches">
            <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
                <SwatchRow title="Neutral backgrounds">
                    <ColorSwatch background=ThemeColor::NeutralBackground1 label="--orb-color-surface-canvas" mono_class=mono_class />
                    <ColorSwatch background=ThemeColor::NeutralBackground3 label="--orb-color-surface-subtle" mono_class=mono_class />
                    <ColorSwatch background=ThemeColor::NeutralBackground4 label="--orb-color-surface-overlay" mono_class=mono_class />
                </SwatchRow>
                <SwatchRow title="Brand and status">
                    <ColorSwatch background=ThemeColor::BrandBackground label="--orb-color-brand-bg" mono_class=mono_class />
                    <ColorSwatch background=ThemeColor::StatusSuccessBackground1 label="--orb-color-status-success-bg" mono_class=mono_class />
                    <ColorSwatch background=ThemeColor::StatusWarningBackground1 label="--orb-color-status-warning-bg" mono_class=mono_class />
                    <ColorSwatch background=ThemeColor::StatusDangerBackground1 label="--orb-color-status-danger-bg" mono_class=mono_class />
                </SwatchRow>
            </Flex>
        </div>
    }
}

#[component]
fn SwatchRow(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
            <SectionTitle>{title}</SectionTitle>
            <Flex wrap=FlexWrap::Wrap gap=SpacingSize::Size120.flex_gap() align=FlexAlign::FlexStart>
                {children()}
            </Flex>
        </Flex>
    }
}

#[component]
fn ColorSwatch(
    background: ThemeColor,
    label: &'static str,
    mono_class: &'static str,
) -> impl IntoView {
    view! {
        <Flex vertical=true align=FlexAlign::Center gap=SpacingSize::Size40.flex_gap()>
            <Box
                width="56px"
                padding=SpacingInset::uniform_px(28)
                background=background
                radius=BorderRadius::Small
                border_color=ThemeColor::NeutralStroke1
                border_width=StrokeWidth::Thin
            ></Box>
            <Caption1 class=mono_class>{label}</Caption1>
        </Flex>
    }
}

#[component]
pub fn ColorSemanticVisual() -> impl IntoView {
    view! {
        <div data-testid="intro-color-semantic">
            <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                <MessageBar intent=MessageBarIntent::Info layout=MessageBarLayout::Singleline>
                    <MessageBarBody>"Info — paired with descriptive text"</MessageBarBody>
                </MessageBar>
                <MessageBar intent=MessageBarIntent::Success layout=MessageBarLayout::Singleline>
                    <MessageBarBody>"Success — changes saved"</MessageBarBody>
                </MessageBar>
                <MessageBar intent=MessageBarIntent::Warning layout=MessageBarLayout::Singleline>
                    <MessageBarBody>"Warning — review before continuing"</MessageBarBody>
                </MessageBar>
                <MessageBar intent=MessageBarIntent::Error layout=MessageBarLayout::Singleline>
                    <MessageBarBody>"Error — action could not complete"</MessageBarBody>
                </MessageBar>
            </Flex>
        </div>
    }
}

#[component]
pub fn ShadowRampVisual() -> impl IntoView {
    const SHADOWS: [(Shadow, &str, MaterialElevation); 6] = [
        (
            Shadow::Shadow2,
            "--orb-elev-raised-xs",
            MaterialElevation::Flat,
        ),
        (
            Shadow::Shadow4,
            "--orb-elev-raised-sm",
            MaterialElevation::Resting,
        ),
        (
            Shadow::Shadow8,
            "--orb-elev-raised-md",
            MaterialElevation::Raised,
        ),
        (
            Shadow::Shadow16,
            "--orb-elev-floating",
            MaterialElevation::Floating,
        ),
        (
            Shadow::Shadow28,
            "--orb-elev-overlay",
            MaterialElevation::Floating,
        ),
        (
            Shadow::Shadow64,
            "--orb-elev-modal",
            MaterialElevation::Modal,
        ),
    ];

    view! {
        <div
            data-testid="intro-elevation-matrix"
            style="padding: 16px; background: var(--orb-color-surface-subtle); border-radius: var(--orb-radius-md);"
        >
            <Flex wrap=FlexWrap::Wrap gap=FlexGap::Medium align=FlexAlign::FlexStart>
                {SHADOWS
                    .into_iter()
                    .map(|(shadow, token, elevation)| {
                        view! {
                            <Material variant=MaterialVariant::Solid elevation=elevation>
                                <Box
                                    padding=SpacingInset::all_m()
                                    background=ThemeColor::NeutralBackground1
                                    shadow=shadow
                                    radius=BorderRadius::Medium
                                >
                                    <Body1 block=true>{token}</Body1>
                                    <Caption1>{shadow.name()}</Caption1>
                                </Box>
                            </Material>
                        }
                    })
                    .collect_view()}
            </Flex>
        </div>
    }
}

#[component]
pub fn TypographyRampVisual() -> impl IntoView {
    use orbital::components::{Caption1 as Cap, Display, Subtitle2 as Sub2, Title3};

    view! {
        <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
            <div data-testid="intro-typography-ramp">
                <Flex
                    vertical=true
                    gap=SpacingSize::Size120.flex_gap()
                    padding=SpacingSize::Size160.inset()
                    full_width=true
                >
                    <TypeRow preset="Caption1"><Cap>"Caption1 — metadata, timestamps"</Cap></TypeRow>
                    <TypeRow preset="Body1"><Body1 block=true>"Body1 — default reading text"</Body1></TypeRow>
                    <TypeRow preset="Subtitle2"><Sub2>"Subtitle2 — section headers"</Sub2></TypeRow>
                    <TypeRow preset="Title3"><Title3>"Title3 — page titles"</Title3></TypeRow>
                    <TypeRow preset="Display"><Display>"Display"</Display></TypeRow>
                </Flex>
            </div>
        </Material>
    }
}

#[component]
fn TypeRow(preset: &'static str, children: Children) -> impl IntoView {
    view! {
        <div style=format!(
            "padding-bottom: var(--orb-space-block-md); border-bottom: {} solid var(--orb-color-border-subtle);",
            StrokeWidth::Thin.css_var()
        )>
            <Flex vertical=true align=FlexAlign::Stretch gap=SpacingSize::Size40.flex_gap()>
                <Caption1>{preset}</Caption1>
                {children()}
            </Flex>
        </div>
    }
}

#[component]
pub fn FontFamilyVisual() -> impl IntoView {
    const SAMPLE: &str = "Orbital 0123";
    const SAMPLES: [(FontFamily, &str, &str, &str); 4] = [
        (
            FontFamily::Base,
            "Base",
            "League Spartan",
            "intro-font-base",
        ),
        (
            FontFamily::Numeric,
            "Numeric",
            "League Spartan (tabular)",
            "intro-font-numeric",
        ),
        (
            FontFamily::Monospace,
            "Monospace",
            "League Mono",
            "intro-font-monospace",
        ),
        (
            FontFamily::Display,
            "Display",
            "Orbitron",
            "intro-font-display",
        ),
    ];

    view! {
        <div data-testid="intro-font-families">
            <Flex wrap=FlexWrap::Wrap gap=SpacingSize::Size160.flex_gap() align=FlexAlign::Stretch>
                {SAMPLES
                    .into_iter()
                    .map(|(family, token_label, font_name, test_id)| {
                        let extra_style = if family == FontFamily::Numeric {
                            "font-variant-numeric: tabular-nums;"
                        } else {
                            ""
                        };
                        let style = format!(
                            "flex: 1; min-width: 160px; font-family: {}; {}",
                            family.css_var(),
                            extra_style
                        );
                        view! {
                            <div data-testid=test_id style=style>
                                <Flex vertical=true gap=SpacingSize::Size40.flex_gap()>
                                    <Caption1Strong>{token_label}</Caption1Strong>
                                    <Caption1>{font_name}</Caption1>
                                    <Body1 block=true>{SAMPLE}</Body1>
                                </Flex>
                            </div>
                        }
                    })
                    .collect_view()}
            </Flex>
        </div>
    }
}

#[component]
pub fn FontWeightVisual() -> impl IntoView {
    const WEIGHTS: [(FontWeight, &str); 3] = [
        (FontWeight::Regular, "Regular"),
        (FontWeight::Semibold, "Semibold"),
        (FontWeight::Bold, "Bold"),
    ];

    view! {
        <div data-testid="intro-font-weights">
            <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                {WEIGHTS
                    .into_iter()
                    .map(|(weight, label)| {
                        view! {
                            <div style=format!("font-weight: {};", weight.css_var())>
                                <Body1 block=true>{format!("{label} — {text}", text = weight.name())}</Body1>
                            </div>
                        }
                    })
                    .collect_view()}
            </Flex>
        </div>
    }
}

#[component]
pub fn FontSizeRampVisual() -> impl IntoView {
    const SIZES: [(FontSize, &str); 5] = [
        (FontSize::Base200, "Caption"),
        (FontSize::Base300, "Body"),
        (FontSize::Base500, "Title"),
        (FontSize::Base700, "Large title"),
        (FontSize::Base1000, "Display"),
    ];

    view! {
        <div data-testid="intro-font-size-ramp">
            <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                {SIZES
                    .into_iter()
                    .map(|(size, label)| {
                        view! {
                            <div style=format!("font-size: {};", size.css_var())>
                                <Body1 block=true>{format!("{label} ({})", size.name())}</Body1>
                            </div>
                        }
                    })
                    .collect_view()}
            </Flex>
        </div>
    }
}
