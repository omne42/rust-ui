use crate::checkbox_field::{CheckboxFieldState, CheckboxFieldStateInput};

pub const DEFAULT_LABEL: &str = "Checkbox option";
pub const DEFAULT_ARIA_LABEL: &str = "Checkbox field";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CheckboxFieldTone {
    #[default]
    Default,
    Quiet,
}

impl CheckboxFieldTone {
    pub fn class_name(self) -> &'static str {
        match self {
            CheckboxFieldTone::Default => "ui-checkbox-field--tone-default",
            CheckboxFieldTone::Quiet => "ui-checkbox-field--tone-quiet",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            CheckboxFieldTone::Default => "default",
            CheckboxFieldTone::Quiet => "quiet",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CheckboxFieldIndicatorPlacement {
    #[default]
    Start,
    End,
}

impl CheckboxFieldIndicatorPlacement {
    pub fn class_name(self) -> &'static str {
        match self {
            CheckboxFieldIndicatorPlacement::Start => "ui-checkbox-field--indicator-start",
            CheckboxFieldIndicatorPlacement::End => "ui-checkbox-field--indicator-end",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            CheckboxFieldIndicatorPlacement::Start => "start",
            CheckboxFieldIndicatorPlacement::End => "end",
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
    if let Some(id_base) = normalize_optional_text(value) {
        id_base
    } else {
        "ui-checkbox-field".to_string()
    }
}

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        (label, true)
    } else {
        (DEFAULT_LABEL.into(), false)
    }
}

pub fn normalize_aria_label(value: Option<String>, fallback: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        (label, true)
    } else if !fallback.trim().is_empty() {
        (fallback.trim().into(), false)
    } else {
        (DEFAULT_ARIA_LABEL.into(), false)
    }
}

pub fn resolve_state(input: CheckboxFieldStateInput) -> CheckboxFieldState {
    let label_source_attr = if input.has_custom_label {
        "custom"
    } else {
        "default"
    };

    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else if input.has_custom_label {
        "label"
    } else {
        "default"
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    let state_attr = if input.invalid && input.checked {
        "checked-invalid"
    } else if input.invalid {
        "invalid"
    } else if input.disabled {
        "disabled"
    } else if input.checked {
        "checked"
    } else {
        "unchecked"
    };

    CheckboxFieldState {
        is_checked: input.checked,
        is_unchecked: !input.checked,
        is_disabled: input.disabled,
        is_invalid: input.invalid,
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        indicator_placement: input.indicator_placement,
        indicator_placement_class: input.indicator_placement.class_name(),
        indicator_placement_attr: input.indicator_placement.as_attr(),
        has_description: input.has_description,
        description_attr: if input.has_description {
            "present"
        } else {
            "absent"
        },
        has_custom_label: input.has_custom_label,
        label_source_attr,
        has_custom_aria_label: input.has_custom_aria_label,
        aria_source_attr,
        has_custom_class_name: input.has_custom_class_name,
        class_source_attr,
        state_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: CheckboxFieldState) -> String {
    let mut classes = vec![
        "ui-checkbox-field".to_string(),
        state.tone_class.into(),
        state.indicator_placement_class.into(),
    ];

    if state.is_checked {
        classes.push("ui-checkbox-field--checked".to_string());
    } else {
        classes.push("ui-checkbox-field--unchecked".to_string());
    }

    if state.is_invalid {
        classes.push("ui-checkbox-field--invalid".to_string());
    }

    if state.is_disabled {
        classes.push("ui-checkbox-field--disabled".to_string());
    }

    if state.has_description {
        classes.push("ui-checkbox-field--with-description".to_string());
    } else {
        classes.push("ui-checkbox-field--no-description".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-checkbox-field--custom-class".to_string());
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
            normalize_optional_text(Some("  Newsletter  ".to_string())),
            Some("Newsletter".to_string())
        );

        assert_eq!(normalize_id_base(None), "ui-checkbox-field");
        assert_eq!(
            normalize_id_base(Some("  docs-checkbox-field  ".to_string())),
            "docs-checkbox-field"
        );

        assert_eq!(
            normalize_label(Some("  Accept terms  ".to_string())),
            ("Accept terms".to_string(), true)
        );
        assert_eq!(normalize_label(None), (DEFAULT_LABEL.into(), false));

        assert_eq!(
            normalize_aria_label(Some("  Custom aria  ".to_string()), "Ignored"),
            ("Custom aria".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(None, "Fallback label"),
            ("Fallback label".to_string(), false)
        );
    }

    #[test]
    fn resolve_state_tracks_state_markers() {
        let state = resolve_state(CheckboxFieldStateInput {
            checked: true,
            disabled: false,
            invalid: true,
            tone: CheckboxFieldTone::Quiet,
            indicator_placement: CheckboxFieldIndicatorPlacement::End,
            has_description: true,
            has_custom_label: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        assert!(state.is_checked);
        assert!(!state.is_unchecked);
        assert!(state.is_invalid);
        assert!(!state.is_disabled);
        assert_eq!(state.tone_class, "ui-checkbox-field--tone-quiet");
        assert_eq!(state.tone_attr, "quiet");
        assert_eq!(
            state.indicator_placement_class,
            "ui-checkbox-field--indicator-end"
        );
        assert_eq!(state.indicator_placement_attr, "end");
        assert_eq!(state.description_attr, "present");
        assert_eq!(state.label_source_attr, "default");
        assert_eq!(state.aria_source_attr, "default");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.state_attr, "checked-invalid");
    }

    #[test]
    fn compose_class_name_includes_state_classes() {
        let state = resolve_state(CheckboxFieldStateInput {
            checked: false,
            disabled: true,
            invalid: false,
            tone: CheckboxFieldTone::Default,
            indicator_placement: CheckboxFieldIndicatorPlacement::Start,
            has_description: false,
            has_custom_label: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-checkbox-field".to_string()), state);

        for expected in [
            "ui-checkbox-field",
            "ui-checkbox-field--tone-default",
            "ui-checkbox-field--indicator-start",
            "ui-checkbox-field--unchecked",
            "ui-checkbox-field--disabled",
            "ui-checkbox-field--no-description",
            "ui-checkbox-field--custom-class",
            "docs-checkbox-field",
        ] {
            assert!(class_name.contains(expected));
        }
    }
}
