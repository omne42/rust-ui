use crate::labeled_value::{LabeledValueState, LabeledValueStateInput};

pub const DEFAULT_LABEL_TEXT: &str = "Label";
pub const DEFAULT_VALUE_TEXT: &str = "—";
pub const DEFAULT_ARIA_LABEL: &str = "Labeled value";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LabeledValueTone {
    #[default]
    Default,
    Subtle,
    Strong,
}

impl LabeledValueTone {
    pub fn class_name(self) -> &'static str {
        match self {
            LabeledValueTone::Default => "ui-labeled-value--tone-default",
            LabeledValueTone::Subtle => "ui-labeled-value--tone-subtle",
            LabeledValueTone::Strong => "ui-labeled-value--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            LabeledValueTone::Default => "default",
            LabeledValueTone::Subtle => "subtle",
            LabeledValueTone::Strong => "strong",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_label_text(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_LABEL_TEXT.to_string(), false)
}

pub fn normalize_value_text(value: Option<String>) -> (String, bool) {
    if let Some(value) = normalize_optional_text(value) {
        return (value, true);
    }

    (DEFAULT_VALUE_TEXT.to_string(), false)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn resolve_state(input: LabeledValueStateInput) -> LabeledValueState {
    let label_source_attr = if input.has_custom_label {
        "custom"
    } else {
        "default"
    };
    let value_source_attr = if input.has_custom_value {
        "custom"
    } else {
        "default"
    };
    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };
    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    LabeledValueState {
        orientation: input.orientation,
        orientation_class: input.orientation.class_name(),
        orientation_attr: input.orientation.as_attr(),
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        has_custom_label: input.has_custom_label,
        has_custom_value: input.has_custom_value,
        has_description: input.has_description,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        label_source_attr,
        value_source_attr,
        aria_source_attr,
        class_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: LabeledValueState) -> String {
    let mut classes = vec![
        "ui-labeled-value".to_string(),
        state.orientation_class.to_string(),
        state.tone_class.to_string(),
    ];

    if state.has_description {
        classes.push("ui-labeled-value--with-description".to_string());
    }
    if state.has_custom_label {
        classes.push("ui-labeled-value--label-custom".to_string());
    }
    if state.has_custom_value {
        classes.push("ui-labeled-value--value-custom".to_string());
    }
    if state.has_custom_aria_label {
        classes.push("ui-labeled-value--aria-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-labeled-value--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::labeled_value::LabeledValueOrientation;

    #[test]
    fn tone_class_names_and_attrs_are_stable() {
        assert_eq!(
            LabeledValueTone::Default.class_name(),
            "ui-labeled-value--tone-default"
        );
        assert_eq!(
            LabeledValueTone::Subtle.class_name(),
            "ui-labeled-value--tone-subtle"
        );
        assert_eq!(
            LabeledValueTone::Strong.class_name(),
            "ui-labeled-value--tone-strong"
        );

        assert_eq!(LabeledValueTone::Default.as_attr(), "default");
        assert_eq!(LabeledValueTone::Subtle.as_attr(), "subtle");
        assert_eq!(LabeledValueTone::Strong.as_attr(), "strong");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-value ".to_string())),
            Some("docs-value".to_string())
        );
    }

    #[test]
    fn normalize_helpers_use_fallbacks() {
        let (label, custom_label) = normalize_label_text(Some("  Status  ".to_string()));
        assert_eq!(label, "Status");
        assert!(custom_label);

        let (value, custom_value) = normalize_value_text(None);
        assert_eq!(value, DEFAULT_VALUE_TEXT);
        assert!(!custom_value);

        let (aria, custom_aria) = normalize_aria_label(Some("  Status row  ".to_string()));
        assert_eq!(aria, "Status row");
        assert!(custom_aria);
    }

    #[test]
    fn resolve_state_tracks_sources_and_layout() {
        let state = resolve_state(LabeledValueStateInput {
            orientation: LabeledValueOrientation::Inline,
            tone: LabeledValueTone::Strong,
            has_custom_label: true,
            has_custom_value: false,
            has_description: true,
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.orientation_attr, "inline");
        assert_eq!(state.tone_attr, "strong");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.value_source_attr, "default");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
        assert!(state.has_description);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-labeled-value".to_string()),
            resolve_state(LabeledValueStateInput {
                orientation: LabeledValueOrientation::Stacked,
                tone: LabeledValueTone::Subtle,
                has_custom_label: false,
                has_custom_value: true,
                has_description: true,
                has_custom_aria_label: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-labeled-value",
            "ui-labeled-value--orientation-stacked",
            "ui-labeled-value--tone-subtle",
            "ui-labeled-value--with-description",
            "ui-labeled-value--value-custom",
            "ui-labeled-value--custom-class",
            "docs-labeled-value",
        ] {
            assert!(class_name.contains(token), "class should include `{token}`");
        }
    }
}
