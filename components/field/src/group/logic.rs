use super::{FieldGroupState, FieldGroupStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Field group";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FieldGroupOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl FieldGroupOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            FieldGroupOrientation::Vertical => "ui-field-group--orientation-vertical",
            FieldGroupOrientation::Horizontal => "ui-field-group--orientation-horizontal",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FieldGroupOrientation::Vertical => "vertical",
            FieldGroupOrientation::Horizontal => "horizontal",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FieldGroupDensity {
    #[default]
    Comfortable,
    Compact,
}

impl FieldGroupDensity {
    pub fn class_name(self) -> &'static str {
        match self {
            FieldGroupDensity::Comfortable => "ui-field-group--density-comfortable",
            FieldGroupDensity::Compact => "ui-field-group--density-compact",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FieldGroupDensity::Comfortable => "comfortable",
            FieldGroupDensity::Compact => "compact",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(value: Option<String>) -> String {
    if let Some(value) = normalize_optional_text(value) {
        value
    } else {
        "ui-field-group".to_string()
    }
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        (label, true)
    } else {
        (DEFAULT_ARIA_LABEL.into(), false)
    }
}

pub fn resolve_state(input: FieldGroupStateInput) -> FieldGroupState {
    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else if input.has_label {
        "label"
    } else {
        "default"
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    let state_attr = if input.invalid && input.disabled {
        "invalid-disabled"
    } else if input.invalid {
        "invalid"
    } else if input.disabled {
        "disabled"
    } else {
        "default"
    };

    FieldGroupState {
        orientation: input.orientation,
        orientation_class: input.orientation.class_name(),
        orientation_attr: input.orientation.as_attr(),
        density: input.density,
        density_class: input.density.class_name(),
        density_attr: input.density.as_attr(),
        is_disabled: input.disabled,
        is_invalid: input.invalid,
        has_label: input.has_label,
        label_attr: if input.has_label { "present" } else { "absent" },
        has_description: input.has_description,
        description_attr: if input.has_description {
            "present"
        } else {
            "absent"
        },
        has_custom_aria_label: input.has_custom_aria_label,
        aria_source_attr,
        has_custom_class_name: input.has_custom_class_name,
        class_source_attr,
        state_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: FieldGroupState) -> String {
    let mut classes = vec![
        "ui-field-group".to_string(),
        state.orientation_class.into(),
        state.density_class.into(),
    ];

    if state.is_disabled {
        classes.push("ui-field-group--disabled".to_string());
    }

    if state.is_invalid {
        classes.push("ui-field-group--invalid".to_string());
    }

    if state.has_label {
        classes.push("ui-field-group--has-label".to_string());
    } else {
        classes.push("ui-field-group--no-label".to_string());
    }

    if state.has_description {
        classes.push("ui-field-group--with-description".to_string());
    } else {
        classes.push("ui-field-group--no-description".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-field-group--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/group/logic.rs"]
mod tests;
