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
#[path = "../../test/text_field/logic.rs"]
mod tests;
