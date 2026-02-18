use crate::fieldset::{
    FieldsetMotion, FieldsetStateInput,
    logic::{self, FieldsetOrientation, FieldsetTone},
};
use leptos::{children::ViewFn, prelude::*};
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn Fieldset(
    children: Children,
    #[prop(optional)] orientation: FieldsetOrientation,
    #[prop(optional)] tone: FieldsetTone,
    #[prop(optional)] is_required: Option<bool>,
    #[prop(optional)] required: bool,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] is_invalid: Option<bool>,
    #[prop(optional)] invalid: bool,
    #[prop(optional, into)] legend: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error_message: Option<String>,
    #[prop(optional, into)] actions: Option<ViewFn>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: FieldsetMotion,
) -> impl IntoView {
    let motion = crate::fieldset::motion::sanitize_motion(motion);
    let has_custom_motion = motion != FieldsetMotion::default();
    let motion_style = StoredValue::new(Some(crate::fieldset::motion::attach_motion(motion)));

    let required = is_required.unwrap_or(required);
    let disabled = is_disabled.unwrap_or(disabled);
    let invalid = is_invalid.unwrap_or(invalid);
    let required_source_attr = if is_required.is_some() {
        "is_required"
    } else if required {
        "required"
    } else {
        "default"
    };
    let disabled_source_attr = if is_disabled.is_some() {
        "is_disabled"
    } else if disabled {
        "disabled"
    } else {
        "default"
    };
    let invalid_source_attr = if is_invalid.is_some() {
        "is_invalid"
    } else if invalid {
        "invalid"
    } else {
        "default"
    };

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let legend = logic::normalize_optional_text(legend);
    let description = logic::normalize_optional_text(description);
    let (error_message, has_custom_error_message) =
        logic::normalize_error_message(error_message, invalid);

    let has_legend = legend.is_some();
    let has_description = description.is_some();
    let has_error_message = error_message.is_some();

    let legend = StoredValue::new(legend);
    let description = StoredValue::new(description);
    let error_message = StoredValue::new(error_message);

    let actions = StoredValue::new(actions);
    let has_actions = actions.get_value().is_some();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(FieldsetStateInput {
            orientation,
            tone,
            required,
            disabled,
            invalid,
            has_legend,
            has_description,
            has_error_message,
            has_actions,
            has_custom_aria_label,
            has_custom_error_message,
            has_custom_class_name,
        })
    });
    let locale = locale_attrs(lang, dir);
    let agent_contract = Memo::new(move |_| logic::resolve_agent_contract(state.get()));

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <fieldset
            class=move || class.get()
            style=move || motion_style.get_value()
            lang=locale.lang.clone()
            dir=locale.dir
            data-slot="fieldset"
            data-orientation=move || state.get().orientation_attr
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-message-kind=move || state.get().message_kind_attr
            data-required=move || state.get().is_required.then_some("true")
            data-required-source=required_source_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-disabled-source=disabled_source_attr
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-invalid-source=invalid_source_attr
            data-has-legend=move || state.get().has_legend.then_some("true")
            data-has-description=move || state.get().has_description.then_some("true")
            data-has-error=move || state.get().has_error_message.then_some("true")
            data-has-actions=move || state.get().has_actions.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-error-source=move || state.get().error_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-ui-schema=move || agent_contract.get().schema_attr
            data-ui-schema-version=move || agent_contract.get().schema_version_attr
            data-ui-intent=move || agent_contract.get().intent_attr
            data-ui-action=move || agent_contract.get().action_attr
            data-ui-state=move || agent_contract.get().state_attr
            data-ui-source=move || agent_contract.get().source_attr
            data-ui-stream-support=move || agent_contract.get().stream_support_attr
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr
            data-ui-stream-mode=move || agent_contract.get().stream_mode_attr
            data-ui-output-status=move || agent_contract.get().output_status_attr
            aria-label=aria_label
            aria-disabled=move || state.get().is_disabled.then_some("true")
            aria-invalid=move || state.get().is_invalid.then_some("true")
        >
            <Show when=move || state.get().has_legend>
                <legend class="ui-fieldset__legend" data-slot="fieldset-legend">
                    {move || legend.get_value().unwrap_or_default()}
                    <Show when=move || state.get().is_required>
                        <span
                            class="ui-fieldset__required-indicator"
                            data-slot="fieldset-required"
                            aria-hidden="true"
                        >
                            "*"
                        </span>
                    </Show>
                </legend>
            </Show>

            <div class="ui-fieldset__group" data-slot="fieldset-field-group">
                {children()}
            </div>

            <Show when=move || state.get().has_actions>
                {move || {
                    actions.get_value().map(|actions| {
                        view! {
                            <div class="ui-fieldset__actions" data-slot="fieldset-actions">
                                {actions.run()}
                            </div>
                        }
                    })
                }}
            </Show>

            <Show when=move || state.get().message_kind_attr == "description">
                <p class="ui-fieldset__description" data-slot="fieldset-description">
                    {move || description.get_value().unwrap_or_default()}
                </p>
            </Show>

            <Show when=move || state.get().message_kind_attr == "error">
                <p class="ui-fieldset__error" data-slot="fieldset-error" role="alert">
                    {move || error_message.get_value().unwrap_or_default()}
                </p>
            </Show>
        </fieldset>
    }
}
