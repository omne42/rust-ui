use crate::button::logic::{
    ButtonInputNormalizationInput, normalize_input as normalize_button_input,
};
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
    let shared_normalized = normalize_button_input(ButtonInputNormalizationInput {
        is_disabled: input.is_disabled,
        is_full_width: false,
        class_name: input.class_name.clone(),
        aria_label: input.aria_label.clone(),
        icon_only_fallback_aria_label: None,
        is_icon_only: false,
        button_type: input.button_type,
    });
    let has_custom_aria_label = shared_normalized.aria_label.is_some();
    let has_custom_class_name = shared_normalized.has_custom_class_name;
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
        is_disabled: shared_normalized.is_disabled,
        is_active: input.is_active,
        has_custom_aria_label,
        aria_label: shared_normalized.aria_label,
        has_custom_class_name,
        class_name: shared_normalized.class_name,
        has_custom_press_handler,
        on_press: input.on_press,
        button_type: shared_normalized.button_type,
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
    let mut classes: Vec<String> = vec!["ui-field-button".to_string()];
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
        classes.push(class_name.into());
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
#[path = "../../test/field/logic.rs"]
mod tests;
