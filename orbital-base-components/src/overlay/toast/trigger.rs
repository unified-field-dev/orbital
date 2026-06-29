use leptos::prelude::*;

/// Context for dismissing the enclosing dispatched toast.
#[derive(Clone, Copy)]
pub struct ToastItemContext {
    pub dismiss: Callback<()>,
}

impl ToastItemContext {
    pub fn expect_context() -> Self {
        expect_context::<Self>()
    }
}

/// Wraps a single interactive child; clicking dismisses the enclosing toast.
#[component]
pub fn BaseToastTrigger(children: Children) -> impl IntoView {
    let ctx = ToastItemContext::expect_context();
    view! {
        <span
            class="orbital-toast-trigger"
            on:click=move |ev| {
                ev.stop_propagation();
                ctx.dismiss.run(());
            }
        >
            {children()}
        </span>
    }
}
