use crate::content::{ContentState, ContentStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Content";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ContentTone {
    #[default]
    Default,
    Muted,
}

impl ContentTone {
    pub fn class_name(self) -> &'static str {
        match self {
            ContentTone::Default => "ui-content--tone-default",
            ContentTone::Muted => "ui-content--tone-muted",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ContentTone::Default => "default",
            ContentTone::Muted => "muted",
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

pub fn resolve_state(input: ContentStateInput) -> ContentState {
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

    let data_state_attr = if input.padded && input.tone == ContentTone::Muted {
        "muted-padded"
    } else if input.padded {
        "padded"
    } else if input.tone == ContentTone::Muted {
        "muted"
    } else {
        "default"
    };

    ContentState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_padded: input.padded,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ContentState) -> String {
    let mut classes = vec!["ui-content".to_string(), state.tone_class.into()];

    if state.is_padded {
        classes.push("ui-content--padded".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-content--custom-class".to_string());
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
    fn content_tone_contract_is_stable() {
        assert_eq!(
            ContentTone::Default.class_name(),
            "ui-content--tone-default"
        );
        assert_eq!(ContentTone::Muted.class_name(), "ui-content--tone-muted");

        assert_eq!(ContentTone::Default.as_attr(), "default");
        assert_eq!(ContentTone::Muted.as_attr(), "muted");
    }

    #[test]
    fn normalize_optional_text_trims_and_drops_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n\t  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-content  ".to_string())),
            Some("docs-content".to_string())
        );
    }

    #[test]
    fn normalize_aria_label_uses_fallback_when_missing() {
        let (label, custom) = normalize_aria_label(Some("  Primary Content  ".to_string()));
        assert_eq!(label, "Primary Content");
        assert!(custom);

        let (label, custom) = normalize_aria_label(Some("  ".to_string()));
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn resolve_state_tracks_flags_and_sources() {
        let state = resolve_state(ContentStateInput {
            tone: ContentTone::Muted,
            padded: true,
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.tone_attr, "muted");
        assert!(state.is_padded);
        assert_eq!(state.data_state_attr, "muted-padded");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_custom_marker_and_user_class() {
        let state = resolve_state(ContentStateInput {
            tone: ContentTone::Default,
            padded: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-content-custom".to_string()), state);

        for token in [
            "ui-content",
            "ui-content--tone-default",
            "ui-content--padded",
            "ui-content--custom-class",
            "docs-content-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
