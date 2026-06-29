use leptos::prelude::*;
use orbital_base_components::ComponentPropDoc;
use turf::inline_style_sheet_values;

use crate::{Body1, Flex, FlexAlign, FlexGap, FlexWrap, TextTag, Title3};

use super::component_doc_markdown::ComponentDocMarkdown;

#[component]
fn ComponentDocPropType(#[prop(into)] type_name: String) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .TypeChip {
            display: inline-flex;
            box-sizing: border-box;
            align-items: center;
            justify-content: center;
            min-width: max-content;
            height: 20px;
            padding: 0 calc(var(--orb-space-inline-xs) + var(--orb-space-inline-2xs));
            border-radius: var(--orb-radius-circular);
            border: var(--orb-stroke-thin) solid var(--orb-color-border-subtle);
            background-color: var(--orb-color-surface-overlay);
            color: var(--orb-color-text-tertiary);
            font-family: var(--orb-type-family-sans);
            font-size: var(--orb-type-size-xs);
            font-weight: var(--orb-type-weight-semibold);
            line-height: var(--orb-type-line-sm);
        }
    };

    view! {
        <style>{style_sheet}</style>
        <span class=class_names.type_chip>{type_name}</span>
    }
}

#[component]
pub fn ComponentDocProps(props: &'static [ComponentPropDoc]) -> impl IntoView {
    if props.is_empty() {
        return view! {
            <Body1 block=true>
                "No properties defined for this component."
            </Body1>
        }
        .into_any();
    }

    view! {
        <Flex vertical=true align=FlexAlign::Stretch gap=FlexGap::Size(24) full_width=true>
            {props
                .iter()
                .map(|prop| {
                    let test_id = format!("preview-doc-prop-{}", prop.name);
                    let type_name = prop.type_name.to_string();
                    let description = prop.description;
                    view! {
                        <div data-testid=test_id>
                        <Flex
                            vertical=true
                            align=FlexAlign::Stretch
                            gap=FlexGap::Small
                            full_width=true
                        >
                            <Flex
                                align=FlexAlign::Center
                                gap=FlexGap::Small
                                wrap=FlexWrap::Wrap
                            >
                                <Title3 tag=TextTag::Span block=false>
                                    {prop.name}
                                </Title3>
                                <ComponentDocPropType type_name=type_name />
                            </Flex>
                            {(!description.is_empty()).then(|| view! {
                                <ComponentDocMarkdown source=description />
                            })}
                        </Flex>
                        </div>
                    }
                })
                .collect_view()}
        </Flex>
    }
    .into_any()
}
