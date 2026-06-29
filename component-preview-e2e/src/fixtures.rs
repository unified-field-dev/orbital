//! Fixture components for macro expansion tests.

use leptos::prelude::*;
use orbital::components::{ComponentPreviewCard, OrbitalComponentView};
use orbital::preview::PreviewRegistration;
use orbital_macros::component_doc;

/// Controlled doc-panel fixture for macro and browser E2E tests.
///
/// Exercises summary, when-to-use, usage (with rustdoc fence), best practices, and preview examples.
///
/// # When to use
///
/// - Validating `#[component_doc]` description composition in tests
///
/// # Usage
///
/// 1. Register this fixture in the preview catalog.
/// 2. Assert Description tab excludes fenced code.
///
/// Docs-only snippet (not live preview):
///
/// ```rust,ignore
/// view! { <FixtureDocPanel label="hidden" /> }
/// ```
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep fixture doc strings stable for regression tests.
///
/// ## Don'ts
///
/// * Do not rely on fenced code appearing in the Description tab.
///
/// # Examples
///
/// ## Default example
/// <!-- default -->
/// <!-- preview -->
/// ```rust,ignore
/// view! {
///     <div data-testid="fixture-doc-panel-preview">
///         <FixtureDocPanel label="Fixture doc panel" />
///     </div>
/// }
/// ```
///
/// ## Secondary example
/// <!-- preview -->
/// ```rust,ignore
/// view! {
///     <div data-testid="fixture-doc-panel-secondary">
///         <FixtureDocPanel label="Secondary" />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Integrations",
    preview = "manual",
    preview_slug = "fixture-doc-panel",
    preview_label = "Fixture Doc Panel"
)]
#[component]
pub fn FixtureDocPanel(
    /// Panel label text.
    label: &'static str,
) -> impl IntoView {
    view! { <span data-testid="fixture-doc-panel">{label}</span> }
}

#[component]
pub fn FixtureDocPanelPreview() -> impl IntoView {
    view! {
        <OrbitalComponentView
            component_name="Fixture Doc Panel"
            component_description_md=FIXTUREDOCPANEL_DESCRIPTION
            component_best_practices_md=FIXTUREDOCPANEL_BEST_PRACTICES
            component_props=FIXTUREDOCPANEL_PROPS
            default_code=FIXTUREDOCPANEL_PREVIEW_EXAMPLE_DEFAULT_EXAMPLE_CODE
            default_example_title=FIXTUREDOCPANEL_PREVIEW_DEFAULT_TITLE
            default_example_id="example-default-example"
            example_anchors=&[
                ("Default example", "default-example"),
                ("Secondary example", "secondary-example"),
            ]
            default={
                view! {
                    <div data-testid="fixture-doc-panel-preview">
                        <FixtureDocPanel label="Fixture doc panel" />
                    </div>
                }
            }
        >
            <ComponentPreviewCard
                title="Secondary example"
                code=FIXTUREDOCPANEL_PREVIEW_EXAMPLE_SECONDARY_EXAMPLE_CODE
                example_id="example-secondary-example"
            >
                {
                    view! {
                        <div data-testid="fixture-doc-panel-secondary">
                            <FixtureDocPanel label="Secondary" />
                        </div>
                    }
                }
            </ComponentPreviewCard>
        </OrbitalComponentView>
    }
}

/// Minimal fixture component for expansion tests.
#[component_doc(
    category = "Integrations",
    preview = "manual",
    preview_slug = "fixture-button",
    preview_label = "Fixture Button"
)]
#[component]
pub fn FixtureButton(
    /// Button label text.
    label: &'static str,
) -> impl IntoView {
    view! { <button data-testid="fixture-button">{label}</button> }
}

#[component]
pub fn FixtureButtonPreview() -> impl IntoView {
    view! {
        <div data-testid="fixture-button-preview">
            <FixtureButton label="Fixture" />
        </div>
    }
}

pub static FIXTURE_DOCPANEL_PREVIEW_REGISTRATION: PreviewRegistration = PreviewRegistration {
    slug: "fixture-doc-panel",
    label: "Fixture Doc Panel",
    section: "Core Components",
    section_priority: 2,
    category: "Integrations",
    category_priority: 100,
    category_default_collapsed: false,
    group: "",
    group_priority: 0,
    nav_item: false,
    icon: icondata::AiExperimentOutlined,
    render: || FixtureDocPanelPreview().into_any(),
};

pub static FIXTURE_BUTTON_PREVIEW_REGISTRATION: PreviewRegistration = PreviewRegistration {
    slug: "fixture-button",
    label: "Fixture Button",
    section: "Core Components",
    section_priority: 2,
    category: "Integrations",
    category_priority: 100,
    category_default_collapsed: false,
    group: "",
    group_priority: 0,
    nav_item: false,
    icon: icondata::AiExperimentOutlined,
    render: || FixtureButtonPreview().into_any(),
};

/// Split-button placeholder preview (alias for [`ActionMenuButton`](orbital_core_components::ActionMenuButton)).
#[component]
pub fn SplitButtonPreview() -> impl IntoView {
    use orbital_core_components::{ActionMenuButton, ActionMenuItems, ButtonAppearance, MenuItem};

    view! {
        <OrbitalComponentView
            component_name="Split Button"
            default={
                view! {
                    <div data-testid="split-button-preview">
                        <ActionMenuButton appearance=ButtonAppearance::Primary>
                            "Save"
                            <ActionMenuItems slot>
                                <MenuItem value="save-as".to_string()>"Save as"</MenuItem>
                                <MenuItem value="export".to_string()>"Export"</MenuItem>
                            </ActionMenuItems>
                        </ActionMenuButton>
                    </div>
                }
            }
        />
    }
}

pub static SPLIT_BUTTON_PREVIEW_REGISTRATION: PreviewRegistration = PreviewRegistration {
    slug: "split-button",
    label: "Split Button",
    section: "Core Components",
    section_priority: 2,
    category: "Inputs",
    category_priority: 30,
    category_default_collapsed: false,
    group: "Buttons",
    group_priority: 10,
    nav_item: false,
    icon: icondata::AiColumnWidthOutlined,
    render: || SplitButtonPreview().into_any(),
};

pub fn manual_preview_registrations() -> &'static [&'static PreviewRegistration] {
    static REGS: &[&PreviewRegistration] = &[
        &FIXTURE_DOCPANEL_PREVIEW_REGISTRATION,
        &FIXTURE_BUTTON_PREVIEW_REGISTRATION,
        &SPLIT_BUTTON_PREVIEW_REGISTRATION,
    ];
    REGS
}
