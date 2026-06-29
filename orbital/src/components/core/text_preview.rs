//! Typography preview page for the component catalog.

use crate::preview::PreviewRegistration;
use leptos::prelude::*;
use orbital_macros::component_doc;

use super::{
    Body1, Body1Strong, Body1Stronger, Body2, Caption1, Caption1Strong, Caption1Stronger, Caption2,
    Caption2Strong, ComponentPreviewCard, Display, Flex, FormHint, FormLabel, LargeTitle,
    OrbitalComponentView, SectionTitle, Subtitle1, Subtitle2, Subtitle2Stronger, Text, TextAlign,
    TextFont, TextSize, TextTag, TextWeight, Title1, Title2, Title3,
};

/// Catalog preview for Orbital typography presets and [`Text`] props.
#[component_doc(
    category = "Typography",
    preview = "manual",
    preview_slug = "text",
    preview_label = "Text"
)]
#[component]
pub fn TextPreview() -> impl IntoView {
    view! {
        <div data-testid="text-preview">
            <OrbitalComponentView
                component_name="Text"
                component_description="Orbital typography gives you scale presets for consistent hierarchy. Page and shell titles: Title3–Title1. Content section and card titles: Subtitle1. Form clusters and TOC rail labels: SectionTitle. Body copy: Body1/Body2. Metadata: Caption1/Caption2. Named section labels must not render below Body1 (14px). Start with a preset, then drop to Text for explicit size, weight, font, or decoration. For forms, use FormLabel and FormHint inside a Field."
                default_example_title="Default"
                default_example_id="example-default"
                default_code=r#"<Body1>"Use Body1 for body copy, or Text when you need explicit size and weight."</Body1>"#
                default_description="Prefer a typography preset for hierarchy; use Text when you need explicit size, weight, or tag control."
                example_anchors=&[
                    ("Default", "default"),
                    ("Typography scale", "type-ramp"),
                    ("Form helpers", "form-helpers"),
                    ("Size", "size"),
                    ("Weight", "weight"),
                    ("Font", "font"),
                    ("Semantic tags", "semantic-tags"),
                    ("Italic, underline, and strikethrough", "italic-underline-and-strikethrough"),
                    ("Truncate", "truncate"),
                    ("Alignment", "alignment"),
                    ("Wrap", "wrap"),
                ]
                default=view! {
                    <Body1>"Use Body1 for body copy, or Text when you need explicit size and weight."</Body1>
                }
            >
                <ComponentPreviewCard
                    title="Typography scale"
                    description="Typography presets from Display through Caption2 — use named components instead of manual size/weight props for consistent hierarchy."
                    example_id="example-type-ramp"
                    code=r#"
<Flex vertical=true>
    <Display>Display</Display>
    <LargeTitle>LargeTitle</LargeTitle>
    <Title1>Title1</Title1>
    <Title2>Title2</Title2>
    <Title3>Title3</Title3>
    <Subtitle1>Subtitle1</Subtitle1>
    <Subtitle2>Subtitle2</Subtitle2>
    <Subtitle2Stronger>Subtitle2Stronger</Subtitle2Stronger>
    <Body1>Body1</Body1>
    <Body1Strong>Body1Strong</Body1Strong>
    <Body1Stronger>Body1Stronger</Body1Stronger>
    <Body2>Body2</Body2>
    <Caption1>Caption1</Caption1>
    <Caption1Strong>Caption1Strong</Caption1Strong>
    <Caption1Stronger>Caption1Stronger</Caption1Stronger>
    <Caption2>Caption2</Caption2>
    <Caption2Strong>Caption2Strong</Caption2Strong>
</Flex>
                    "#
                >
                    <div data-testid="text-type-ramp">
                        <Flex vertical=true>
                        <Display>"Display"</Display>
                        <LargeTitle>"LargeTitle"</LargeTitle>
                        <Title1>"Title1"</Title1>
                        <Title2>"Title2"</Title2>
                        <Title3>"Title3"</Title3>
                        <Subtitle1>"Subtitle1"</Subtitle1>
                        <Subtitle2>"Subtitle2"</Subtitle2>
                        <Subtitle2Stronger>"Subtitle2Stronger"</Subtitle2Stronger>
                        <Body1>"Body1"</Body1>
                        <Body1Strong>"Body1Strong"</Body1Strong>
                        <Body1Stronger>"Body1Stronger"</Body1Stronger>
                        <Body2>"Body2"</Body2>
                        <Caption1>"Caption1"</Caption1>
                        <Caption1Strong>"Caption1Strong"</Caption1Strong>
                        <Caption1Stronger>"Caption1Stronger"</Caption1Stronger>
                        <Caption2>"Caption2"</Caption2>
                        <Caption2Strong>"Caption2Strong"</Caption2Strong>
                        </Flex>
                    </div>
                </ComponentPreviewCard>

                <ComponentPreviewCard
                    title="Form helpers"
                    description="FormLabel, FormHint, and SectionTitle presets for accessible field labels and grouped settings headings."
                    example_id="example-form-helpers"
                    code=r#"
<Flex vertical=true>
    <FormLabel>"Email address"</FormLabel>
    <FormHint>"We'll never share your email."</FormHint>
    <SectionTitle>"Account settings"</SectionTitle>
</Flex>
                    "#
                >
                    <div data-testid="text-form-helpers">
                        <Flex vertical=true>
                        <FormLabel>"Email address"</FormLabel>
                        <FormHint>"We'll never share your email."</FormHint>
                        <SectionTitle>"Account settings"</SectionTitle>
                        </Flex>
                    </div>
                </ComponentPreviewCard>

                <ComponentPreviewCard
                    title="Size"
                    description="TextSize tokens from 100 (smallest) to 1000 (largest) — pick a step on the typography scale instead of raw font-size values."
                    example_id="example-size"
                    code=r#"
<Flex vertical=true>
    <Text size=TextSize::S100>"100"</Text>
    <Text size=TextSize::S200>"200"</Text>
    <Text size=TextSize::S300>"300"</Text>
    <Text size=TextSize::S400>"400"</Text>
    <Text size=TextSize::S500>"500"</Text>
    <Text size=TextSize::S600>"600"</Text>
    <Text size=TextSize::S700>"700"</Text>
    <Text size=TextSize::S800>"800"</Text>
    <Text size=TextSize::S900>"900"</Text>
    <Text size=TextSize::S1000>"1000"</Text>
</Flex>
                    "#
                >
                    <div data-testid="text-sizes">
                        <Flex vertical=true>
                        <Text size=TextSize::S100>"100"</Text>
                        <Text size=TextSize::S200>"200"</Text>
                        <Text size=TextSize::S300>"300"</Text>
                        <Text size=TextSize::S400>"400"</Text>
                        <Text size=TextSize::S500>"500"</Text>
                        <Text size=TextSize::S600>"600"</Text>
                        <Text size=TextSize::S700>"700"</Text>
                        <Text size=TextSize::S800>"800"</Text>
                        <Text size=TextSize::S900>"900"</Text>
                        <Text size=TextSize::S1000>"1000"</Text>
                        </Flex>
                    </div>
                </ComponentPreviewCard>

                <ComponentPreviewCard
                    title="Weight"
                    description="Font weight presets from Regular through Bold — combine with size tokens for emphasis without custom CSS."
                    example_id="example-weight"
                    code=r#"
<Flex vertical=true>
    <Text weight=TextWeight::Regular>"Regular"</Text>
    <Text weight=TextWeight::Medium>"Medium"</Text>
    <Text weight=TextWeight::Semibold>"Semibold"</Text>
    <Text weight=TextWeight::Bold>"Bold"</Text>
</Flex>
                    "#
                >
                    <div data-testid="text-weights">
                        <Flex vertical=true>
                        <Text weight=TextWeight::Regular>"Regular"</Text>
                        <Text weight=TextWeight::Medium>"Medium"</Text>
                        <Text weight=TextWeight::Semibold>"Semibold"</Text>
                        <Text weight=TextWeight::Bold>"Bold"</Text>
                        </Flex>
                    </div>
                </ComponentPreviewCard>

                <ComponentPreviewCard
                    title="Font"
                    description="Base, Numeric (tabular figures), and Monospace font families — use Numeric for aligned counters and Monospace for code snippets."
                    example_id="example-font"
                    code=r#"
<Flex vertical=true>
    <Text font=TextFont::Base>"Base font"</Text>
    <Text font=TextFont::Numeric>"Numeric font (tabular nums): 0123456789"</Text>
    <Text font=TextFont::Monospace>"Monospace font"</Text>
</Flex>
                    "#
                >
                    <div data-testid="text-fonts">
                        <Flex vertical=true>
                        <Text font=TextFont::Base>"Base font"</Text>
                        <Text font=TextFont::Numeric>"Numeric font (tabular nums): 0123456789"</Text>
                        <Text font=TextFont::Monospace>"Monospace font"</Text>
                        </Flex>
                    </div>
                </ComponentPreviewCard>

                <ComponentPreviewCard
                    title="Semantic tags"
                    description="Render Text as native HTML elements (h1–h3, p, strong, code, pre) for accessible document structure and SEO."
                    example_id="example-semantic-tags"
                    code=r#"
<Flex vertical=true>
    <Text tag=TextTag::H1 block=true>"Heading 1"</Text>
    <Text tag=TextTag::H2 block=true>"Heading 2"</Text>
    <Text tag=TextTag::H3 block=true>"Heading 3"</Text>
    <Text tag=TextTag::P block=true>"Paragraph text"</Text>
    <Text tag=TextTag::Span>"Inline span"</Text>
    <Text tag=TextTag::Strong>"Strong text"</Text>
    <Text tag=TextTag::Em>"Emphasized text"</Text>
    <Text tag=TextTag::Code>"Inline code"</Text>
    <Text tag=TextTag::Pre block=true>"Preformatted block"</Text>
</Flex>
                    "#
                >
                    <div data-testid="text-semantic-tags">
                        <Flex vertical=true>
                        <Text tag=TextTag::H1 block=true>"Heading 1"</Text>
                        <Text tag=TextTag::H2 block=true>"Heading 2"</Text>
                        <Text tag=TextTag::H3 block=true>"Heading 3"</Text>
                        <Text tag=TextTag::P block=true>"Paragraph text"</Text>
                        <Text tag=TextTag::Span>"Inline span"</Text>
                        <Text tag=TextTag::Strong>"Strong text"</Text>
                        <Text tag=TextTag::Em>"Emphasized text"</Text>
                        <Text tag=TextTag::Code>"Inline code"</Text>
                        <Text tag=TextTag::Pre block=true>"Preformatted block"</Text>
                        </Flex>
                    </div>
                </ComponentPreviewCard>

                <ComponentPreviewCard
                    title="Italic, underline, and strikethrough"
                    description="Text decoration props for emphasis, links, and deleted-state copy — combine underline and strikethrough when needed."
                    example_id="example-italic-underline-and-strikethrough"
                    code=r#"
<Flex vertical=true>
    <Text italic=true>"Italic text"</Text>
    <Text underline=true>"Underlined text"</Text>
    <Text strikethrough=true>"Strikethrough text"</Text>
    <Text underline=true strikethrough=true>"Underlined and struck through"</Text>
</Flex>
                    "#
                >
                    <div data-testid="text-decorations">
                        <Flex vertical=true>
                        <Text italic=true>"Italic text"</Text>
                        <Text underline=true>"Underlined text"</Text>
                        <Text strikethrough=true>"Strikethrough text"</Text>
                        <Text underline=true strikethrough=true>
                            "Underlined and struck through"
                        </Text>
                        </Flex>
                    </div>
                </ComponentPreviewCard>

                <ComponentPreviewCard
                    title="Truncate"
                    description="Single-line ellipsis when copy exceeds a fixed-width container — pair with block=true inside a bounded wrapper div. Truncated text may be unreadable to screen readers unless the full content is available elsewhere."
                    example_id="example-truncate"
                    code=r#"
<div style="width: 240px; border: 1px solid var(--orb-color-border-default); padding: 8px;">
    <Text block=true truncate=true>
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
    </Text>
</div>
                    "#
                >
                    <div style="width: 240px; border: var(--orb-stroke-thin) solid var(--orb-color-border-default); padding: var(--orb-space-block-sm);">
                        <Text block=true truncate=true test_id="text-truncate">
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                        </Text>
                    </div>
                </ComponentPreviewCard>

                <ComponentPreviewCard
                    title="Alignment"
                    description="Block-level text alignment from start through justify — use justify for multi-line paragraphs that should fill the line box."
                    example_id="example-alignment"
                    code=r#"
<div style="width: 360px; border: 1px solid var(--orb-color-border-default); padding: 8px;">
    <Text block=true align=TextAlign::Start>"Aligned to start"</Text>
    <Text block=true align=TextAlign::Center>"Aligned to center"</Text>
    <Text block=true align=TextAlign::End>"Aligned to end"</Text>
    <Text block=true align=TextAlign::Justify>
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit."
    </Text>
</div>
                    "#
                >
                    <div style="width: 360px; border: var(--orb-stroke-thin) solid var(--orb-color-border-default); padding: var(--orb-space-block-sm);">
                        <Text block=true align=TextAlign::Start test_id="text-align-start">
                            "Aligned to start"
                        </Text>
                        <Text block=true align=TextAlign::Center>"Aligned to center"</Text>
                        <Text block=true align=TextAlign::End>"Aligned to end"</Text>
                        <Text block=true align=TextAlign::Justify>
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit."
                        </Text>
                    </div>
                </ComponentPreviewCard>

                <ComponentPreviewCard
                    title="Wrap"
                    description="Control line breaking in narrow containers — nowrap keeps a single line (may overflow), wrap breaks across lines."
                    example_id="example-wrap"
                    code=r#"
<div style="width: 180px; border: 1px solid var(--orb-color-border-default); padding: 8px;">
    <Text block=true wrap=false>"This nowrap text stays on a single line and may overflow."</Text>
    <Text block=true wrap=true>"This wrapping text breaks across multiple lines when space is limited."</Text>
</div>
                    "#
                >
                    <div style="width: 180px; border: var(--orb-stroke-thin) solid var(--orb-color-border-default); padding: var(--orb-space-block-sm);">
                        <Text block=true wrap=false test_id="text-nowrap">
                            "This nowrap text stays on a single line and may overflow."
                        </Text>
                        <Text block=true wrap=true>
                            "This wrapping text breaks across multiple lines when space is limited."
                        </Text>
                    </div>
                </ComponentPreviewCard>
            </OrbitalComponentView>
        </div>
    }
}

#[cfg(feature = "preview")]
pub static TEXTPREVIEW_PREVIEW_REGISTRATION: PreviewRegistration = PreviewRegistration {
    slug: "text",
    label: "Text",
    section: "Getting Started",
    section_priority: 1,
    category: "Typography",
    category_priority: 100,
    category_default_collapsed: true,
    group: "",
    group_priority: 0,
    nav_item: false,
    icon: icondata::AiFileTextOutlined,
    render: || TextPreview().into_any(),
};
