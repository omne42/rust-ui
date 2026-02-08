use crate::description::{DescriptionState, DescriptionStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Description";
pub const DEFAULT_TEXT: &str = "—";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DescriptionTone {
    #[default]
    Default,
    Muted,
    Negative,
}

impl DescriptionTone {
    pub fn class_name(self) -> &'static str {
        match self {
            DescriptionTone::Default => "ui-description--tone-default",
            DescriptionTone::Muted => "ui-description--tone-muted",
            DescriptionTone::Negative => "ui-description--tone-negative",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            DescriptionTone::Default => "default",
            DescriptionTone::Muted => "muted",
            DescriptionTone::Negative => "negative",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DescriptionElement {
    Span,
    #[default]
    Paragraph,
    Div,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_content(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_TEXT.to_string())
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn resolve_state(input: DescriptionStateInput) -> DescriptionState {
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

    let data_state_attr = if input.disabled {
        "disabled"
    } else if input.truncate {
        "truncate"
    } else {
        "default"
    };

    DescriptionState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_disabled: input.disabled,
        is_truncated: input.truncate,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: DescriptionState) -> String {
    let mut classes = vec!["ui-description".to_string(), state.tone_class.to_string()];

    if state.is_disabled {
        classes.push("ui-description--disabled".to_string());
    }

    if state.is_truncated {
        classes.push("ui-description--truncate".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-description--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::description::DescriptionStateInput;

    #[test]
    fn tone_and_element_contracts_are_stable() {
        assert_eq!(
            DescriptionTone::Default.class_name(),
            "ui-description--tone-default"
        );
        assert_eq!(
            DescriptionTone::Muted.class_name(),
            "ui-description--tone-muted"
        );
        assert_eq!(
            DescriptionTone::Negative.class_name(),
            "ui-description--tone-negative"
        );

        assert_eq!(DescriptionTone::Default.as_attr(), "default");
        assert_eq!(DescriptionTone::Muted.as_attr(), "muted");
        assert_eq!(DescriptionTone::Negative.as_attr(), "negative");

        assert_eq!(DescriptionElement::default(), DescriptionElement::Paragraph);
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  helper  ".to_string())),
            Some("helper".to_string())
        );

        assert_eq!(normalize_content(Some("  hint  ".to_string())), "hint");
        assert_eq!(normalize_content(Some(" \n ".to_string())), DEFAULT_TEXT);

        let (label, custom) = normalize_aria_label(Some("  Form help  ".to_string()));
        assert_eq!(label, "Form help");
        assert!(custom);

        let (label, custom) = normalize_aria_label(None);
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn resolve_state_tracks_sources_and_priority() {
        let state = resolve_state(DescriptionStateInput {
            tone: DescriptionTone::Muted,
            disabled: false,
            truncate: true,
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.tone_attr, "muted");
        assert!(!state.is_disabled);
        assert!(state.is_truncated);
        assert_eq!(state.data_state_attr, "truncate");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-description".to_string()),
            resolve_state(DescriptionStateInput {
                tone: DescriptionTone::Negative,
                disabled: true,
                truncate: true,
                has_custom_aria_label: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-description",
            "ui-description--tone-negative",
            "ui-description--disabled",
            "ui-description--truncate",
            "ui-description--custom-class",
            "docs-description",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should contain `{token}`"
            );
        }
    }
}
