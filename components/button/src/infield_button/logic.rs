use crate::infield_button::{InfieldButtonState, InfieldButtonStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "InfieldButton";

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_state(input: InfieldButtonStateInput) -> InfieldButtonState {
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

    InfieldButtonState {
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
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: InfieldButtonState) -> String {
    let mut classes = vec!["ui-infield-button".to_string()];

    if state.is_quiet {
        classes.push("ui-infield-button--quiet".to_string());
    }

    if state.is_invalid {
        classes.push("ui-infield-button--invalid".to_string());
    }

    if state.is_disabled {
        classes.push("ui-infield-button--disabled".to_string());
    }

    if state.is_forced_active {
        classes.push("ui-infield-button--active".to_string());
    }

    if state.has_custom_press_handler {
        classes.push("ui-infield-button--custom-handler".to_string());
    }

    if state.has_custom_aria_label {
        classes.push("ui-infield-button--custom-aria-label".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-infield-button--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/infield_button/logic.rs"]
mod tests;
