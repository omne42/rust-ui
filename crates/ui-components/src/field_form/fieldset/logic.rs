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
mod tests {
    use super::*;

    #[test]
    fn compose_class_name_appends_state_and_custom_class() {
        let state = resolve_state(FieldsetStateInput {
            orientation: FieldsetOrientation::Vertical,
            tone: FieldsetTone::Default,
            required: true,
            disabled: true,
            invalid: false,
            has_legend: true,
            has_description: true,
            has_error_message: false,
            has_actions: true,
            has_custom_aria_label: false,
            has_custom_error_message: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-fieldset-custom".to_string()), state);

        for expected in [
            "ui-fieldset",
            "ui-fieldset--orientation-vertical",
            "ui-fieldset--tone-default",
            "ui-fieldset--required",
            "ui-fieldset--disabled",
            "ui-fieldset--has-legend",
            "ui-fieldset--has-description",
            "ui-fieldset--has-actions",
            "ui-fieldset--custom-class",
            "docs-fieldset-custom",
        ] {
            assert!(
                class_name.contains(expected),
                "expected class `{expected}` in `{class_name}`"
            );
        }
    }

    #[test]
    fn resolve_agent_contract_emits_machine_readable_markers() {
        let state = resolve_state(FieldsetStateInput {
            orientation: FieldsetOrientation::Horizontal,
            tone: FieldsetTone::Muted,
            required: false,
            disabled: false,
            invalid: true,
            has_legend: true,
            has_description: false,
            has_error_message: true,
            has_actions: false,
            has_custom_aria_label: true,
            has_custom_error_message: true,
            has_custom_class_name: true,
        });
        let contract = resolve_agent_contract(state);

        assert_eq!(contract.schema_attr, "ui.fieldset.agent-contract");
        assert_eq!(contract.schema_version_attr, "1");
        assert_eq!(contract.intent_attr, "form-grouping");
        assert_eq!(contract.action_attr, "initialize");
        assert_eq!(contract.state_attr, "invalid");
        assert_eq!(contract.source_attr, "custom");
        assert_eq!(contract.stream_support_attr, "unsupported");
        assert_eq!(contract.stream_fallback_attr, "snapshot");
        assert_eq!(contract.stream_mode_attr, "snapshot");
        assert_eq!(contract.output_status_attr, "verified");
    }
}
