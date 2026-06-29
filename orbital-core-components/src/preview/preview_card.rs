//! Variant preview card for component gallery pages.

use super::component_doc_markdown::ComponentDocMarkdown;
use super::preview_card_body::OrbitalPreviewCardBody;
use crate::{Flex, FlexGap, Subtitle1};
use leptos::prelude::*;
use turf::inline_style_sheet_values;

#[component]
pub fn ComponentPreviewCard(
    #[prop(optional, into)] title: MaybeProp<&'static str>,
    #[prop(optional, into)] description: MaybeProp<&'static str>,
    #[prop(optional, into)] code: MaybeProp<&'static str>,
    /// DOM id for in-page anchor navigation (e.g. `example-default-stub`).
    #[prop(optional, into)]
    example_id: MaybeProp<&'static str>,
    children: Children,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .ExampleTarget {
            scroll-margin-top: var(--orb-space-block-2xl);
        }

        .Description {
            color: var(--orb-color-text-secondary);
            font-size: 14px;
            line-height: 1.43;

            & > div p {
                margin: 0;
                color: inherit;
            }
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div class=class_names.example_target id=move || example_id.get()>
            <Flex vertical=true gap=FlexGap::Small full_width=true>
                <Subtitle1>{move || title.get().unwrap_or("Default")}</Subtitle1>
                {move || description.get().filter(|d| !d.is_empty()).map(|text| {
                    view! {
                        <div class=class_names.description>
                            <ComponentDocMarkdown source=text />
                        </div>
                    }
                })}
                <OrbitalPreviewCardBody code=code>{children()}</OrbitalPreviewCardBody>
            </Flex>
        </div>
    }
}
