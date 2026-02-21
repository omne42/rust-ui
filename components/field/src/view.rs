use std::borrow::Cow;

use crate::{
    FieldMotion, FieldStateInput,
    logic::{self, FieldOrientation, FieldTone},
    motion,
};
use leptos::prelude::*;
use ui_headless::{A11yDirection, FieldOptions, use_field};

const REQUIRED_INDICATOR_TEXT: &str = "*";
const REQUIRED_INDICATOR_ARIA_HIDDEN: &str = "true";

fn render_required_indicator() -> impl IntoView {
    view! {
        <span class="ui-field__required-indicator" data-slot="field-required" aria-hidden=REQUIRED_INDICATOR_ARIA_HIDDEN>
            {REQUIRED_INDICATOR_TEXT}
        </span>
    }
}

fn render_field_label(
    has_label: Signal<bool>,
    is_required: Signal<bool>,
    label_text: StoredValue<Cow<'static, str>>,
) -> impl IntoView {
    view! {
        <Show when=move || has_label.get()>
            <label class="ui-field__label" data-slot="field-label">
                {move || label_text.get_value()}
                <Show when=move || is_required.get()>
                    {render_required_indicator()}
                </Show>
            </label>
        </Show>
    }
}

fn render_field_description(
    shows_description: Signal<bool>,
    description_text: StoredValue<Cow<'static, str>>,
) -> impl IntoView {
    view! {
        <Show when=move || shows_description.get()>
            <p class="ui-field__description" data-slot="field-description">
                {move || description_text.get_value()}
            </p>
        </Show>
    }
}

fn render_field_error(
    shows_error: Signal<bool>,
    error_message_text: StoredValue<Cow<'static, str>>,
) -> impl IntoView {
    view! {
        <Show when=move || shows_error.get()>
            <p class="ui-field__error" data-slot="field-error" role="alert">
                {move || error_message_text.get_value()}
            </p>
        </Show>
    }
}

#[component]
pub fn Field(
    #[prop(optional)] orientation: FieldOrientation,
    #[prop(optional)] tone: FieldTone,
    #[prop(optional)] is_required: Option<bool>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] is_invalid: Option<bool>,
    #[prop(optional)] invalid: Option<bool>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error_message: Option<String>,
    #[prop(optional)] motion: FieldMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let required_source = logic::resolve_required_source(is_required, required);
    let required_source_attr = required_source.as_data_attr();
    let is_required = logic::resolve_is_required(is_required, required);
    let disabled_source = logic::resolve_disabled_source(is_disabled, disabled);
    let disabled_source_attr = disabled_source.as_data_attr();
    let is_disabled = logic::resolve_is_disabled(is_disabled, disabled);
    let invalid_source = logic::resolve_invalid_source(is_invalid, invalid);
    let invalid_source_attr = invalid_source.as_data_attr();
    let is_invalid = logic::resolve_is_invalid(is_invalid, invalid);

    let content = logic::resolve_content(logic::FieldContentInput {
        label,
        description,
        error_message,
        aria_label,
        lang,
        class_name,
        is_invalid,
    });

    let has_label = content.has_label;
    let has_description = content.has_description;
    let has_error_message = content.has_error_message;
    let has_custom_aria_label = content.has_custom_aria_label;
    let has_custom_error_message = content.has_custom_error_message;
    let has_custom_class_name = content.has_custom_class_name;

    let label_text = StoredValue::new(content.label_text);
    let description_text = StoredValue::new(content.description_text);
    let error_message_text = StoredValue::new(content.error_message_text);
    let class_name = StoredValue::new(content.class_name.map(std::borrow::Cow::into_owned));
    let aria_label = StoredValue::new(content.aria_label.into_owned());
    let lang = StoredValue::new(content.lang.map(std::borrow::Cow::into_owned));
    let motion = motion::sanitize_motion(motion);
    let motion_source_attr = motion::source_attr(motion);
    let motion_style = StoredValue::new(motion::attach_motion(motion));

    let state = Memo::new(move |_| {
        logic::resolve_state(FieldStateInput {
            orientation,
            tone,
            required: is_required,
            disabled: is_disabled,
            invalid: is_invalid,
            has_label,
            has_description,
            has_error_message,
            has_custom_aria_label,
            has_custom_error_message,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));
    let has_label_state = Signal::derive(move || state.get().has_label);
    let is_required_state = Signal::derive(move || state.get().is_required);
    let shows_description = Signal::derive(move || state.get().message_kind_attr == "description");
    let shows_error = Signal::derive(move || state.get().message_kind_attr == "error");
    let headless = Memo::new(move |_| {
        use_field(FieldOptions {
            state: state.get(),
            aria_label: aria_label.get_value(),
            lang: lang.get_value(),
            dir,
        })
    });
    let agent_contract = Memo::new(move |_| {
        logic::resolve_agent_contract(
            state.get(),
            required_source,
            disabled_source,
            invalid_source,
            motion_source_attr,
        )
    });

    view! {
        <div
            class=move || class.get()
            style=move || motion_style.get_value()
            data-slot="field"
            data-motion-source=motion_source_attr
            data-custom-motion=(motion_source_attr == "custom").then_some("true")
            data-orientation=move || headless.get().attrs.data_orientation
            data-tone=move || headless.get().attrs.data_tone
            data-state=move || headless.get().attrs.data_state
            data-message-kind=move || headless.get().attrs.data_message_kind
            data-required=move || headless.get().attrs.data_required
            data-disabled=move || headless.get().attrs.data_disabled
            data-invalid=move || headless.get().attrs.data_invalid
            data-required-source=required_source_attr
            data-disabled-source=disabled_source_attr
            data-invalid-source=invalid_source_attr
            data-has-label=move || headless.get().attrs.data_has_label
            data-has-description=move || headless.get().attrs.data_has_description
            data-has-error=move || headless.get().attrs.data_has_error
            data-aria-source=move || headless.get().attrs.data_aria_source
            data-error-source=move || headless.get().attrs.data_error_source
            data-custom-class=move || headless.get().attrs.data_custom_class
            data-class-source=move || headless.get().attrs.data_class_source
            data-ui-schema=move || agent_contract.get().schema
            data-ui-schema-version=move || agent_contract.get().schema_version
            data-ui-intent=move || agent_contract.get().intent
            data-ui-action=move || agent_contract.get().action
            data-ui-state=move || agent_contract.get().state
            data-ui-source=move || agent_contract.get().source
            data-ui-source-required=move || agent_contract.get().source_required
            data-ui-source-disabled=move || agent_contract.get().source_disabled
            data-ui-source-invalid=move || agent_contract.get().source_invalid
            data-ui-source-motion=move || agent_contract.get().source_motion
            data-ui-source-aria=move || agent_contract.get().source_aria
            data-ui-source-error=move || agent_contract.get().source_error
            data-ui-source-class=move || agent_contract.get().source_class
            data-ui-stream-mode=move || agent_contract.get().stream_mode
            data-ui-stream-support=move || agent_contract.get().stream_support
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback
            data-ui-output-mode=move || agent_contract.get().output_mode
            data-ui-output-status=move || agent_contract.get().output_status
            aria-label=move || headless.get().attrs.aria_label
            aria-disabled=move || headless.get().attrs.aria_disabled
            aria-invalid=move || headless.get().attrs.aria_invalid
            lang=move || headless.get().attrs.lang
            dir=move || headless.get().attrs.dir
        >
            {render_field_label(has_label_state, is_required_state, label_text)}

            <div class="ui-field__control" data-slot="field-control">
                {children()}
            </div>

            {render_field_description(shows_description, description_text)}

            {render_field_error(shows_error, error_message_text)}
        </div>
    }
}
