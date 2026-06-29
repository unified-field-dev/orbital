use leptos::{html, prelude::*};
use orbital_base_components::{new_field_id, FieldInjection, Rule, SwitchRuleTrigger};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::switch_styles;
use super::types::{SwitchBind, SwitchLabel};

/// Switch toggles a boolean setting that takes effect immediately — dark mode,
/// notifications, feature flags.
///
/// Bind `checked` with [`SwitchBind`], add an inline `label` for the setting name, and
/// wrap in [`Field`](crate::Field) when the form needs a section label or validation message.
/// Use [`Checkbox`](crate::Checkbox) when the user must explicitly opt in before submit.
///
/// # When to use
///
/// - Immediate on/off settings (dark mode, notifications)
/// - Compact boolean controls where a checkbox label feels too heavy
/// - Forms that submit a named boolean value
///
/// # Usage
///
/// 1. Bind `checked` via [`SwitchBind`] (typically `RwSignal<bool>`).
/// 2. Pass `label` through [`SwitchLabel`] for the adjacent caption.
/// 3. Set `id` explicitly when multiple switches appear on one page.
/// 4. Attach `rules` on [`SwitchBind`] when Field should show validation errors.
///
/// Grouped props: [`SwitchBind`] (checked, name, value, rules, disabled),
/// [`SwitchLabel`] (label). At call sites you can write `bind=signal` via [`From`] on [`SwitchBind`].
///
/// # Boolean choice controls
///
/// When `Switch` is not the right fit:
///
/// - **Independent yes/no, submit with form** — [`Checkbox`](crate::Checkbox).
/// - **Immediate on/off setting** — `Switch` (this component).
/// - **Exactly one of several options** — [`Radio`](crate::Radio) inside [`RadioGroup`](crate::RadioGroup).
///
/// # Best Practices
///
/// ## Do's
///
/// * Wrap in [`Field`](crate::Field) when a visible label or validation message is required
/// * Bind `checked` with [`SwitchBind`] for two-way sync
/// * Use `disabled` on [`SwitchBind`] while saving or when the setting is locked
///
/// ## Don'ts
///
/// * Do not use for explicit opt-in before form submit — prefer [`Checkbox`](crate::Checkbox)
/// * Do not use for multi-select lists — use [`Checkbox`](crate::Checkbox)
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## On
/// Switch in the on position; bind `checked` via [`SwitchBind`] (or pass a signal with `bind=signal`).
/// <!-- preview -->
/// ```rust
/// let on = RwSignal::new(true);
/// view! {
///     <div data-testid="switch-preview">
///         <Switch bind=on label="Dark mode" />
///     </div>
/// }
/// ```
///
/// ## Off
/// Default off state for settings that start disabled until the user toggles them on.
/// <!-- preview -->
/// ```rust
/// let off = RwSignal::new(false);
/// view! {
///     <div data-testid="switch-off">
///         <Switch bind=off label="Notifications" />
///     </div>
/// }
/// ```
///
/// ## Disabled
/// Locked setting: shows the current on/off state but ignores pointer and keyboard toggles.
/// <!-- preview -->
/// ```rust
/// let on = RwSignal::new(true);
/// view! {
///     <div data-testid="switch-disabled">
///         <Switch
///             bind=SwitchBind {
///                 checked: on.into(),
///                 disabled: Signal::from(true),
///                 ..Default::default()
///             }
///             label="Locked setting"
///         />
///     </div>
/// }
/// ```
///
/// ## Field wrapper
/// Field provides the form label and id wiring; the switch keeps its own inline caption for the setting.
/// <!-- preview -->
/// ```rust
/// use crate::{Field, Switch, SwitchBind};
/// let enabled = RwSignal::new(true);
/// view! {
///     <div data-testid="switch-field">
///         <Field label="Email alerts" name="alerts">
///             <Switch bind=enabled label="Send weekly summary" />
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Keyboard
/// The native switch input receives focus; Space toggles the checked state.
/// <!-- preview -->
/// ```rust
/// let on = RwSignal::new(false);
/// view! {
///     <div data-testid="switch-keyboard">
///         <Switch bind=on label="Press Space to toggle" id="switch-keyboard-demo" />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "switch",
    preview_label = "Switch",
    preview_icon = icondata::AiSwapOutlined,
)]
#[component]
pub fn Switch(
    /// Checked state binding, identity, validation rules, and disabled state.
    #[prop(optional, into)]
    bind: SwitchBind,
    /// Visible label associated with the switch.
    #[prop(optional, into)]
    label: SwitchLabel,
    /// Extra CSS class names merged onto the switch root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Explicit `id` for the switch input; auto-generated when omitted.
    #[prop(optional, into)]
    id: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orbital-switch", switch_styles());

    let SwitchBind {
        checked,
        name,
        value,
        rules,
        disabled,
    } = bind;
    let SwitchLabel { label } = label;

    let (field_id, name) = FieldInjection::use_id_and_name(id, name);
    let fallback_id = StoredValue::new(new_field_id());
    let resolved_id =
        Signal::derive(move || field_id.get().unwrap_or_else(|| fallback_id.get_value()));

    let checked = StoredValue::new(checked);
    let validate = Rule::validate(rules, checked, name);
    let input_ref = NodeRef::<html::Input>::new();

    let on_change = move |_| {
        if let Some(input) = input_ref.get_untracked() {
            let new_checked = input.checked();
            let changed = checked.with_value(|value| {
                if value.get_untracked() != new_checked {
                    value.set(new_checked);
                    true
                } else {
                    false
                }
            });
            if changed {
                validate.run(Some(SwitchRuleTrigger::Change));
            }
        }
    };

    let wrapper_class = Memo::new(move |_| {
        let mut parts = vec!["orbital-switch".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    view! {
        <div class=wrapper_class>
            <input
                class="orbital-switch__input"
                role="switch"
                type="checkbox"
                id=resolved_id
                name=name
                value=move || value.get()
                prop:checked=move || checked.get_value().get()
                disabled=move || disabled.get().then_some("")
                node_ref=input_ref
                on:change=on_change
            />
            <div aria-hidden="true" class="orbital-switch__indicator">
                <div class="orbital-switch__thumb" />
            </div>
            {move || {
                label.get().map(|text| {
                    view! {
                        <label class="orbital-switch__label" for=resolved_id.get()>
                            {text}
                        </label>
                    }
                })
            }}
        </div>
    }
}
