pub use ui_state_primitives::fieldset::{
    DEFAULT_ARIA_LABEL, DEFAULT_ERROR_MESSAGE, FieldsetOrientation, FieldsetState,
    FieldsetStateInput, FieldsetTone, normalize_aria_label, normalize_error_message,
    normalize_optional_text, resolve_state,
};

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
        state_attr: state.data_state_attr,
        source_attr: state.class_source_attr,
        stream_support_attr: "unsupported",
        stream_fallback_attr: "snapshot",
        stream_mode_attr: "snapshot",
        output_status_attr: "verified",
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: FieldsetState) -> String {
    let mut classes = vec![
        "ui-fieldset".to_string(),
        state.orientation_class.into(),
        state.tone_class.into(),
    ];

    if state.is_required {
        classes.push("ui-fieldset--required".to_string());
    }

    if state.is_disabled {
        classes.push("ui-fieldset--disabled".to_string());
    }

    if state.is_invalid {
        classes.push("ui-fieldset--invalid".to_string());
    }

    if state.has_legend {
        classes.push("ui-fieldset--has-legend".to_string());
    }

    if state.has_description {
        classes.push("ui-fieldset--has-description".to_string());
    }

    if state.has_error_message {
        classes.push("ui-fieldset--has-error".to_string());
    }

    if state.has_actions {
        classes.push("ui-fieldset--has-actions".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-fieldset--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
