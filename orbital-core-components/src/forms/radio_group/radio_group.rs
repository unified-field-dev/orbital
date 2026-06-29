use leptos::prelude::*;
use orbital_base_components::BaseRadioGroup;
use orbital_style::inject_style;

use super::styles::radio_group_styles;
use super::types::{RadioGroupBind, RadioGroupLayout};

/// Mutually exclusive choice group for [`Radio`](crate::Radio) options.
///
/// Bind the group's selected value to `Option<String>` — `None` means no selection yet,
/// not an empty string. Wrap in [`Field`](crate::Field) when the group needs a visible
/// label or validation messaging.
///
/// Catalog entry: preview slug `radio` documents the Radio + RadioGroup pair together.
/// Set [`RadioGroupLayout::Horizontal`] for compact option rows in a horizontal layout.
///
/// # Boolean choice controls
///
/// When `RadioGroup` is not the right fit:
///
/// - **Independent yes/no, submit with form** — [`Checkbox`](crate::Checkbox).
/// - **Immediate on/off setting** — [`Switch`](crate::Switch).
/// - **Exactly one of several options** — `RadioGroup` (this component) with [`Radio`](crate::Radio) children.
#[component]
pub fn RadioGroup(
    /// Two-way binding for the selected option value, plus optional id, name, and validation rules.
    #[prop(optional, into)]
    bind: RadioGroupBind,
    /// Additional CSS class names merged onto the root `orbital-radio-group` container.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Stack options vertically or arrange them in a horizontal row.
    #[prop(optional, into)]
    layout: Signal<RadioGroupLayout>,
    children: Children,
) -> impl IntoView {
    inject_style("orbital-radio", radio_group_styles());

    let RadioGroupBind {
        value,
        id,
        name,
        rules,
    } = bind;
    let class = MaybeProp::derive(move || {
        let mut parts = vec!["orbital-radio-group".to_string()];
        if layout.get() == RadioGroupLayout::Horizontal {
            parts.push("orbital-radio-group--horizontal".to_string());
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        Some(parts.join(" "))
    });

    view! {
        <BaseRadioGroup
            class=class
            id=id
            name=name
            value=value
            rules=rules
        >
            {children()}
        </BaseRadioGroup>
    }
}
