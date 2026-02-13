use crate::field::{
    FieldMotion, FieldStateInput,
    logic::{self, FieldOrientation, FieldTone},
    motion,
};
use leptos::prelude::*;

#[component]
pub fn Field(
    #[prop(optional)] orientation: FieldOrientation,
    #[prop(optional)] tone: FieldTone,
    #[prop(optional)] required: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] invalid: bool,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error_message: Option<String>,
    #[prop(optional)] motion: FieldMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let label = logic::normalize_optional_text(label);
    let description = logic::normalize_optional_text(description);
    let (error_message, has_custom_error_message) =
        logic::normalize_error_message(error_message, invalid);

    let has_label = label.is_some();
    let has_description = description.is_some();
    let has_error_message = error_message.is_some();

    let label = StoredValue::new(label);
    let description = StoredValue::new(description);
    let error_message = StoredValue::new(error_message);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != FieldMotion::default();

    let state = Memo::new(move |_| {
        logic::resolve_state(FieldStateInput {
            orientation,
            tone,
            required,
            disabled,
            invalid,
            has_label,
            has_description,
            has_error_message,
            has_custom_aria_label,
            has_custom_error_message,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <div
            class=move || class.get()
            data-slot="field"
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-orientation=move || state.get().orientation_attr
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-message-kind=move || state.get().message_kind_attr
            data-required=move || state.get().is_required.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-has-label=move || state.get().has_label.then_some("true")
            data-has-description=move || state.get().has_description.then_some("true")
            data-has-error=move || state.get().has_error_message.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-error-source=move || state.get().error_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            aria-label=aria_label
            aria-disabled=move || state.get().is_disabled.then_some("true")
            aria-invalid=move || state.get().is_invalid.then_some("true")
        >
            <Show when=move || state.get().has_label>
                <label class="ui-field__label" data-slot="field-label">
                    {move || label.get_value().unwrap_or_default()}
                    <Show when=move || state.get().is_required>
                        <span class="ui-field__required-indicator" data-slot="field-required" aria-hidden="true">
                            "*"
                        </span>
                    </Show>
                </label>
            </Show>

            <div class="ui-field__control" data-slot="field-control">
                {children()}
            </div>

            <Show when=move || state.get().message_kind_attr == "description">
                <p class="ui-field__description" data-slot="field-description">
                    {move || description.get_value().unwrap_or_default()}
                </p>
            </Show>

            <Show when=move || state.get().message_kind_attr == "error">
                <p class="ui-field__error" data-slot="field-error" role="alert">
                    {move || error_message.get_value().unwrap_or_default()}
                </p>
            </Show>
        </div>
    }
}
