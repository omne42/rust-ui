use crate::logic_button::{LogicButtonState, LogicButtonStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Logic operator";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LogicButtonVariant {
    #[default]
    And,
    Or,
}

impl LogicButtonVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            LogicButtonVariant::And => "ui-logic-button--variant-and",
            LogicButtonVariant::Or => "ui-logic-button--variant-or",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            LogicButtonVariant::And => "and",
            LogicButtonVariant::Or => "or",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    ui_state_primitives::button::normalize_optional_text(value)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_state(input: LogicButtonStateInput) -> LogicButtonState {
    LogicButtonState {
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_attr(),
        is_disabled: input.disabled,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_press_handler: input.has_custom_press_handler,
        data_state_attr: if input.disabled { "disabled" } else { "ready" },
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

pub fn compose_class_name(base_class_name: Option<String>, state: LogicButtonState) -> String {
    let mut classes = vec!["ui-logic-button".to_string(), state.variant_class.into()];

    if state.is_disabled {
        classes.push("ui-logic-button--disabled".to_string());
    }

    if state.has_custom_press_handler {
        classes.push("ui-logic-button--custom-handler".to_string());
    }

    if state.has_custom_aria_label {
        classes.push("ui-logic-button--custom-aria-label".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-logic-button--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/logic_button/logic.rs"]
mod tests;
