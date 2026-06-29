use leptos::{context::Provider, prelude::*};

use super::injection::GridInjection;

#[component]
pub fn BaseGrid(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = 1u16.into(), into)] cols: Signal<u16>,
    #[prop(optional, into)] x_gap: Signal<u16>,
    #[prop(optional, into)] y_gap: Signal<u16>,
    children: Children,
) -> impl IntoView {
    let style = Memo::new(move |_| {
        let mut style = String::from("display: grid;");
        style.push_str(&format!(
            "grid-template-columns: repeat({}, minmax(0px, 1fr));",
            cols.get()
        ));
        style.push_str(&format!("grid-gap: {}px {}px;", y_gap.get(), x_gap.get()));
        style
    });

    view! {
        <Provider value=GridInjection::new(x_gap)>
            <div
                class=move || {
                    let extra = class.get().unwrap_or_default();
                    if extra.is_empty() {
                        "orbital-grid".to_string()
                    } else {
                        format!("orbital-grid {extra}")
                    }
                }
                style=move || style.get()
            >
                {children()}
            </div>
        </Provider>
    }
}
