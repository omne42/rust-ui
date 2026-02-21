use crate::{
    FieldsetMotion,
    logic::{self, FieldsetOrientation, FieldsetTone},
};
use leptos::{children::ViewFn, prelude::*};
use ui_headless::{A11yDirection, fieldset_attrs, use_controllable_state};

const SLOT_FIELDSET: &str = "fieldset";
const SLOT_FIELDSET_LEGEND: &str = "fieldset-legend";
const SLOT_FIELDSET_REQUIRED: &str = "fieldset-required";
const SLOT_FIELDSET_FIELD_GROUP: &str = "fieldset-field-group";
const SLOT_FIELDSET_ACTIONS: &str = "fieldset-actions";
const SLOT_FIELDSET_DESCRIPTION: &str = "fieldset-description";
const SLOT_FIELDSET_ERROR: &str = "fieldset-error";
const FIELDSET_REQUIRED_INDICATOR_TEXT: &str = "*";
const ROLE_ALERT: &str = "alert";

fn render_legend_block(
    state: Memo<logic::FieldsetState>,
    view_state: Memo<logic::FieldsetViewState>,
) -> impl IntoView {
    view! {
        <Show when=move || state.get().has_legend>
            <legend class="ui-fieldset__legend" data-slot=SLOT_FIELDSET_LEGEND>
                {move || view_state.get().legend.clone().unwrap_or_default()}
                <Show when=move || state.get().is_required>
                    <span
                        class="ui-fieldset__required-indicator"
                        data-slot=SLOT_FIELDSET_REQUIRED
                        aria-hidden="true"
                    >
                        {FIELDSET_REQUIRED_INDICATOR_TEXT}
                    </span>
                </Show>
            </legend>
        </Show>
    }
}

fn render_actions_block(
    state: Memo<logic::FieldsetState>,
    actions: StoredValue<Option<ViewFn>>,
) -> impl IntoView {
    view! {
        <Show when=move || state.get().has_actions>
            <div class="ui-fieldset__actions" data-slot=SLOT_FIELDSET_ACTIONS>
                {move || actions.get_value().map(|actions| actions.run())}
            </div>
        </Show>
    }
}

fn render_message_block(
    state: Memo<logic::FieldsetState>,
    view_state: Memo<logic::FieldsetViewState>,
) -> impl IntoView {
    view! {
        <>
            <Show when=move || state.get().message_kind == logic::FieldsetMessageKind::Description>
                <p class="ui-fieldset__description" data-slot=SLOT_FIELDSET_DESCRIPTION>
                    {move || view_state.get().description.clone().unwrap_or_default()}
                </p>
            </Show>

            <Show when=move || state.get().message_kind == logic::FieldsetMessageKind::Error>
                <p class="ui-fieldset__error" data-slot=SLOT_FIELDSET_ERROR role=ROLE_ALERT>
                    {move || view_state.get().error_message.clone().unwrap_or_default()}
                </p>
            </Show>
        </>
    }
}

#[component]
pub fn Fieldset(
    children: Children,
    #[prop(optional)] orientation: FieldsetOrientation,
    #[prop(optional)] tone: FieldsetTone,
    #[prop(optional)] is_required: Option<bool>,
    #[prop(optional)] default_is_required: Option<bool>,
    #[prop(optional)] on_is_required_change: Option<Callback<bool>>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] default_is_disabled: Option<bool>,
    #[prop(optional)] on_is_disabled_change: Option<Callback<bool>>,
    #[prop(optional)] is_invalid: Option<bool>,
    #[prop(optional)] default_is_invalid: Option<bool>,
    #[prop(optional)] on_is_invalid_change: Option<Callback<bool>>,
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
    let motion = crate::motion::sanitize_motion(motion);
    let has_custom_motion = motion != FieldsetMotion::default();
    let motion_style = StoredValue::new(Some(crate::motion::attach_motion(motion)));

    let required_axis = logic::normalize_boolean_axis(logic::FieldsetBooleanAxisInput {
        value: is_required,
        default_value: default_is_required,
        has_on_change: on_is_required_change.is_some(),
        value_source_attr: "is_required",
        default_source_attr: "default_is_required",
        change_source_attr: "on_is_required_change",
    });
    let disabled_axis = logic::normalize_boolean_axis(logic::FieldsetBooleanAxisInput {
        value: is_disabled,
        default_value: default_is_disabled,
        has_on_change: on_is_disabled_change.is_some(),
        value_source_attr: "is_disabled",
        default_source_attr: "default_is_disabled",
        change_source_attr: "on_is_disabled_change",
    });
    let invalid_axis = logic::normalize_boolean_axis(logic::FieldsetBooleanAxisInput {
        value: is_invalid,
        default_value: default_is_invalid,
        has_on_change: on_is_invalid_change.is_some(),
        value_source_attr: "is_invalid",
        default_source_attr: "default_is_invalid",
        change_source_attr: "on_is_invalid_change",
    });

    let required_state = use_controllable_state(
        required_axis
            .controlled_value
            .map(|value| Signal::derive(move || value)),
        Some(required_axis.initial_value),
        on_is_required_change,
    );
    let disabled_state = use_controllable_state(
        disabled_axis
            .controlled_value
            .map(|value| Signal::derive(move || value)),
        Some(disabled_axis.initial_value),
        on_is_disabled_change,
    );
    let invalid_state = use_controllable_state(
        invalid_axis
            .controlled_value
            .map(|value| Signal::derive(move || value)),
        Some(invalid_axis.initial_value),
        on_is_invalid_change,
    );
    let required = required_state.value;
    let disabled = disabled_state.value;
    let invalid = invalid_state.value;

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let actions = StoredValue::new(actions);
    let has_actions = actions.get_value().is_some();

    let legend_input = legend;
    let description_input = description;
    let error_message_input = error_message;
    let class_name_input = class_name;
    let view_state = Memo::new(move |_| {
        logic::resolve_view_state(logic::FieldsetViewStateInput {
            orientation,
            tone,
            required: required.get(),
            required_source_attr: required_axis.value_source_attr,
            required_control_mode_attr: required_axis.control_mode_attr,
            required_change_source_attr: required_axis.change_source_attr,
            disabled: disabled.get(),
            disabled_source_attr: disabled_axis.value_source_attr,
            disabled_control_mode_attr: disabled_axis.control_mode_attr,
            disabled_change_source_attr: disabled_axis.change_source_attr,
            invalid: invalid.get(),
            invalid_source_attr: invalid_axis.value_source_attr,
            invalid_control_mode_attr: invalid_axis.control_mode_attr,
            invalid_change_source_attr: invalid_axis.change_source_attr,
            legend: legend_input.clone(),
            description: description_input.clone(),
            error_message: error_message_input.clone(),
            class_name: class_name_input.clone(),
            has_actions,
            has_custom_aria_label,
        })
    });
    let state_view_state = view_state;
    let state = Memo::new(move |_| state_view_state.get().state);
    let aria_label = StoredValue::new(aria_label);
    let lang = StoredValue::new(lang);
    let a11y = Memo::new(move |_| {
        fieldset_attrs(
            aria_label.get_value(),
            state.get().is_disabled,
            state.get().is_invalid,
            lang.get_value(),
            dir,
        )
    });
    let agent_contract = Memo::new(move |_| logic::resolve_agent_contract(state.get()));

    let class_view_state = view_state;
    let class = Memo::new(move |_| {
        let resolved = class_view_state.get();
        logic::compose_class_name(resolved.class_name.clone(), resolved.state)
    });

    view! {
        <fieldset
            class=move || class.get()
            style=move || motion_style.get_value()
            lang=move || a11y.get().lang.clone()
            dir=move || a11y.get().dir
            data-slot=SLOT_FIELDSET
            data-orientation=move || state.get().orientation_attr
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-message-kind=move || state.get().message_kind_attr
            data-required=move || state.get().is_required.then_some("true")
            data-required-source=move || view_state.get().required_source_attr
            data-required-control-mode=move || view_state.get().required_control_mode_attr
            data-required-change-source=move || view_state.get().required_change_source_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-disabled-source=move || view_state.get().disabled_source_attr
            data-disabled-control-mode=move || view_state.get().disabled_control_mode_attr
            data-disabled-change-source=move || view_state.get().disabled_change_source_attr
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-invalid-source=move || view_state.get().invalid_source_attr
            data-invalid-control-mode=move || view_state.get().invalid_control_mode_attr
            data-invalid-change-source=move || view_state.get().invalid_change_source_attr
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
            aria-label=move || a11y.get().aria_label.clone()
            aria-disabled=move || a11y.get().aria_disabled
            aria-invalid=move || a11y.get().aria_invalid
        >
            {render_legend_block(state, view_state)}

            <div class="ui-fieldset__group" data-slot=SLOT_FIELDSET_FIELD_GROUP>
                {children()}
            </div>

            {render_actions_block(state, actions)}
            {render_message_block(state, view_state)}
        </fieldset>
    }
}
