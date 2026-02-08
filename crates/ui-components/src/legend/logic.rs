use crate::legend::{LegendState, LegendStateInput};

pub const DEFAULT_TEXT: &str = "Group";
pub const DEFAULT_REQUIRED_INDICATOR: &str = "*";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LegendTone {
    #[default]
    Default,
    Muted,
    Strong,
}

impl LegendTone {
    pub fn class_name(self) -> &'static str {
        match self {
            LegendTone::Default => "ui-legend--tone-default",
            LegendTone::Muted => "ui-legend--tone-muted",
            LegendTone::Strong => "ui-legend--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            LegendTone::Default => "default",
            LegendTone::Muted => "muted",
            LegendTone::Strong => "strong",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_text(value: Option<String>) -> (String, bool) {
    if let Some(text) = normalize_optional_text(value) {
        (text, true)
    } else {
        (DEFAULT_TEXT.to_string(), false)
    }
}

pub fn normalize_required_indicator(value: Option<String>) -> (String, bool) {
    if let Some(indicator) = normalize_optional_text(value) {
        (indicator, true)
    } else {
        (DEFAULT_REQUIRED_INDICATOR.to_string(), false)
    }
}

pub fn resolve_state(input: LegendStateInput) -> LegendState {
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

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    LegendState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        is_required: input.required,
        is_optional: !input.required,
        is_disabled: input.disabled,
        is_enabled: !input.disabled,
        has_custom_text: input.has_custom_text,
        has_custom_indicator: input.has_custom_indicator,
        has_custom_class_name: input.has_custom_class_name,
        text_source_attr,
        indicator_source_attr,
        class_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: LegendState) -> String {
    let mut classes = vec!["ui-legend".to_string(), state.tone_class.to_string()];

    if state.is_required {
        classes.push("ui-legend--required".to_string());
    }

    if state.is_disabled {
        classes.push("ui-legend--disabled".to_string());
    }

    if state.has_custom_text {
        classes.push("ui-legend--text-custom".to_string());
    }

    if state.has_custom_indicator {
        classes.push("ui-legend--indicator-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-legend--custom-class".to_string());
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
    fn tone_class_names_and_attrs_are_stable() {
        assert_eq!(LegendTone::Default.class_name(), "ui-legend--tone-default");
        assert_eq!(LegendTone::Muted.class_name(), "ui-legend--tone-muted");
        assert_eq!(LegendTone::Strong.class_name(), "ui-legend--tone-strong");

        assert_eq!(LegendTone::Default.as_attr(), "default");
        assert_eq!(LegendTone::Muted.as_attr(), "muted");
        assert_eq!(LegendTone::Strong.as_attr(), "strong");
    }

    #[test]
    fn normalize_helpers_fallback_to_defaults() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Preferences  ".to_string())),
            Some("Preferences".to_string())
        );

        assert_eq!(
            normalize_text(Some("  Notification settings  ".to_string())),
            ("Notification settings".to_string(), true)
        );
        assert_eq!(normalize_text(None), (DEFAULT_TEXT.to_string(), false));

        assert_eq!(
            normalize_required_indicator(Some("  (required)  ".to_string())),
            ("(required)".to_string(), true)
        );
        assert_eq!(
            normalize_required_indicator(None),
            (DEFAULT_REQUIRED_INDICATOR.to_string(), false)
        );
    }

    #[test]
    fn resolve_state_tracks_required_disabled_and_sources() {
        let state = resolve_state(LegendStateInput {
            tone: LegendTone::Strong,
            required: true,
            disabled: true,
            has_custom_text: true,
            has_custom_indicator: false,
            has_custom_class_name: true,
        });

        assert_eq!(state.tone_attr, "strong");
        assert!(state.is_required);
        assert!(!state.is_optional);
        assert!(state.is_disabled);
        assert_eq!(state.text_source_attr, "custom");
        assert_eq!(state.indicator_source_attr, "default");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(LegendStateInput {
            tone: LegendTone::Muted,
            required: true,
            disabled: false,
            has_custom_text: false,
            has_custom_indicator: true,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-legend".to_string()), state);
        for token in [
            "ui-legend",
            "ui-legend--tone-muted",
            "ui-legend--required",
            "ui-legend--indicator-custom",
            "ui-legend--custom-class",
            "docs-legend",
        ] {
            assert!(class_name.contains(token), "class should contain `{token}`");
        }
    }
}
