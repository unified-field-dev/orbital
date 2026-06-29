//! Compact density stepper for catalog previews and local theme demos.

use leptos::prelude::*;
use orbital_theme::{set_density, Density, Theme};

use crate::{Body1, Button, ButtonAppearance, Caption1, Flex, FlexAlign, FlexGap, FlexWrap};

fn density_label(density: Density) -> &'static str {
    match density {
        Density::Compact => "Compact",
        Density::Default => "Default",
        Density::Spacious => "Spacious",
    }
}

fn step_density(current: Density, delta: i32) -> Option<Density> {
    Some(match (current, delta) {
        (Density::Compact, -1) => return None,
        (Density::Compact, _) => Density::Default,
        (Density::Default, -1) => Density::Compact,
        (Density::Default, 1) => Density::Spacious,
        (Density::Spacious, 1) => return None,
        (Density::Spacious, -1) => Density::Default,
        _ => return None,
    })
}

/// Step compact / default / spacious density on the active theme signal.
#[component]
pub fn ThemeDensityStepper(
    /// Optional theme signal; defaults to the nearest [`Theme::use_rw_theme`] context.
    #[prop(optional)]
    theme: Option<RwSignal<Theme>>,
) -> impl IntoView {
    let theme = theme.unwrap_or_else(Theme::use_rw_theme);
    let at_min = Memo::new(move |_| theme.with(|t| t.options.density == Density::Compact));
    let at_max = Memo::new(move |_| theme.with(|t| t.options.density == Density::Spacious));
    let label = Memo::new(move |_| density_label(theme.with(|t| t.options.density)).to_string());

    view! {
        <Flex align=FlexAlign::Center gap=FlexGap::Small wrap=FlexWrap::Wrap>
            <Caption1>"Density"</Caption1>
            <span data-testid="theme-density-decrease">
                <Button
                    appearance=ButtonAppearance::Subtle
                    icon=icondata::AiMinusOutlined
                    disabled=at_min
                    on_click=Callback::new(move |_| {
                        let current = theme.get_untracked().options.density;
                        if let Some(next) = step_density(current, -1) {
                            set_density(theme, next);
                        }
                    })
                />
            </span>
            <span data-testid="theme-density-value"><Body1 block=false>{label}</Body1></span>
            <span data-testid="theme-density-increase">
                <Button
                    appearance=ButtonAppearance::Subtle
                    icon=icondata::AiPlusOutlined
                    disabled=at_max
                    on_click=Callback::new(move |_| {
                        let current = theme.get_untracked().options.density;
                        if let Some(next) = step_density(current, 1) {
                            set_density(theme, next);
                        }
                    })
                />
            </span>
        </Flex>
    }
}
