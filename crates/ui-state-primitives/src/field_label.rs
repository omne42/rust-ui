pub const DEFAULT_TEXT: &str = "Field";
pub const DEFAULT_REQUIRED_INDICATOR: &str = "*";
pub const DEFAULT_ARIA_LABEL: &str = "Field label";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FieldLabelTone {
    #[default]
    Default,
    Muted,
    Strong,
}

impl FieldLabelTone {
    pub fn class_name(self) -> &'static str {
        match self {
            FieldLabelTone::Default => "ui-field-label--tone-default",
            FieldLabelTone::Muted => "ui-field-label--tone-muted",
            FieldLabelTone::Strong => "ui-field-label--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FieldLabelTone::Default => "default",
            FieldLabelTone::Muted => "muted",
            FieldLabelTone::Strong => "strong",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldLabelStateInput {
    pub tone: FieldLabelTone,
    pub required: bool,
    pub disabled: bool,
    pub has_for_id: bool,
    pub has_custom_text: bool,
    pub has_custom_indicator: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldLabelState {
    pub tone: FieldLabelTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_required: bool,
    pub is_optional: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_for_id: bool,
    pub has_custom_text: bool,
    pub has_custom_indicator: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub text_source_attr: &'static str,
    pub indicator_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_text(value: Option<String>) -> (String, bool) {
    if let Some(text) = normalize_optional_text(value) {
        (text, true)
    } else {
        (DEFAULT_TEXT.into(), false)
    }
}

pub fn normalize_required_indicator(value: Option<String>) -> (String, bool) {
    if let Some(indicator) = normalize_optional_text(value) {
        (indicator, true)
    } else {
        (DEFAULT_REQUIRED_INDICATOR.into(), false)
    }
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        (label, true)
    } else {
        (DEFAULT_ARIA_LABEL.into(), false)
    }
}

pub fn resolve_state(input: FieldLabelStateInput) -> FieldLabelState {
    let text_source_attr = if input.has_custom_text {
        "custom"
    } else {
        "default"
    };

    let indicator_source_attr = if input.has_custom_indicator {
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

    FieldLabelState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_required: input.required,
        is_optional: !input.required,
        is_disabled: input.disabled,
        is_enabled: !input.disabled,
        has_for_id: input.has_for_id,
        has_custom_text: input.has_custom_text,
        has_custom_indicator: input.has_custom_indicator,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        text_source_attr,
        indicator_source_attr,
        aria_source_attr,
        class_source_attr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tone_class_names_and_attrs_are_stable() {
        assert_eq!(
            FieldLabelTone::Default.class_name(),
            "ui-field-label--tone-default"
        );
        assert_eq!(
            FieldLabelTone::Muted.class_name(),
            "ui-field-label--tone-muted"
        );
        assert_eq!(
            FieldLabelTone::Strong.class_name(),
            "ui-field-label--tone-strong"
        );

        assert_eq!(FieldLabelTone::Default.as_attr(), "default");
        assert_eq!(FieldLabelTone::Muted.as_attr(), "muted");
        assert_eq!(FieldLabelTone::Strong.as_attr(), "strong");
    }

    #[test]
    fn normalize_helpers_use_trimmed_custom_values_or_defaults() {
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Project owner  ".to_string())),
            Some("Project owner".to_string())
        );

        assert_eq!(
            normalize_text(Some("  Team  ".to_string())),
            ("Team".to_string(), true)
        );
        assert_eq!(normalize_text(None), (DEFAULT_TEXT.into(), false));

        assert_eq!(
            normalize_required_indicator(Some("  (required)  ".to_string())),
            ("(required)".to_string(), true)
        );
        assert_eq!(
            normalize_required_indicator(None),
            (DEFAULT_REQUIRED_INDICATOR.into(), false)
        );

        assert_eq!(
            normalize_aria_label(Some("  Field heading  ".to_string())),
            ("Field heading".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.into(), false)
        );
    }

    #[test]
    fn resolve_state_tracks_visibility_and_source_markers() {
        let state = resolve_state(FieldLabelStateInput {
            tone: FieldLabelTone::Strong,
            required: true,
            disabled: true,
            has_for_id: true,
            has_custom_text: true,
            has_custom_indicator: false,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.tone_attr, "strong");
        assert!(state.is_required);
        assert!(!state.is_optional);
        assert!(state.is_disabled);
        assert!(!state.is_enabled);
        assert!(state.has_for_id);
        assert_eq!(state.text_source_attr, "custom");
        assert_eq!(state.indicator_source_attr, "default");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }
}
