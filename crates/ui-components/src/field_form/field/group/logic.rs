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
mod tests {
    use super::*;

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Group  ".to_string())),
            Some("Group".to_string())
        );

        assert_eq!(normalize_id_base(None), "ui-field-group");
        assert_eq!(normalize_id_base(Some("  docs  ".to_string())), "docs");

        assert_eq!(
            normalize_aria_label(Some("  Controls  ".to_string())),
            ("Controls".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.into(), false)
        );
    }

    #[test]
    fn resolve_state_tracks_semantic_markers() {
        let state = resolve_state(FieldGroupStateInput {
            orientation: FieldGroupOrientation::Horizontal,
            density: FieldGroupDensity::Compact,
            disabled: true,
            invalid: true,
            has_label: false,
            has_description: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        assert_eq!(
            state.orientation_class,
            "ui-field-group--orientation-horizontal"
        );
        assert_eq!(state.orientation_attr, "horizontal");
        assert_eq!(state.density_class, "ui-field-group--density-compact");
        assert_eq!(state.density_attr, "compact");
        assert_eq!(state.state_attr, "invalid-disabled");
        assert_eq!(state.label_attr, "absent");
        assert_eq!(state.description_attr, "present");
        assert_eq!(state.aria_source_attr, "default");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_classes() {
        let state = resolve_state(FieldGroupStateInput {
            orientation: FieldGroupOrientation::Vertical,
            density: FieldGroupDensity::Comfortable,
            disabled: false,
            invalid: true,
            has_label: true,
            has_description: false,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-field-group".to_string()), state);

        for expected in [
            "ui-field-group",
            "ui-field-group--orientation-vertical",
            "ui-field-group--density-comfortable",
            "ui-field-group--invalid",
            "ui-field-group--has-label",
            "ui-field-group--no-description",
            "ui-field-group--custom-class",
            "docs-field-group",
        ] {
            assert!(class_name.contains(expected));
        }
    }
}
