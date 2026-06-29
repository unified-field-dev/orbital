use leptos::prelude::*;
use orbital::components::{
    Anchor, AnchorLink, Aside, Body1, Body1Strong, Caption1, Content, ContentWithAside, Flex,
    FlexAlign, FlexGap, Link, MessageBar, MessageBarBody, MessageBarIntent, MessageBarLayout,
    SectionTitle, SpacingSize, Subtitle1, TextTag, Title1, Title2,
};

use super::introduction_demos::{
    BorderRadiusDemo, ColorSemanticDemo, ColorTokenSwatches, ElevationMatrixDemo,
    MaterialFrostDemo, MaterialScrimDemo, MaterialSolidDemo, MaterialVariantRecap,
    SpacingProximityDemo, SpacingRampDemo, StrokeWidthDemo, TypographyFontsDemo,
    TypographyFormHelpersDemo, TypographyRampDemo,
};

/// Orbital design language introduction — author-facing guide at `/`.
#[component]
pub fn IntroductionPage() -> impl IntoView {
    let (style_sheet, styles) = turf::inline_style_sheet_values! {
        .Page {
            padding-block: var(--orb-space-block-2xl);
        }
        .HeroLead {
            margin-bottom: var(--orb-space-block-2xl);
        }
        .Chapter {
            scroll-margin-top: var(--orb-space-block-2xl);
            border-bottom: 1px solid var(--orb-color-border-subtle);
            padding-bottom: var(--orb-space-block-2xl);
        }
        .Section {
            scroll-margin-top: var(--orb-space-block-lg);
        }
        .Table {
            display: grid;
            gap: var(--orb-space-block-xs);
            font-size: var(--orb-type-size-sm);
            line-height: var(--orb-type-line-md);
            color: var(--orb-color-text-primary);
        }
        .TableHeader {
            display: contents;
            font-weight: var(--orb-type-weight-semibold);
            color: var(--orb-color-text-tertiary);
        }
        .TableRow {
            display: contents;
        }
        .TableCell {
            padding: var(--orb-space-block-sm) var(--orb-space-inline-md);
            border-bottom: var(--orb-stroke-thin) solid var(--orb-color-border-muted);
        }
        .Mono {
            font-family: var(--orb-type-family-mono);
            font-size: var(--orb-type-size-xs);
        }
    };

    let classes = TocClassNames {
        chapter: styles.chapter,
        section: styles.section,
        table: styles.table,
        table_header: styles.table_header,
        table_row: styles.table_row,
        table_cell: styles.table_cell,
        mono: styles.mono,
    };

    view! {
        <style>{style_sheet}</style>
        <div data-testid="preview-index">
            <div class=styles.page data-testid="introduction-page">
                <ContentWithAside>
                    <Content slot>
                        <IntroHero class=styles.hero_lead />
                        <Flex
                            vertical=true
                            align=FlexAlign::Stretch
                            gap=FlexGap::Size(56)
                            full_width=true
                        >
                            <PrinciplesChapter classes=classes />
                            <LayoutChapter classes=classes />
                            <ColorChapter classes=classes />
                            <ElevationChapter classes=classes />
                            <MaterialChapter classes=classes />
                            <TypographyChapter classes=classes />
                            <MotionChapter classes=classes />
                        </Flex>
                    </Content>
                    <Aside slot>
                        <Flex vertical=true align=FlexAlign::Stretch gap=FlexGap::Small full_width=true>
                            <SectionTitle>"On this page"</SectionTitle>
                            <Anchor>
                                <AnchorLink title="Principles".to_string() href="#principles" />
                                <AnchorLink title="Layout".to_string() href="#layout" />
                                <AnchorLink title="Shape".to_string() href="#layout-shape" />
                                <AnchorLink title="Color".to_string() href="#color" />
                                <AnchorLink title="Elevation".to_string() href="#elevation" />
                                <AnchorLink title="Material".to_string() href="#material" />
                                <AnchorLink title="Typography".to_string() href="#typography" />
                                <AnchorLink title="Motion".to_string() href="#motion" />
                            </Anchor>
                        </Flex>
                    </Aside>
                </ContentWithAside>
            </div>
        </div>
    }
}

#[component]
fn IntroHero(class: &'static str) -> impl IntoView {
    view! {
        <Flex vertical=true align=FlexAlign::Stretch gap=FlexGap::Medium full_width=true>
            <Title1 tag=TextTag::H1 block=true test_id="preview-page-title">
                "Introduction"
            </Title1>
            <Body1 block=true>
                "Orbital is a Leptos component library for building focused, accessible product interfaces. "
                "The sections below define spacing, type, color, surfaces, and motion in concrete terms. "
                "Start with four principles that explain why those rules exist."
            </Body1>
            <Body1 block=true class=class>
                "Use the anchor rail to jump to a chapter, or browse individual components in the sidebar."
            </Body1>
        </Flex>
    }
}

#[derive(Clone, Copy)]
struct TocClassNames {
    chapter: &'static str,
    section: &'static str,
    table: &'static str,
    table_header: &'static str,
    table_row: &'static str,
    table_cell: &'static str,
    mono: &'static str,
}

#[component]
fn ChapterHeading(id: &'static str, title: &'static str) -> impl IntoView {
    view! {
        <div id=id>
            <Title2>{title}</Title2>
        </div>
    }
}

#[component]
fn SectionHeading(id: &'static str, title: &'static str, class: &'static str) -> impl IntoView {
    view! {
        <div id=id class=class>
            <Subtitle1>{title}</Subtitle1>
        </div>
    }
}

#[component]
fn PrinciplesChapter(classes: TocClassNames) -> impl IntoView {
    view! {
        <Flex
            vertical=true
            align=FlexAlign::Stretch
            gap=FlexGap::Size(32)
            full_width=true
            class=classes.chapter
        >
            <ChapterHeading id="principles" title="Principles" />

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading
                    id="principles-use"
                    title="How to use these principles"
                    class=classes.section
                />
                <MessageBar intent=MessageBarIntent::Info layout=MessageBarLayout::Multiline>
                    <MessageBarBody>
                        "Principles are review criteria—not slogans. Walk the four principles before shipping and let them break ties between valid options."
                    </MessageBarBody>
                </MessageBar>
                <Body1 block=true>
                    "Before shipping a page, walk the four principles "
                    "and check whether spacing, type, and interaction choices support them. When two implementation "
                    "options are valid, the principle that serves the current task breaks the tie."
                </Body1>
            </Flex>

            <PrincipleBlock
                id="principles-familiar"
                title="Familiar on every surface"
                class=classes.section
                body="Interfaces adapt to the device and build on patterns users already know. Invest custom UX only where the product has a signature moment worth learning."
                practice="Use Orbital shell layouts before inventing new chrome. Prefer responsive padding and reflow over fixed desktop layouts shrunk to mobile. Reach for standard components over one-off styled wrappers."
                avoid="Novel navigation on every app; hiding core actions behind unique gestures; desktop-only density on phone widths."
            />

            <PrincipleBlock
                id="principles-focus"
                title="Built for focus"
                class=classes.section
                body="Draw attention to the next action and the information needed for the current task. Remove noise so people stay in flow."
                practice="Keep the content canvas as the lightest, most prominent surface. One primary action per region; secondary actions use subtle button appearances. Use typography hierarchy—one clear page title."
                avoid="Dense walls of controls; competing primary buttons; decorative color and motion that do not carry meaning."
            />

            <PrincipleBlock
                id="principles-inclusive"
                title="Inclusive by design"
                class=classes.section
                body="Design for a range of abilities, preferences, and contexts from the start—not as a retrofit."
                practice="Never rely on color alone for status; pair with text or icon. Ensure keyboard reachability and visible focus. Respect prefers-reduced-motion; label icon-only controls."
                avoid="Placeholder-only error states; contrast that fails accessibility targets; motion that cannot be reduced."
            />

            <PrincipleBlock
                id="principles-cohesive"
                title="Cohesive product character"
                class=classes.section
                body="Apps built with Orbital should feel like parts of one platform: shared tokens, shared motion, shared patterns."
                practice="Use design tokens for color, spacing, shadow, and radius—no ad-hoc hex in app code. Compose with Material, Stack, and typography presets. Brand accent sparingly on CTAs and selection."
                avoid="Per-app shadow and radius values; forked copies of shell components; brand color as the default page background."
            />

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading
                    id="principles-glance"
                    title="Principles at a glance"
                    class=classes.section
                />
                <div class=classes.table style="grid-template-columns: 2fr 1fr;">
                    <div class=classes.table_header>
                        <div class=classes.table_cell>"Principle"</div>
                        <div class=classes.table_cell>"Primary chapters"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Familiar on every surface"</div>
                        <div class=classes.table_cell>"Layout, shell patterns, responsive"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Built for focus"</div>
                        <div class=classes.table_cell>"Material, Elevation, Typography"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Inclusive by design"</div>
                        <div class=classes.table_cell>"Typography, Color, Motion"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Cohesive product character"</div>
                        <div class=classes.table_cell>"Tokens, Material, Typography presets"</div>
                    </div>
                </div>
            </Flex>
        </Flex>
    }
}

#[component]
fn PrincipleBlock(
    id: &'static str,
    title: &'static str,
    class: &'static str,
    body: &'static str,
    practice: &'static str,
    avoid: &'static str,
) -> impl IntoView {
    view! {
        <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
            <SectionHeading id=id title=title class=class />
            <Body1 block=true>{body}</Body1>
            <Body1Strong block=true>"In practice"</Body1Strong>
            <Body1 block=true>{practice}</Body1>
            <Body1Strong block=true>"Avoid"</Body1Strong>
            <Body1 block=true>{avoid}</Body1>
        </Flex>
    }
}

#[component]
fn LayoutChapter(classes: TocClassNames) -> impl IntoView {
    view! {
        <Flex
            vertical=true
            align=FlexAlign::Stretch
            gap=FlexGap::Size(32)
            full_width=true
            class=classes.chapter
        >
            <ChapterHeading id="layout" title="Layout" />

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading
                    id="layout-spacing"
                    title="Spacing and proximity"
                    class=classes.section
                />
                <Body1 block=true>
                    "Space is a grouping tool. When two controls share a small gap, people read them as one decision. "
                    "When sections are separated by a full ramp step or more, the page reads in clear chunks without divider lines."
                </Body1>
                <Body1 block=true>
                    "Orbital constrains gaps to "
                    <span class=classes.mono>"SpacingSize"</span>
                    " and CSS spacing tokens—avoid one-off margin values that break rhythm across pages."
                </Body1>
                <Caption1>"Common spacing ramp values"</Caption1>
                <div class=classes.table style="grid-template-columns: 2fr 1fr;">
                    <div class=classes.table_header>
                        <div class=classes.table_cell>"Token"</div>
                        <div class=classes.table_cell>"Pixels"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Size40"</div>
                        <div class=classes.table_cell>"4px"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Size80"</div>
                        <div class=classes.table_cell>"8px"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Size120"</div>
                        <div class=classes.table_cell>"12px"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Size160 (default)"</div>
                        <div class=classes.table_cell>"16px"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Size240"</div>
                        <div class=classes.table_cell>"24px"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Size320"</div>
                        <div class=classes.table_cell>"32px"</div>
                    </div>
                </div>
                <SpacingRampDemo />
                <SpacingProximityDemo />
                <Body1 block=true>
                    "Shell content padding is responsive: 32px desktop ("
                    <span class=classes.mono>"Size320"</span>
                    "), 24px tablet ("
                    <span class=classes.mono>"Size240"</span>
                    "), 16px mobile ("
                    <span class=classes.mono>"Size160"</span>
                    "). Regions in the shell body use a 12px gap ("
                    <span class=classes.mono>"Size120"</span>
                    ") so chrome does not butt together."
                </Body1>
            </Flex>

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading
                    id="layout-shape"
                    title="Shape — radius and stroke"
                    class=classes.section
                />
                <Body1 block=true>
                    "Border radius and stroke width are first-class tokens. Use "
                    <span class=classes.mono>"BorderRadius"</span>
                    " and "
                    <span class=classes.mono>"StrokeWidth"</span>
                    " enums on "
                    <span class=classes.mono>"Box"</span>
                    " and in runtime "
                    <span class=classes.mono>"style"</span>
                    " strings via "
                    <span class=classes.mono>"Display"</span>
                    " / "
                    <span class=classes.mono>"css_var()"</span>
                    "."
                </Body1>
                <BorderRadiusDemo />
                <StrokeWidthDemo />
            </Flex>

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading
                    id="layout-components"
                    title="Choosing a layout component"
                    class=classes.section
                />
                <Body1 block=true>
                    "Start with the simplest primitive that fits. "
                    <span class=classes.mono>"Stack"</span>
                    " is the default for even-gap vertical sections. "
                    <span class=classes.mono>"Space"</span>
                    " distributes children to opposite edges. "
                    "Reach for "
                    <span class=classes.mono>"Flex"</span>
                    " when you need wrap, inline placement, fill, or inset padding."
                </Body1>
                <div class=classes.table style="grid-template-columns: 1.2fr 2fr;">
                    <div class=classes.table_header>
                        <div class=classes.table_cell>"Component"</div>
                        <div class=classes.table_cell>"Use when"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>
                            <Link href="/box">"Box"</Link>
                        </div>
                        <div class=classes.table_cell>"One wrapper needs token-based padding or sizing—not sibling distribution"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>
                            <Link href="/stack">"Stack"</Link>
                        </div>
                        <div class=classes.table_cell>"Even gaps on one axis (forms, button rows); defaults to column + full-width"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>
                            <Link href="/space">"Space"</Link>
                        </div>
                        <div class=classes.table_cell>"Opposite-edge distribution (page headers, footer action bars); defaults to space-between + full-width"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>
                            <Link href="/flex">"Flex"</Link>
                        </div>
                        <div class=classes.table_cell>"Full 1D control: wrap, inline, fill, inset padding, or custom alignment"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>
                            <Link href="/grid">"Grid"</Link>
                        </div>
                        <div class=classes.table_cell>"Two-dimensional layouts where column span and offset matter"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>
                            <Link href="/auto-grid">"AutoGrid"</Link>
                        </div>
                        <div class=classes.table_cell>"Fluid card walls that reflow by viewport without manual breakpoints"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>
                            <Link href="/container">"Container"</Link>
                        </div>
                        <div class=classes.table_cell>"Page-level max-width and horizontal centering inside the shell"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>
                            <Link href="/content-with-aside">"ContentWithAside"</Link>
                        </div>
                        <div class=classes.table_cell>"Documentation pages with a growing content column and sticky aside rail"</div>
                    </div>
                </div>
            </Flex>

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading id="layout-grid" title="Grid" class=classes.section />
                <Body1 block=true>
                    "A column grid splits the content area into even tracks—use "
                    <span class=classes.mono>"Grid"</span>
                    " and "
                    <span class=classes.mono>"GridItem"</span>
                    " when column span matters. When tile count and width vary, "
                    <span class=classes.mono>"AutoGrid"</span>
                    " reflows columns using minmax so you do not maintain breakpoint-specific column counts by hand."
                </Body1>
            </Flex>

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading id="layout-alignment" title="Alignment" class=classes.section />
                <Body1 block=true>
                    "Misaligned baselines make forms feel broken faster than wrong colors. "
                    "In a row of controls, align on the cross axis with Flex or Stack align props. "
                    "Pair icons and labels by centering the glyph and start-aligning the label text."
                </Body1>
            </Flex>

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading
                    id="layout-responsive"
                    title="Responsive design"
                    class=classes.section
                />
                <Body1 block=true>
                    "Responsive layout is a set of techniques: reposition (stack to row), resize (container max-width), "
                    "reflow (AutoGrid columns), show/hide (nav collapse), and re-architect (master/detail split on desktop, single pane on mobile)."
                </Body1>
                <Caption1>"Breakpoint reference"</Caption1>
                <div class=classes.table style="grid-template-columns: 1fr 1fr 1fr;">
                    <div class=classes.table_header>
                        <div class=classes.table_cell>"Size class"</div>
                        <div class=classes.table_cell>"Range"</div>
                        <div class=classes.table_cell>"Breakpoint"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Small"</div>
                        <div class=classes.table_cell>"320–479px"</div>
                        <div class=classes.table_cell>"< 480px"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Large"</div>
                        <div class=classes.table_cell>"640–1023px"</div>
                        <div class=classes.table_cell>"< 1024px"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"X-Large"</div>
                        <div class=classes.table_cell>"1024px+"</div>
                        <div class=classes.table_cell>"≥ 1024px"</div>
                    </div>
                </div>
            </Flex>
        </Flex>
    }
}

#[component]
fn ColorChapter(classes: TocClassNames) -> impl IntoView {
    view! {
        <Flex
            vertical=true
            align=FlexAlign::Stretch
            gap=FlexGap::Size(32)
            full_width=true
            class=classes.chapter
        >
            <ChapterHeading id="color" title="Color" />
            <MessageBar intent=MessageBarIntent::Warning layout=MessageBarLayout::Multiline>
                <MessageBarBody>
                    "Never rely on color alone for status or meaning—always pair semantic color with text, icons, or labels."
                </MessageBarBody>
            </MessageBar>
            <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
                <Body1 block=true>
                    "Color expresses style, communicates meaning, and supports hierarchy. "
                    "Orbital organizes color into neutral, shared accent, and brand palettes applied through design tokens."
                </Body1>
                <Body1 block=true>
                    "Neutral colors carry surfaces, text, and layout chrome. Shared accent colors highlight reusable components. "
                    "Brand colors identify the product—use them sparingly on large surfaces."
                </Body1>
                <Body1 block=true>
                    "Semantic status colors (success, warning, danger) build on real-world associations. "
                    "Always pair them with text or icons—never rely on color alone."
                </Body1>
                <Body1 block=true>
                    "Lighter neutrals on primary focus surfaces and darker neutrals on surrounding chrome draw the eye to the task. "
                    "Interaction states progress from rest through hover, pressed, and selected; focus adds a thicker stroke."
                </Body1>
                <Caption1>"Key token families"</Caption1>
                <Body1 block=true>
                    <span class=classes.mono>"--orb-color-surface-*"</span>
                    ", "
                    <span class=classes.mono>"--orb-color-text-*"</span>
                    ", "
                    <span class=classes.mono>"--orb-color-brand-bg"</span>
                    ", "
                    <span class=classes.mono>"--orb-color-status-success-fg"</span>
                    ", and related status tokens."
                </Body1>
                <ColorTokenSwatches mono_class=classes.mono />
                <Caption1>"Semantic color with text"</Caption1>
                <ColorSemanticDemo />
            </Flex>
        </Flex>
    }
}

#[component]
fn ElevationChapter(classes: TocClassNames) -> impl IntoView {
    view! {
        <Flex
            vertical=true
            align=FlexAlign::Stretch
            gap=FlexGap::Size(32)
            full_width=true
            class=classes.chapter
        >
            <ChapterHeading id="elevation" title="Elevation" />

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading
                    id="elevation-depth"
                    title="Depth and shadow"
                    class=classes.section
                />
                <Body1 block=true>
                    "Elevation is a hierarchy signal. A resting panel, a raised card, a dropdown, and a modal dialog each occupy a different depth. "
                    "Orbital encodes depth with named shadow tokens so surfaces feel coherent instead of each team tuning shadows by eye."
                </Body1>
            </Flex>

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading
                    id="elevation-ramp"
                    title="Elevation ramp"
                    class=classes.section
                />
                <div class=classes.table style="grid-template-columns: 1fr 1.5fr;">
                    <div class=classes.table_header>
                        <div class=classes.table_cell>"Token"</div>
                        <div class=classes.table_cell>"Typical use"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>
                            <span class=classes.mono>"--orb-elev-raised-sm"</span>
                        </div>
                        <div class=classes.table_cell>"Cards, list items, content canvas, AppBar"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>
                            <span class=classes.mono>"--orb-elev-raised-md"</span>
                        </div>
                        <div class=classes.table_cell>"Emphasized cards, raised command bars"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>
                            <span class=classes.mono>"--orb-elev-floating"</span>
                        </div>
                        <div class=classes.table_cell>"Callouts, transient panels"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>
                            <span class=classes.mono>"--orb-elev-modal"</span>
                        </div>
                        <div class=classes.table_cell>"Dialogs and modal panels"</div>
                    </div>
                </div>
                <ElevationMatrixDemo />
                <Body1 block=true>
                    "Match elevation to how long the surface stays and how much it blocks what is beneath. "
                    "Persistent page content stays at resting elevation. Blocking dialogs sit at the top of the ramp."
                </Body1>
            </Flex>

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading
                    id="elevation-shell"
                    title="Shell conventions"
                    class=classes.section
                />
                <div class=classes.table style="grid-template-columns: 1fr 1fr;">
                    <div class=classes.table_header>
                        <div class=classes.table_cell>"Region"</div>
                        <div class=classes.table_cell>"Elevation"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"AppBar"</div>
                        <div class=classes.table_cell>"Resting (--orb-elev-raised-sm)"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Content canvas"</div>
                        <div class=classes.table_cell>"Resting (--orb-elev-raised-sm)"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Drawer / transient overlay"</div>
                        <div class=classes.table_cell>"Floating (--orb-elev-floating)"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Dialogs"</div>
                        <div class=classes.table_cell>"Modal (--orb-elev-modal)"</div>
                    </div>
                </div>
            </Flex>
        </Flex>
    }
}

#[component]
fn MaterialChapter(classes: TocClassNames) -> impl IntoView {
    view! {
        <Flex
            vertical=true
            align=FlexAlign::Stretch
            gap=FlexGap::Size(32)
            full_width=true
            class=classes.chapter
        >
            <ChapterHeading id="material" title="Material" />

            <Body1 block=true>
                "Material describes what a surface feels like—opaque workspace, frosted glass, tinted backdrop, or dimmed scrim—not how far it floats. "
                "Depth is elevation (see above). Pick variant first, then elevation where shadow applies."
            </Body1>

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading id="material-solid" title="Solid" class=classes.section />
                <Body1 block=true>
                    "Solid material is the default for anything that stays on screen while the user works—page body, cards, nav rails. "
                    "Differentiate regions with background step tokens and elevation, not one-off hex fills."
                </Body1>
                <div class=classes.table style="grid-template-columns: 1fr 1.5fr;">
                    <div class=classes.table_header>
                        <div class=classes.table_cell>"Region"</div>
                        <div class=classes.table_cell>"Background token"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Shell ground"</div>
                        <div class=classes.table_cell>
                            <span class=classes.mono>"--orb-color-surface-subtle"</span>
                        </div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Content canvas"</div>
                        <div class=classes.table_cell>
                            <span class=classes.mono>"--orb-color-surface-canvas"</span>
                        </div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Top bar"</div>
                        <div class=classes.table_cell>
                            <span class=classes.mono>"--orb-color-surface-canvas"</span>
                        </div>
                    </div>
                </div>
                <MaterialSolidDemo />
            </Flex>

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading id="material-frost" title="Frost" class=classes.section />
                <Body1 block=true>
                    "Frost surfaces are semi-transparent with a frosted backdrop. "
                    "Use them for light-dismiss, transient overlays—menus and popovers—not for primary reading surfaces."
                </Body1>
                <MaterialFrostDemo />
            </Flex>

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading id="material-scrim" title="Scrim" class=classes.section />
                <Body1 block=true>
                    "Scrim is a dimmed overlay that blocks interaction with the page beneath. "
                    "Pair it with dialog content at modal elevation. Always provide a clear dismiss path and move keyboard focus into the elevated content."
                </Body1>
                <MaterialScrimDemo />
            </Flex>

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading
                    id="material-component"
                    title="The Material component"
                    class=classes.section
                />
                <Body1 block=true>
                    "The Material component declares surface finish and depth in one place. "
                    "Set variant to opaque, frosted, tinted, or dimmed. Set elevation when the surface should lift off its parent."
                </Body1>
                <MaterialVariantRecap />
                <Body1 block=true>
                    <Link href="/material">"View Material preview →"</Link>
                </Body1>
            </Flex>
        </Flex>
    }
}

#[component]
fn TypographyChapter(classes: TocClassNames) -> impl IntoView {
    view! {
        <Flex
            vertical=true
            align=FlexAlign::Stretch
            gap=FlexGap::Size(32)
            full_width=true
            class=classes.chapter
        >
            <ChapterHeading id="typography" title="Typography" />

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading
                    id="typography-ramp"
                    title="Hierarchy and type ramp"
                    class=classes.section
                />
                <Body1 block=true>
                    "The type ramp is a ladder of paired font size, line height, and weight presets. "
                    "Default reading text is Body1; scale up for titles, down for metadata."
                </Body1>
                <div class=classes.table style="grid-template-columns: 1fr 1fr 2fr;">
                    <div class=classes.table_header>
                        <div class=classes.table_cell>"Preset"</div>
                        <div class=classes.table_cell>"Size"</div>
                        <div class=classes.table_cell>"Typical use"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Caption1"</div>
                        <div class=classes.table_cell>"10px"</div>
                        <div class=classes.table_cell>"Metadata, timestamps"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Body1"</div>
                        <div class=classes.table_cell>"14px"</div>
                        <div class=classes.table_cell>"Default body text"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Subtitle1"</div>
                        <div class=classes.table_cell>"16px semibold"</div>
                        <div class=classes.table_cell>"Card titles, content section headers"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Title3"</div>
                        <div class=classes.table_cell>"20px semibold"</div>
                        <div class=classes.table_cell>"Page titles in the app shell"</div>
                    </div>
                    <div class=classes.table_row>
                        <div class=classes.table_cell>"Display"</div>
                        <div class=classes.table_cell>"60px semibold"</div>
                        <div class=classes.table_cell>"Hero and marketing statements"</div>
                    </div>
                </div>
                <TypographyRampDemo />
                <TypographyFontsDemo />
                <Body1 block=true>
                    <Link href="/text">"View full type specimen →"</Link>
                </Body1>
            </Flex>

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading
                    id="typography-presets"
                    title="Choosing a preset"
                    class=classes.section
                />
                <Body1 block=true>
                    "If you reach for a raw element with inline font styles, check the ramp first—there is almost always a preset that fits. "
                    "Use Title3 for app bar and panel titles, Subtitle1 for card and content section titles, SectionTitle for TOC rails and dense form groups, FormLabel for field labels, and FormHint for helper text below fields."
                </Body1>
                <TypographyFormHelpersDemo />
            </Flex>

            <Flex vertical=true gap=SpacingSize::Size320.flex_gap()>
                <SectionHeading
                    id="typography-styling"
                    title="Styling text"
                    class=classes.section
                />
                <Body1 block=true>
                    "Use sentence case for UI strings. Default start alignment for paragraphs and forms; center only for short, intentional focus. "
                    "Typography carries meaning through size and weight first, color second—muted captions use foreground tokens, not random gray hex. "
                    "Body text needs sufficient contrast against its background."
                </Body1>
            </Flex>
        </Flex>
    }
}

#[component]
fn MotionChapter(classes: TocClassNames) -> impl IntoView {
    view! {
        <Flex
            vertical=true
            align=FlexAlign::Stretch
            gap=FlexGap::Size(32)
            full_width=true
            class=classes.chapter
        >
            <ChapterHeading id="motion" title="Motion" />
            <MessageBar intent=MessageBarIntent::Info layout=MessageBarLayout::Multiline>
                <MessageBarBody>
                    "Motion confirms visibility changes—it should never distract from the task. "
                    "Always honor "
                    <span class=classes.mono>"prefers-reduced-motion"</span>
                    "; reserve decorative scroll effects for marketing surfaces, not routine data entry."
                </MessageBarBody>
            </MessageBar>
            <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
                <Body1 block=true>
                    "Orbital motion is a layered API in "
                    <span class=classes.mono>"orbital-motion"</span>
                    ": "
                    <Link href="/motion-tokens" inline=true>"MotionDuration"</Link>
                    " and "
                    <Link href="/motion-tokens" inline=true>"MotionCurve"</Link>
                    " tokens map to Orbital "
                    <span class=classes.mono>"--duration*"</span>
                    " and "
                    <span class=classes.mono>"--curve*"</span>
                    " theme variables; "
                    <Link href="/motion-atoms" inline=true>"PresenceMotion"</Link>
                    " presets pair enter/exit atoms; "
                    <Link href="/orbital-presence" inline=true>"OrbitalPresence"</Link>
                    " applies them to conditional children; and "
                    <Link href="/orbital-presence-group" inline=true>"OrbitalPresenceGroup"</Link>
                    " staggers keyed lists. See the "
                    <Link href="/motion" inline=true>"Motion overview"</Link>
                    " for the full developer story."
                </Body1>
                <Body1 block=true>
                    "Start with a "
                    <span class=classes.mono>"PresenceMotion"</span>
                    " preset ("
                    <span class=classes.mono>"fade"</span>
                    ", "
                    <span class=classes.mono>"slide"</span>
                    ", and others), wrap show/hide content in "
                    <span class=classes.mono>"OrbitalPresence"</span>
                    ", and tune timing with "
                    <span class=classes.mono>"with_duration"</span>
                    " / "
                    <span class=classes.mono>"with_curve"</span>
                    "—not arbitrary CSS transitions. Dialogs, drawers, toasts, and menus already integrate this stack internally; reach for "
                    <span class=classes.mono>"OrbitalPresence"</span>
                    " when you add your own mount/unmount surfaces."
                </Body1>
                <Body1 block=true>
                    <span class=classes.mono>"OrbitalPresence"</span>
                    " respects reduced motion by default ("
                    <span class=classes.mono>"respect_reduced_motion=true"</span>
                    "). Use "
                    <span class=classes.mono>"use_reduced_motion"</span>
                    " for imperative checks, or read "
                    <Link href="/motion-reduced-motion" inline=true>"Reduced motion"</Link>
                    " in the catalog."
                </Body1>
            </Flex>
        </Flex>
    }
}
