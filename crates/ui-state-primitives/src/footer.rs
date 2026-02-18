pub const DEFAULT_ARIA_LABEL: &str = "Footer";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FooterTone {
    #[default]
    Default,
    Muted,
}

impl FooterTone {
    pub fn class_name(self) -> &'static str {
        match self {
            FooterTone::Default => "ui-footer--tone-default",
            FooterTone::Muted => "ui-footer--tone-muted",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FooterTone::Default => "default",
            FooterTone::Muted => "muted",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FooterStateInput {
    pub tone: FooterTone,
    pub bordered: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FooterState {
    pub tone: FooterTone,
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

pub fn resolve_state(input: FooterStateInput) -> FooterState {
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

    let data_state_attr = if input.bordered && input.tone == FooterTone::Muted {
        "muted-bordered"
    } else if input.bordered {
        "bordered"
    } else if input.tone == FooterTone::Muted {
        "muted"
    } else {
        "default"
    };

    FooterState {
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

pub fn compose_class_name(base_class_name: Option<String>, state: FooterState) -> String {
    let mut classes = vec!["ui-footer".to_string(), state.tone_class.to_string()];

    if state.is_bordered {
        classes.push("ui-footer--bordered".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-footer--custom-class".to_string());
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
    fn footer_tone_contract_is_stable() {
        assert_eq!(FooterTone::Default.class_name(), "ui-footer--tone-default");
        assert_eq!(FooterTone::Muted.class_name(), "ui-footer--tone-muted");

        assert_eq!(FooterTone::Default.as_attr(), "default");
        assert_eq!(FooterTone::Muted.as_attr(), "muted");
    }

    #[test]
    fn normalize_optional_text_trims_and_drops_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n\t  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-footer  ".to_string())),
            Some("docs-footer".to_string())
        );
    }

    #[test]
    fn normalize_aria_label_uses_fallback_when_missing() {
        let (label, custom) = normalize_aria_label(Some("  Dialog Footer  ".to_string()));
        assert_eq!(label, "Dialog Footer");
        assert!(custom);

        let (label, custom) = normalize_aria_label(Some("  ".to_string()));
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn resolve_state_tracks_flags_and_sources() {
        let state = resolve_state(FooterStateInput {
            tone: FooterTone::Muted,
            bordered: true,
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.tone_attr, "muted");
        assert!(state.is_bordered);
        assert_eq!(state.data_state_attr, "muted-bordered");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_custom_marker_and_user_class() {
        let state = resolve_state(FooterStateInput {
            tone: FooterTone::Default,
            bordered: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-footer-custom".to_string()), state);

        for token in [
            "ui-footer",
            "ui-footer--tone-default",
            "ui-footer--bordered",
            "ui-footer--custom-class",
            "docs-footer-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
