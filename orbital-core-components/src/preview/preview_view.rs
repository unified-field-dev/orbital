//! Full component preview page shell (tabs + default example).

use super::component_doc_markdown::ComponentDocMarkdown;
use super::component_doc_props::ComponentDocProps;
use super::preview_card::ComponentPreviewCard;
use crate::{
    Anchor, AnchorLink, Aside, Body1, Content, ContentWithAside, Flex, FlexAlign, FlexGap,
    SectionTitle, Tab, TabList, TextTag, Title1,
};
use leptos::prelude::*;
use leptos::tachys::view::any_view::IntoAny;
use orbital_base_components::ComponentPropDoc;
use orbital_base_components::PreviewRenderMode;
use turf::inline_style_sheet_values;

#[component]
pub fn OrbitalComponentView<IV>(
    component_name: &'static str,
    #[prop(optional)] component_description: Option<&'static str>,
    #[prop(optional)] component_description_md: Option<&'static str>,
    #[prop(optional)] component_props: Option<&'static [ComponentPropDoc]>,
    #[prop(optional)] component_best_practices: Option<&'static str>,
    #[prop(optional)] component_best_practices_md: Option<&'static str>,
    default: IV,
    #[prop(optional, into)] default_code: MaybeProp<&'static str>,
    #[prop(optional, into)] default_example_title: MaybeProp<&'static str>,
    #[prop(optional, into)] default_description: MaybeProp<&'static str>,
    /// DOM id for the default example anchor target (e.g. `example-default-stub`).
    #[prop(optional, into)]
    default_example_id: MaybeProp<&'static str>,
    /// `(title, slug)` pairs for the in-page example aside rail.
    #[prop(optional)]
    example_anchors: Option<&'static [(&'static str, &'static str)]>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView
where
    IV: IntoView + 'static,
{
    if use_context::<PreviewRenderMode>() == Some(PreviewRenderMode::BareDefault) {
        return view! {
            <div data-testid="debug-bare-preview">
                {default}
            </div>
        }
        .into_any();
    }

    let show_aside = example_anchors.is_some_and(|anchors| !anchors.is_empty());

    view! {
        <PreviewPageShell
            component_name=component_name
            component_description=component_description
            component_description_md=component_description_md
            component_props=component_props
            component_best_practices=component_best_practices
            component_best_practices_md=component_best_practices_md
            default_code=default_code
            default_example_title=default_example_title
            default_description=default_description
            default_example_id=default_example_id
            example_anchors=example_anchors
            show_aside=show_aside
            default=default
            children=children
        />
    }
    .into_any()
}

#[component]
fn PreviewPageShell<IV>(
    component_name: &'static str,
    #[prop(default = None)] component_description: Option<&'static str>,
    #[prop(default = None)] component_description_md: Option<&'static str>,
    #[prop(default = None)] component_props: Option<&'static [ComponentPropDoc]>,
    #[prop(default = None)] component_best_practices: Option<&'static str>,
    #[prop(default = None)] component_best_practices_md: Option<&'static str>,
    default: IV,
    #[prop(optional, into)] default_code: MaybeProp<&'static str>,
    #[prop(optional, into)] default_example_title: MaybeProp<&'static str>,
    #[prop(optional, into)] default_description: MaybeProp<&'static str>,
    #[prop(optional, into)] default_example_id: MaybeProp<&'static str>,
    #[prop(default = None)] example_anchors: Option<&'static [(&'static str, &'static str)]>,
    show_aside: bool,
    #[prop(default = None)] children: Option<Children>,
) -> impl IntoView
where
    IV: IntoView + 'static,
{
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .InfoSection {
            border-bottom: 1px solid var(--orb-color-border-subtle);
            padding-bottom: var(--orb-space-block-xl);
        }

        .Examples {
            padding-top: var(--orb-space-block-xl);
        }
    };

    let page_body = view! {
        <PreviewDocAndExamples
            component_name=component_name
            component_description=component_description
            component_description_md=component_description_md
            component_props=component_props
            component_best_practices=component_best_practices
            component_best_practices_md=component_best_practices_md
            default_code=default_code
            default_example_title=default_example_title
            default_description=default_description
            default_example_id=default_example_id
            info_section_class=class_names.info_section
            examples_class=class_names.examples
            default=default
            children=children
        />
    };

    view! {
        <style>{style_sheet}</style>
        {if show_aside {
            view! {
                <ContentWithAside>
                    <Content slot>
                        {page_body}
                    </Content>
                    <Aside slot>
                        <PreviewExampleAside anchors=example_anchors.unwrap_or(&[]) />
                    </Aside>
                </ContentWithAside>
            }.into_any()
        } else {
            page_body.into_any()
        }}
    }
}

#[component]
fn PreviewDocAndExamples<IV>(
    component_name: &'static str,
    #[prop(default = None)] component_description: Option<&'static str>,
    #[prop(default = None)] component_description_md: Option<&'static str>,
    #[prop(default = None)] component_props: Option<&'static [ComponentPropDoc]>,
    #[prop(default = None)] component_best_practices: Option<&'static str>,
    #[prop(default = None)] component_best_practices_md: Option<&'static str>,
    default: IV,
    #[prop(optional, into)] default_code: MaybeProp<&'static str>,
    #[prop(optional, into)] default_example_title: MaybeProp<&'static str>,
    #[prop(optional, into)] default_description: MaybeProp<&'static str>,
    #[prop(optional, into)] default_example_id: MaybeProp<&'static str>,
    info_section_class: &'static str,
    examples_class: &'static str,
    #[prop(default = None)] children: Option<Children>,
) -> impl IntoView
where
    IV: IntoView + 'static,
{
    let active_tab = RwSignal::new("description".to_string());

    view! {
        <Flex vertical=true align=FlexAlign::Stretch gap=FlexGap::Size(32) full_width=true>
            <div data-testid="preview-doc-panel">
                <Flex vertical=true align=FlexAlign::Stretch gap=FlexGap::Medium full_width=true>
                    <Title1 tag=TextTag::H1 block=true test_id="preview-page-title">
                        {component_name}
                    </Title1>

                    <Flex
                        vertical=true
                        align=FlexAlign::Stretch
                        gap=FlexGap::Medium
                        full_width=true
                        class=info_section_class
                    >
                        <TabList selected_value=active_tab>
                            <Tab value="description">"Description"</Tab>
                            <Tab value="best_practices">"Best Practices"</Tab>
                            <Tab value="properties">"Properties"</Tab>
                        </TabList>

                        <div data-testid="preview-doc-content">
                            {move || match active_tab.get().as_str() {
                                "description" => {
                                    if let Some(md) = component_description_md.filter(|s| !s.is_empty()) {
                                        view! { <ComponentDocMarkdown source=md /> }.into_any()
                                    } else {
                                        view! {
                                            <Body1 block=true>
                                                {component_description.unwrap_or("No description documented for this component.")}
                                            </Body1>
                                        }.into_any()
                                    }
                                }
                                "best_practices" => {
                                    if let Some(md) = component_best_practices_md.filter(|s| !s.is_empty()) {
                                        view! { <ComponentDocMarkdown source=md /> }.into_any()
                                    } else {
                                        view! {
                                            <Body1 block=true>
                                                {component_best_practices.unwrap_or("No best practices documented for this component.")}
                                            </Body1>
                                        }.into_any()
                                    }
                                }
                                "properties" => {
                                    view! {
                                        <ComponentDocProps props=component_props.unwrap_or(&[]) />
                                    }.into_any()
                                }
                                _ => view! {
                                    <Body1 block=true>
                                        {component_description.unwrap_or("No description documented for this component.")}
                                    </Body1>
                                }.into_any(),
                            }}
                        </div>
                    </Flex>
                </Flex>
            </div>

            <div data-testid="preview-examples">
                <Flex
                    vertical=true
                    align=FlexAlign::Stretch
                    gap=FlexGap::Size(40)
                    full_width=true
                    class=examples_class
                >
                    <ComponentPreviewCard
                        title=default_example_title
                        description=default_description
                        code=default_code
                        example_id=default_example_id
                    >
                        {default}
                    </ComponentPreviewCard>
                    {children
                        .map(|children| children())
                        .unwrap_or_else(|| ().into_any())}
                </Flex>
            </div>
        </Flex>
    }
}

#[component]
fn PreviewExampleAside(anchors: &'static [(&'static str, &'static str)]) -> impl IntoView {
    view! {
        <Flex vertical=true align=FlexAlign::Stretch gap=FlexGap::Small full_width=true>
            <SectionTitle>"On this page"</SectionTitle>
            <div data-testid="preview-example-nav">
                <Anchor>
                    {anchors
                        .iter()
                        .map(|(title, slug)| {
                            let href = format!("#example-{slug}");
                            view! {
                                <AnchorLink title=title.to_string() href=href />
                            }
                        })
                        .collect_view()}
                </Anchor>
            </div>
        </Flex>
    }
}
