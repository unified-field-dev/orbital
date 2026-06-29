//! Properties tab for motion preview pages.

use leptos::prelude::*;
use turf::inline_style_sheet_values;

use super::component_doc_markdown::ComponentDocMarkdown;
use super::types::ComponentPropDoc;

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
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Props {
            display: flex;
            flex-direction: column;
            gap: var(--orb-space-block-lg);
            width: 100%;
        }
        .PropRow {
            display: flex;
            flex-direction: column;
            gap: var(--orb-space-block-sm);
            width: 100%;
        }
        .PropHeader {
            display: flex;
            align-items: center;
            flex-wrap: wrap;
            gap: var(--orb-space-inline-sm);
        }
        .PropName {
            font-size: var(--orb-type-size-md);
            font-weight: var(--orb-type-weight-semibold);
            color: var(--orb-color-text-primary);
        }
        .Body {
            font-size: var(--orb-type-size-sm);
            line-height: var(--orb-type-line-md);
            color: var(--orb-color-text-primary);
        }
    };

    if props.is_empty() {
        return view! {
            <style>{style_sheet}</style>
            <p class=class_names.body>"No properties defined for this component."</p>
        }
        .into_any();
    }

    view! {
        <style>{style_sheet}</style>
        <div class=class_names.props>
            {props.iter().map(|prop| {
                let test_id = format!("preview-doc-prop-{}", prop.name);
                let type_name = prop.type_name.to_string();
                let description = prop.description;
                view! {
                    <div class=class_names.prop_row data-testid=test_id>
                        <div class=class_names.prop_header>
                            <span class=class_names.prop_name>{prop.name}</span>
                            <ComponentDocPropType type_name=type_name />
                        </div>
                        {(!description.is_empty()).then(|| view! {
                            <ComponentDocMarkdown source=description />
                        })}
                    </div>
                }
            }).collect_view()}
        </div>
    }
    .into_any()
}
