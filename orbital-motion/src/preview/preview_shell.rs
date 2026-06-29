//! Preview shell for motion catalog pages (avoids cyclic dependency on orbital-core-components).

use leptos::prelude::*;
use leptos::tachys::view::any_view::IntoAny;
use orbital_style::inject_style;
use turf::inline_style_sheet_values;

use super::component_doc_markdown::ComponentDocMarkdown;
use super::component_doc_props::ComponentDocProps;
use super::components::preview_button_styles;
use super::tab::{PreviewTab, PreviewTabList};
pub use super::types::{ComponentPropDoc, PreviewRenderMode};

fn ensure_preview_shell_styles() {
    inject_style("orbital-motion-preview-button", preview_button_styles());
}

#[component]
fn OrbitalPreviewCardBody(
    #[prop(optional, into)] code: MaybeProp<&'static str>,
    children: Children,
) -> impl IntoView {
    let (show_code, set_show_code) = signal(false);
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Frame {
            width: 100%;
            border: 1px solid var(--orb-color-border-subtle);
            border-radius: var(--orb-radius-md);
            background: var(--orb-color-surface-canvas);
            box-shadow: var(--orb-elev-raised-xs);
        }
        .PreviewDemo {
            display: flex;
            justify-content: center;
            padding: var(--orb-space-block-lg) var(--orb-space-inline-lg);
        }
        .Toolbar {
            display: flex;
            justify-content: flex-end;
            padding: var(--orb-space-block-sm) var(--orb-space-inline-lg);
            border-top: 1px solid var(--orb-color-border-subtle);
        }
        .Code {
            padding: var(--orb-space-block-lg) var(--orb-space-inline-lg);
            background: var(--orb-color-surface-subtle);
            border-top: 1px solid var(--orb-color-border-subtle);
            font-family: var(--orb-type-family-mono);
            font-size: var(--orb-type-size-xs);
            white-space: pre-wrap;
            color: var(--orb-color-text-primary);
        }
    };
    let code_opt = move || code.get();

    view! {
        <style>{style_sheet}</style>
        <div class=class_names.frame>
            <div class=class_names.preview_demo>{children()}</div>
            <Show when=move || code_opt().is_some()>
                <div class=class_names.toolbar>
                    <super::components::PreviewButton
                        on_click=Callback::new(move |_| set_show_code.update(|v| *v = !*v))
                    >
                        {move || if show_code.get() { "Hide code" } else { "Show code" }}
                    </super::components::PreviewButton>
                </div>
                <Show when=move || show_code.get()>
                    {move || code_opt().map(|code_str| view! {
                        <pre class=class_names.code>{code_str}</pre>
                    })}
                </Show>
            </Show>
        </div>
    }
}

#[component]
pub fn ComponentPreviewCard(
    #[prop(optional, into)] title: MaybeProp<&'static str>,
    #[prop(optional, into)] description: MaybeProp<&'static str>,
    #[prop(optional, into)] code: MaybeProp<&'static str>,
    #[prop(optional, into)] example_id: MaybeProp<&'static str>,
    children: Children,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .ExampleTarget {
            scroll-margin-top: var(--orb-space-block-2xl);
        }
        .Stack {
            display: flex;
            flex-direction: column;
            gap: var(--orb-space-block-sm);
            width: 100%;
        }
        .Title {
            font-size: var(--orb-type-size-lg);
            font-weight: var(--orb-type-weight-semibold);
            line-height: var(--orb-type-line-xl);
            margin: 0;
            color: var(--orb-color-text-primary);
        }
        .Description {
            color: var(--orb-color-text-secondary);
            font-size: var(--orb-type-size-sm);
            line-height: var(--orb-type-line-md);
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div class=class_names.example_target id=move || example_id.get()>
            <div class=class_names.stack>
                <h3 class=class_names.title>{move || title.get().unwrap_or("Default")}</h3>
                {move || description.get().filter(|d| !d.is_empty()).map(|text| {
                    view! {
                        <div class=class_names.description>
                            <ComponentDocMarkdown source=text />
                        </div>
                    }
                })}
                <OrbitalPreviewCardBody code=code>{children()}</OrbitalPreviewCardBody>
            </div>
        </div>
    }
}

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
    #[prop(optional, into)] default_example_id: MaybeProp<&'static str>,
    #[prop(optional)] example_anchors: Option<&'static [(&'static str, &'static str)]>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView
where
    IV: IntoView + 'static,
{
    ensure_preview_shell_styles();

    if use_context::<PreviewRenderMode>() == Some(PreviewRenderMode::BareDefault) {
        return view! {
            <div data-testid="debug-bare-preview">{default}</div>
        }
        .into_any();
    }

    let active_tab = RwSignal::new("description".to_string());
    let show_aside = example_anchors.is_some_and(|anchors| !anchors.is_empty());

    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Page {
            display: flex;
            flex-direction: column;
            gap: var(--orb-space-block-2xl);
            width: 100%;
            font-family: var(--orb-type-family-sans);
            color: var(--orb-color-text-primary);
        }
        .InfoSection {
            border-bottom: 1px solid var(--orb-color-border-subtle);
            padding-bottom: var(--orb-space-block-xl);
        }
        .DocStack {
            display: flex;
            flex-direction: column;
            gap: var(--orb-space-block-md);
            width: 100%;
        }
        .Title {
            font-size: var(--orb-type-size-2xl);
            font-weight: var(--orb-type-weight-semibold);
            line-height: 1.286;
            margin: 0;
            display: block;
        }
        .Body {
            font-size: var(--orb-type-size-sm);
            line-height: var(--orb-type-line-md);
            margin: 0;
        }
        .Examples {
            padding-top: var(--orb-space-block-xl);
        }
        .ExamplesStack {
            display: flex;
            flex-direction: column;
            gap: 40px;
            width: 100%;
        }
        .AsideLayout {
            display: grid;
            grid-template-columns: 1fr 200px;
            gap: var(--orb-space-inline-xl);
            width: 100%;
        }
        .AsideTitle {
            font-size: var(--orb-type-size-md);
            font-weight: var(--orb-type-weight-semibold);
            color: var(--orb-color-text-primary);
            margin: 0 0 var(--orb-space-block-sm);
        }
        .AsideList {
            margin: 0;
            padding: 0;
            list-style: none;
            display: flex;
            flex-direction: column;
            gap: var(--orb-space-block-xs);
        }
        .AsideLink {
            color: var(--orb-color-brand-link);
            text-decoration: none;
            font-size: var(--orb-type-size-sm);
        }
        .AsideLink:hover {
            text-decoration: underline;
        }
    };

    let page = view! {
        <style>{style_sheet}</style>
        <div class=class_names.page>
            <div data-testid="preview-doc-panel">
                <div class=class_names.doc_stack>
                    <h1 class=class_names.title data-testid="preview-page-title">{component_name}</h1>
                    <div class=class_names.info_section>
                        <PreviewTabList selected_value=active_tab>
                            <PreviewTab value="description">"Description"</PreviewTab>
                            <PreviewTab value="best_practices">"Best Practices"</PreviewTab>
                            <PreviewTab value="properties">"Properties"</PreviewTab>
                        </PreviewTabList>
                        <div data-testid="preview-doc-content">
                            {move || match active_tab.get().as_str() {
                                "description" => {
                                    if let Some(md) = component_description_md.filter(|s| !s.is_empty()) {
                                        view! { <ComponentDocMarkdown source=md /> }.into_any()
                                    } else {
                                        view! {
                                            <p class=class_names.body>
                                                {component_description.unwrap_or("No description documented for this component.")}
                                            </p>
                                        }.into_any()
                                    }
                                }
                                "best_practices" => {
                                    if let Some(md) = component_best_practices_md.filter(|s| !s.is_empty()) {
                                        view! { <ComponentDocMarkdown source=md /> }.into_any()
                                    } else {
                                        view! {
                                            <p class=class_names.body>
                                                {component_best_practices.unwrap_or("No best practices documented for this component.")}
                                            </p>
                                        }.into_any()
                                    }
                                }
                                "properties" => view! {
                                    <ComponentDocProps props=component_props.unwrap_or(&[]) />
                                }.into_any(),
                                _ => view! {
                                    <p class=class_names.body>
                                        {component_description.unwrap_or("No description documented for this component.")}
                                    </p>
                                }.into_any(),
                            }}
                        </div>
                    </div>
                </div>
            </div>
            <div data-testid="preview-examples" class=class_names.examples>
                <div class=class_names.examples_stack>
                    <ComponentPreviewCard
                        title=default_example_title
                        description=default_description
                        code=default_code
                        example_id=default_example_id
                    >
                        {default}
                    </ComponentPreviewCard>
                    {children.map(|children| children()).unwrap_or_else(|| ().into_any())}
                </div>
            </div>
        </div>
    };

    view! {
        {if show_aside {
            view! {
                <div class=class_names.aside_layout>
                    {page}
                    <aside data-testid="preview-example-nav">
                        <p class=class_names.aside_title>"On this page"</p>
                        <ul class=class_names.aside_list>
                            {example_anchors.unwrap_or(&[]).iter().map(|(title, slug)| {
                                let href = format!("#example-{slug}");
                                let label = (*title).to_string();
                                view! { <li><a class=class_names.aside_link href=href>{label}</a></li> }
                            }).collect_view()}
                        </ul>
                    </aside>
                </div>
            }.into_any()
        } else {
            page.into_any()
        }}
    }
    .into_any()
}
