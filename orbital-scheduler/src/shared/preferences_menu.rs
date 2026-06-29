//! Preferences menu with toggle switches (SC-14, SC-27).

use leptos::{html::Div, prelude::*};
use orbital_core_components::{
    Button, Flex, FlexAlign, FlexGap, Popover, PopoverTrigger, PopoverTriggerType, Switch, Text,
};

use super::preferences::{WEEK_STARTS_MONDAY, WEEK_STARTS_SUNDAY};

/// Popover menu exposing scheduler preference toggles.
#[component]
pub fn SchedulerPreferencesMenu(
    show_weekends: RwSignal<bool>,
    ampm: RwSignal<bool>,
    week_starts_on: RwSignal<u8>,
    #[prop(optional, into)] label: MaybeProp<String>,
    /// Portal mount target so preference toggles stay inside preview wrappers in E2E.
    #[prop(default = None)]
    mount: Option<NodeRef<Div>>,
) -> impl IntoView {
    let week_starts_monday = RwSignal::new(week_starts_on.get_untracked() == WEEK_STARTS_MONDAY);

    Effect::new(move |_| {
        let monday = week_starts_monday.get();
        let next = if monday {
            WEEK_STARTS_MONDAY
        } else {
            WEEK_STARTS_SUNDAY
        };
        if week_starts_on.get_untracked() != next {
            week_starts_on.set(next);
        }
    });

    let menu_label = move || label.get().unwrap_or_else(|| "Preferences".to_string());

    view! {
        <Popover trigger_type=PopoverTriggerType::Click mount=mount>
            <PopoverTrigger slot>
                <div data-testid="scheduler-preferences-menu-trigger">
                    <Button>{menu_label}</Button>
                </div>
            </PopoverTrigger>
            <div
                class="orb-scheduler-preferences-panel"
                data-testid="scheduler-preferences-panel"
            >
                <Flex vertical=true gap=FlexGap::Small align=FlexAlign::Stretch>
                    <div data-testid="scheduler-pref-ampm">
                        <Switch
                            bind=ampm
                            label="12-hour clock".to_string()
                        />
                    </div>
                    <div data-testid="scheduler-pref-show-weekends">
                        <Switch
                            bind=show_weekends
                            label="Show weekends".to_string()
                        />
                    </div>
                    <div data-testid="scheduler-pref-week-starts-monday">
                        <Switch
                            bind=week_starts_monday
                            label="Week starts Monday".to_string()
                        />
                    </div>
                    <Text>"Changes apply to the scheduler grid and labels."</Text>
                </Flex>
            </div>
        </Popover>
    }
}
