pub use ui_state_primitives::text_field::DEFAULT_LABEL;

use leptos::prelude::*;
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
    SetValue,
    None,
}

impl ValueChangeSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::OnValueChange => "on_value_change",
            Self::SetValue => "set_value",
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
    pub value: Option<Signal<String>>,
    pub default_value: Option<String>,
    pub on_value_change: Option<Callback<String>>,
    pub set_value: Option<WriteSignal<String>>,
}

pub struct ValueAxisState {
    pub value: Option<Signal<String>>,
    pub default_value: String,
    pub on_value_change: Option<Callback<String>>,
    pub is_controlled: bool,
    pub control_mode_attr: &'static str,
    pub default_value_source_attr: &'static str,
    pub value_change_source_attr: &'static str,
    pub has_value_change_handler: bool,
}

pub fn normalize_default_value(default_value: Option<String>) -> String {
    default_value.unwrap_or_default()
}

pub fn normalize_on_value_change_handler(
    on_value_change: Option<Callback<String>>,
    set_value: Option<WriteSignal<String>>,
) -> Option<Callback<String>> {
    on_value_change
        .or_else(|| set_value.map(|set_value| Callback::new(move |next| set_value.set(next))))
}

pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState {
    let is_controlled = input.value.is_some();
    let has_default_value = input.default_value.is_some();
    let has_on_value_change = input.on_value_change.is_some();
    let has_legacy_set_value = input.set_value.is_some();
    let default_value = normalize_default_value(input.default_value);
    let on_value_change = normalize_on_value_change_handler(input.on_value_change, input.set_value);

    let control_mode = if is_controlled {
        ValueControlMode::Controlled
    } else {
        ValueControlMode::Uncontrolled
    };
    let value_change_source = if has_on_value_change {
        ValueChangeSource::OnValueChange
    } else if has_legacy_set_value {
        ValueChangeSource::SetValue
    } else {
        ValueChangeSource::None
    };
    let default_value_source_attr = source_attr_from_presence(has_default_value);
    let has_value_change_handler = has_on_value_change || has_legacy_set_value;

    ValueAxisState {
        value: input.value,
        default_value,
        on_value_change,
        is_controlled,
        control_mode_attr: control_mode.as_attr(),
        default_value_source_attr,
        value_change_source_attr: value_change_source.as_attr(),
        has_value_change_handler,
    }
}

pub struct AccessibilityStateInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
    pub is_read_only: Option<bool>,
    pub read_only: bool,
    pub is_required: Option<Signal<bool>>,
    pub required: Option<Signal<bool>>,
    pub is_invalid: Option<Signal<bool>>,
    pub invalid: Option<Signal<bool>>,
}

pub struct AccessibilityState {
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub is_required: Signal<bool>,
    pub is_invalid: Signal<bool>,
}

pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState {
    let is_required = input
        .is_required
        .or(input.required)
        .unwrap_or_else(|| Signal::derive(|| false));
    let is_invalid = input
        .is_invalid
        .or(input.invalid)
        .unwrap_or_else(|| Signal::derive(|| false));

    AccessibilityState {
        is_disabled: input.is_disabled.unwrap_or(input.disabled),
        is_read_only: input.is_read_only.unwrap_or(input.read_only),
        is_required,
        is_invalid,
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
    fn normalize_on_value_change_handler_prefers_on_value_change() {
        let (from_on_value_change, set_from_on_value_change) = signal(String::new());
        let (_legacy_value, set_legacy_value) = signal(String::new());
        let on_value_change = Callback::new(move |next: String| set_from_on_value_change.set(next));

        let handler =
            normalize_on_value_change_handler(Some(on_value_change), Some(set_legacy_value))
                .expect("handler should exist");
        handler.run("new-value".to_string());

        assert_eq!(
            from_on_value_change.get_untracked(),
            "new-value",
            "on_value_change should have priority over legacy set_value"
        );
    }

    #[test]
    fn normalize_on_value_change_handler_falls_back_to_set_value_alias() {
        let (legacy_value, set_legacy_value) = signal(String::new());
        let handler = normalize_on_value_change_handler(None, Some(set_legacy_value))
            .expect("legacy set_value should map to normalized on_value_change");

        handler.run("legacy".to_string());
        assert_eq!(legacy_value.get_untracked(), "legacy");
    }

    #[test]
    fn normalize_value_axis_tracks_mode_and_source_markers() {
        let (value, _set_value) = signal("controlled".to_string());
        let state = normalize_value_axis(ValueAxisInput {
            value: Some(value.into()),
            default_value: Some("fallback".to_string()),
            on_value_change: None,
            set_value: None,
        });

        assert!(state.is_controlled);
        assert_eq!(state.control_mode_attr, "controlled");
        assert_eq!(state.default_value, "fallback");
        assert_eq!(state.default_value_source_attr, "custom");
        assert_eq!(state.value_change_source_attr, "none");
        assert!(!state.has_value_change_handler);
    }

    #[test]
    fn normalize_value_axis_prefers_on_value_change_over_set_value_alias() {
        let (from_on_value_change, set_from_on_value_change) = signal(String::new());
        let (_legacy_value, set_legacy_value) = signal(String::new());
        let on_value_change = Callback::new(move |next: String| set_from_on_value_change.set(next));

        let state = normalize_value_axis(ValueAxisInput {
            value: None,
            default_value: None,
            on_value_change: Some(on_value_change),
            set_value: Some(set_legacy_value),
        });
        let handler = state
            .on_value_change
            .expect("value axis should keep normalized callback");
        handler.run("next".to_string());

        assert_eq!(state.control_mode_attr, "uncontrolled");
        assert_eq!(state.default_value_source_attr, "default");
        assert_eq!(state.value_change_source_attr, "on_value_change");
        assert!(state.has_value_change_handler);
        assert_eq!(from_on_value_change.get_untracked(), "next");
    }

    #[test]
    fn value_axis_enum_attrs_are_closed_machine_readable_values() {
        assert_eq!(ValueControlMode::Controlled.as_attr(), "controlled");
        assert_eq!(ValueControlMode::Uncontrolled.as_attr(), "uncontrolled");
        assert_eq!(
            ValueChangeSource::OnValueChange.as_attr(),
            "on_value_change"
        );
        assert_eq!(ValueChangeSource::SetValue.as_attr(), "set_value");
        assert_eq!(ValueChangeSource::None.as_attr(), "none");
    }

    #[test]
    fn normalize_accessibility_state_prefers_is_prefixed_inputs() {
        let (preferred_required, _set_preferred_required) = signal(true);
        let (legacy_required, _set_legacy_required) = signal(false);
        let (preferred_invalid, _set_preferred_invalid) = signal(true);
        let (legacy_invalid, _set_legacy_invalid) = signal(false);

        let state = normalize_accessibility_state(AccessibilityStateInput {
            is_disabled: Some(true),
            disabled: false,
            is_read_only: Some(true),
            read_only: false,
            is_required: Some(preferred_required.into()),
            required: Some(legacy_required.into()),
            is_invalid: Some(preferred_invalid.into()),
            invalid: Some(legacy_invalid.into()),
        });

        assert!(state.is_disabled);
        assert!(state.is_read_only);
        assert!(state.is_required.get_untracked());
        assert!(state.is_invalid.get_untracked());
    }

    #[test]
    fn normalize_accessibility_state_falls_back_to_legacy_aliases() {
        let state = normalize_accessibility_state(AccessibilityStateInput {
            is_disabled: None,
            disabled: true,
            is_read_only: None,
            read_only: true,
            is_required: None,
            required: None,
            is_invalid: None,
            invalid: None,
        });

        assert!(state.is_disabled);
        assert!(state.is_read_only);
        assert!(!state.is_required.get_untracked());
        assert!(!state.is_invalid.get_untracked());
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
