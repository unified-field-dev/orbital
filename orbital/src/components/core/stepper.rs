//! Stepper component for multi-step workflows.
//!
//! A generic stepper for deployment pipelines, setup wizards, onboarding
//! flows, and similar multi-step processes. Supports vertical (default)
//! and horizontal orientations with status indicators and connector lines.

use leptos::prelude::*;
use leptos::tachys::view::any_view::{AnyView, IntoAny};
use orbital_macros::component_doc;
use turf::inline_style_sheet_values;

use super::{Body1, Caption1};
use crate::primitives::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StepStatus {
    #[default]
    Pending,
    Active,
    Done,
    Failed,
    /// Step omitted from the flow (e.g. optional phase not enabled).
    Skipped,
}

#[slot]
pub struct Step {
    #[prop(into)]
    pub label: String,
    pub status: StepStatus,
    #[prop(optional, into)]
    pub message: Option<String>,
}

/// A status-driven timeline for wizards, deployments, and onboarding flows.
///
/// Renders steps with icons, connector lines, and an optional progress bar. Declare steps as `<Step slot:steps label=… status=… />` children — each step carries its own [`StepStatus`] rather than a single active index.
///
/// # When to use
///
/// - Setup wizards, deployment pipelines, and onboarding flows
/// - Vertical timelines (default) or horizontal wizard headers via `vertical=false`
/// - Optional `progress` bar (0–100) for long-running operations
///
/// # Don't
///
/// - **Not for carousel slide dots** — use `CarouselStepper` inside a `Carousel` instead
/// - Display-only today — parent drives `status` updates; steps are not clickable navigation
///
/// # Examples
///
/// ## Default
/// Vertical stepper with mixed step statuses and an active-step message. Use for deployment pipelines, setup wizards, and any multi-step workflow timeline.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="stepper-preview">
///     <Stepper>
///         <Step slot:steps label="Build Docker image" status=StepStatus::Done />
///         <Step slot:steps label="Export image" status=StepStatus::Active message="Compressing layers..." />
///         <Step slot:steps label="Transfer to server" status=StepStatus::Pending />
///         <Step slot:steps label="Start container" status=StepStatus::Pending />
///     </Stepper>
///     </div>
/// }
/// ```
///
/// ## All Done
/// Every step marked complete with green check icons and solid connectors. Use when a process has finished successfully and you want a clear completion state.
/// <!-- preview -->
/// ```rust
/// view! {
///     <Stepper>
///         <Step slot:steps label="Download" status=StepStatus::Done />
///         <Step slot:steps label="Install" status=StepStatus::Done />
///         <Step slot:steps label="Configure" status=StepStatus::Done />
///     </Stepper>
/// }
/// ```
///
/// ## With Failure
/// Failed step with a danger icon, error message, and dashed connectors to pending steps. Use when a step fails and you need to surface the failure reason inline.
/// <!-- preview -->
/// ```rust
/// view! {
///     <Stepper>
///         <Step slot:steps label="Validate input" status=StepStatus::Done />
///         <Step slot:steps label="Connect to server" status=StepStatus::Failed message="Connection refused" />
///         <Step slot:steps label="Upload data" status=StepStatus::Pending />
///     </Stepper>
/// }
/// ```
///
/// ## Horizontal Wizard
/// Horizontal layout with step labels centered under icons, suited to wizard headers. Use at the top of multi-page forms where steps read left to right.
/// <!-- preview -->
/// ```rust
/// view! {
///     <Stepper vertical=false>
///         <Step slot:steps label="Account" status=StepStatus::Done />
///         <Step slot:steps label="Profile" status=StepStatus::Active />
///         <Step slot:steps label="Preferences" status=StepStatus::Pending />
///         <Step slot:steps label="Review" status=StepStatus::Pending />
///     </Stepper>
/// }
/// ```
///
/// ## With Progress Bar
/// Overall progress bar below the steps, driven by a reactive 0–100 signal. Use for long-running operations where percent complete matters alongside step status.
/// <!-- preview -->
/// ```rust
/// view! {
///     <Stepper progress=Signal::derive(|| 60u32)>
///         <Step slot:steps label="Downloading" status=StepStatus::Done />
///         <Step slot:steps label="Installing" status=StepStatus::Active message="Package 3/5" />
///         <Step slot:steps label="Configuring" status=StepStatus::Pending />
///     </Stepper>
/// }
/// ```
///
/// ## Deployment Pipeline
/// Realistic seven-step deployment with progress bar and an active upload message. Use as a reference for CI/CD or release pipelines with many sequential phases.
/// <!-- preview -->
/// ```rust
/// view! {
///     <Stepper progress=Signal::derive(|| 40u32)>
///         <Step slot:steps label="Build Docker image" status=StepStatus::Done />
///         <Step slot:steps label="Export image" status=StepStatus::Done />
///         <Step slot:steps label="Transfer to server" status=StepStatus::Active message="Uploading 1.2 GB..." />
///         <Step slot:steps label="Install container runtime" status=StepStatus::Pending />
///         <Step slot:steps label="Load image" status=StepStatus::Pending />
///         <Step slot:steps label="Start container" status=StepStatus::Pending />
///         <Step slot:steps label="Health check" status=StepStatus::Pending />
///     </Stepper>
/// }
/// ```
#[component_doc(
    category = "Feedback",
    preview_slug = "stepper",
    preview_label = "Stepper",
    preview_icon = icondata::AiOrderedListOutlined,
)]
#[component]
pub fn Stepper(
    /// Step children declared via `<Step slot:steps ... />`.
    #[prop(optional, default = Vec::new())]
    steps: Vec<Step>,
    /// When true (default), steps stack vertically. When false, steps flow horizontally.
    #[prop(optional, default = true)]
    vertical: bool,
    /// Optional overall progress (0-100). When provided, renders a ProgressBar below the steps.
    #[prop(optional, into)]
    progress: Option<Signal<u32>>,
) -> impl IntoView {
    let total = steps.len();

    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Wrapper {
            display: flex;
            flex-direction: column;
            gap: 8px;
        }

        // ── Vertical ────────────────────────────────────────────────

        .Vertical {
            display: flex;
            flex-direction: column;
        }

        .StepRowV {
            display: flex;
            flex-direction: row;
            align-items: flex-start;
            gap: 12px;
        }

        .IconColumnV {
            display: flex;
            flex-direction: column;
            align-items: center;
            flex-shrink: 0;
            width: 24px;
        }

        .ConnectorV {
            width: 2px;
            flex: 1;
            min-height: 16px;
            margin-top: 4px;
            margin-bottom: 4px;
        }

        .ConnectorSolid {
            border-left: 2px solid var(--orb-color-status-success-fg);
        }

        .ConnectorActive {
            border-left: 2px solid var(--orb-color-brand-fg);
        }

        .ConnectorDashed {
            border-left: 2px dashed var(--orb-color-border-default);
        }

        .ContentV {
            display: flex;
            flex-direction: column;
            gap: 2px;
            padding-bottom: 4px;
        }

        // ── Horizontal ──────────────────────────────────────────────

        .Horizontal {
            display: flex;
            flex-direction: row;
            align-items: flex-start;
        }

        .StepItemH {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 8px;
            flex-shrink: 0;
        }

        .IconRowH {
            display: flex;
            flex-direction: row;
            align-items: center;
            width: 100%;
        }

        .ConnectorH {
            height: 2px;
            flex: 1;
        }

        .ConnectorHSolid {
            border-top: 2px solid var(--orb-color-status-success-fg);
        }

        .ConnectorHActive {
            border-top: 2px solid var(--orb-color-brand-fg);
        }

        .ConnectorHDashed {
            border-top: 2px dashed var(--orb-color-border-default);
        }

        .ContentH {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 2px;
            text-align: center;
            max-width: 120px;
        }

        // ── Shared ──────────────────────────────────────────────────

        .IconWrap {
            display: flex;
            align-items: center;
            justify-content: center;
            width: 24px;
            height: 24px;
            flex-shrink: 0;
        }

        .StatusDone {
            color: var(--orb-color-status-success-fg);
        }

        .StatusActive {
            color: var(--orb-color-brand-fg);
        }

        .StatusFailed {
            color: var(--orb-color-status-danger-fg);
        }

        .StatusPending {
            color: var(--orb-color-text-tertiary);
        }

        .StatusSkipped {
            color: var(--orb-color-text-tertiary);
        }

        .Message {
            color: var(--orb-color-text-tertiary);
        }
    };

    let icon_class = |status: StepStatus| -> String {
        let base = &class_names.icon_wrap;
        let modifier = match status {
            StepStatus::Done => &class_names.status_done,
            StepStatus::Active => &class_names.status_active,
            StepStatus::Failed => &class_names.status_failed,
            StepStatus::Pending => &class_names.status_pending,
            StepStatus::Skipped => &class_names.status_skipped,
        };
        format!("{base} {modifier}")
    };

    let connector_v_class = |status: StepStatus| -> String {
        let base = &class_names.connector_v;
        let modifier = match status {
            StepStatus::Done => &class_names.connector_solid,
            StepStatus::Active => &class_names.connector_active,
            StepStatus::Pending | StepStatus::Failed | StepStatus::Skipped => {
                &class_names.connector_dashed
            }
        };
        format!("{base} {modifier}")
    };

    let connector_h_class = |status: StepStatus| -> String {
        let base = &class_names.connector_h;
        let modifier = match status {
            StepStatus::Done => &class_names.connector_h_solid,
            StepStatus::Active => &class_names.connector_h_active,
            StepStatus::Pending | StepStatus::Failed | StepStatus::Skipped => {
                &class_names.connector_h_dashed
            }
        };
        format!("{base} {modifier}")
    };

    let step_icon = |status: StepStatus| -> AnyView {
        match status {
            StepStatus::Done => view! {
                <Icon icon=icondata::AiCheckCircleOutlined width="20px" height="20px" />
            }
            .into_any(),
            StepStatus::Active => view! {
                <Spinner size=SpinnerSize::ExtraTiny />
            }
            .into_any(),
            StepStatus::Failed => view! {
                <Icon icon=icondata::AiCloseCircleOutlined width="20px" height="20px" />
            }
            .into_any(),
            StepStatus::Pending => view! {
                <Icon icon=icondata::AiClockCircleOutlined width="20px" height="20px" />
            }
            .into_any(),
            StepStatus::Skipped => view! {
                <Icon icon=icondata::BiSkipNextRegular width="20px" height="20px" />
            }
            .into_any(),
        }
    };

    let steps_view = if vertical {
        view! {
            <div class=class_names.vertical>
                {steps.into_iter().enumerate().map(|(i, step)| {
                    let is_last = i == total - 1;
                    let ic = icon_class(step.status);
                    let cc = connector_v_class(step.status);
                    view! {
                        <div class=class_names.step_row_v>
                            <div class=class_names.icon_column_v>
                                <div class=ic>{step_icon(step.status)}</div>
                                {if !is_last {
                                    view! { <div class=cc></div> }.into_any()
                                } else {
                                    ().into_any()
                                }}
                            </div>
                            <div class=class_names.content_v>
                                <Body1>{step.label}</Body1>
                                {step.message.map(|msg| view! {
                                    <Caption1 class=class_names.message>{msg}</Caption1>
                                })}
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        }
        .into_any()
    } else {
        view! {
            <div class=class_names.horizontal>
                {steps.into_iter().enumerate().map(|(i, step)| {
                    let is_last = i == total - 1;
                    let ic = icon_class(step.status);
                    let cc = connector_h_class(step.status);
                    view! {
                        <div class=class_names.step_item_h style="flex: 1;">
                            <div class=class_names.icon_row_h>
                                <div class=ic>{step_icon(step.status)}</div>
                                {if !is_last {
                                    view! { <div class=cc></div> }.into_any()
                                } else {
                                    ().into_any()
                                }}
                            </div>
                            <div class=class_names.content_h>
                                <Body1>{step.label}</Body1>
                                {step.message.map(|msg| view! {
                                    <Caption1 class=class_names.message>{msg}</Caption1>
                                })}
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        }
        .into_any()
    };

    view! {
        <style>{style_sheet}</style>
        <div class=class_names.wrapper>
            {steps_view}
            {progress.map(|p| view! {
                <ProgressBar value=Signal::derive(move || p.get() as f64 / 100.0) />
            })}
        </div>
    }
}
