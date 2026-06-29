//! Shared preview controls for date-picker doc examples.

use leptos::prelude::*;
use orbital_core_components::{
    Field, Flex, FlexAlign, FlexGap, FlexWrap, Select, SelectAppearance, SelectBind,
};
use orbital_theme::{set_density, use_theme_options, Density, Theme};

/// Vertical stack for picker doc examples (density knobs, component, bind readout).
#[component]
pub fn PickerPreviewExample(
    #[prop(optional, into)] data_testid: MaybeProp<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <div data-testid=data_testid>
            <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Center full_width=true>
                {children()}
            </Flex>
        </div>
    }
}

/// Horizontal control row for preview toggles, selects, and other inline knobs.
#[component]
pub fn PickerPreviewControls(children: Children) -> impl IntoView {
    view! {
        <Flex
            gap=FlexGap::Medium
            wrap=FlexWrap::Wrap
            align=FlexAlign::End
            full_width=true
        >
            {children()}
        </Flex>
    }
}

/// Density selector wired to global theme density for preview pages.
#[component]
pub fn PickerPreviewKnobs(
    /// When false, renders nothing (FAQ preview omits density per worksheet).
    #[prop(default = true)]
    show_density: bool,
) -> impl IntoView {
    if !show_density {
        return ().into_any();
    }

    let theme = Theme::use_rw_theme();
    let theme_options = use_theme_options();
    let density_value = RwSignal::new(String::new());

    Effect::new(move |_| {
        let theme_density = theme_options.get().density;
        let label = match theme_density {
            Density::Compact => "compact",
            Density::Default => "default",
            Density::Spacious => "spacious",
        };
        if density_value.get_untracked() != label {
            density_value.set(label.to_string());
        }
    });

    Effect::new(move |_| {
        let value = density_value.get();
        let density = match value.as_str() {
            "compact" => Density::Compact,
            "spacious" => Density::Spacious,
            _ => Density::Default,
        };
        if theme_options.get().density != density {
            set_density(theme, density);
        }
    });

    view! {
        <div data-testid="picker-preview-knobs">
            <PickerPreviewControls>
                <Field label="Density" name="preview_density">
                <Select
                    bind=SelectBind::from(density_value)
                    appearance=SelectAppearance {
                        default_value: Some("default".to_string()),
                        ..Default::default()
                    }
                >
                    <option value="compact">"Compact"</option>
                    <option value="default">"Default"</option>
                    <option value="spacious">"Spacious"</option>
                </Select>
            </Field>
            </PickerPreviewControls>
        </div>
    }
    .into_any()
}
