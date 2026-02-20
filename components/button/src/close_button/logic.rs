use crate::close_button::{CloseButtonState, CloseButtonStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Close";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CloseButtonVariant {
    #[default]
    Default,
    OverBackground,
}

impl CloseButtonVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            CloseButtonVariant::Default => "ui-close-button--variant-default",
            CloseButtonVariant::OverBackground => "ui-close-button--variant-over-background",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            CloseButtonVariant::Default => "default",
            CloseButtonVariant::OverBackground => "over-background",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CloseButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl CloseButtonSize {
    pub fn class_name(self) -> &'static str {
        match self {
            CloseButtonSize::Sm => "ui-close-button--size-sm",
            CloseButtonSize::Md => "ui-close-button--size-md",
            CloseButtonSize::Lg => "ui-close-button--size-lg",
            CloseButtonSize::Xl => "ui-close-button--size-xl",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            CloseButtonSize::Sm => "sm",
            CloseButtonSize::Md => "md",
            CloseButtonSize::Lg => "lg",
            CloseButtonSize::Xl => "xl",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>, default: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (default.into(), false)
}

pub fn resolve_state(input: CloseButtonStateInput) -> CloseButtonState {
    CloseButtonState {
        variant: input.variant,
        size: input.size,
        variant_class: input.variant.class_name(),
        size_class: input.size.class_name(),
        variant_attr: input.variant.as_attr(),
        size_attr: input.size.as_attr(),
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

pub fn compose_class_name(base_class_name: Option<String>, state: CloseButtonState) -> String {
    let mut classes = vec![
        "ui-close-button".to_string(),
        state.variant_class.into(),
        state.size_class.into(),
    ];

    if state.is_disabled {
        classes.push("ui-close-button--disabled".to_string());
    }

    if state.has_custom_press_handler {
        classes.push("ui-close-button--custom-handler".to_string());
    }

    if state.has_custom_aria_label {
        classes.push("ui-close-button--custom-aria-label".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-close-button--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/close_button/logic.rs"]
mod tests;
