use crate::error_message::{
    ErrorMessageMotion,
    logic::{self, ErrorMessageElement, ErrorMessageTone},
};
use leptos::prelude::*;
use ui_headless::{A11yDirection, ErrorMessageOptions, use_error_message};

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
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
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
    let aria_label = StoredValue::new(aria_label);
    let lang = StoredValue::new(lang);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(logic::ErrorMessageStateInput {
            tone,
            disabled,
            truncate,
            has_custom_message,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });
    let semantics = Memo::new(move |_| {
        use_error_message(ErrorMessageOptions {
            state: state.get(),
            aria_label: aria_label.get_value(),
            lang: lang.get_value(),
            dir,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));
    let ui_action = Memo::new(move |_| {
        if semantics.get().state.is_disabled {
            "read-only"
        } else {
            "announce-error"
        }
    });
    let ui_output_status = Memo::new(move |_| {
        if semantics.get().state.is_disabled {
            "draft"
        } else {
            "verified"
        }
    });

    match element {
        ErrorMessageElement::Span => view! {
            <span
                class=move || class.get()
                data-slot="error-message"
                slot="errorMessage"
                data-tone=move || semantics.get().attrs.data_tone
                data-state=move || semantics.get().attrs.data_state
                data-disabled=move || semantics.get().attrs.data_disabled
                data-truncate=move || semantics.get().attrs.data_truncate
                data-message-source=move || semantics.get().attrs.data_message_source
                data-aria-source=move || semantics.get().attrs.data_aria_source
                data-custom-class=move || semantics.get().attrs.data_custom_class
                data-class-source=move || semantics.get().attrs.data_class_source
                data-motion-source=motion_source
                data-custom-motion=(motion != ErrorMessageMotion::default()).then_some("true")
                data-ui-schema="ui.error-message.agent-contract.v1"
                data-ui-schema-version="1"
                data-ui-intent="form-validation-feedback"
                data-ui-action=move || ui_action.get()
                data-ui-state=move || semantics.get().state.state
                data-ui-source=move || semantics.get().state.message_source
                data-ui-stream-support="optional"
                data-ui-stream-fallback="snapshot"
                data-ui-stream-mode="snapshot"
                data-ui-output-status=move || ui_output_status.get()
                data-stream-mode="snapshot"
                data-stream-fallback="snapshot"
                data-output-status=move || ui_output_status.get()
                role=move || semantics.get().attrs.role
                aria-live=move || semantics.get().attrs.aria_live
                aria-label=move || semantics.get().attrs.aria_label.clone()
                aria-disabled=move || semantics.get().attrs.aria_disabled
                lang=move || semantics.get().attrs.lang.clone()
                dir=move || semantics.get().attrs.dir
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
                data-tone=move || semantics.get().attrs.data_tone
                data-state=move || semantics.get().attrs.data_state
                data-disabled=move || semantics.get().attrs.data_disabled
                data-truncate=move || semantics.get().attrs.data_truncate
                data-message-source=move || semantics.get().attrs.data_message_source
                data-aria-source=move || semantics.get().attrs.data_aria_source
                data-custom-class=move || semantics.get().attrs.data_custom_class
                data-class-source=move || semantics.get().attrs.data_class_source
                data-motion-source=motion_source
                data-custom-motion=(motion != ErrorMessageMotion::default()).then_some("true")
                data-ui-schema="ui.error-message.agent-contract.v1"
                data-ui-schema-version="1"
                data-ui-intent="form-validation-feedback"
                data-ui-action=move || ui_action.get()
                data-ui-state=move || semantics.get().state.state
                data-ui-source=move || semantics.get().state.message_source
                data-ui-stream-support="optional"
                data-ui-stream-fallback="snapshot"
                data-ui-stream-mode="snapshot"
                data-ui-output-status=move || ui_output_status.get()
                data-stream-mode="snapshot"
                data-stream-fallback="snapshot"
                data-output-status=move || ui_output_status.get()
                role=move || semantics.get().attrs.role
                aria-live=move || semantics.get().attrs.aria_live
                aria-label=move || semantics.get().attrs.aria_label.clone()
                aria-disabled=move || semantics.get().attrs.aria_disabled
                lang=move || semantics.get().attrs.lang.clone()
                dir=move || semantics.get().attrs.dir
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
                data-tone=move || semantics.get().attrs.data_tone
                data-state=move || semantics.get().attrs.data_state
                data-disabled=move || semantics.get().attrs.data_disabled
                data-truncate=move || semantics.get().attrs.data_truncate
                data-message-source=move || semantics.get().attrs.data_message_source
                data-aria-source=move || semantics.get().attrs.data_aria_source
                data-custom-class=move || semantics.get().attrs.data_custom_class
                data-class-source=move || semantics.get().attrs.data_class_source
                data-motion-source=motion_source
                data-custom-motion=(motion != ErrorMessageMotion::default()).then_some("true")
                data-ui-schema="ui.error-message.agent-contract.v1"
                data-ui-schema-version="1"
                data-ui-intent="form-validation-feedback"
                data-ui-action=move || ui_action.get()
                data-ui-state=move || semantics.get().state.state
                data-ui-source=move || semantics.get().state.message_source
                data-ui-stream-support="optional"
                data-ui-stream-fallback="snapshot"
                data-ui-stream-mode="snapshot"
                data-ui-output-status=move || ui_output_status.get()
                data-stream-mode="snapshot"
                data-stream-fallback="snapshot"
                data-output-status=move || ui_output_status.get()
                role=move || semantics.get().attrs.role
                aria-live=move || semantics.get().attrs.aria_live
                aria-label=move || semantics.get().attrs.aria_label.clone()
                aria-disabled=move || semantics.get().attrs.aria_disabled
                lang=move || semantics.get().attrs.lang.clone()
                dir=move || semantics.get().attrs.dir
            >
                {text.get_value()}
            </div>
        }
        .into_any(),
    }
}
