use crate::button::logic::normalize_optional_text;
use crate::button::{ButtonColor, ButtonType, ButtonVariant};
use leptos::prelude::Callback;
use ui_headless::OnPress;
use ui_state_primitives::button::{ButtonStateCoreInput, resolve_state_core};

pub const DEFAULT_ARIA_LABEL: &str = "FieldButton";

pub struct FieldButtonResolveInput {
    pub is_quiet: bool,
    pub is_invalid: bool,
    pub is_disabled: bool,
    pub is_active: bool,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
    pub button_type: ButtonType,
    pub on_press: Option<OnPress>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldButtonTone {
    Default,
    Quiet,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldButtonValidation {
    Default,
    Invalid,
}

pub struct FieldButtonNormalizedInput {
    pub tone: FieldButtonTone,
    pub validation: FieldButtonValidation,
    pub is_disabled: bool,
    pub is_active: bool,
    pub has_custom_aria_label: bool,
    pub aria_label: Option<String>,
    pub has_custom_class_name: bool,
    pub class_name: Option<String>,
    pub has_custom_press_handler: bool,
    pub on_press: Option<OnPress>,
    pub button_type: ButtonType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldButtonStateInput {
    pub tone: FieldButtonTone,
    pub validation: FieldButtonValidation,
    pub is_disabled: bool,
    pub is_active: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_press_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldButtonState {
    pub variant: ButtonVariant,
    pub color: ButtonColor,
    pub tone: FieldButtonTone,
    pub validation: FieldButtonValidation,
    pub is_disabled: bool,
    pub is_active: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_press_handler: bool,
}

pub struct FieldButtonResolved {
    pub variant: ButtonVariant,
    pub color: ButtonColor,
    pub is_disabled: bool,
    pub class_name: String,
    pub button_type: ButtonType,
    pub aria_label: String,
    pub on_press: OnPress,
}

pub fn normalize_input(input: FieldButtonResolveInput) -> FieldButtonNormalizedInput {
    let normalized_aria_label = normalize_optional_text(input.aria_label);
    let normalized_class_name = normalize_optional_text(input.class_name);
    let has_custom_aria_label = normalized_aria_label.is_some();
    let has_custom_class_name = normalized_class_name.is_some();
    let has_custom_press_handler = input.on_press.is_some();

    FieldButtonNormalizedInput {
        tone: if input.is_quiet {
            FieldButtonTone::Quiet
        } else {
            FieldButtonTone::Default
        },
        validation: if input.is_invalid {
            FieldButtonValidation::Invalid
        } else {
            FieldButtonValidation::Default
        },
        is_disabled: input.is_disabled,
        is_active: input.is_active,
        has_custom_aria_label,
        aria_label: normalized_aria_label,
        has_custom_class_name,
        class_name: normalized_class_name,
        has_custom_press_handler,
        on_press: input.on_press,
        button_type: input.button_type,
    }
}

pub fn resolve_state(input: FieldButtonStateInput) -> FieldButtonState {
    let core = resolve_state_core(ButtonStateCoreInput {
        is_disabled: input.is_disabled,
        is_loading: false,
        is_icon_only: false,
        is_full_width: false,
        has_start_content: false,
        has_end_content: false,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: false,
    });
    let variant = match input.tone {
        FieldButtonTone::Default => ButtonVariant::Default,
        FieldButtonTone::Quiet => ButtonVariant::Ghost,
    };
    let color = match input.validation {
        FieldButtonValidation::Default => ButtonColor::Default,
        FieldButtonValidation::Invalid => ButtonColor::Danger,
    };

    FieldButtonState {
        variant,
        color,
        tone: input.tone,
        validation: input.validation,
        is_disabled: core.is_disabled,
        is_active: input.is_active,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_press_handler: input.has_custom_press_handler,
    }
}

pub fn compose_class_name(state: FieldButtonState, custom_class_name: Option<&str>) -> String {
    let mut classes = vec!["ui-field-button".to_string()];
    if matches!(state.tone, FieldButtonTone::Quiet) {
        classes.push("ui-field-button--quiet".to_string());
    }
    if matches!(state.validation, FieldButtonValidation::Invalid) {
        classes.push("ui-field-button--invalid".to_string());
    }
    if state.is_active {
        classes.push("ui-field-button--active".to_string());
        classes.push("is-active".to_string());
    }
    if state.is_disabled {
        classes.push("ui-field-button--disabled".to_string());
    }
    if state.has_custom_press_handler {
        classes.push("ui-field-button--custom-handler".to_string());
    }
    if state.has_custom_aria_label {
        classes.push("ui-field-button--custom-aria-label".to_string());
    }
    if state.has_custom_class_name {
        classes.push("ui-field-button--custom-class".to_string());
    }
    if let Some(class_name) = custom_class_name {
        classes.push(class_name.to_string());
    }

    classes.join(" ")
}

pub fn resolve_props(input: FieldButtonResolveInput) -> FieldButtonResolved {
    let normalized = normalize_input(input);
    let state = resolve_state(FieldButtonStateInput {
        tone: normalized.tone,
        validation: normalized.validation,
        is_disabled: normalized.is_disabled,
        is_active: normalized.is_active,
        has_custom_aria_label: normalized.has_custom_aria_label,
        has_custom_class_name: normalized.has_custom_class_name,
        has_custom_press_handler: normalized.has_custom_press_handler,
    });

    FieldButtonResolved {
        variant: state.variant,
        color: state.color,
        is_disabled: state.is_disabled,
        class_name: compose_class_name(state, normalized.class_name.as_deref()),
        button_type: normalized.button_type,
        aria_label: normalized
            .aria_label
            .unwrap_or_else(|| DEFAULT_ARIA_LABEL.to_string()),
        on_press: normalized.on_press.unwrap_or_else(|| Callback::new(|_| {})),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resolve(input: FieldButtonResolveInput) -> FieldButtonResolved {
        resolve_props(input)
    }

    #[test]
    fn resolve_props_defaults_to_field_button_aria_label() {
        let state = resolve(FieldButtonResolveInput {
            is_quiet: false,
            is_invalid: false,
            is_disabled: false,
            is_active: false,
            aria_label: None,
            class_name: None,
            button_type: ButtonType::Button,
            on_press: None,
        });

        assert_eq!(state.aria_label, DEFAULT_ARIA_LABEL);
    }

    #[test]
    fn resolve_props_maps_quiet_and_invalid_to_button_tokens() {
        let state = resolve(FieldButtonResolveInput {
            is_quiet: true,
            is_invalid: true,
            is_disabled: false,
            is_active: false,
            aria_label: None,
            class_name: None,
            button_type: ButtonType::Button,
            on_press: None,
        });

        assert_eq!(state.variant, ButtonVariant::Ghost);
        assert_eq!(state.color, ButtonColor::Danger);
    }

    #[test]
    fn resolve_state_centralizes_quiet_invalid_and_active_markers() {
        let state = resolve_state(FieldButtonStateInput {
            tone: FieldButtonTone::Quiet,
            validation: FieldButtonValidation::Invalid,
            is_disabled: false,
            is_active: true,
            has_custom_aria_label: false,
            has_custom_class_name: false,
            has_custom_press_handler: false,
        });

        assert_eq!(state.variant, ButtonVariant::Ghost);
        assert_eq!(state.color, ButtonColor::Danger);
        assert!(state.is_active);
    }

    #[test]
    fn resolve_state_consumes_button_state_primitive_for_disabled_contract() {
        let state = resolve_state(FieldButtonStateInput {
            tone: FieldButtonTone::Default,
            validation: FieldButtonValidation::Default,
            is_disabled: true,
            is_active: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
            has_custom_press_handler: false,
        });

        assert!(state.is_disabled);
    }

    #[test]
    fn resolve_props_marks_custom_class_and_handler_sources() {
        let state = resolve(FieldButtonResolveInput {
            is_quiet: false,
            is_invalid: false,
            is_disabled: false,
            is_active: false,
            aria_label: Some("Open".to_string()),
            class_name: Some("docs-field-button-custom".to_string()),
            button_type: ButtonType::Button,
            on_press: Some(Callback::new(|_| {})),
        });

        assert!(state.class_name.contains("ui-field-button--custom-class"));
        assert!(state.class_name.contains("ui-field-button--custom-handler"));
        assert!(
            state
                .class_name
                .contains("ui-field-button--custom-aria-label")
        );
        assert!(state.class_name.contains("docs-field-button-custom"));
    }

    #[test]
    fn resolve_props_adds_active_marker_when_forced_active() {
        let state = resolve(FieldButtonResolveInput {
            is_quiet: false,
            is_invalid: false,
            is_disabled: false,
            is_active: true,
            aria_label: None,
            class_name: None,
            button_type: ButtonType::Button,
            on_press: None,
        });

        assert!(state.class_name.contains("ui-field-button--active"));
        assert!(state.class_name.contains("is-active"));
    }

    #[test]
    fn compose_class_name_uses_state_only_and_appends_custom_class() {
        let class_name = compose_class_name(
            FieldButtonState {
                variant: ButtonVariant::Default,
                color: ButtonColor::Default,
                tone: FieldButtonTone::Quiet,
                validation: FieldButtonValidation::Default,
                is_disabled: true,
                is_active: false,
                has_custom_aria_label: false,
                has_custom_class_name: true,
                has_custom_press_handler: false,
            },
            Some("docs-field-button-custom"),
        );

        assert!(class_name.contains("ui-field-button--quiet"));
        assert!(class_name.contains("ui-field-button--disabled"));
        assert!(class_name.contains("ui-field-button--custom-class"));
        assert!(class_name.contains("docs-field-button-custom"));
    }
}
