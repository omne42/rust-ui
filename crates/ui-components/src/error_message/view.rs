use crate::error_message::{
    ErrorMessageMotion, ErrorMessageStateInput,
    logic::{self, ErrorMessageElement, ErrorMessageTone},
};
use leptos::prelude::*;

#[component]
pub fn ErrorMessage(
    text: String,
    #[prop(optional)] tone: ErrorMessageTone,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] truncate: bool,
    #[prop(optional)] element: ErrorMessageElement,
    #[prop(optional)] motion: ErrorMessageMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let motion = crate::error_message::motion::sanitize_motion(motion);
    let motion_source = if motion == ErrorMessageMotion::default() {
        "default"
    } else {
        "custom"
    };

    let (text, has_custom_message) = logic::normalize_message(Some(text));
    let text = StoredValue::new(text);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(ErrorMessageStateInput {
            tone,
            disabled,
            truncate,
            has_custom_message,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    match element {
        ErrorMessageElement::Span => view! {
            <span
                class=move || class.get()
                data-slot="error-message"
                slot="errorMessage"
                data-tone=move || state.get().tone_attr
                data-state=move || state.get().data_state_attr
                data-disabled=move || state.get().is_disabled.then_some("true")
                data-truncate=move || state.get().is_truncated.then_some("true")
                data-message-source=move || state.get().message_source_attr
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                data-motion-source=motion_source
                data-custom-motion=(motion != ErrorMessageMotion::default()).then_some("true")
                aria-label=aria_label
                aria-disabled=move || state.get().is_disabled.then_some("true")
                role="alert"
            >
                {text.get_value()}
            </span>
        }
        .into_any(),
        ErrorMessageElement::Paragraph => view! {
            <p
                class=move || class.get()
                data-slot="error-message"
                slot="errorMessage"
                data-tone=move || state.get().tone_attr
                data-state=move || state.get().data_state_attr
                data-disabled=move || state.get().is_disabled.then_some("true")
                data-truncate=move || state.get().is_truncated.then_some("true")
                data-message-source=move || state.get().message_source_attr
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                data-motion-source=motion_source
                data-custom-motion=(motion != ErrorMessageMotion::default()).then_some("true")
                aria-label=aria_label
                aria-disabled=move || state.get().is_disabled.then_some("true")
                role="alert"
            >
                {text.get_value()}
            </p>
        }
        .into_any(),
        ErrorMessageElement::Div => view! {
            <div
                class=move || class.get()
                data-slot="error-message"
                slot="errorMessage"
                data-tone=move || state.get().tone_attr
                data-state=move || state.get().data_state_attr
                data-disabled=move || state.get().is_disabled.then_some("true")
                data-truncate=move || state.get().is_truncated.then_some("true")
                data-message-source=move || state.get().message_source_attr
                data-aria-source=move || state.get().aria_source_attr
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-class-source=move || state.get().class_source_attr
                data-motion-source=motion_source
                data-custom-motion=(motion != ErrorMessageMotion::default()).then_some("true")
                aria-label=aria_label
                aria-disabled=move || state.get().is_disabled.then_some("true")
                role="alert"
            >
                {text.get_value()}
            </div>
        }
        .into_any(),
    }
}
