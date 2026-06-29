use leptos::{either::EitherOf3, prelude::*};
use orbital_base_components::FieldValidationState;

/// Inline validation message with status icon (error, success, warning).
#[component]
pub fn FieldValidationMessage(#[prop(into)] state: FieldValidationState) -> impl IntoView {
    match state {
        FieldValidationState::Error(message) => EitherOf3::A(view! {
            <div class="orbital-field__validation-message">
                <span class="orbital-field__validation-message-icon orbital-field__validation-message-icon--error">
                    <svg
                        fill="currentColor"
                        aria-hidden="true"
                        width="12"
                        height="12"
                        viewBox="0 0 12 12"
                    >
                        <path
                            d="M6 11A5 5 0 1 0 6 1a5 5 0 0 0 0 10Zm-.75-2.75a.75.75 0 1 1 1.5 0 .75.75 0 0 1-1.5 0Zm.26-4.84a.5.5 0 0 1 .98 0l.01.09v2.59a.5.5 0 0 1-1 0V3.41Z"
                            fill="currentColor"
                        ></path>
                    </svg>
                </span>
                {message}
            </div>
        }),
        FieldValidationState::Success(message) => EitherOf3::B(view! {
            <div class="orbital-field__validation-message">
                <span class="orbital-field__validation-message-icon orbital-field__validation-message-icon--success">
                    <svg
                        fill="currentColor"
                        aria-hidden="true"
                        width="12"
                        height="12"
                        viewBox="0 0 12 12"
                    >
                        <path
                            d="M1 6a5 5 0 1 1 10 0A5 5 0 0 1 1 6Zm7.35-.9a.5.5 0 1 0-.7-.7L5.5 6.54 4.35 5.4a.5.5 0 1 0-.7.7l1.5 1.5c.2.2.5.2.7 0l2.5-2.5Z"
                            fill="currentColor"
                        ></path>
                    </svg>
                </span>
                {message}
            </div>
        }),
        FieldValidationState::Warning(message) => EitherOf3::C(view! {
            <div class="orbital-field__validation-message">
                <span class="orbital-field__validation-message-icon orbital-field__validation-message-icon--warning">
                    <svg
                        fill="currentColor"
                        aria-hidden="true"
                        width="12"
                        height="12"
                        viewBox="0 0 12 12"
                    >
                        <path
                            d="M5.21 1.46a.9.9 0 0 1 1.58 0l4.09 7.17a.92.92 0 0 1-.79 1.37H1.91a.92.92 0 0 1-.79-1.37l4.1-7.17ZM5.5 4.5v1a.5.5 0 0 0 1 0v-1a.5.5 0 0 0-1 0ZM6 6.75a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Z"
                            fill="currentColor"
                        ></path>
                    </svg>
                </span>
                {message}
            </div>
        }),
    }
}
