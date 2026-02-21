use crate::picker_button::{PickerButtonState, PickerButtonStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "PickerButton";

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    ui_state_primitives::button::normalize_optional_text(value)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_inner_class_name(value: Option<String>) -> String {
    value.unwrap_or_default()
}

pub fn resolve_state(input: PickerButtonStateInput) -> PickerButtonState {
    let data_state_attr = if input.disabled && input.invalid {
        "invalid-disabled"
    } else if input.disabled {
        "disabled"
    } else if input.invalid {
        "invalid"
    } else if input.forced_active {
        "active"
    } else if input.quiet {
        "quiet"
    } else {
        "default"
    };

    PickerButtonState {
        is_quiet: input.quiet,
        is_invalid: input.invalid,
        is_disabled: input.disabled,
        is_forced_active: input.forced_active,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_press_handler: input.has_custom_press_handler,
        quiet_attr: if input.quiet { "true" } else { "false" },
        invalid_attr: if input.invalid { "true" } else { "false" },
        disabled_attr: if input.disabled { "true" } else { "false" },
        active_mode_attr: if input.forced_active {
            "forced"
        } else {
            "interactive"
        },
        data_state_attr,
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        handler_source_attr: if input.has_custom_press_handler {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: PickerButtonState) -> String {
    let mut classes = vec!["ui-picker-button".to_string()];

    if state.is_quiet {
        classes.push("ui-picker-button--quiet".to_string());
    }

    if state.is_invalid {
        classes.push("ui-picker-button--invalid".to_string());
    }

    if state.is_disabled {
        classes.push("ui-picker-button--disabled".to_string());
    }

    if state.is_forced_active {
        classes.push("ui-picker-button--active".to_string());
    }

    if state.has_custom_press_handler {
        classes.push("ui-picker-button--custom-handler".to_string());
    }

    if state.has_custom_aria_label {
        classes.push("ui-picker-button--custom-aria-label".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-picker-button--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/picker_button/logic.rs"]
mod tests;
