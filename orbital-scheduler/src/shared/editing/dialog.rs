//! Event create/edit dialog (SC-09, SC-21).

use chrono::NaiveDate;
use leptos::prelude::*;
use orbital_core_components::{
    Button, ButtonAppearance, Dialog, DialogActions, DialogBody, DialogContent, DialogSurface,
    DialogTitle, Field, Input, Select,
};
use orbital_date_pickers::{DateTimePicker, DatetimeLocale};

use super::types::{EventDialogMode, EventDialogRequest};
use crate::shared::interaction::{
    commit_events_with_lazy, next_event_id, SchedulerInteractionContext,
};
use crate::{default_creation_range, master_event_id, occurrence_start_from_instance};
use crate::{flatten_resource_rows, PlannedEvent};

/// Shared event dialog mounted by scheduler products.
#[component]
pub fn SchedulerEventDialog(
    dialog_open: RwSignal<bool>,
    dialog_request: RwSignal<Option<EventDialogRequest>>,
) -> impl IntoView {
    let ctx = expect_context::<SchedulerInteractionContext>();

    let title = RwSignal::new(String::new());
    let start = RwSignal::new(None::<orbital_base_components::OrbitalDateTime>);
    let end = RwSignal::new(None::<orbital_base_components::OrbitalDateTime>);
    let resource_value = RwSignal::new(String::new());
    let editing_id = RwSignal::new(None::<String>);

    let init_from_request = move |req: EventDialogRequest| match req.mode {
        EventDialogMode::Create {
            default_start,
            default_end,
            resource_id,
            ..
        } => {
            editing_id.set(None);
            title.set(String::new());
            start.set(Some(default_start));
            end.set(Some(default_end));
            resource_value.set(resource_id.unwrap_or_default());
        }
        EventDialogMode::Edit { event_id } => {
            let events = ctx.events.get_untracked();
            let master_id = master_event_id(&event_id);
            if let Some(event) = events.iter().find(|e| e.id == master_id) {
                editing_id.set(Some(master_id.to_string()));
                title.set(event.title.clone());
                let (start_val, end_val) = if event_id.contains("::") {
                    if let Some(inst_start) =
                        occurrence_start_from_instance(&event_id, event.start.timezone())
                    {
                        let delta = event.end.instant() - event.start.instant();
                        let inst_end = orbital_base_components::OrbitalDateTime::from_instant(
                            inst_start.instant() + delta,
                            event.end.timezone(),
                        );
                        (inst_start, inst_end)
                    } else {
                        (event.start, event.end)
                    }
                } else {
                    (event.start, event.end)
                };
                start.set(Some(start_val));
                end.set(Some(end_val));
                resource_value.set(event.resource_id.clone().unwrap_or_default());
            }
        }
    };

    Effect::new(move |_| {
        if let Some(req) = dialog_request.get() {
            init_from_request(req);
        }
    });

    let is_valid = move || match (start.get(), end.get()) {
        (Some(s), Some(e)) => e.instant() > s.instant() && !title.get().trim().is_empty(),
        _ => false,
    };

    let on_save = {
        let ctx = ctx.clone();
        Callback::new(move |_ev: leptos::ev::MouseEvent| {
            let Some(start_val) = start.get_untracked() else {
                return;
            };
            let Some(end_val) = end.get_untracked() else {
                return;
            };
            let title_val = title.get_untracked().trim().to_string();
            let resource_id = {
                let v = resource_value.get_untracked();
                if v.is_empty() {
                    None
                } else {
                    Some(v)
                }
            };

            let mut list = ctx.events.get_untracked();
            if let Some(id) = editing_id.get_untracked() {
                if let Some(event) = list.iter_mut().find(|e| e.id == id) {
                    event.title = title_val;
                    event.start = start_val;
                    event.end = end_val;
                    event.resource_id = resource_id;
                }
            } else {
                let id = next_event_id(&list);
                list.push(PlannedEvent::new(id, title_val, start_val, end_val));
                if let Some(last) = list.last_mut() {
                    last.resource_id = resource_id;
                }
            }
            commit_events_with_lazy(
                ctx.events,
                ctx.scheduler_events,
                ctx.lazy_context.as_ref(),
                list,
            );
            dialog_open.set(false);
            dialog_request.set(None);
        })
    };

    let on_cancel = Callback::new(move |_ev: leptos::ev::MouseEvent| {
        dialog_open.set(false);
        dialog_request.set(None);
    });

    let display_tz_signal = Signal::derive(move || ctx.display_timezone.get());

    view! {
        <DatetimeLocale default_timezone=display_tz_signal>
            <Dialog open=dialog_open>
                <DialogSurface>
                    <DialogBody>
                        <DialogTitle>
                            {move || {
                                if editing_id.get().is_some() {
                                    "Edit event"
                                } else {
                                    "New event"
                                }
                            }}
                        </DialogTitle>
                        <DialogContent>
                            <div data-testid="scheduler-event-dialog">
                                <Field label="Title">
                                    <div data-testid="scheduler-event-dialog-title">
                                        <Input bind=title />
                                    </div>
                                </Field>
                                <Field label="Start">
                                    <div data-testid="scheduler-event-dialog-start">
                                        <DateTimePicker bind=start />
                                    </div>
                                </Field>
                                <Field label="End">
                                    <div data-testid="scheduler-event-dialog-end">
                                        <DateTimePicker bind=end />
                                    </div>
                                </Field>
                                {move || {
                                    let resources = ctx.resources.get();
                                    if resources.is_empty() {
                                        return ().into_any();
                                    }
                                    let rows = flatten_resource_rows(&resources);
                                    view! {
                                        <Field label="Resource">
                                            <div data-testid="scheduler-event-dialog-resource">
                                                <Select bind=resource_value>
                                                    <option value="">"None"</option>
                                                    {rows
                                                        .into_iter()
                                                        .map(|row| {
                                                            view! {
                                                                <option value=row.id.clone()>
                                                                    {row.title}
                                                                </option>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </Select>
                                            </div>
                                        </Field>
                                    }
                                    .into_any()
                                }}
                            </div>
                        </DialogContent>
                        <DialogActions>
                            <Button
                                appearance=ButtonAppearance::Secondary
                                on_click=on_cancel
                            >
                                "Cancel"
                            </Button>
                            <div data-testid="scheduler-event-dialog-save">
                                <Button
                                    appearance=ButtonAppearance::Primary
                                    disabled=Signal::derive(move || !is_valid())
                                    on_click=on_save
                                >
                                    "Save"
                                </Button>
                            </div>
                        </DialogActions>
                    </DialogBody>
                </DialogSurface>
            </Dialog>
        </DatetimeLocale>
    }
}

/// Open the dialog for click-to-create on an empty timed slot.
pub fn open_create_from_slot(
    ctx: &SchedulerInteractionContext,
    day: NaiveDate,
    resource_id: Option<String>,
    click_minutes: f64,
) {
    let tz = ctx.display_timezone.get_untracked();
    if let Some((default_start, default_end)) = default_creation_range(click_minutes, day, tz) {
        ctx.open_dialog.run(EventDialogRequest {
            mode: EventDialogMode::Create {
                day,
                resource_id,
                default_start,
                default_end,
            },
        });
    }
}

/// Open the dialog for create with defaults from visible anchor day at 9am.
pub fn open_create_default(
    ctx: &SchedulerInteractionContext,
    anchor: orbital_base_components::OrbitalDateTime,
) {
    let day = anchor
        .wall_date()
        .unwrap_or_else(|| chrono::Utc::now().date_naive());
    open_create_from_slot(ctx, day, None, 9.0 * 60.0);
}
