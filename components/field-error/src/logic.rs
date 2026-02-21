use std::borrow::Cow;

use ui_state_primitives::error_message::{
    ErrorMessageState, ErrorMessageStateInput, ErrorMessageTone,
    resolve_state as resolve_error_message_state,
};
use ui_state_primitives::field_error::{
    FieldErrorState, FieldErrorStateInput, resolve_state as resolve_primitive_state,
};

pub use ui_state_primitives::field_error::FieldErrorTone;

pub const DEFAULT_ARIA_LABEL: &str = "FieldError";
pub const DEFAULT_MESSAGE: &str = "Invalid value";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldErrorLogicInput {
    pub tone: FieldErrorTone,
    pub is_visible: Option<bool>,
    pub visible: bool,
    pub is_disabled: Option<bool>,
    pub disabled: bool,
    pub is_icon_visible: Option<bool>,
    pub show_icon: bool,
    pub message: Option<String>,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
    pub default_message: Option<String>,
    pub default_aria_label: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldErrorControlInputs {
    pub visible: bool,
    pub disabled: bool,
    pub show_icon: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldErrorViewModel {
    pub state: FieldErrorState,
    pub message: Option<String>,
    pub aria_label: String,
    pub class_name: Option<String>,
    pub has_custom_message: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

pub fn normalize_control_inputs(
    is_visible: Option<bool>,
    visible: bool,
    is_disabled: Option<bool>,
    disabled: bool,
    is_icon_visible: Option<bool>,
    show_icon: bool,
) -> FieldErrorControlInputs {
    FieldErrorControlInputs {
        visible: is_visible.unwrap_or(visible),
        disabled: is_disabled.unwrap_or(disabled),
        show_icon: is_icon_visible.unwrap_or(show_icon),
    }
}

pub fn resolve_view_model(input: FieldErrorLogicInput) -> FieldErrorViewModel {
    let normalized_inputs = normalize_control_inputs(
        input.is_visible,
        input.visible,
        input.is_disabled,
        input.disabled,
        input.is_icon_visible,
        input.show_icon,
    );
    let default_aria_label = normalize_optional_text(input.default_aria_label)
        .unwrap_or_else(|| DEFAULT_ARIA_LABEL.into());
    let default_message =
        normalize_optional_text(input.default_message).unwrap_or_else(|| DEFAULT_MESSAGE.into());
    let (aria_label, has_custom_aria_label) =
        normalize_aria_label_with_default(input.aria_label, default_aria_label.as_str());
    let (message, has_custom_message) = normalize_message_with_default(
        input.message,
        normalized_inputs.visible,
        default_message.as_str(),
    );
    let has_message = message.is_some();
    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    let state = resolve_state(FieldErrorStateInput {
        tone: input.tone,
        visible: normalized_inputs.visible,
        disabled: normalized_inputs.disabled,
        show_icon: normalized_inputs.show_icon,
        has_message,
        has_custom_aria_label,
        has_custom_message,
        has_custom_class_name,
    });

    FieldErrorViewModel {
        state,
        message,
        aria_label,
        class_name,
        has_custom_message,
        has_custom_aria_label,
        has_custom_class_name,
    }
}

pub fn to_error_message_tone(tone: FieldErrorTone) -> ErrorMessageTone {
    match tone {
        FieldErrorTone::Auto => ErrorMessageTone::Auto,
        FieldErrorTone::Neutral => ErrorMessageTone::Neutral,
        FieldErrorTone::Negative => ErrorMessageTone::Negative,
    }
}

pub fn resolve_headless_state(
    tone: FieldErrorTone,
    disabled: bool,
    has_custom_message: bool,
    has_custom_aria_label: bool,
    has_custom_class_name: bool,
) -> ErrorMessageState {
    resolve_error_message_state(ErrorMessageStateInput {
        tone: to_error_message_tone(tone),
        disabled,
        truncate: false,
        has_custom_message,
        has_custom_aria_label,
        has_custom_class_name,
    })
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

#[cfg(test)]
pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    normalize_aria_label_with_default(value, DEFAULT_ARIA_LABEL)
}

pub fn normalize_aria_label_with_default(
    value: Option<String>,
    default_aria_label: &str,
) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (default_aria_label.into(), false)
}

#[cfg(test)]
pub fn normalize_message(value: Option<String>, visible: bool) -> (Option<String>, bool) {
    normalize_message_with_default(value, visible, DEFAULT_MESSAGE)
}

pub fn normalize_message_with_default(
    value: Option<String>,
    visible: bool,
    default_message: &str,
) -> (Option<String>, bool) {
    if !visible {
        return (None, false);
    }

    if let Some(message) = normalize_optional_text(value) {
        return (Some(message), true);
    }

    (Some(default_message.into()), false)
}

pub fn resolve_state(input: FieldErrorStateInput) -> FieldErrorState {
    resolve_primitive_state(input)
}

pub fn compose_class_name(base_class_name: Option<String>, state: FieldErrorState) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![
        Cow::Borrowed("ui-field-error"),
        Cow::Borrowed(state.tone_class),
    ];

    if state.is_visible {
        classes.push(Cow::Borrowed("ui-field-error--visible"));
    }

    if state.is_disabled {
        classes.push(Cow::Borrowed("ui-field-error--disabled"));
    }

    if state.show_icon {
        classes.push(Cow::Borrowed("ui-field-error--with-icon"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-field-error--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .iter()
        .map(Cow::as_ref)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
