pub use ui_state_primitives::text_field::DEFAULT_LABEL;

use ui_state_primitives::text_field::{
    normalize_optional_text, resolve_input_type, resolve_label, source_attr_from_presence,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueControlMode {
    Controlled,
    Uncontrolled,
}

impl ValueControlMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueChangeSource {
    OnValueChange,
    None,
}

impl ValueChangeSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::OnValueChange => "on_value_change",
            Self::None => "none",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextFieldAgentSchemaVersion {
    V1,
}

impl TextFieldAgentSchemaVersion {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextFieldAgentIntent {
    FormTextInput,
}

impl TextFieldAgentIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::FormTextInput => "form-text-input",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextFieldAgentActionModel {
    InputFocusBlurValidate,
}

impl TextFieldAgentActionModel {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::InputFocusBlurValidate => "input|focus|blur|validate",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextFieldAgentContract {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_model_attr: &'static str,
    pub state_axis_attr: &'static str,
    pub source_axis_attr: &'static str,
}

pub fn text_field_agent_contract() -> TextFieldAgentContract {
    TextFieldAgentContract {
        schema_attr: "ui.text-field",
        schema_version_attr: TextFieldAgentSchemaVersion::V1.as_attr(),
        intent_attr: TextFieldAgentIntent::FormTextInput.as_attr(),
        action_model_attr: TextFieldAgentActionModel::InputFocusBlurValidate.as_attr(),
        state_axis_attr: "state|value|requirement|disabled|readonly|focus-visible",
        source_axis_attr: "label|description|error|placeholder|type|class|motion|value-axis",
    }
}

pub struct ValueAxisInput {
    pub has_controlled_value: bool,
    pub default_value: Option<String>,
    pub has_on_value_change: bool,
}

pub struct ValueAxisState {
    pub default_value: String,
    pub is_controlled: bool,
    pub control_mode_attr: &'static str,
    pub default_value_source_attr: &'static str,
    pub value_change_source_attr: &'static str,
    pub has_value_change_handler: bool,
}

pub fn normalize_default_value(default_value: Option<String>) -> String {
    default_value.unwrap_or_default()
}

pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState {
    let is_controlled = input.has_controlled_value;
    let has_default_value = input.default_value.is_some();
    let has_on_value_change = input.has_on_value_change;
    let default_value = normalize_default_value(input.default_value);

    let control_mode = if is_controlled {
        ValueControlMode::Controlled
    } else {
        ValueControlMode::Uncontrolled
    };
    let value_change_source = if has_on_value_change {
        ValueChangeSource::OnValueChange
    } else {
        ValueChangeSource::None
    };
    let default_value_source_attr = source_attr_from_presence(has_default_value);
    let has_value_change_handler = has_on_value_change;

    ValueAxisState {
        default_value,
        is_controlled,
        control_mode_attr: control_mode.as_attr(),
        default_value_source_attr,
        value_change_source_attr: value_change_source.as_attr(),
        has_value_change_handler,
    }
}

pub struct AccessibilityStateInput {
    pub is_disabled: Option<bool>,
    pub is_read_only: Option<bool>,
}

pub struct AccessibilityState {
    pub is_disabled: bool,
    pub is_read_only: bool,
}

pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState {
    AccessibilityState {
        is_disabled: input.is_disabled.unwrap_or(false),
        is_read_only: input.is_read_only.unwrap_or(false),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextFieldResolvedProps {
    pub label: String,
    pub label_source_attr: &'static str,
    pub description: Option<String>,
    pub error: Option<String>,
    pub placeholder: Option<String>,
    pub input_type: &'static str,
    pub type_source_attr: &'static str,
    pub class: String,
    pub has_custom_class_name: bool,
    pub description_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn resolve_props(
    label: String,
    description: Option<String>,
    error: Option<String>,
    placeholder: Option<String>,
    input_type: Option<&'static str>,
    class_name: Option<String>,
) -> TextFieldResolvedProps {
    let (label, label_source_attr) = resolve_label(label);
    let description = normalize_optional_text(description);
    let error = normalize_optional_text(error);
    let placeholder = normalize_optional_text(placeholder);
    let class_name = normalize_optional_text(class_name);

    let has_custom_class_name = class_name.is_some();
    let class = compose_class_name(class_name);

    let (input_type, type_source_attr) = resolve_input_type(input_type);
    let description_source_attr = source_attr_from_presence(description.is_some());
    let error_source_attr = source_attr_from_presence(error.is_some());
    let placeholder_source_attr = source_attr_from_presence(placeholder.is_some());
    let class_source_attr = source_attr_from_presence(has_custom_class_name);

    TextFieldResolvedProps {
        label,
        label_source_attr,
        description,
        error,
        placeholder,
        input_type,
        type_source_attr,
        class,
        has_custom_class_name,
        description_source_attr,
        error_source_attr,
        placeholder_source_attr,
        class_source_attr,
    }
}

fn compose_class_name(class_name: Option<String>) -> String {
    let base_class = "ui-text-field".to_string();
    class_name
        .as_ref()
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_props_normalizes_text_and_tracks_sources() {
        let resolved = resolve_props(
            "  Name  ".to_string(),
            Some("  description  ".to_string()),
            Some("  error  ".to_string()),
            Some("  placeholder  ".to_string()),
            Some("email"),
            Some("  docs-class  ".to_string()),
        );

        assert_eq!(resolved.label, "Name");
        assert_eq!(resolved.label_source_attr, "custom");
        assert_eq!(resolved.description.as_deref(), Some("description"));
        assert_eq!(resolved.error.as_deref(), Some("error"));
        assert_eq!(resolved.placeholder.as_deref(), Some("placeholder"));
        assert_eq!(resolved.input_type, "email");
        assert_eq!(resolved.type_source_attr, "custom");
        assert!(resolved.has_custom_class_name);
        assert_eq!(resolved.class, "ui-text-field docs-class");
        assert_eq!(resolved.description_source_attr, "custom");
        assert_eq!(resolved.error_source_attr, "custom");
        assert_eq!(resolved.placeholder_source_attr, "custom");
        assert_eq!(resolved.class_source_attr, "custom");
    }

    #[test]
    fn resolve_props_applies_defaults_for_blank_inputs() {
        let resolved = resolve_props(
            "   ".to_string(),
            Some("   ".to_string()),
            Some("\n\t".to_string()),
            None,
            None,
            Some("   ".to_string()),
        );

        assert_eq!(resolved.label, DEFAULT_LABEL);
        assert_eq!(resolved.label_source_attr, "default");
        assert_eq!(resolved.description, None);
        assert_eq!(resolved.error, None);
        assert_eq!(resolved.placeholder, None);
        assert_eq!(resolved.input_type, "text");
        assert_eq!(resolved.type_source_attr, "default");
        assert!(!resolved.has_custom_class_name);
        assert_eq!(resolved.class, "ui-text-field");
        assert_eq!(resolved.description_source_attr, "default");
        assert_eq!(resolved.error_source_attr, "default");
        assert_eq!(resolved.placeholder_source_attr, "default");
        assert_eq!(resolved.class_source_attr, "default");
    }

    #[test]
    fn normalize_default_value_uses_empty_string_when_absent() {
        assert_eq!(normalize_default_value(None), String::new());
        assert_eq!(
            normalize_default_value(Some("prefilled".to_string())),
            "prefilled".to_string()
        );
    }

    #[test]
    fn normalize_value_axis_tracks_mode_and_source_markers() {
        let state = normalize_value_axis(ValueAxisInput {
            has_controlled_value: true,
            default_value: Some("fallback".to_string()),
            has_on_value_change: false,
        });

        assert!(state.is_controlled);
        assert_eq!(state.control_mode_attr, "controlled");
        assert_eq!(state.default_value, "fallback");
        assert_eq!(state.default_value_source_attr, "custom");
        assert_eq!(state.value_change_source_attr, "none");
        assert!(!state.has_value_change_handler);
    }

    #[test]
    fn normalize_value_axis_tracks_on_value_change_source() {
        let state = normalize_value_axis(ValueAxisInput {
            has_controlled_value: false,
            default_value: None,
            has_on_value_change: true,
        });

        assert_eq!(state.control_mode_attr, "uncontrolled");
        assert_eq!(state.default_value_source_attr, "default");
        assert_eq!(state.value_change_source_attr, "on_value_change");
        assert!(state.has_value_change_handler);
    }

    #[test]
    fn value_axis_enum_attrs_are_closed_machine_readable_values() {
        assert_eq!(ValueControlMode::Controlled.as_attr(), "controlled");
        assert_eq!(ValueControlMode::Uncontrolled.as_attr(), "uncontrolled");
        assert_eq!(
            ValueChangeSource::OnValueChange.as_attr(),
            "on_value_change"
        );
        assert_eq!(ValueChangeSource::None.as_attr(), "none");
    }

    #[test]
    fn normalize_accessibility_state_prefers_is_prefixed_inputs() {
        let state = normalize_accessibility_state(AccessibilityStateInput {
            is_disabled: Some(true),
            is_read_only: Some(true),
        });

        assert!(state.is_disabled);
        assert!(state.is_read_only);
    }

    #[test]
    fn normalize_accessibility_state_uses_defaults_when_values_are_absent() {
        let state = normalize_accessibility_state(AccessibilityStateInput {
            is_disabled: None,
            is_read_only: None,
        });

        assert!(!state.is_disabled);
        assert!(!state.is_read_only);
    }

    #[test]
    fn text_field_agent_contract_is_typed_and_stable() {
        let contract = text_field_agent_contract();

        assert_eq!(contract.schema_attr, "ui.text-field");
        assert_eq!(
            contract.schema_version_attr,
            TextFieldAgentSchemaVersion::V1.as_attr()
        );
        assert_eq!(
            contract.intent_attr,
            TextFieldAgentIntent::FormTextInput.as_attr()
        );
        assert_eq!(
            contract.action_model_attr,
            TextFieldAgentActionModel::InputFocusBlurValidate.as_attr()
        );
        assert_eq!(
            contract.state_axis_attr,
            "state|value|requirement|disabled|readonly|focus-visible"
        );
        assert_eq!(
            contract.source_axis_attr,
            "label|description|error|placeholder|type|class|motion|value-axis"
        );
    }
}
