use crate::{
    ErrorMessageMotion,
    logic::{self, ErrorMessageElement, ErrorMessageTone},
};
use leptos::prelude::*;
use ui_headless::{A11yDirection, ErrorMessageOptions, use_error_message};

#[component]
pub fn ErrorMessage(
    text: String,
    #[prop(optional)] tone: ErrorMessageTone,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] is_truncated: Option<bool>,
    #[prop(optional)] truncate: Option<bool>,
    #[prop(optional)] element: ErrorMessageElement,
    #[prop(optional)] motion: ErrorMessageMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let motion = crate::motion::sanitize_motion(motion);
    let motion_source = crate::motion::source_attr(motion);
    let motion_style = StoredValue::new(crate::motion::attach_motion(None, motion));
    let model = logic::resolve_model(logic::ErrorMessageModelInput {
        tone,
        is_disabled,
        disabled,
        is_truncated,
        truncate,
        text: Some(text),
        aria_label,
        class_name,
    });
    let text = StoredValue::new(model.text);
    let aria_label = StoredValue::new(model.aria_label);
    let class_name = StoredValue::new(model.class_name);
    let state = StoredValue::new(model.state);
    let lang = StoredValue::new(lang);

    let semantics = Memo::new(move |_| {
        use_error_message(ErrorMessageOptions {
            state: state.get_value(),
            aria_label: aria_label.get_value(),
            lang: lang.get_value(),
            dir,
        })
    });

    let class =
        Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get_value()));
    match element {
        ErrorMessageElement::Span => view! {
            <span
                class=move || class.get()
                data-slot="error-message"
                slot="errorMessage"
                style=move || motion_style.get_value()
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
                data-ui-schema=move || semantics.get().attrs.data_ui_schema
                data-ui-schema-version=move || semantics.get().attrs.data_ui_schema_version
                data-ui-intent=move || semantics.get().attrs.data_ui_intent
                data-ui-action=move || semantics.get().attrs.data_ui_action
                data-ui-state=move || semantics.get().state.state
                data-ui-source=move || semantics.get().state.message_source
                data-ui-stream-support=move || semantics.get().attrs.data_ui_stream_support
                data-ui-stream-fallback=move || semantics.get().attrs.data_ui_stream_fallback
                data-ui-stream-mode=move || semantics.get().attrs.data_ui_stream_mode
                data-ui-output-status=move || semantics.get().attrs.data_ui_output_status
                data-stream-mode=move || semantics.get().attrs.data_stream_mode
                data-stream-fallback=move || semantics.get().attrs.data_stream_fallback
                data-output-status=move || semantics.get().attrs.data_output_status
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
                style=move || motion_style.get_value()
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
                data-ui-schema=move || semantics.get().attrs.data_ui_schema
                data-ui-schema-version=move || semantics.get().attrs.data_ui_schema_version
                data-ui-intent=move || semantics.get().attrs.data_ui_intent
                data-ui-action=move || semantics.get().attrs.data_ui_action
                data-ui-state=move || semantics.get().state.state
                data-ui-source=move || semantics.get().state.message_source
                data-ui-stream-support=move || semantics.get().attrs.data_ui_stream_support
                data-ui-stream-fallback=move || semantics.get().attrs.data_ui_stream_fallback
                data-ui-stream-mode=move || semantics.get().attrs.data_ui_stream_mode
                data-ui-output-status=move || semantics.get().attrs.data_ui_output_status
                data-stream-mode=move || semantics.get().attrs.data_stream_mode
                data-stream-fallback=move || semantics.get().attrs.data_stream_fallback
                data-output-status=move || semantics.get().attrs.data_output_status
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
                style=move || motion_style.get_value()
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
                data-ui-schema=move || semantics.get().attrs.data_ui_schema
                data-ui-schema-version=move || semantics.get().attrs.data_ui_schema_version
                data-ui-intent=move || semantics.get().attrs.data_ui_intent
                data-ui-action=move || semantics.get().attrs.data_ui_action
                data-ui-state=move || semantics.get().state.state
                data-ui-source=move || semantics.get().state.message_source
                data-ui-stream-support=move || semantics.get().attrs.data_ui_stream_support
                data-ui-stream-fallback=move || semantics.get().attrs.data_ui_stream_fallback
                data-ui-stream-mode=move || semantics.get().attrs.data_ui_stream_mode
                data-ui-output-status=move || semantics.get().attrs.data_ui_output_status
                data-stream-mode=move || semantics.get().attrs.data_stream_mode
                data-stream-fallback=move || semantics.get().attrs.data_stream_fallback
                data-output-status=move || semantics.get().attrs.data_output_status
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
