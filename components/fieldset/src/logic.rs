use std::borrow::Cow;

pub use ui_state_primitives::fieldset::{
    DEFAULT_ARIA_LABEL, DEFAULT_ERROR_MESSAGE, FieldsetBooleanAxisInput, FieldsetDataState,
    FieldsetMessageKind, FieldsetOrientation, FieldsetState, FieldsetStateInput, FieldsetTone,
    normalize_aria_label, normalize_boolean_axis, normalize_error_message, normalize_optional_text,
    resolve_state,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldsetViewStateInput {
    pub orientation: FieldsetOrientation,
    pub tone: FieldsetTone,
    pub required: bool,
    pub required_source_attr: &'static str,
    pub required_control_mode_attr: &'static str,
    pub required_change_source_attr: &'static str,
    pub disabled: bool,
    pub disabled_source_attr: &'static str,
    pub disabled_control_mode_attr: &'static str,
    pub disabled_change_source_attr: &'static str,
    pub invalid: bool,
    pub invalid_source_attr: &'static str,
    pub invalid_control_mode_attr: &'static str,
    pub invalid_change_source_attr: &'static str,
    pub legend: Option<String>,
    pub description: Option<String>,
    pub error_message: Option<String>,
    pub class_name: Option<String>,
    pub has_actions: bool,
    pub has_custom_aria_label: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldsetViewState {
    pub state: FieldsetState,
    pub legend: Option<String>,
    pub description: Option<String>,
    pub error_message: Option<String>,
    pub class_name: Option<String>,
    pub required_source_attr: &'static str,
    pub required_control_mode_attr: &'static str,
    pub required_change_source_attr: &'static str,
    pub disabled_source_attr: &'static str,
    pub disabled_control_mode_attr: &'static str,
    pub disabled_change_source_attr: &'static str,
    pub invalid_source_attr: &'static str,
    pub invalid_control_mode_attr: &'static str,
    pub invalid_change_source_attr: &'static str,
}

pub fn resolve_view_state(input: FieldsetViewStateInput) -> FieldsetViewState {
    let legend = normalize_optional_text(input.legend);
    let description = normalize_optional_text(input.description);
    let (error_message, has_custom_error_message) =
        normalize_error_message(input.error_message, input.invalid);
    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    let state = resolve_state(FieldsetStateInput {
        orientation: input.orientation,
        tone: input.tone,
        required: input.required,
        disabled: input.disabled,
        invalid: input.invalid,
        has_legend: legend.is_some(),
        has_description: description.is_some(),
        has_error_message: error_message.is_some(),
        has_actions: input.has_actions,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_error_message,
        has_custom_class_name,
    });

    FieldsetViewState {
        state,
        legend,
        description,
        error_message,
        class_name,
        required_source_attr: input.required_source_attr,
        required_control_mode_attr: input.required_control_mode_attr,
        required_change_source_attr: input.required_change_source_attr,
        disabled_source_attr: input.disabled_source_attr,
        disabled_control_mode_attr: input.disabled_control_mode_attr,
        disabled_change_source_attr: input.disabled_change_source_attr,
        invalid_source_attr: input.invalid_source_attr,
        invalid_control_mode_attr: input.invalid_control_mode_attr,
        invalid_change_source_attr: input.invalid_change_source_attr,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldsetAgentContract {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
    pub state_attr: &'static str,
    pub source_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub stream_mode_attr: &'static str,
    pub output_status_attr: &'static str,
}

pub fn resolve_agent_contract(state: FieldsetState) -> FieldsetAgentContract {
    FieldsetAgentContract {
        schema_attr: "ui.fieldset.agent-contract",
        schema_version_attr: "1",
        intent_attr: "form-grouping",
        action_attr: "initialize",
        state_attr: state.data_state.as_attr(),
        source_attr: state.class_source_attr,
        stream_support_attr: "unsupported",
        stream_fallback_attr: "snapshot",
        stream_mode_attr: "snapshot",
        output_status_attr: "verified",
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: FieldsetState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-fieldset"),
        Cow::Borrowed(state.orientation_class),
        Cow::Borrowed(state.tone_class),
    ];

    if state.is_required {
        classes.push(Cow::Borrowed("ui-fieldset--required"));
    }

    if state.is_disabled {
        classes.push(Cow::Borrowed("ui-fieldset--disabled"));
    }

    if state.is_invalid {
        classes.push(Cow::Borrowed("ui-fieldset--invalid"));
    }

    if state.has_legend {
        classes.push(Cow::Borrowed("ui-fieldset--has-legend"));
    }

    if state.has_description {
        classes.push(Cow::Borrowed("ui-fieldset--has-description"));
    }

    if state.has_error_message {
        classes.push(Cow::Borrowed("ui-fieldset--has-error"));
    }

    if state.has_actions {
        classes.push(Cow::Borrowed("ui-fieldset--has-actions"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-fieldset--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .iter()
        .map(|class_name| class_name.as_ref())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
