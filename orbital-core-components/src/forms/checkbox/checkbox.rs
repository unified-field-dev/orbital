use leptos::{html, prelude::*};
use orbital_base_components::{new_field_id, CheckboxSize, FormBind};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::checkbox_styles;
use crate::Icon;

/// Checkbox captures an independent yes/no choice — consent flags, multi-select lists,
/// and form options that stay checked until cleared.
///
/// Bind `checked` to a boolean signal, provide a `label` for accessible naming, and reach
/// for [`Switch`](crate::Switch) instead when the setting applies immediately on toggle.
///
/// # When to use
///
/// - Consent and terms acceptance before form submit
/// - Multi-select lists where each option is independent
/// - Boolean form fields that persist until the user clears them
///
/// # Usage
///
/// 1. Create a boolean signal and pass it as `checked` for two-way binding.
/// 2. Set `label` for an adjacent clickable caption.
/// 3. Pass `value` when the checkbox participates in a named group.
/// 4. Wrap in [`Field`](crate::Field) when the control needs a field label or validation messaging.
///
/// # Boolean choice controls
///
/// When `Checkbox` is not the right fit:
///
/// - **Independent yes/no, submit with form** — `Checkbox` (this component).
/// - **Immediate on/off setting** — [`Switch`](crate::Switch) (dark mode, notifications).
/// - **Exactly one of several options** — [`Radio`](crate::Radio) inside [`RadioGroup`](crate::RadioGroup).
///
/// # Best Practices
///
/// ## Do's
///
/// * Bind `checked` with a signal or [`FormBind`](crate::FormBind) for two-way sync
/// * Provide a visible `label` or wrap in [`Field`](crate::Field) for accessible naming
/// * Use `size` to match surrounding text density
///
/// ## Don'ts
///
/// * Do not use for on/off settings that take effect immediately — prefer [`Switch`](crate::Switch)
/// * Do not use when exactly one of several options must be chosen — use [`Radio`](crate::Radio) inside [`RadioGroup`](crate::RadioGroup)
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Checked
/// Checked state with an adjacent label; bind `checked` to a boolean signal for two-way sync.
/// <!-- preview -->
/// ```rust
/// let on = RwSignal::new(true);
/// view! {
///     <div data-testid="checkbox-preview">
///         <Checkbox checked=on label="Enable notifications" />
///     </div>
/// }
/// ```
///
/// ## Unchecked
/// Default off state before the user opts in; the indicator stays empty until `checked` becomes true.
/// <!-- preview -->
/// ```rust
/// let off = RwSignal::new(false);
/// view! {
///     <div data-testid="checkbox-unchecked">
///         <Checkbox checked=off label="Subscribe" />
///     </div>
/// }
/// ```
///
/// ## Disabled
/// Disabled checkboxes inside a disabled fieldset cannot be toggled.
/// <!-- preview -->
/// ```rust
/// let on = RwSignal::new(true);
/// view! {
///     <div data-testid="checkbox-disabled">
///         <fieldset disabled>
///             <Checkbox checked=on label="Disabled option" />
///         </fieldset>
///     </div>
/// }
/// ```
///
/// ## Size matrix
/// Medium and large indicator sizes for dense or prominent layouts.
/// <!-- preview -->
/// ```rust
/// use crate::{Checkbox, CheckboxSize, Flex};
/// let medium = RwSignal::new(true);
/// let large = RwSignal::new(false);
/// view! {
///     <div data-testid="checkbox-size-matrix">
///         <Flex vertical=true>
///             <div data-testid="checkbox-size-medium">
///                 <Checkbox checked=medium label="Medium" />
///             </div>
///             <div data-testid="checkbox-size-large">
///                 <Checkbox checked=large label="Large" size=CheckboxSize::Large />
///             </div>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Field wrapper
/// Field supplies the form label; the checkbox keeps its own inline caption.
/// <!-- preview -->
/// ```rust
/// use crate::{Checkbox, Field};
/// let agree = RwSignal::new(false);
/// view! {
///     <div data-testid="checkbox-field">
///         <Field label="Terms" name="terms">
///             <Checkbox checked=agree label="I accept the license" />
///         </Field>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "checkbox",
    preview_label = "Checkbox",
    preview_icon = icondata::AiCheckSquareOutlined,
)]
#[component]
pub fn Checkbox(
    /// Extra CSS class names merged onto the checkbox root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Two-way checked state (signal, store field, or plain initial value).
    #[prop(optional, into)]
    checked: FormBind<bool>,
    /// Value submitted when the checkbox is checked in a group.
    #[prop(optional, into)]
    value: Option<String>,
    /// Visible label text associated with the checkbox.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Visual size of the checkbox indicator.
    #[prop(optional, into)]
    size: Signal<CheckboxSize>,
    /// When true, the checkbox cannot be toggled.
    #[prop(default = false.into(), into)]
    disabled: Signal<bool>,
    /// Called after the checked state changes with the new value.
    #[prop(optional)]
    on_change: Option<Callback<bool>>,
) -> impl IntoView {
    inject_style("orbital-checkbox", checkbox_styles());

    let id = StoredValue::new(new_field_id());
    let item_value = StoredValue::new(value);
    let input_ref = NodeRef::<html::Input>::new();
    let checked = StoredValue::new(checked);

    let on_change = {
        let checked = checked;
        let on_change = on_change;
        move |_| {
            if let Some(input) = input_ref.get_untracked() {
                let value = input.checked();
                checked.get_value().set(value);
                if let Some(handler) = on_change.as_ref() {
                    handler.run(value);
                }
            }
        }
    };

    let wrapper_class = Memo::new(move |_| {
        let mut parts = vec!["orbital-checkbox".to_string()];
        if checked.get_value().get() {
            parts.push("orbital-checkbox--checked".to_string());
        }
        parts.push(format!("orbital-checkbox--{}", size.get().as_str()));
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    view! {
        <span class=wrapper_class>
            <input
                class="orbital-checkbox__input"
                type="checkbox"
                id=move || id.get_value()
                value=move || item_value.get_value()
                prop:checked=move || checked.get_value().get()
                prop:disabled=move || disabled.get()
                node_ref=input_ref
                on:change=on_change
            />
            <div aria-hidden="true" class="orbital-checkbox__indicator">
                {move || {
                    checked.get_value().get().then(|| {
                        let (width, height) = match size.get() {
                            CheckboxSize::Medium => ("12px", "12px"),
                            CheckboxSize::Large => ("16px", "16px"),
                        };
                        view! {
                            <Icon
                                icon=icondata::AiCheckOutlined
                                width=width
                                height=height
                                class="orbital-checkbox__check-icon"
                            />
                        }
                    })
                }}
            </div>
            {move || {
                label.get().map(|text| {
                    view! {
                        <label class="orbital-checkbox__label" for=id.get_value()>
                            {text}
                        </label>
                    }
                })
            }}
        </span>
    }
}
