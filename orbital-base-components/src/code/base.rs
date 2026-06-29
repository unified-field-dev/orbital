use leptos::{either::EitherOf3, prelude::*};

#[component]
pub fn BaseCode(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] text: Option<String>,
    #[prop(optional)] inner_html: Option<String>,
) -> impl IntoView {
    let root_class = Memo::new(move |_| {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            "orbital-code".to_string()
        } else {
            format!("orbital-code {extra}")
        }
    });

    view! {
        <code class=root_class>
            {if let Some(inner_html) = inner_html {
                EitherOf3::A(view! { <pre inner_html=inner_html></pre> })
            } else if let Some(text) = text {
                EitherOf3::B(view! { <pre>{text}</pre> })
            } else {
                EitherOf3::C(())
            }}
        </code>
    }
}
