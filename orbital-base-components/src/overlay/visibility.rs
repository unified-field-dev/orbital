use leptos::{
    either::Either,
    ev::{self, on},
    html,
    leptos_dom::helpers::TimeoutHandle,
    prelude::*,
    tachys::html::node_ref::node_ref,
};
use std::time::Duration;

use super::dom_events::on_click_outside;
use super::element_ref::AnyElement;
use super::trigger::OverlayTriggerType;

pub const HOVER_HIDE_DELAY_MS: u64 = 100;

pub fn should_schedule_hover_hide(trigger_type: OverlayTriggerType) -> bool {
    trigger_type.uses_hover()
}

pub fn should_toggle_on_click(trigger_type: OverlayTriggerType) -> bool {
    trigger_type.uses_click()
}

#[derive(Clone, Default)]
pub struct OverlayLifecycle {
    pub on_open: Option<Callback<()>>,
    pub on_close: Option<Callback<()>>,
}

#[derive(Clone, Copy, Debug)]
pub struct OverlayHoverDelays {
    pub show_delay_ms: u64,
    pub hide_delay_ms: u64,
}

impl Default for OverlayHoverDelays {
    fn default() -> Self {
        Self {
            show_delay_ms: 0,
            hide_delay_ms: HOVER_HIDE_DELAY_MS,
        }
    }
}

#[derive(Clone, Copy)]
pub struct UseOverlayVisibility {
    pub is_show: RwSignal<bool>,
    trigger_ref: NodeRef<AnyElement>,
    panel_ref: NodeRef<html::Div>,
    show_timer: StoredValue<Option<TimeoutHandle>>,
    hide_timer: StoredValue<Option<TimeoutHandle>>,
    trigger_type: OverlayTriggerType,
    delays: OverlayHoverDelays,
}

impl UseOverlayVisibility {
    pub fn new(
        trigger_type: OverlayTriggerType,
        panel_ref: NodeRef<html::Div>,
        lifecycle: Option<OverlayLifecycle>,
        delays: OverlayHoverDelays,
    ) -> Self {
        let is_show = RwSignal::new(false);
        let trigger_ref = NodeRef::<AnyElement>::new();
        let show_timer = StoredValue::new(None::<TimeoutHandle>);
        let hide_timer = StoredValue::new(None::<TimeoutHandle>);

        if trigger_type.uses_click() {
            let is_show = is_show;
            on_click_outside(
                move || {
                    if !is_show.get_untracked() {
                        return None;
                    }
                    let mut els = Vec::new();
                    if let Some(trigger_el) = trigger_ref.try_get_untracked().flatten() {
                        els.push(trigger_el);
                    }
                    if let Some(panel_el) = panel_ref.try_get_untracked().flatten() {
                        els.push(panel_el.into());
                    }
                    Some(els)
                },
                move || is_show.set(false),
            );
        }

        if let Some(lifecycle) = lifecycle {
            Effect::watch(
                move || is_show.get(),
                move |shown, prev, _| {
                    if prev == Some(shown) {
                        return;
                    }
                    if *shown {
                        if let Some(on_open) = lifecycle.on_open {
                            on_open.run(());
                        }
                    } else if let Some(on_close) = lifecycle.on_close {
                        on_close.run(());
                    }
                },
                false,
            );
        }

        #[cfg(not(feature = "ssr"))]
        Owner::on_cleanup(move || {
            show_timer.update_value(|handle| {
                if let Some(handle) = handle.take() {
                    handle.clear();
                }
            });
            hide_timer.update_value(|handle| {
                if let Some(handle) = handle.take() {
                    handle.clear();
                }
            });
        });

        Self {
            is_show,
            trigger_ref,
            panel_ref,
            show_timer,
            hide_timer,
            trigger_type,
            delays,
        }
    }

    pub fn panel_ref(&self) -> NodeRef<html::Div> {
        self.panel_ref
    }

    pub fn on_mouse_enter(&self, _: ev::MouseEvent) {
        if !self.trigger_type.uses_hover() {
            return;
        }
        self.clear_hide_timer();
        if self.delays.show_delay_ms == 0 {
            self.is_show.set(true);
        } else {
            self.schedule_show();
        }
    }

    pub fn on_mouse_leave(&self, _: ev::MouseEvent) {
        if !should_schedule_hover_hide(self.trigger_type) {
            return;
        }
        self.clear_show_timer();
        self.schedule_hide();
    }

    pub fn attach_trigger<T>(&self, trigger: T, trigger_type: OverlayTriggerType) -> impl IntoView
    where
        T: AddAnyAttr + IntoView + Send + 'static,
    {
        let trigger = trigger.add_any_attr(node_ref(self.trigger_ref));
        let with_focus = trigger
            .add_any_attr(on(ev::focusin, {
                let vis = *self;
                move |e| vis.on_focus_in(e)
            }))
            .add_any_attr(on(ev::focusout, {
                let vis = *self;
                move |e| vis.on_focus_out(e)
            }));
        match trigger_type {
            OverlayTriggerType::Click => Either::Left(with_focus.add_any_attr(on(ev::click, {
                let is_show = self.is_show;
                move |_| is_show.update(|show| *show = !*show)
            }))),
            OverlayTriggerType::Hover => Either::Right(
                with_focus
                    .add_any_attr(on(ev::mouseenter, {
                        let vis = *self;
                        move |e| vis.on_mouse_enter(e)
                    }))
                    .add_any_attr(on(ev::mouseleave, {
                        let vis = *self;
                        move |e| vis.on_mouse_leave(e)
                    })),
            ),
        }
    }

    fn on_focus_in(&self, _: ev::FocusEvent) {
        if self.trigger_type.uses_click() {
            return;
        }
        self.clear_hide_timer();
        if self.delays.show_delay_ms == 0 {
            self.is_show.set(true);
        } else {
            self.schedule_show();
        }
    }

    fn on_focus_out(&self, _: ev::FocusEvent) {
        if self.trigger_type.uses_click() {
            return;
        }
        self.clear_show_timer();
        if should_schedule_hover_hide(self.trigger_type) {
            self.schedule_hide();
        } else {
            self.is_show.set(false);
        }
    }

    fn clear_show_timer(&self) {
        self.show_timer.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
        });
    }

    fn clear_hide_timer(&self) {
        self.hide_timer.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
        });
    }

    fn schedule_show(&self) {
        let is_show = self.is_show;
        let delay_ms = self.delays.show_delay_ms;
        self.show_timer.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
            *handle =
                set_timeout_with_handle(move || is_show.set(true), Duration::from_millis(delay_ms))
                    .ok();
        });
    }

    fn schedule_hide(&self) {
        let is_show = self.is_show;
        let delay_ms = self.delays.hide_delay_ms;
        self.hide_timer.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
            *handle = set_timeout_with_handle(
                move || is_show.set(false),
                Duration::from_millis(delay_ms),
            )
            .ok();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hover_delay_is_100ms() {
        assert_eq!(HOVER_HIDE_DELAY_MS, 100);
    }

    #[test]
    fn click_mode_skips_hover_hide() {
        assert!(!should_schedule_hover_hide(OverlayTriggerType::Click));
        assert!(should_schedule_hover_hide(OverlayTriggerType::Hover));
    }

    #[test]
    fn click_mode_toggles() {
        assert!(should_toggle_on_click(OverlayTriggerType::Click));
        assert!(!should_toggle_on_click(OverlayTriggerType::Hover));
    }
}
