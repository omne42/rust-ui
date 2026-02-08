use crate::text::{TextState, TextStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Text";
pub const DEFAULT_TEXT: &str = "—";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextTone {
    #[default]
    Default,
    Subtle,
    Strong,
}

impl TextTone {
    pub fn class_name(self) -> &'static str {
        match self {
            TextTone::Default => "ui-text--tone-default",
            TextTone::Subtle => "ui-text--tone-subtle",
            TextTone::Strong => "ui-text--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TextTone::Default => "default",
            TextTone::Subtle => "subtle",
            TextTone::Strong => "strong",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Start,
    Center,
    End,
    Justify,
}

impl TextAlign {
    pub fn class_name(self) -> &'static str {
        match self {
            TextAlign::Start => "ui-text--align-start",
            TextAlign::Center => "ui-text--align-center",
            TextAlign::End => "ui-text--align-end",
            TextAlign::Justify => "ui-text--align-justify",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TextAlign::Start => "start",
            TextAlign::Center => "center",
            TextAlign::End => "end",
            TextAlign::Justify => "justify",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextWeight {
    #[default]
    Regular,
    Medium,
    Semibold,
    Bold,
}

impl TextWeight {
    pub fn class_name(self) -> &'static str {
        match self {
            TextWeight::Regular => "ui-text--weight-regular",
            TextWeight::Medium => "ui-text--weight-medium",
            TextWeight::Semibold => "ui-text--weight-semibold",
            TextWeight::Bold => "ui-text--weight-bold",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TextWeight::Regular => "regular",
            TextWeight::Medium => "medium",
            TextWeight::Semibold => "semibold",
            TextWeight::Bold => "bold",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextElement {
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

pub fn resolve_state(input: TextStateInput) -> TextState {
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

    TextState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        align: input.align,
        align_class: input.align.class_name(),
        align_attr: input.align.as_attr(),
        weight: input.weight,
        weight_class: input.weight.class_name(),
        weight_attr: input.weight.as_attr(),
        is_disabled: input.disabled,
        is_truncated: input.truncate,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: TextState) -> String {
    let mut classes = vec![
        "ui-text".to_string(),
        state.tone_class.to_string(),
        state.align_class.to_string(),
        state.weight_class.to_string(),
    ];

    if state.is_disabled {
        classes.push("ui-text--disabled".to_string());
    }
    if state.is_truncated {
        classes.push("ui-text--truncate".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-text--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::TextStateInput;

    #[test]
    fn class_and_attr_contracts_are_stable() {
        assert_eq!(TextTone::Default.class_name(), "ui-text--tone-default");
        assert_eq!(TextAlign::Center.class_name(), "ui-text--align-center");
        assert_eq!(TextWeight::Bold.class_name(), "ui-text--weight-bold");

        assert_eq!(TextTone::Subtle.as_attr(), "subtle");
        assert_eq!(TextAlign::Justify.as_attr(), "justify");
        assert_eq!(TextWeight::Medium.as_attr(), "medium");
    }

    #[test]
    fn normalization_helpers_use_defaults() {
        assert_eq!(normalize_content(Some("  hello  ".to_string())), "hello");
        assert_eq!(normalize_content(Some("   ".to_string())), DEFAULT_TEXT);

        let (aria, is_custom) = normalize_aria_label(None);
        assert_eq!(aria, DEFAULT_ARIA_LABEL);
        assert!(!is_custom);
    }

    #[test]
    fn resolve_state_tracks_sources_and_flags() {
        let state = resolve_state(TextStateInput {
            tone: TextTone::Strong,
            align: TextAlign::End,
            weight: TextWeight::Semibold,
            disabled: false,
            truncate: true,
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.tone_attr, "strong");
        assert_eq!(state.align_attr, "end");
        assert_eq!(state.weight_attr, "semibold");
        assert_eq!(state.data_state_attr, "truncate");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_markers() {
        let class_name = compose_class_name(
            Some("docs-text".to_string()),
            resolve_state(TextStateInput {
                tone: TextTone::Subtle,
                align: TextAlign::Center,
                weight: TextWeight::Bold,
                disabled: true,
                truncate: true,
                has_custom_aria_label: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-text",
            "ui-text--tone-subtle",
            "ui-text--align-center",
            "ui-text--weight-bold",
            "ui-text--disabled",
            "ui-text--truncate",
            "ui-text--custom-class",
            "docs-text",
        ] {
            assert!(class_name.contains(token), "class should include `{token}`");
        }
    }
}
