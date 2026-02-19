pub use ui_state_primitives::button::normalize_optional_text;
pub use ui_state_primitives::textarea::{
    TextareaSourceAttr, TextareaState, TextareaStateInput, resolve_label_with_fallback,
    resolve_state,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueControlModeAttr {
    Controlled,
    Uncontrolled,
}

impl ValueControlModeAttr {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueChangeSourceAttr {
    OnValueChange,
    None,
}

impl ValueChangeSourceAttr {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnValueChange => "on_value_change",
            Self::None => "none",
        }
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
    pub control_mode_attr: ValueControlModeAttr,
    pub default_value_source_attr: TextareaSourceAttr,
    pub value_change_source_attr: ValueChangeSourceAttr,
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

    let control_mode_attr = if is_controlled {
        ValueControlModeAttr::Controlled
    } else {
        ValueControlModeAttr::Uncontrolled
    };
    let default_value_source_attr = if has_default_value {
        TextareaSourceAttr::Custom
    } else {
        TextareaSourceAttr::Default
    };
    let value_change_source_attr = if has_on_value_change {
        ValueChangeSourceAttr::OnValueChange
    } else {
        ValueChangeSourceAttr::None
    };
    let has_value_change_handler = has_on_value_change;

    ValueAxisState {
        default_value,
        is_controlled,
        control_mode_attr,
        default_value_source_attr,
        value_change_source_attr,
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

pub fn compose_class_name(class_name: Option<String>, state: TextareaState) -> String {
    let mut classes = vec![
        "ui-textarea".to_string(),
        format!("ui-textarea--state-{}", state.state_attr.as_str()),
        format!("ui-textarea--value-{}", state.value_attr.as_str()),
        format!(
            "ui-textarea--requirement-{}",
            state.requirement_attr.as_str()
        ),
    ];

    if state.has_custom_class_name {
        classes.push("ui-textarea--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn normalize_default_value_uses_empty_string_when_absent() {
        assert_eq!(normalize_default_value(None), String::new());
        assert_eq!(
            normalize_default_value(Some("prefilled".to_string())),
            "prefilled".to_string()
        );
    }

    #[test]
    fn normalize_value_axis_centralizes_default_priority_and_sources() {
        let state = normalize_value_axis(ValueAxisInput {
            has_controlled_value: true,
            default_value: Some("default".to_string()),
            has_on_value_change: false,
        });

        assert!(state.is_controlled);
        assert_eq!(state.control_mode_attr, ValueControlModeAttr::Controlled);
        assert_eq!(state.default_value, "default");
        assert_eq!(state.default_value_source_attr, TextareaSourceAttr::Custom);
        assert_eq!(state.value_change_source_attr, ValueChangeSourceAttr::None);
        assert!(!state.has_value_change_handler);
    }

    #[test]
    fn normalize_value_axis_tracks_on_value_change_source() {
        let state = normalize_value_axis(ValueAxisInput {
            has_controlled_value: false,
            default_value: None,
            has_on_value_change: true,
        });

        assert_eq!(state.control_mode_attr, ValueControlModeAttr::Uncontrolled);
        assert_eq!(state.default_value_source_attr, TextareaSourceAttr::Default);
        assert_eq!(
            state.value_change_source_attr,
            ValueChangeSourceAttr::OnValueChange
        );
        assert!(state.has_value_change_handler);
    }

    #[test]
    fn normalize_value_axis_uses_closed_enumerated_source_markers() {
        for has_controlled_value in [false, true] {
            for has_default_value in [false, true] {
                for has_on_value_change in [false, true] {
                    let default_value = has_default_value.then(|| "default-value".to_string());
                    let state = normalize_value_axis(ValueAxisInput {
                        has_controlled_value,
                        default_value,
                        has_on_value_change,
                    });

                    assert!(
                        matches!(
                            state.control_mode_attr,
                            ValueControlModeAttr::Controlled | ValueControlModeAttr::Uncontrolled
                        ),
                        "unexpected `data-value-control-mode` value: {}",
                        state.control_mode_attr.as_str()
                    );
                    assert!(
                        matches!(
                            state.default_value_source_attr,
                            TextareaSourceAttr::Custom | TextareaSourceAttr::Default
                        ),
                        "unexpected `data-default-value-source` value: {}",
                        state.default_value_source_attr.as_str()
                    );
                    assert!(
                        matches!(
                            state.value_change_source_attr,
                            ValueChangeSourceAttr::OnValueChange | ValueChangeSourceAttr::None
                        ),
                        "unexpected `data-value-change-source` value: {}",
                        state.value_change_source_attr.as_str()
                    );
                }
            }
        }
    }

    #[test]
    fn resolve_label_with_fallback_uses_default_for_blank_values() {
        assert_eq!(
            resolve_label_with_fallback(
                "  ".to_string(),
                ui_state_primitives::textarea::DEFAULT_LABEL
            ),
            (ui_state_primitives::textarea::DEFAULT_LABEL.into(), false)
        );
        assert_eq!(
            resolve_label_with_fallback(
                "  Release summary  ".to_string(),
                ui_state_primitives::textarea::DEFAULT_LABEL,
            ),
            ("Release summary".to_string(), true)
        );
    }

    #[test]
    fn resolve_state_tracks_sources_and_rows_markers() {
        let state = resolve_state(TextareaStateInput {
            disabled: false,
            read_only: true,
            required: true,
            invalid: false,
            has_value: true,
            has_custom_label: true,
            has_custom_description: true,
            has_custom_error: false,
            has_custom_placeholder: true,
            has_custom_rows: true,
            has_custom_class_name: false,
        });

        assert_eq!(
            state.state_attr,
            ui_state_primitives::textarea::TextareaVisualStateAttr::Readonly
        );
        assert_eq!(
            state.value_attr,
            ui_state_primitives::textarea::TextareaValueAttr::Filled
        );
        assert_eq!(
            state.requirement_attr,
            ui_state_primitives::textarea::TextareaRequirementAttr::Required
        );
        assert_eq!(state.label_source_attr, TextareaSourceAttr::Custom);
        assert_eq!(state.description_source_attr, TextareaSourceAttr::Custom);
        assert_eq!(state.error_source_attr, TextareaSourceAttr::Default);
        assert_eq!(state.placeholder_source_attr, TextareaSourceAttr::Custom);
        assert_eq!(state.rows_source_attr, TextareaSourceAttr::Custom);
        assert_eq!(state.class_source_attr, TextareaSourceAttr::Default);
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let state = resolve_state(TextareaStateInput {
            disabled: true,
            read_only: false,
            required: false,
            invalid: false,
            has_value: false,
            has_custom_label: false,
            has_custom_description: false,
            has_custom_error: false,
            has_custom_placeholder: false,
            has_custom_rows: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-textarea".to_string()), state);

        for token in [
            "ui-textarea",
            "ui-textarea--state-disabled",
            "ui-textarea--value-empty",
            "ui-textarea--requirement-optional",
            "ui-textarea--custom-class",
            "docs-textarea",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
