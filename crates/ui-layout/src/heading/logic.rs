use crate::heading::{HeadingState, HeadingStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Heading";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HeadingLevel {
    H1,
    H2,
    #[default]
    H3,
    H4,
    H5,
    H6,
}

impl HeadingLevel {
    pub fn class_name(self) -> &'static str {
        match self {
            HeadingLevel::H1 => "ui-heading--level-1",
            HeadingLevel::H2 => "ui-heading--level-2",
            HeadingLevel::H3 => "ui-heading--level-3",
            HeadingLevel::H4 => "ui-heading--level-4",
            HeadingLevel::H5 => "ui-heading--level-5",
            HeadingLevel::H6 => "ui-heading--level-6",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            HeadingLevel::H1 => "1",
            HeadingLevel::H2 => "2",
            HeadingLevel::H3 => "3",
            HeadingLevel::H4 => "4",
            HeadingLevel::H5 => "5",
            HeadingLevel::H6 => "6",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HeadingTone {
    #[default]
    Default,
    Strong,
    Muted,
}

impl HeadingTone {
    pub fn class_name(self) -> &'static str {
        match self {
            HeadingTone::Default => "ui-heading--tone-default",
            HeadingTone::Strong => "ui-heading--tone-strong",
            HeadingTone::Muted => "ui-heading--tone-muted",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            HeadingTone::Default => "default",
            HeadingTone::Strong => "strong",
            HeadingTone::Muted => "muted",
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

pub fn resolve_state(input: HeadingStateInput) -> HeadingState {
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

    let data_state_attr = if input.truncate {
        "truncate"
    } else if input.tone == HeadingTone::Strong {
        "strong"
    } else if input.tone == HeadingTone::Muted {
        "muted"
    } else {
        "default"
    };

    HeadingState {
        level: input.level,
        level_class: input.level.class_name(),
        level_attr: input.level.as_attr(),
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_truncated: input.truncate,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: HeadingState) -> String {
    let mut classes = vec![
        "ui-heading".to_string(),
        state.level_class.into(),
        state.tone_class.into(),
    ];

    if state.is_truncated {
        classes.push("ui-heading--truncate".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-heading--custom-class".to_string());
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
    fn heading_level_and_tone_contracts_are_stable() {
        assert_eq!(HeadingLevel::H1.class_name(), "ui-heading--level-1");
        assert_eq!(HeadingLevel::H6.class_name(), "ui-heading--level-6");
        assert_eq!(HeadingLevel::H3.as_attr(), "3");

        assert_eq!(
            HeadingTone::Default.class_name(),
            "ui-heading--tone-default"
        );
        assert_eq!(HeadingTone::Strong.class_name(), "ui-heading--tone-strong");
        assert_eq!(HeadingTone::Muted.class_name(), "ui-heading--tone-muted");
        assert_eq!(HeadingTone::Muted.as_attr(), "muted");
    }

    #[test]
    fn normalize_optional_text_trims_and_drops_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n\t  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-heading  ".to_string())),
            Some("docs-heading".to_string())
        );
    }

    #[test]
    fn normalize_aria_label_uses_fallback_when_missing() {
        let (label, custom) = normalize_aria_label(Some("  Dialog Title  ".to_string()));
        assert_eq!(label, "Dialog Title");
        assert!(custom);

        let (label, custom) = normalize_aria_label(Some("  ".to_string()));
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn resolve_state_tracks_level_tone_and_sources() {
        let state = resolve_state(HeadingStateInput {
            level: HeadingLevel::H4,
            tone: HeadingTone::Strong,
            truncate: true,
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.level_attr, "4");
        assert_eq!(state.tone_attr, "strong");
        assert!(state.is_truncated);
        assert_eq!(state.data_state_attr, "truncate");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_custom_marker_and_user_class() {
        let state = resolve_state(HeadingStateInput {
            level: HeadingLevel::H2,
            tone: HeadingTone::Muted,
            truncate: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-heading-custom".to_string()), state);

        for token in [
            "ui-heading",
            "ui-heading--level-2",
            "ui-heading--tone-muted",
            "ui-heading--custom-class",
            "docs-heading-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
