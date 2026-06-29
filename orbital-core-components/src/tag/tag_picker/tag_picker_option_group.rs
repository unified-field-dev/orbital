use leptos::prelude::*;

/// Labeled group of [`TagPickerOption`](super::tag_picker_option::TagPickerOption) children.
#[component]
pub fn TagPickerOptionGroup(
    /// Optional CSS class merged onto the group root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Label of the group.
    #[prop(into)]
    label: String,
    children: Children,
) -> impl IntoView {
    let group_class = Memo::new(move |_| {
        let mut parts = vec!["orbital-tag-picker-option-group".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    view! {
        <div role="group" class=group_class>
            <span role="presentation" class="orbital-tag-picker-option-group__label">
                {label}
            </span>
            {children()}
        </div>
    }
}
