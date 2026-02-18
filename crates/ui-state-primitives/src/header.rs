pub const DEFAULT_ARIA_LABEL: &str = "Header";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HeaderTone {
    #[default]
    Default,
    Strong,
}

impl HeaderTone {
    pub fn class_name(self) -> &'static str {
        match self {
            HeaderTone::Default => "ui-header--tone-default",
            HeaderTone::Strong => "ui-header--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            HeaderTone::Default => "default",
            HeaderTone::Strong => "strong",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeaderStateInput {
    pub tone: HeaderTone,
    pub bordered: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeaderState {
    pub tone: HeaderTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub is_bordered: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn resolve_state(input: HeaderStateInput) -> HeaderState {
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

    let data_state_attr = if input.bordered && input.tone == HeaderTone::Strong {
        "strong-bordered"
    } else if input.bordered {
        "bordered"
    } else if input.tone == HeaderTone::Strong {
        "strong"
    } else {
        "default"
    };

    HeaderState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_bordered: input.bordered,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_tone_contract_is_stable() {
        assert_eq!(HeaderTone::Default.class_name(), "ui-header--tone-default");
        assert_eq!(HeaderTone::Strong.class_name(), "ui-header--tone-strong");

        assert_eq!(HeaderTone::Default.as_attr(), "default");
        assert_eq!(HeaderTone::Strong.as_attr(), "strong");
    }

    #[test]
    fn normalize_optional_text_trims_and_drops_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n\t  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-header  ".to_string())),
            Some("docs-header".to_string())
        );
    }

    #[test]
    fn normalize_aria_label_uses_fallback_when_missing() {
        let (label, custom) = normalize_aria_label(Some("  Dialog Header  ".to_string()));
        assert_eq!(label, "Dialog Header");
        assert!(custom);

        let (label, custom) = normalize_aria_label(Some("  ".to_string()));
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn resolve_state_tracks_flags_and_sources() {
        let state = resolve_state(HeaderStateInput {
            tone: HeaderTone::Strong,
            bordered: true,
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.tone_attr, "strong");
        assert!(state.is_bordered);
        assert_eq!(state.data_state_attr, "strong-bordered");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }
}
