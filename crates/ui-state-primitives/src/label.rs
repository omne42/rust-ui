pub const DEFAULT_ARIA_LABEL: &str = "Label";
pub const DEFAULT_REQUIRED_INDICATOR: &str = "*";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LabelEmphasis {
    #[default]
    Default,
    Subtle,
    Strong,
}

impl LabelEmphasis {
    pub fn class_name(self) -> &'static str {
        match self {
            LabelEmphasis::Default => "ui-label--emphasis-default",
            LabelEmphasis::Subtle => "ui-label--emphasis-subtle",
            LabelEmphasis::Strong => "ui-label--emphasis-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            LabelEmphasis::Default => "default",
            LabelEmphasis::Subtle => "subtle",
            LabelEmphasis::Strong => "strong",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelStateInput {
    pub emphasis: LabelEmphasis,
    pub required: bool,
    pub disabled: bool,
    pub has_for_id: bool,
    pub has_custom_label: bool,
    pub has_custom_indicator: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelState {
    pub emphasis: LabelEmphasis,
    pub emphasis_class: &'static str,
    pub emphasis_attr: &'static str,
    pub is_required: bool,
    pub is_optional: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_for_id: bool,
    pub has_custom_label: bool,
    pub has_custom_indicator: bool,
    pub has_custom_class_name: bool,
    pub label_source_attr: &'static str,
    pub indicator_source_attr: &'static str,
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

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn normalize_required_indicator(value: Option<String>) -> (String, bool) {
    if let Some(indicator) = normalize_optional_text(value) {
        return (indicator, true);
    }

    (DEFAULT_REQUIRED_INDICATOR.into(), false)
}

pub fn resolve_state(input: LabelStateInput) -> LabelState {
    let label_source_attr = if input.has_custom_label {
        "custom"
    } else {
        "default"
    };
    let indicator_source_attr = if input.has_custom_indicator {
        "custom"
    } else {
        "default"
    };
    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    LabelState {
        emphasis: input.emphasis,
        emphasis_class: input.emphasis.class_name(),
        emphasis_attr: input.emphasis.as_attr(),
        is_required: input.required,
        is_optional: !input.required,
        is_disabled: input.disabled,
        is_enabled: !input.disabled,
        has_for_id: input.has_for_id,
        has_custom_label: input.has_custom_label,
        has_custom_indicator: input.has_custom_indicator,
        has_custom_class_name: input.has_custom_class_name,
        label_source_attr,
        indicator_source_attr,
        class_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: LabelState) -> String {
    let mut classes = vec!["ui-label".to_string(), state.emphasis_class.into()];

    if state.is_required {
        classes.push("ui-label--required".to_string());
    }
    if state.is_disabled {
        classes.push("ui-label--disabled".to_string());
    }
    if state.has_for_id {
        classes.push("ui-label--for".to_string());
    }
    if state.has_custom_label {
        classes.push("ui-label--text-custom".to_string());
    }
    if state.has_custom_indicator {
        classes.push("ui-label--indicator-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-label--custom-class".to_string());
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
    fn emphasis_class_names_and_attrs_are_stable() {
        assert_eq!(
            LabelEmphasis::Default.class_name(),
            "ui-label--emphasis-default"
        );
        assert_eq!(
            LabelEmphasis::Subtle.class_name(),
            "ui-label--emphasis-subtle"
        );
        assert_eq!(
            LabelEmphasis::Strong.class_name(),
            "ui-label--emphasis-strong"
        );

        assert_eq!(LabelEmphasis::Default.as_attr(), "default");
        assert_eq!(LabelEmphasis::Subtle.as_attr(), "subtle");
        assert_eq!(LabelEmphasis::Strong.as_attr(), "strong");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" Name ".to_string())),
            Some("Name".to_string())
        );
    }

    #[test]
    fn normalize_helpers_fallback_to_defaults() {
        let (label, custom_label) = normalize_label_text(Some("  Username  ".to_string()));
        assert_eq!(label, "Username");
        assert!(custom_label);

        let (label, custom_label) = normalize_label_text(None);
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom_label);

        let (indicator, custom_indicator) =
            normalize_required_indicator(Some(" (req) ".to_string()));
        assert_eq!(indicator, "(req)");
        assert!(custom_indicator);

        let (indicator, custom_indicator) = normalize_required_indicator(None);
        assert_eq!(indicator, DEFAULT_REQUIRED_INDICATOR);
        assert!(!custom_indicator);
    }

    #[test]
    fn resolve_state_tracks_required_disabled_and_sources() {
        let state = resolve_state(LabelStateInput {
            emphasis: LabelEmphasis::Strong,
            required: true,
            disabled: true,
            has_for_id: true,
            has_custom_label: true,
            has_custom_indicator: false,
            has_custom_class_name: true,
        });

        assert_eq!(state.emphasis_attr, "strong");
        assert!(state.is_required);
        assert!(!state.is_optional);
        assert!(state.is_disabled);
        assert!(state.has_for_id);
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.indicator_source_attr, "default");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(LabelStateInput {
            emphasis: LabelEmphasis::Subtle,
            required: true,
            disabled: false,
            has_for_id: true,
            has_custom_label: false,
            has_custom_indicator: true,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-label".to_string()), state);
        for token in [
            "ui-label",
            "ui-label--emphasis-subtle",
            "ui-label--required",
            "ui-label--for",
            "ui-label--indicator-custom",
            "ui-label--custom-class",
            "docs-label",
        ] {
            assert!(class_name.contains(token), "class should contain `{token}`");
        }
    }
}
