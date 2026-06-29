use leptos::{children::ToChildren, context::Provider, html, prelude::*};
use orbital_base_components::{
    listbox_keyboard_event, new_field_id, use_active_descendant, AnchorWidth, AnchoredPanel,
    AnchoredPositioner, BaseListbox, FormBind, Handler, InputRef, Placement,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;
use std::collections::HashMap;

use super::styles::auto_complete_styles;
use super::types::{AutoCompleteAppearance, AutoCompleteBind, AutoCompleteEvents};
use crate::{Input, InputAppearance, InputBind, InputEvents};

#[derive(Clone)]
struct AutoCompleteInjection {
    value: FormBind<String>,
    options: StoredValue<HashMap<String, String>>,
    on_select: Callback<String>,
    filter_needle: Signal<String>,
}

impl AutoCompleteInjection {
    fn expect_context() -> Self {
        expect_context()
    }

    fn insert_option(&self, id: String, value: String) {
        self.options.update_value(|options| {
            options.insert(id, value);
        });
    }

    fn remove_option(&self, id: &str) {
        self.options.update_value(|options| {
            options.remove(id);
        });
    }

    fn is_selected(&self, value: &str) -> bool {
        self.value.get() == value
    }
}

/// AutoComplete helps users type ahead against a fixed suggestion list while keeping
/// free text in the bound value — entity lookup, tagging prep, or search fields where
/// not every query matches an option.
///
/// For strict single- or multi-select without free text, use [`Combobox`](crate::Combobox).
/// For short native lists, use [`Select`](crate::Select).
///
/// # When to use
///
/// - Search-as-you-type fields with a fixed option set
/// - Entity lookup where typing narrows candidates
/// - Forms where free text is allowed but suggestions speed entry
///
/// # Usage
///
/// 1. Bind a `String` signal via [`AutoCompleteBind`].
/// 2. Add [`AutoCompleteOption`] children for each suggestion.
/// 3. Use `events.on_select` when the app needs a side effect on pick.
/// 4. Set `appearance.clear_after_select` to reset the field after a pick, or `blur_after_select` to dismiss the list on selection.
///
/// # Form picker controls
///
/// When `AutoComplete` is not the right fit:
///
/// - **Short fixed list, native form post** — [`Select`](crate::Select).
/// - **Strict pick from list, including multiselect** — [`Combobox`](crate::Combobox).
/// - **Typeahead with free text allowed** — `AutoComplete` (this component).
///
/// # Best Practices
///
/// ## Do's
///
/// * Provide distinct `value` strings for each option
/// * Use `clear_after_select` when the field should reset after pick
/// * Use `blur_after_select` when the list should close without keeping focus in the input
///
/// ## Don'ts
///
/// * Do not use for multiselect — prefer [`Combobox`](crate::Combobox)
/// * Do not use for strict single-select dropdowns — prefer [`Combobox`](crate::Combobox) or [`Select`](crate::Select)
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Basic autocomplete
/// Type-to-filter suggestions appear in a list below the input as users narrow the option set.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="auto-complete-preview">
///         <AutoComplete bind=value appearance=AutoCompleteAppearance::with_placeholder("Search")>
///             <AutoCompleteOption value="Alpha".to_string()>"Alpha"</AutoCompleteOption>
///             <AutoCompleteOption value="Beta".to_string()>"Beta"</AutoCompleteOption>
///         </AutoComplete>
///     </div>
/// }
/// ```
///
/// ## Selection callback
/// `on_select` fires when a suggestion is picked so the app can react with side effects or logging.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::Handler;
/// let value = RwSignal::new(String::new());
/// let selected = RwSignal::new(String::new());
/// view! {
///     <div data-testid="auto-complete-select">
///         <AutoComplete
///             bind=value
///             events=AutoCompleteEvents { on_select: Some(Handler::from({
///                 move |v: String| selected.set(v)
///             })) }
///         >
///             <AutoCompleteOption value="One".to_string()>"One"</AutoCompleteOption>
///             <AutoCompleteOption value="Two".to_string()>"Two"</AutoCompleteOption>
///         </AutoComplete>
///     </div>
/// }
/// ```
///
/// ## Clear after select
/// Input resets after each pick—useful for quick repeated lookups without manual clearing.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="auto-complete-clear">
///         <AutoComplete
///             bind=value
///             appearance=AutoCompleteAppearance {
///                 clear_after_select: Signal::from(true),
///                 ..Default::default()
///             }
///         >
///             <AutoCompleteOption value="Apple".to_string()>"Apple"</AutoCompleteOption>
///         </AutoComplete>
///     </div>
/// }
/// ```
///
/// ## Blur after select
/// Moves focus away from the input after pick to signal the entry flow is complete.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="auto-complete-blur">
///         <AutoComplete
///             bind=value
///             appearance=AutoCompleteAppearance {
///                 blur_after_select: Signal::from(true),
///                 ..Default::default()
///             }
///         >
///             <AutoCompleteOption value="North".to_string()>"North"</AutoCompleteOption>
///         </AutoComplete>
///     </div>
/// }
/// ```
///
/// ## Disabled state
/// Blocks typing and selection when the field is unavailable in the current form context.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="auto-complete-disabled">
///         <AutoComplete bind=value appearance=AutoCompleteAppearance::disabled()>
///             <AutoCompleteOption value="Disabled".to_string()>"Disabled"</AutoCompleteOption>
///         </AutoComplete>
///     </div>
/// }
/// ```
///
/// ## Keyboard list navigation
/// Arrow keys and Enter select options without leaving the keyboard—open the list, move, and confirm.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="auto-complete-keyboard">
///         <AutoComplete bind=value>
///             <AutoCompleteOption value="First".to_string()>"First"</AutoCompleteOption>
///             <AutoCompleteOption value="Second".to_string()>"Second"</AutoCompleteOption>
///             <AutoCompleteOption value="Third".to_string()>"Third"</AutoCompleteOption>
///         </AutoComplete>
///     </div>
/// }
/// ```
///
/// ## In field
/// AutoComplete nested in a [`Field`](crate::Field) with an associated label for standard form layouts.
/// <!-- preview -->
/// ```rust
/// use crate::{Field, AutoCompleteOption};
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="auto-complete-field">
///         <Field label="City">
///             <AutoComplete bind=value>
///                 <AutoCompleteOption value="Seattle".to_string()>"Seattle"</AutoCompleteOption>
///                 <AutoCompleteOption value="Portland".to_string()>"Portland"</AutoCompleteOption>
///             </AutoComplete>
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Size matrix
/// Small and large input presets for dense toolbars versus prominent form fields.
/// <!-- preview -->
/// ```rust
/// use crate::{AutoCompleteAppearance, AutoCompleteOption, AutoCompleteSize, Flex};
/// let small_value = RwSignal::new(String::new());
/// let large_value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="auto-complete-size-matrix">
///         <Flex vertical=true>
///             <div data-testid="auto-complete-size-small">
///                 <AutoComplete bind=small_value appearance=AutoCompleteAppearance::from(AutoCompleteSize::Small)>
///                     <AutoCompleteOption value="s".to_string()>"Small"</AutoCompleteOption>
///                 </AutoComplete>
///             </div>
///             <div data-testid="auto-complete-size-large">
///                 <AutoComplete bind=large_value appearance=AutoCompleteAppearance::from(AutoCompleteSize::Large)>
///                     <AutoCompleteOption value="l".to_string()>"Large"</AutoCompleteOption>
///                 </AutoComplete>
///             </div>
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "auto-complete",
    preview_label = "AutoComplete",
    preview_icon = icondata::AiSearchOutlined,
)]
#[component]
pub fn AutoComplete(
    /// Value binding and optional field identity.
    #[prop(optional, into)]
    bind: AutoCompleteBind,
    /// Placeholder, disabled state, size, and post-select behavior flags.
    #[prop(optional, into)]
    appearance: AutoCompleteAppearance,
    /// Selection callback and other event hooks.
    #[prop(optional, into)]
    events: AutoCompleteEvents,
    /// Extra CSS class names merged onto the root wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// [`AutoCompleteOption`] children defining the suggestion list entries.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-auto-complete", auto_complete_styles());

    let AutoCompleteBind { value, id, name } = bind;
    let AutoCompleteAppearance {
        placeholder,
        disabled,
        size,
        clear_after_select,
        blur_after_select,
    } = appearance;
    let AutoCompleteEvents { on_select } = events;

    let listbox_ref = NodeRef::<html::Div>::new();
    let input_ref = orbital_base_components::ComponentRef::<InputRef>::new();
    let open_listbox = RwSignal::new(false);
    let options = StoredValue::new(HashMap::<String, String>::new());
    let listbox_id = StoredValue::new(format!("orbital-auto-complete-listbox-{}", new_field_id()));

    let value_for_select = value.clone();
    let input_ref_for_blur = input_ref.clone();

    let on_select_cb = Callback::new(move |option_value: String| {
        if clear_after_select.get_untracked() {
            value_for_select.set(String::new());
        } else {
            value_for_select.set(option_value.clone());
        }
        if let Some(on_select) = on_select.as_ref() {
            on_select(option_value.clone());
        }
        open_listbox.set(false);
        if blur_after_select.get_untracked() {
            if let Some(input) = input_ref_for_blur.get_untracked() {
                input.blur();
            }
        }
    });

    let (set_listbox, active_descendant_controller) = use_active_descendant(|el| {
        el.class_list().contains("orbital-auto-complete-option")
            && !el
                .class_list()
                .contains("orbital-auto-complete-option--hidden")
    });

    let on_blur = {
        let active_descendant_controller = active_descendant_controller.clone();
        move |_| {
            active_descendant_controller.blur();
            open_listbox.set(false);
        }
    };

    let on_keydown = {
        let active_descendant_controller = active_descendant_controller.clone();
        let on_select_cb = on_select_cb;
        move |e| {
            listbox_keyboard_event(
                e,
                open_listbox,
                false,
                &active_descendant_controller,
                move |option| {
                    options.with_value(|all| {
                        if let Some(value) = all.get(&option.id()) {
                            on_select_cb.run(value.clone());
                        }
                    });
                },
            );
        }
    };

    let input_bind = InputBind {
        value: value.clone(),
        id,
        name,
        ..Default::default()
    };
    let input_appearance = InputAppearance {
        placeholder,
        disabled,
        size: Signal::derive(move || size.get().into()),
        ..Default::default()
    };
    let input_events = InputEvents {
        on_focus: Some(Handler::from(move |_| open_listbox.set(true))),
        on_blur: Some(Handler::from(on_blur)),
        allow_value: Some(Handler::<String, bool>::with(move |_| {
            if !open_listbox.get_untracked() {
                open_listbox.set(true);
            }
            true
        })),
    };

    let wrapper_class = Memo::new(move |_| {
        let mut parts = vec!["orbital-auto-complete".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    let filter_needle = Signal::derive({
        let value = value.clone();
        move || value.get().trim().to_ascii_lowercase()
    });

    view! {
        <AnchoredPositioner panel=AnchoredPanel {
            show: open_listbox.read_only().into(),
            width: Some(AnchorWidth::Target),
            placement: Placement::BottomStart,
            auto_height: false,
            arrow: None,
            motion: None,
            children: ToChildren::to_children(move || {
                view! {
                    <Provider value=AutoCompleteInjection {
                        value: value.clone(),
                        options,
                        on_select: on_select_cb,
                        filter_needle,
                    }>
                        <BaseListbox class="orbital-auto-complete__listbox" listbox_ref set_listbox>
                            <div id=move || listbox_id.get_value()>{children()}</div>
                        </BaseListbox>
                    </Provider>
                }
            }),
        }>
            <div class=wrapper_class on:keydown=on_keydown>
                <Input
                    bind=input_bind
                    appearance=input_appearance
                    events=input_events
                    comp_ref=input_ref
                />
            </div>
        </AnchoredPositioner>
    }
}

#[component]
pub fn AutoCompleteOption(
    #[prop(optional, into)] class: MaybeProp<String>,
    value: String,
    children: Children,
) -> impl IntoView {
    let auto_complete = AutoCompleteInjection::expect_context();
    let listbox = orbital_base_components::ListboxInjection::expect_context();
    let is_selected = Memo::new({
        let value = value.clone();
        let auto_complete = auto_complete.clone();
        move |_| auto_complete.is_selected(&value)
    });
    let matches_filter = Memo::new({
        let value = value.clone();
        let filter_needle = auto_complete.filter_needle;
        move |_| {
            let needle = filter_needle.get();
            needle.is_empty() || value.to_ascii_lowercase().contains(&needle)
        }
    });
    let id = new_field_id();
    {
        auto_complete.insert_option(id.clone(), value.clone());
        let id = id.clone();
        let auto_complete = auto_complete.clone();
        listbox.trigger();
        on_cleanup(move || {
            auto_complete.remove_option(&id);
            listbox.trigger();
        });
    }

    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-auto-complete-option".to_string()];
                if !matches_filter.get() {
                    parts.push("orbital-auto-complete-option--hidden".to_string());
                }
                if is_selected.get() {
                    parts.push("orbital-auto-complete-option--selected".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role="option"
            id=id
            aria-selected=move || if is_selected.get() { "true" } else { "false" }
            on:click=move |_| auto_complete.on_select.run(value.clone())
        >
            {children()}
        </div>
    }
}
