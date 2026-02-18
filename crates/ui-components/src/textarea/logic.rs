use leptos::prelude::*;

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
    SetValue,
    None,
}

impl ValueChangeSourceAttr {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnValueChange => "on_value_change",
            Self::SetValue => "set_value",
            Self::None => "none",
        }
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
    pub control_mode_attr: ValueControlModeAttr,
    pub default_value_source_attr: TextareaSourceAttr,
    pub value_change_source_attr: ValueChangeSourceAttr,
    pub has_value_change_handler: bool,
}

pub fn normalize_default_value(default_value: Option<String>) -> String {
    default_value.unwrap_or_default()
}

pub fn normalize_on_value_change_handler(
    on_value_change: Option<Callback<String>>,
    set_value: Option<WriteSignal<String>>,
) -> Option<Callback<String>> {
    on_value_change.or_else(|| {
        set_value.map(|set_value| Callback::new(move |next: String| set_value.set(next)))
    })
}

pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState {
    let is_controlled = input.value.is_some();
    let has_default_value = input.default_value.is_some();
    let has_on_value_change = input.on_value_change.is_some();
    let has_legacy_set_value = input.set_value.is_some();
    let default_value = normalize_default_value(input.default_value);
    let on_value_change = normalize_on_value_change_handler(input.on_value_change, input.set_value);

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
    } else if has_legacy_set_value {
        ValueChangeSourceAttr::SetValue
    } else {
        ValueChangeSourceAttr::None
    };
    let has_value_change_handler = has_on_value_change || has_legacy_set_value;

    ValueAxisState {
        value: input.value,
        default_value,
        on_value_change,
        is_controlled,
        control_mode_attr,
        default_value_source_attr,
        value_change_source_attr,
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
            .expect("legacy set_value should be normalized to on_value_change callback");

        handler.run("legacy-updated".to_string());

        assert_eq!(legacy_value.get_untracked(), "legacy-updated");
    }

    #[test]
    fn normalize_value_axis_centralizes_default_priority_and_sources() {
        let (value, _set_value) = signal("controlled".to_string());
        let state = normalize_value_axis(ValueAxisInput {
            value: Some(value.into()),
            default_value: Some("default".to_string()),
            on_value_change: None,
            set_value: None,
        });

        assert!(state.is_controlled);
        assert_eq!(state.control_mode_attr, ValueControlModeAttr::Controlled);
        assert_eq!(state.default_value, "default");
        assert_eq!(state.default_value_source_attr, TextareaSourceAttr::Custom);
        assert_eq!(state.value_change_source_attr, ValueChangeSourceAttr::None);
        assert!(!state.has_value_change_handler);
    }

    #[test]
    fn normalize_value_axis_prefers_on_value_change_over_set_value_alias() {
        let (on_value_change_value, set_on_value_change_value) = signal(String::new());
        let (_legacy_value, set_legacy_value) = signal(String::new());
        let on_value_change =
            Callback::new(move |next: String| set_on_value_change_value.set(next));
        let state = normalize_value_axis(ValueAxisInput {
            value: None,
            default_value: None,
            on_value_change: Some(on_value_change),
            set_value: Some(set_legacy_value),
        });

        let handler = state
            .on_value_change
            .expect("value axis should keep normalized callback");
        handler.run("prioritized".to_string());

        assert_eq!(state.control_mode_attr, ValueControlModeAttr::Uncontrolled);
        assert_eq!(state.default_value_source_attr, TextareaSourceAttr::Default);
        assert_eq!(
            state.value_change_source_attr,
            ValueChangeSourceAttr::OnValueChange
        );
        assert!(state.has_value_change_handler);
        assert_eq!(on_value_change_value.get_untracked(), "prioritized");
    }

    #[test]
    fn normalize_value_axis_uses_closed_enumerated_source_markers() {
        for has_value in [false, true] {
            for has_default_value in [false, true] {
                for has_on_value_change in [false, true] {
                    for has_set_value in [false, true] {
                        let value = if has_value {
                            let (value, _set_value) = signal("controlled".to_string());
                            Some(value.into())
                        } else {
                            None
                        };
                        let default_value = has_default_value.then(|| "default-value".to_string());
                        let on_value_change = if has_on_value_change {
                            Some(Callback::new(|_next: String| {}))
                        } else {
                            None
                        };
                        let set_value = if has_set_value {
                            let (_legacy_value, set_legacy_value) = signal(String::new());
                            Some(set_legacy_value)
                        } else {
                            None
                        };
                        let state = normalize_value_axis(ValueAxisInput {
                            value,
                            default_value,
                            on_value_change,
                            set_value,
                        });

                        assert!(
                            matches!(
                                state.control_mode_attr,
                                ValueControlModeAttr::Controlled
                                    | ValueControlModeAttr::Uncontrolled
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
                                ValueChangeSourceAttr::OnValueChange
                                    | ValueChangeSourceAttr::SetValue
                                    | ValueChangeSourceAttr::None
                            ),
                            "unexpected `data-value-change-source` value: {}",
                            state.value_change_source_attr.as_str()
                        );
                    }
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
            (
                ui_state_primitives::textarea::DEFAULT_LABEL.to_string(),
                false
            )
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
