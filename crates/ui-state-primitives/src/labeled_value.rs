pub const DEFAULT_LABEL_TEXT: &str = "Label";
pub const DEFAULT_VALUE_TEXT: &str = "—";
pub const DEFAULT_ARIA_LABEL: &str = "Labeled value";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LabeledValueOrientation {
    #[default]
    Stacked,
    Inline,
}

impl LabeledValueOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            LabeledValueOrientation::Stacked => "ui-labeled-value--orientation-stacked",
            LabeledValueOrientation::Inline => "ui-labeled-value--orientation-inline",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            LabeledValueOrientation::Stacked => "stacked",
            LabeledValueOrientation::Inline => "inline",
        }
    }
}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabeledValueStateInput {
    pub orientation: LabeledValueOrientation,
    pub tone: LabeledValueTone,
    pub has_custom_label: bool,
    pub has_custom_value: bool,
    pub has_description: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabeledValueState {
    pub orientation: LabeledValueOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub tone: LabeledValueTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub has_custom_label: bool,
    pub has_custom_value: bool,
    pub has_description: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub label_source_attr: &'static str,
    pub value_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_label_text(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_LABEL_TEXT.into(), false)
}

pub fn normalize_value_text(value: Option<String>) -> (String, bool) {
    if let Some(value) = normalize_optional_text(value) {
        return (value, true);
    }

    (DEFAULT_VALUE_TEXT.into(), false)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_and_tone_contracts_are_stable() {
        assert_eq!(
            LabeledValueOrientation::Stacked.class_name(),
            "ui-labeled-value--orientation-stacked"
        );
        assert_eq!(LabeledValueOrientation::Inline.as_attr(), "inline");
        assert_eq!(
            LabeledValueTone::Default.class_name(),
            "ui-labeled-value--tone-default"
        );
        assert_eq!(LabeledValueTone::Strong.as_attr(), "strong");
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(
            normalize_label_text(Some("  Status  ".to_string())),
            ("Status".to_string(), true)
        );
        assert_eq!(
            normalize_value_text(None),
            (DEFAULT_VALUE_TEXT.into(), false)
        );
        assert_eq!(
            normalize_aria_label(Some(" ".to_string())),
            (DEFAULT_ARIA_LABEL.into(), false)
        );
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
}
