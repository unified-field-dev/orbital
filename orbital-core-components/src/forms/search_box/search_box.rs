use leptos::prelude::*;
use orbital_macros::component_doc;

use crate::{
    Icon, Input, InputAppearance, InputBind, InputEvents, InputPrefix, InputRef, InputType,
};
use orbital_base_components::ComponentRef;
use orbital_base_components::Handler;

/// SearchBox is a search-styled [`Input`](crate::Input) preset — magnifier prefix, native
/// `search` type, and the same bind/appearance grouping as Input.
///
/// Use it for toolbar filters and list queries; debounce expensive lookups in your app code.
/// For custom icons or a clear button, compose [`Input`](crate::Input)
/// with [`InputPrefix`](crate::InputPrefix) and [`InputSuffix`](crate::InputSuffix) instead.
/// Also known as a search input in product copy.
///
/// # When to use
///
/// - Toolbar and header search fields
/// - Filter boxes that query lists or tables
/// - Any single-line search where a leading magnifier icon is expected
///
/// # Usage
///
/// 1. Bind `value` via [`SearchBoxBind`] (typically an `RwSignal<String>`).
/// 2. Set `placeholder` on [`SearchBoxAppearance`] to hint the query format.
/// 3. Wrap in [`Field`](crate::Field) when the search needs a visible label or validation.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use for query/filter UX — not for generic text entry (use [`Input`](crate::Input))
/// * Debounce server requests in app code rather than on every keystroke when costly
/// * Pair with Field when the search is a labeled form control
///
/// ## Don'ts
///
/// * Do not use when a plain [`Input`](crate::Input) with custom prefix/suffix is enough
/// * Do not put `data-testid` on the component — wrap with a native element
/// * Do not omit an accessible name when used without a visible Field label
///
/// # Examples
///
/// ## Default search box
/// Search icon prefix and native `search` input type.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="search-box-preview">
///         <SearchBox bind=value appearance=SearchBoxAppearance::with_placeholder("Search") />
///     </div>
/// }
/// ```
///
/// ## Disabled
/// Non-editable search field while a query is in flight.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new("query".to_string());
/// view! {
///     <div data-testid="search-box-disabled">
///         <SearchBox bind=value appearance=SearchBoxAppearance::disabled() />
///     </div>
/// }
/// ```
///
/// ## With Field
/// Labeled search in a filter form.
/// <!-- preview -->
/// ```rust
/// use crate::Field;
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="search-box-field">
///         <Field label="Search">
///             <SearchBox bind=value appearance=SearchBoxAppearance::with_placeholder("Filter items…") />
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Custom placeholder
/// Distinct placeholder copy for scoped search (people, documents, etc.) without changing the SearchBox API.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="search-box-placeholder">
///         <SearchBox bind=value appearance=SearchBoxAppearance::with_placeholder("Find people…") />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "search-box",
    preview_label = "Search Box",
    preview_icon = icondata::AiSearchOutlined,
)]
#[component]
pub fn SearchBox(
    /// Value binding, field name, and optional validation rules.
    #[prop(optional, into)]
    bind: SearchBoxBind,
    /// Placeholder, disabled state, size, and parser/format options.
    #[prop(optional, into)]
    appearance: SearchBoxAppearance,
    /// Focus, blur, and search-submit callbacks.
    #[prop(optional, into)]
    events: SearchBoxEvents,
    /// Extra CSS class names merged onto the search input wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Imperative handle for `focus`, `blur`, and `select` on the underlying input.
    #[prop(optional)]
    comp_ref: ComponentRef<InputRef>,
) -> impl IntoView {
    let input_bind = InputBind {
        value: bind.value,
        name: bind.name,
        rules: bind.rules,
        ..Default::default()
    };
    let input_appearance = InputAppearance {
        input_type: Signal::from(InputType::Search),
        placeholder: appearance.placeholder,
        disabled: appearance.disabled,
        size: appearance.size,
        parser: appearance.parser,
        format: appearance.format,
        ..Default::default()
    };
    let input_events = InputEvents {
        on_focus: events.on_focus,
        on_blur: events.on_blur,
        allow_value: events.allow_value,
    };

    view! {
        <Input
            class=class
            bind=input_bind
            appearance=input_appearance
            events=input_events
            comp_ref=comp_ref
        >
            <InputPrefix slot>
                <Icon icon=icondata::AiSearchOutlined />
            </InputPrefix>
        </Input>
    }
}

/// Value binding and validation for [`SearchBox`].
#[derive(Default)]
pub struct SearchBoxBind {
    pub value: orbital_base_components::FormBind<String>,
    pub name: MaybeProp<String>,
    pub rules: Vec<orbital_base_components::InputRule>,
}

impl SearchBoxBind {
    pub fn new(value: impl Into<orbital_base_components::FormBind<String>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl From<RwSignal<String>> for SearchBoxBind {
    fn from(value: RwSignal<String>) -> Self {
        Self::new(value)
    }
}

/// Visual attributes for [`SearchBox`].
#[derive(Default)]
pub struct SearchBoxAppearance {
    pub placeholder: MaybeProp<String>,
    pub disabled: Signal<bool>,
    pub size: Signal<orbital_base_components::InputSize>,
    pub parser: Option<Handler<String, Option<String>>>,
    pub format: Option<Handler<String, String>>,
}

impl SearchBoxAppearance {
    pub fn with_placeholder(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: MaybeProp::from(placeholder.into()),
            ..Default::default()
        }
    }

    pub fn disabled() -> Self {
        Self {
            disabled: Signal::from(true),
            ..Default::default()
        }
    }
}

/// Event callbacks for [`SearchBox`].
#[derive(Default)]
pub struct SearchBoxEvents {
    pub on_focus: Option<Handler<leptos::ev::FocusEvent>>,
    pub on_blur: Option<Handler<leptos::ev::FocusEvent>>,
    pub allow_value: Option<Handler<String, bool>>,
}
