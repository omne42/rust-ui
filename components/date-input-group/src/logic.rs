use crate::{DateInputGroupState, DateInputGroupStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Date input group";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DateInputGroupVariant {
    #[default]
    Primary,
    Secondary,
}

impl DateInputGroupVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            DateInputGroupVariant::Primary => "ui-date-input-group--variant-primary",
            DateInputGroupVariant::Secondary => "ui-date-input-group--variant-secondary",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            DateInputGroupVariant::Primary => "primary",
            DateInputGroupVariant::Secondary => "secondary",
        }
    }
}

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

pub fn resolve_state(input: DateInputGroupStateInput) -> DateInputGroupState {
    let width_class = if input.full_width {
        "ui-date-input-group--full-width"
    } else {
        "ui-date-input-group--fit-width"
    };

    let width_attr = if input.full_width { "full" } else { "fit" };

    let data_state_attr = if input.disabled && input.invalid {
        "disabled-invalid"
    } else if input.disabled {
        "disabled"
    } else if input.invalid {
        "invalid"
    } else if input.segmented {
        "segmented"
    } else {
        "default"
    };

    DateInputGroupState {
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_attr(),
        width_class,
        width_attr,
        is_full_width: input.full_width,
        is_disabled: input.disabled,
        is_invalid: input.invalid,
        is_segmented: input.segmented,
        has_prefix: input.has_prefix,
        has_suffix: input.has_suffix,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
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

pub fn compose_class_name(base_class_name: Option<String>, state: DateInputGroupState) -> String {
    let mut classes = vec![
        "ui-date-input-group".to_string(),
        state.variant_class.into(),
        state.width_class.into(),
    ];

    if state.is_disabled {
        classes.push("ui-date-input-group--disabled".to_string());
    }

    if state.is_invalid {
        classes.push("ui-date-input-group--invalid".to_string());
    }

    if state.is_segmented {
        classes.push("ui-date-input-group--segmented".to_string());
    }

    if state.has_prefix {
        classes.push("ui-date-input-group--has-prefix".to_string());
    }

    if state.has_suffix {
        classes.push("ui-date-input-group--has-suffix".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-date-input-group--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
