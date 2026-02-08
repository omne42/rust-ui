use crate::keyboard::{KeyboardState, KeyboardStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Keyboard";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum KeyboardTone {
    #[default]
    Default,
    Muted,
}

impl KeyboardTone {
    pub fn class_name(self) -> &'static str {
        match self {
            KeyboardTone::Default => "ui-keyboard--tone-default",
            KeyboardTone::Muted => "ui-keyboard--tone-muted",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            KeyboardTone::Default => "default",
            KeyboardTone::Muted => "muted",
        }
    }
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

pub fn resolve_state(input: KeyboardStateInput) -> KeyboardState {
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

    let data_state_attr = if input.compact {
        "compact"
    } else if input.tone == KeyboardTone::Muted {
        "muted"
    } else {
        "default"
    };

    KeyboardState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_compact: input.compact,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: KeyboardState) -> String {
    let mut classes = vec!["ui-keyboard".to_string(), state.tone_class.to_string()];

    if state.is_compact {
        classes.push("ui-keyboard--compact".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-keyboard--custom-class".to_string());
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
    fn keyboard_tone_contract_is_stable() {
        assert_eq!(
            KeyboardTone::Default.class_name(),
            "ui-keyboard--tone-default"
        );
        assert_eq!(KeyboardTone::Muted.class_name(), "ui-keyboard--tone-muted");

        assert_eq!(KeyboardTone::Default.as_attr(), "default");
        assert_eq!(KeyboardTone::Muted.as_attr(), "muted");
    }

    #[test]
    fn normalize_optional_text_trims_and_drops_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n\t  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-keyboard  ".to_string())),
            Some("docs-keyboard".to_string())
        );
    }

    #[test]
    fn normalize_aria_label_uses_fallback_when_missing() {
        let (label, custom) = normalize_aria_label(Some("  Keyboard Command  ".to_string()));
        assert_eq!(label, "Keyboard Command");
        assert!(custom);

        let (label, custom) = normalize_aria_label(Some("  ".to_string()));
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn resolve_state_tracks_tone_compact_and_sources() {
        let state = resolve_state(KeyboardStateInput {
            tone: KeyboardTone::Muted,
            compact: true,
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.tone_attr, "muted");
        assert!(state.is_compact);
        assert_eq!(state.data_state_attr, "compact");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_custom_marker_and_user_class() {
        let state = resolve_state(KeyboardStateInput {
            tone: KeyboardTone::Default,
            compact: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-keyboard-custom".to_string()), state);

        for token in [
            "ui-keyboard",
            "ui-keyboard--tone-default",
            "ui-keyboard--compact",
            "ui-keyboard--custom-class",
            "docs-keyboard-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
